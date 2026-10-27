//! What the library page and the home rows show of the games: how they are ordered and filtered,
//! what players found out about each one, and the shelves of the home screen. Pure reading of the
//! model, no drawing.

use anyps5_core::online::api::{AccountId, Verdict};
use anyps5_core::settings::NEW_GAMES_KEPT;
use anyps5_core::{library::Game, paths::GameId, play_together};

use crate::app::model::{GameState, Model, SizeState};
use crate::screens::library_strings::LibraryText;
use crate::ui::focus::HomeFocus;
use crate::ui::i18n::Arg;
use crate::ui::icons::Icon;
use crate::ui::sound::Cue;

/// How the library page orders its games. The code is what the preferences keep.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Sort {
    Recent,
    /// The order of the tile row on the home screen.
    #[default]
    Name,
    Size,
    Status,
}

impl Sort {
    pub const ALL: [Sort; 4] = [Sort::Name, Sort::Recent, Sort::Size, Sort::Status];

    pub fn code(self) -> &'static str {
        match self {
            Sort::Recent => "recent",
            Sort::Name => "name",
            Sort::Size => "size",
            Sort::Status => "status",
        }
    }

    /// An unknown or empty code is the default.
    pub fn from_code(code: &str) -> Sort {
        Sort::ALL
            .into_iter()
            .find(|s| s.code() == code)
            .unwrap_or_default()
    }

    pub fn next(self) -> Sort {
        let i = Sort::ALL.iter().position(|s| *s == self).unwrap_or(0);
        Sort::ALL[(i + 1) % Sort::ALL.len()]
    }
}

/// Which games the library page shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Filter {
    #[default]
    All,
    Ready,
    NeedsUnlock,
    Together,
}

impl Filter {
    pub const ALL: [Filter; 4] = [
        Filter::All,
        Filter::Ready,
        Filter::NeedsUnlock,
        Filter::Together,
    ];

    pub fn code(self) -> &'static str {
        match self {
            Filter::All => "all",
            Filter::Ready => "ready",
            Filter::NeedsUnlock => "locked",
            Filter::Together => "together",
        }
    }

    pub fn from_code(code: &str) -> Filter {
        Filter::ALL
            .into_iter()
            .find(|f| f.code() == code)
            .unwrap_or_default()
    }

    pub fn next(self) -> Filter {
        let i = Filter::ALL.iter().position(|f| *f == self).unwrap_or(0);
        Filter::ALL[(i + 1) % Filter::ALL.len()]
    }
}

/// One thing the player may want to know about a game before starting it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compat {
    RunsWell,
    RunsWithIssues,
    NoStart,
    /// The game finds others on the LAN, so it can be played together.
    Lan,
}

/// What players found, as a library line. `None` below the number of reports the cloud needs.
pub fn verdict_compat(verdict: Verdict) -> Compat {
    match verdict {
        Verdict::Playable => Compat::RunsWell,
        Verdict::Menus | Verdict::InGame => Compat::RunsWithIssues,
        Verdict::NoStart => Compat::NoStart,
    }
}

/// The tags of a game, most important first: the verdict, then LAN. "Unknown" is no tag.
pub fn compat_of(model: &Model, game: &Game) -> Vec<Compat> {
    let mut tags = Vec::new();
    if let Some((verdict, _)) = model.online.runs.verdict(&game.title_id, &game.version) {
        tags.push(verdict_compat(verdict));
    }
    if play_together::lan_title(&game.title_id).is_some() {
        tags.push(Compat::Lan);
    }
    tags
}

/// How near a state is to being played: ready first, then work under way, then locked.
fn status_rank(state: &GameState) -> u8 {
    match state {
        GameState::Running | GameState::Starting => 0,
        GameState::Installed { .. } => 1,
        GameState::Installing { .. }
        | GameState::Uninstalling { .. }
        | GameState::Cancelling { .. }
        | GameState::Preparing { .. }
        | GameState::Queued => 2,
        GameState::Locked => 3,
        GameState::Failed(_) => 4,
        GameState::Unreadable => 5,
    }
}

fn matches_filter(model: &Model, game: &Game, filter: Filter) -> bool {
    match filter {
        Filter::All => true,
        Filter::Ready => matches!(
            model.state_of(game),
            GameState::Installed { .. } | GameState::Running | GameState::Starting
        ),
        Filter::NeedsUnlock => matches!(
            model.state_of(game),
            GameState::Locked | GameState::Failed(_)
        ),
        Filter::Together => play_together::lan_title(&game.title_id).is_some(),
    }
}

/// The text a query is matched against, in lower case.
fn matches_query(game: &Game, query: &str) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty()
        || game.name.to_lowercase().contains(&query)
        || game.title_id.to_lowercase().contains(&query)
}

