//! The Game Hub and the trophy overview in the panel host: gather what the pages show from the
//! model (trophies, play time, captures, friends, Play Together, run reports, music), hand it to
//! the screens as words and pictures and carry out what they ask for. The screens never touch the
//! model.

use std::time::{SystemTime, UNIX_EPOCH};

use anyps5_core::captures::{self as core_captures, Capture};
use anyps5_core::play_together;
use anyps5_core::trophies::{Grade as CoreGrade, TrophySet};

use super::PanelHost;
use crate::app::captures::Who;
use crate::app::captures::images::{Key, Kind as ImageKind};
use crate::app::model::GameState;
use crate::app::overlay::{Overlay, OverlayCx};
use crate::screens::captures::Thumb;
use crate::screens::format;
use crate::screens::home::primary_look;
use crate::screens::hub::{Block, CaptureBlock, HubAction, HubData, THUMBS, TrophyBlock};
use crate::screens::hub_strings::HubText as H;
use crate::screens::online::ago_text;
use crate::screens::overview::{OverviewAction, OverviewData, OverviewRow};
use crate::screens::profile::parse_stamp;
use crate::screens::run_strings::RunText;
use crate::screens::strings::playtime_text;
use crate::screens::{is_plain_ready, state_label};
use crate::ui::chrome::TileImage;
use crate::ui::i18n::{Arg, Language};

/// The size game icons are asked for: the one the library grid has already loaded.
const GAME_ICON: u32 = 256;
/// "This week": the last seven days, today included.
const WEEK_DAYS: i64 = 7;

fn now_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64)
}

/// The place of a grade in the counts: platinum first.
fn grade_slot(grade: CoreGrade) -> usize {
    match grade {
        CoreGrade::Platinum => 0,
        CoreGrade::Gold => 1,
        CoreGrade::Silver => 2,
        CoreGrade::Bronze => 3,
    }
}

/// What a game's trophy list says: earned, all of them, by grade, and the newest one earned as
/// `(name, grade, unix seconds)`. Only the game's own set counts, as in the profile.
pub(super) struct TrophyFacts {
    pub earned: usize,
    pub total: usize,
    pub grades: [u32; 4],
    pub latest: Option<(String, CoreGrade, i64)>,
}

pub(super) fn trophy_facts(set: &TrophySet) -> TrophyFacts {
    let main = || set.trophies.iter().filter(|t| t.service_label == 0);
    let mut grades = [0u32; 4];
    let mut latest: Option<(String, CoreGrade, i64)> = None;
    for trophy in main().filter(|t| t.is_unlocked()) {
        grades[grade_slot(trophy.grade)] += 1;
        let at = trophy.unlocked_secs().unwrap_or(0);
        if latest.as_ref().is_none_or(|(_, _, newest)| at > *newest) {
            latest = Some((trophy.name.clone(), trophy.grade, at));
        }
    }
    TrophyFacts {
        earned: main().filter(|t| t.is_unlocked()).count(),
        total: main().count(),
        grades,
        latest,
    }
}

fn when(at: i64, now: i64, language: Language) -> String {
    ago_text(
        now,
        u64::try_from(at).unwrap_or(0),
        format::local_offset_seconds(at.saturating_mul(1_000_000)),
        language,
    )
}

