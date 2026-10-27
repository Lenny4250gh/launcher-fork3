//! "Played With Recently" (docs/DESIGN.md 17): the page of the Friends section that lists the
//! people the player shared a party or a session with, newest first. Each person is one row with
//! the actions on them at its end (the way a friend's row has its own): Add Friend while they
//! are not a friend, View Profile and Block. The page ends with the switch for recording new
//! people and the question that clears the list. Nothing here is shown to anyone else.

use anyps5_core::online::api::PlayedWith;
use anyps5_core::online::played_with::Standing;

use super::{OnlineAction, OnlineData, OnlinePanel, Page, ago_text};
use crate::screens::format;
use crate::screens::online_feed_strings::FeedText as F;
use crate::screens::online_strings::OnlineText as T;
use crate::screens::played_with_strings::PlayedWithText as PW;
use crate::screens::rows::{Change, Row};
use crate::ui::i18n::{Arg, Language};
use crate::ui::icons::Icon;
use crate::ui::state_page::{self, StatePage};
use crate::ui::theme::space;

/// What a person's row can do, in the order of its buttons.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PlayedAct {
    AddFriend,
    Profile,
    Block,
}

impl PlayedAct {
    fn label(self, language: Language) -> &'static str {
        match self {
            PlayedAct::AddFriend => PW::AddFriend.text(language),
            PlayedAct::Profile => F::ViewProfile.text(language),
            PlayedAct::Block => T::Block.text(language),
        }
    }
}

/// The buttons of a person's row: adding only while they are not a friend and no request is out.
pub(super) fn played_actions(standing: Standing) -> Vec<PlayedAct> {
    let mut acts = Vec::new();
    if standing == Standing::Open {
        acts.push(PlayedAct::AddFriend);
    }
    acts.push(PlayedAct::Profile);
    acts.push(PlayedAct::Block);
    acts
}

impl OnlinePanel {
    /// Where a person stands: the cloud's word, the friends the launcher holds, the requests out.
    fn played_standing(data: &OnlineData<'_>, person: &PlayedWith) -> Standing {
        let online = data.online;
        let id = person.account.id;
        let request_out = online.friends.outgoing.iter().any(|r| r.account.id == id);
        online
            .played_with
            .standing(person, online.friend(id).is_some(), request_out)
    }

    /// The line under a person's name: the mark, what they played, and when.
    fn played_detail(data: &OnlineData<'_>, person: &PlayedWith, standing: Standing) -> String {
        let language = data.language;
        let mut parts: Vec<String> = Vec::new();
        match standing {
            Standing::Friend => parts.push(PW::Friend.text(language).to_owned()),
            Standing::Requested => parts.push(PW::RequestSent.text(language).to_owned()),
            Standing::Open => {}
        }
        // The title is data from the cloud, shown as it came.
        if let Some(title) = person.title.as_deref().filter(|t| !t.trim().is_empty()) {
            parts.push(PW::Played.format(language, &[Arg::Str(title)]));
        }
        let offset = data.utc_offset.unwrap_or_else(|| {
            format::local_offset_seconds(
                i64::try_from(person.last)
                    .unwrap_or(0)
                    .saturating_mul(1_000_000),
            )
        });
        parts.push(ago_text(data.now, person.last, offset, language));
        parts.join(" · ")
    }

    /// The cloud is known to be out (still connecting is not offline).
    fn played_offline(online: &anyps5_core::online::Online) -> bool {
        !online.is_online() && online.cloud == Some(anyps5_core::online::Cloud::Offline)
    }

    /// The state the page shows in place of the list: loading, failed, or nothing yet. `None`
    /// while there are people to list.
    pub(super) fn played_state(
        &self,
        data: &OnlineData<'_>,
    ) -> Option<(Icon, &'static str, Option<&'static str>)> {
        let (online, language) = (data.online, data.language);
        let state = &online.played_with;
        if state.count() > 0 {
            None
        } else if state.list.is_none() && state.failed {
            Some((
                Icon::WARNING,
                PW::FailedTitle.text(language),
                Some(PW::FailedNote.text(language)),
            ))
        } else if state.list.is_none() && Self::played_offline(online) {
            // The cloud is out, so no answer is coming: say so instead of loading for ever.
            Some((Icon::CLOUD, F::Offline.text(language), None))
        } else if state.list.is_none() {
            Some((
                Icon::ARROWS_CLOCKWISE,
                PW::LoadingTitle.text(language),
                None,
            ))
        } else {
            Some((
                Icon::CLOCK_COUNTER_CLOCKWISE,
                PW::EmptyTitle.text(language),
                Some(PW::EmptyNote.text(language)),
            ))
        }
    }