/// The stamp of the last play, empty when the game was never played.
fn last_played<'a>(model: &'a Model, game: &Game) -> &'a str {
    model
        .prefs
        .last_played
        .get(game.id.as_str())
        .map_or("", String::as_str)
}

impl Model {
    pub fn library_sort(&self) -> Sort {
        Sort::from_code(&self.prefs.library_sort)
    }

    pub fn library_filter(&self) -> Filter {
        Filter::from_code(&self.prefs.library_filter)
    }

    pub fn set_library_sort(&mut self, sort: Sort) {
        self.edit_prefs(|p| p.library_sort = sort.code().to_owned());
        if sort == Sort::Size {
            self.measure_all();
        }
    }

    pub fn set_library_filter(&mut self, filter: Filter) {
        self.edit_prefs(|p| p.library_filter = filter.code().to_owned());
    }

    /// Starts measuring the games whose size is not known yet (the size order needs them all).
    pub fn measure_all(&mut self) {
        for id in self.games.iter().map(|g| g.id.clone()).collect::<Vec<_>>() {
            if !self.sizes.contains_key(&id) {
                self.sizes.insert(id.clone(), SizeState::Measuring);
                self.effects
                    .push(crate::app::model::Effect::MeasureSize(id));
            }
        }
    }

    /// The games of the library page in the order and with the filter the player chose: indexes
    /// into `games`.
    pub fn library_order(&self) -> Vec<usize> {
        let filter = self.library_filter();
        let mut order: Vec<usize> = (0..self.games.len())
            .filter(|&i| {
                let game = &self.games[i];
                matches_filter(self, game, filter) && matches_query(game, &self.library_query)
            })
            .collect();
        let name = |i: &usize| self.games[*i].name.to_lowercase();
        match self.library_sort() {
            Sort::Name => order.sort_by_key(name),
            Sort::Recent => order.sort_by(|a, b| {
                let (la, lb) = (
                    last_played(self, &self.games[*a]),
                    last_played(self, &self.games[*b]),
                );
                lb.cmp(la).then_with(|| name(a).cmp(&name(b)))
            }),
            Sort::Size => order.sort_by(|a, b| {
                let size = |i: &usize| match self.sizes.get(&self.games[*i].id) {
                    Some(SizeState::Known(bytes)) => *bytes,
                    _ => 0,
                };
                size(b).cmp(&size(a)).then_with(|| name(a).cmp(&name(b)))
            }),
            Sort::Status => order.sort_by(|a, b| {
                let rank = |i: &usize| status_rank(&self.state_of(&self.games[*i]));
                rank(a).cmp(&rank(b)).then_with(|| name(a).cmp(&name(b)))
            }),
        }
        order
    }
}

// ---------------------------------------------------------------------------------------------
// The shelves of the home screen
// ---------------------------------------------------------------------------------------------

/// The most games one shelf shows.
pub const SHELF_MAX: usize = 6;

/// The three shelves. At most two are on screen at once (`Model::shelves`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShelfKind {
    Friends,
    Recent,
    New,
}

/// One game on a shelf.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShelfItem {
    pub game: GameId,
    /// A friend who plays it now and whose session can be joined.
    pub join: Option<AccountId>,
    /// The friend who plays it now, by handle.
    pub friend: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shelf {
    pub kind: ShelfKind,
    pub items: Vec<ShelfItem>,
}

impl Model {
    /// The shelves that have something to show, at most two: friends first (it is live), then
    /// recently played, then new. A shelf that would only repeat the selected game alone is
    /// still shown: it is a way to jump, not a list.
    pub fn shelves(&self) -> Vec<Shelf> {
        self.shelves_with(SHELF_MAX)
    }