impl PanelHost {
    pub(super) fn hub_ui(&mut self, cx: &mut OverlayCx<'_>, wanted: bool) {
        let folder = cx.model.prefs.screenshots_dir(&cx.model.paths);
        let Some(game) = cx.model.selected_game().cloned() else {
            if wanted {
                cx.model.dismiss_overlay();
            }
            if self.hub.is_open() {
                self.hub.close();
            }
            return;
        };
        if wanted && !self.hub.is_open() {
            self.hub.open();
            self.hub_service.begin(&folder);
            cx.model
                .online
                .request_runs(&game.title_id, &game.version, false);
        } else if !wanted && self.hub.is_open() {
            self.hub.close();
        }
        let events = self.take_events(wanted);
        if !self.hub.is_active() {
            if self.hub_service.images.bytes() > 0 || !self.hub_service.captures().is_empty() {
                self.hub_service.release();
            }
            return;
        }
        cx.model.read_trophies_of(std::slice::from_ref(&game.id));
        self.hub_service.poll();
        self.hub_service.images.poll();

        let language = cx.model.language();
        let accent = cx.shell.accent_color();
        let now = now_seconds();
        let state = cx.model.state_of(&game);

        // The art and the line under the name.
        let art = game.icon.as_deref().and_then(|path| {
            cx.textures.get(path, GAME_ICON).map(|loaded| TileImage {
                texture: loaded.id,
                aspect: loaded.aspect,
            })
        });
        let mut subtitle = vec![];
        if !game.version.is_empty() {
            subtitle.push(game.version.clone());
        }
        if !is_plain_ready(&state) {
            subtitle.push(state_label(&state, language).text);
        }
        let (primary_title, primary_icon, _) = primary_look(&state, language);

        // Trophies.
        let set = cx.model.trophies.get(&game.id);
        let (trophies, has_trophy_list) = match set.map(trophy_facts).filter(|f| f.total > 0) {
            Some(f) => {
                let note = match &f.latest {
                    Some((name, grade, _)) => H::LatestTrophy.format(
                        language,
                        &[
                            Arg::Str(name),
                            Arg::Str(crate::screens::convert::grade(*grade).title(language)),
                        ],
                    ),
                    None => H::NoneEarned.text(language).to_owned(),
                };
                (
                    TrophyBlock {
                        figure: H::TrophyCount.format(
                            language,
                            &[Arg::Int(f.earned as i64), Arg::Int(f.total as i64)],
                        ),
                        fraction: Some(f.earned as f32 / f.total as f32),
                        grades: f.grades,
                        note,
                    },
                    true,
                )
            }
            None => (
                TrophyBlock {
                    figure: String::new(),
                    fraction: None,
                    grades: [0; 4],
                    note: H::NoTrophyList.text(language).to_owned(),
                },
                false,
            ),
        };

        // Play time: the total, this week and the last time.
        let played = cx.model.prefs.playtime(&game.id);
        let time = if played.is_played() {
            let week = cx.model.prefs.playtime_recent(&game.id, WEEK_DAYS, now);
            let mut lines = Vec::new();
            if week >= 60 {
                lines.push(
                    H::ThisWeek.format(language, &[Arg::Str(&playtime_text(week, language))]),
                );
            }
            if let Some(at) = cx
                .model
                .prefs
                .last_played
                .get(game.id.as_str())
                .and_then(|stamp| parse_stamp(stamp))
            {
                lines.push(H::LastPlayed.format(language, &[Arg::Str(&when(at, now, language))]));
            }
            Block::value(
                playtime_text(played.seconds, language),
                (!lines.is_empty()).then(|| lines.join("\n")),
            )
        } else {
            Block::note(H::NeverPlayed.text(language))
        };

        // Friends who play it now.
        let names: Vec<String> = cx
            .model
            .online
            .friends_playing(&game.title_id)
            .into_iter()
            .map(str::to_owned)
            .collect();
        let friends = if !names.is_empty() {
            Block::note(H::FriendsPlaying.format(language, &[Arg::Str(&names.join(", "))]))
        } else if cx.model.online.is_signed_in() {
            Block::note(H::NoFriendsPlaying.text(language))
        } else {
            Block::note(H::FriendsSignedOut.text(language))
        };

        // Play Together, from the table of games that can be played together.
        let together = match play_together::lan_title(&game.title_id) {
            Some(lan) => Block::value(
                H::TogetherUpTo.format(language, &[Arg::Int(i64::from(lan.max_players))]),
                None,
            ),
            None => Block::note(H::TogetherNone.text(language)),
        };

        // What players found, once enough of them said.
        let compat = match cx.model.online.runs.verdict(&game.title_id, &game.version) {
            Some((verdict, reports)) => Block::note(RunText::Verdict.format(
                language,
                &[
                    Arg::Str(RunText::of(verdict).text(language)),
                    Arg::Int(i64::from(reports)),
                ],
            )),
            None if cx.model.online.is_online() => Block::note(H::CompatNone.text(language)),
            None => Block::note(H::CompatOffline.text(language)),
        };

        // The game's own music, when the game has one.
        let music = match (
            &game.music,
            cx.model.prefs.game_music && cx.model.prefs.music,
        ) {
            (None, _) => Block::note(H::MusicNone.text(language)),
            (Some(_), true) => Block::value(H::MusicOn.text(language), None),
            (Some(_), false) => Block::value(H::MusicOff.text(language), None),
        };

        // This game's captures, the newest first.
        let games = &cx.model.games;
        let (list, grouped, images, _) = self.hub_service.frame(games, core_captures::Filter::All);
        let place = games.iter().position(|g| g.id == game.id);
        let mine: Vec<usize> = grouped
            .iter()
            .find(|(who, _)| matches!(who, Who::Game(i) if Some(*i) == place))
            .map(|(_, items)| items.clone())
            .unwrap_or_default();
        let thumbs: Vec<Option<TileImage>> = mine
            .iter()
            .take(THUMBS)
            .map(|i| {
                let capture: &Capture = &list[*i];
                let kind = match capture.kind {
                    core_captures::Kind::Screenshot => ImageKind::Thumb,
                    core_captures::Kind::Clip => ImageKind::Clip,
                };
                match images.get(Key {
                    path: capture.path.clone(),
                    kind,
                }) {
                    Thumb::Ready(image) => Some(image),
                    Thumb::Pending | Thumb::Unavailable => None,
                }
            })
            .collect();
        let captures = CaptureBlock {
            count: match mine.len() {
                0 => None,
                1 => Some(H::CaptureOne.text(language).to_owned()),
                n => Some(H::CaptureCount.format(language, &[Arg::Int(n as i64)])),
            },
            thumbs,
            empty: H::NoCaptures.text(language).to_owned(),
        };

        let data = HubData {
            language,
            accent,
            name: &game.name,
            subtitle: subtitle.join(" \u{b7} "),
            art,
            primary_title: primary_title.to_owned(),
            primary_icon,
            primary_ready: !matches!(state, GameState::Starting),
            trophies,
            has_trophy_list,
            time,
            captures,
            friends,
            together,
            compat,
            music,
        };
        let actions = self.hub.show(cx.shell, cx.ctx, &events, &data);
        self.hub_service.images.end_frame();
        for action in actions {
            let model = &mut *cx.model;
            match action {
                HubAction::Close => model.close_hub(),
                HubAction::Primary => {
                    model.primary_action();
                    if !matches!(state, GameState::Starting) {
                        model.dismiss_overlay();
                    }
                }
                HubAction::OpenTrophies => model.open(Overlay::Trophies),
                HubAction::OpenOverview => model.open_overview(Some(Overlay::GameHub)),
                HubAction::OpenCaptures => model.open(Overlay::Captures),
            }
        }
        if !self.hub.is_active() {
            self.hub_service.release();
        }
    }