    /// The rows of the page: the people, then the switch, the privacy note and Clear the List.
    pub(super) fn played_rows(&self, data: &OnlineData<'_>, rows: &mut Vec<Row>) {
        let (online, language) = (data.online, data.language);
        let state = &online.played_with;
        for person in state.people() {
            let standing = Self::played_standing(data, person);
            let acts = played_actions(standing);
            let labels: Vec<String> = acts.iter().map(|a| a.label(language).to_owned()).collect();
            let selected = self
                .played_action
                .unwrap_or(0)
                .min(labels.len().saturating_sub(1));
            rows.push(
                Row::buttons("playedRow", person.account.handle.clone(), labels, selected)
                    .avatar(self.avatar_of(data, &person.account, None))
                    .detail(Self::played_detail(data, person, standing))
                    .reveal(),
            );
        }
        if state.list.is_none() {
            // Not loaded: a failure can be tried again, and nothing else is on offer.
            if state.failed || Self::played_offline(online) {
                rows.push(
                    Row::command("playedRetry", F::TryAgain.text(language))
                        .icon(Icon::ARROWS_CLOCKWISE),
                );
            }
            return;
        }
        let mut remember = Row::toggle(
            "rememberPlayed",
            PW::Remember.text(language),
            state.remember(),
        )
        .icon(Icon::CLOCK_COUNTER_CLOCKWISE)
        .detail(PW::RememberDetail.text(language));
        remember.group_start = !rows.is_empty();
        rows.push(remember);
        if !state.remember() {
            rows.push(Row::info(
                "rememberOff",
                PW::RememberOffNote.text(language),
                None,
            ));
        }
        rows.push(Row::info(
            "playedPrivate",
            PW::PrivateNote.text(language),
            None,
        ));
        if state.count() > 0 {
            rows.push(
                Row::command("clearPlayed", PW::ClearList.text(language))
                    .icon(Icon::TRASH)
                    .destructive()
                    .group_start(),
            );
        }
    }

    /// The row of the Friends page that leads here.
    pub(super) fn played_home_row(&self, data: &OnlineData<'_>) -> Option<Row> {
        let (online, language) = (data.online, data.language);
        let count = online.played_with.count();
        // A player with no friends and nobody on the list sees the page that says how to add one,
        // not another row.
        if !online.is_online() || (online.friends.friends.is_empty() && count == 0) {
            return None;
        }
        let mut row =
            Row::action("playedWith", PW::Title.text(language)).icon(Icon::CLOCK_COUNTER_CLOCKWISE);
        match count {
            0 => {}
            1 => row = row.detail(PW::PeopleOne.text(language)),
            n => row = row.detail(PW::PeopleMany.format(language, &[Arg::Int(n as i64)])),
        }
        Some(row)
    }

    /// The page's header, in the state pages' look, when the list has no people.
    pub(super) fn played_header(&self, ui: &mut egui::Ui, data: &OnlineData<'_>) {
        if let Some((icon, title, note)) = self.played_state(data) {
            // Only as tall as the block needs: the switch and the notes stand right under it.
            let page = StatePage {
                icon,
                title,
                note,
                loading: icon == Icon::ARROWS_CLOCKWISE,
                action: None,
            };
            let height = state_page::height(ui.painter(), &page) + 2.0 * space::XL;
            let (room, _) = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), height),
                egui::Sense::hover(),
            );
            state_page::show(ui, room, &page, false);
        }
    }

    /// A switch or a choice on the page. `true` when it was one of ours.
    pub(super) fn played_change(
        &mut self,
        change: &Change,
        data: &OnlineData<'_>,
        actions: &mut Vec<OnlineAction>,
    ) -> bool {
        match change {
            Change::Toggle {
                row: "rememberPlayed",
                on,
            } => actions.push(OnlineAction::PlayedWithRemember(*on)),
            Change::Choice {
                row: "playedRow",
                index,
            } => self.played_action = Some(*index),
            Change::Press {
                row: "playedRow",
                index,
            } => {
                let item = self.occurrence(data, "playedRow");
                if let Some(person) = data.online.played_with.people().nth(item) {
                    let id = person.account.id;
                    let handle = person.account.handle.clone();
                    let standing = Self::played_standing(data, person);
                    match played_actions(standing).get(*index).copied() {
                        Some(PlayedAct::AddFriend) => {
                            actions.push(OnlineAction::AddFriend(handle));
                        }
                        Some(PlayedAct::Profile) => self.open_profile(id),
                        Some(PlayedAct::Block) => self.ask(super::Ask::Block(id), data),
                        None => {}
                    }
                }
            }
            _ => return false,
        }
        true
    }

    /// A command row of the page ran. `false` for any other row.
    pub(super) fn played_run(
        &mut self,
        row: &str,
        data: &OnlineData<'_>,
        actions: &mut Vec<OnlineAction>,
    ) -> bool {
        match row {
            "playedWith" => self.go(Page::PlayedWith, data),
            "playedRetry" => actions.push(OnlineAction::PlayedWithRefresh(true)),
            "clearPlayed" => self.ask(super::Ask::ClearPlayedWith, data),
            _ => return false,
        }
        true
    }
}