    /// [`Model::shelves`] with at most `cap` games on each shelf: the screen has room for that many.
    pub fn shelves_with(&self, cap: usize) -> Vec<Shelf> {
        let mut shelves = Vec::new();
        // Friends who play a game the player owns.
        let mut friends: Vec<ShelfItem> = Vec::new();
        for friend in &self.online.friends.friends {
            let Some(activity) = friend.presence.activity.as_ref() else {
                continue;
            };
            let Some(game) = self.games.iter().find(|g| g.title_id == activity.title_id) else {
                continue;
            };
            if friends.iter().any(|i| i.game == game.id) {
                continue;
            }
            let joinable = play_together::Offer::of_friend(friend).is_ok();
            friends.push(ShelfItem {
                game: game.id.clone(),
                join: joinable.then_some(friend.account.id),
                friend: Some(friend.account.handle.clone()),
            });
        }
        friends.truncate(cap);
        if !friends.is_empty() {
            shelves.push(Shelf {
                kind: ShelfKind::Friends,
                items: friends,
            });
        }
        // Recently played, newest first.
        let mut played: Vec<&Game> = self
            .games
            .iter()
            .filter(|g| !last_played(self, g).is_empty())
            .collect();
        played.sort_by(|a, b| last_played(self, b).cmp(last_played(self, a)));
        let recent: Vec<ShelfItem> = played
            .into_iter()
            .take(cap)
            .map(|g| ShelfItem {
                game: g.id.clone(),
                join: None,
                friend: None,
            })
            .collect();
        if !recent.is_empty() {
            shelves.push(Shelf {
                kind: ShelfKind::Recent,
                items: recent,
            });
        }
        // New: found after the first scan and never played.
        let new: Vec<ShelfItem> = self
            .prefs
            .new_games
            .iter()
            .rev()
            .filter_map(|id| self.games.iter().find(|g| g.id.as_str() == id))
            .filter(|g| last_played(self, g).is_empty())
            .take(cap)
            .map(|g| ShelfItem {
                game: g.id.clone(),
                join: None,
                friend: None,
            })
            .collect();
        if !new.is_empty() {
            shelves.push(Shelf {
                kind: ShelfKind::New,
                items: new,
            });
        }
        shelves.truncate(2);
        shelves
    }

    /// Every item of the shelves in the order the focus walks them: shelf by shelf, left to right.
    pub fn shelf_items(&self) -> Vec<ShelfItem> {
        self.shelves_with(self.shelf_cap)
            .into_iter()
            .flat_map(|s| s.items)
            .collect()
    }
}

impl Model {
    /// Games the scan found that were not there before: the newest is selected on the home
    /// screen (unless something else has the player's attention), a toast names it with its
    /// icon, and the shelf "New" keeps them until they are played.
    pub(crate) fn announce_new_games(&mut self, fresh: &[GameId]) {
        let Some(first) = fresh.first().and_then(|id| self.game(id)).cloned() else {
            return;
        };
        let ids: Vec<String> = fresh.iter().map(|id| id.as_str().to_owned()).collect();
        self.edit_prefs(|p| {
            p.new_games.retain(|n| !ids.contains(n));
            p.new_games.extend(ids.iter().cloned());
            let extra = p.new_games.len().saturating_sub(NEW_GAMES_KEPT);
            p.new_games.drain(..extra);
        });
        if self.overlay.is_none()
            && self.starting.is_none()
            && self.running.is_none()
            && let Some(position) = self.games.iter().position(|g| g.id == first.id)
        {
            self.selected = position;
            self.focus = HomeFocus::Tiles;
            self.measure_selected();
        }
        let language = self.language();
        let text = if fresh.len() == 1 {
            LibraryText::NewGame.format(language, &[Arg::Str(&first.name)])
        } else {
            LibraryText::NewGames.format(language, &[Arg::Int(fresh.len() as i64)])
        };
        self.toast_with(text.clone(), Icon::SPARKLE, Cue::Select);
        if let Some(toast) = self.toast.as_mut().filter(|t| t.text == text) {
            toast.art = first.icon.clone();
        }
    }

    /// Select on a shelf: join the friend's session, or put the game on the tile row.
    pub(crate) fn shelf_action(&mut self, index: usize) {
        let Some(item) = self.shelf_items().get(index).cloned() else {
            return;
        };
        if let Some(friend) = item.join {
            crate::app::together::join_friend(self, friend);
            return;
        }
        if let Some(position) = self.games.iter().position(|g| g.id == item.game) {
            self.selected = position;
            self.focus = HomeFocus::Tiles;
            self.sound(Cue::Select);
            self.measure_selected();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_round_trip_and_unknown_is_the_default() {
        for sort in Sort::ALL {
            assert_eq!(Sort::from_code(sort.code()), sort);
        }
        for filter in Filter::ALL {
            assert_eq!(Filter::from_code(filter.code()), filter);
        }
        assert_eq!(Sort::from_code(""), Sort::Name);
        assert_eq!(Sort::from_code("nonsense"), Sort::Name);
        assert_eq!(Filter::from_code("nonsense"), Filter::All);
    }

    #[test]
    fn cycling_visits_every_choice_once() {
        let mut sort = Sort::Recent;
        for _ in 0..Sort::ALL.len() {
            sort = sort.next();
        }
        assert_eq!(sort, Sort::Recent);
        assert_eq!(Filter::Together.next(), Filter::All);
    }

    #[test]
    fn a_verdict_is_a_line_in_words() {
        assert_eq!(verdict_compat(Verdict::Playable), Compat::RunsWell);
        assert_eq!(verdict_compat(Verdict::Menus), Compat::RunsWithIssues);
        assert_eq!(verdict_compat(Verdict::InGame), Compat::RunsWithIssues);
        assert_eq!(verdict_compat(Verdict::NoStart), Compat::NoStart);
    }
}