    pub(super) fn overview_ui(&mut self, cx: &mut OverlayCx<'_>, wanted: bool) {
        if wanted && !self.overview.is_open() {
            self.overview.open();
        } else if !wanted && self.overview.is_open() {
            self.overview.close();
        }
        let events = self.take_events(wanted);
        if !self.overview.is_active() {
            return;
        }
        // Every game's trophy file is read once, off the UI thread.
        let ids: Vec<_> = cx.model.games.iter().map(|g| g.id.clone()).collect();
        cx.model.read_trophies_of(&ids);
        let language = cx.model.language();
        let accent = cx.shell.accent_color();
        let mut entries: Vec<(Option<i64>, OverviewRow)> = Vec::new();
        let mut earned_all = 0usize;
        let mut loading = false;
        for game in &cx.model.games {
            let Some(set) = cx.model.trophies.get(&game.id) else {
                loading = true;
                continue;
            };
            let facts = trophy_facts(set);
            if facts.total == 0 {
                continue;
            }
            earned_all += facts.earned;
            let art = game.icon.as_deref().and_then(|path| {
                cx.textures.get(path, GAME_ICON).map(|loaded| TileImage {
                    texture: loaded.id,
                    aspect: loaded.aspect,
                })
            });
            let latest_at = facts.latest.as_ref().map(|(_, _, at)| *at);
            let latest = match latest_at {
                Some(at) => H::OverviewLatest.format(
                    language,
                    &[Arg::Str(&format::short_date(
                        at,
                        format::local_offset_seconds(at.saturating_mul(1_000_000)),
                        language,
                    ))],
                ),
                None => H::OverviewNothingYet.text(language).to_owned(),
            };
            let figure_earned = Arg::Int(facts.earned as i64);
            let label = H::OverviewRowName.format(
                language,
                &[
                    Arg::Str(&game.name),
                    figure_earned,
                    Arg::Int(facts.total as i64),
                    Arg::Str(&latest),
                ],
            );
            entries.push((
                latest_at,
                OverviewRow {
                    title_id: game.title_id.clone(),
                    name: game.name.clone(),
                    art,
                    earned: facts.earned,
                    total: facts.total,
                    fraction: facts.earned as f32 / facts.total as f32,
                    grades: facts.grades,
                    latest,
                    label,
                },
            ));
        }
        // The game with the newest trophy first, then the ones with none by name.
        entries.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.name.cmp(&b.1.name)));
        let rows: Vec<OverviewRow> = entries.into_iter().map(|(_, row)| row).collect();
        let summary = if rows.is_empty() {
            String::new()
        } else {
            H::OverviewSummary.format(
                language,
                &[Arg::Int(earned_all as i64), Arg::Int(rows.len() as i64)],
            )
        };
        let data = OverviewData {
            language,
            accent,
            summary,
            loading: loading && rows.is_empty(),
            rows: &rows,
        };
        let actions = self.overview.show(cx.shell, cx.ctx, &events, &data);
        for action in actions {
            match action {
                OverviewAction::Close => cx.model.close_overview(),
                OverviewAction::OpenGame(title_id) => {
                    cx.model
                        .open_hub_of(&title_id, Some(Overlay::TrophyOverview));
                }
            }
        }
    }
}
