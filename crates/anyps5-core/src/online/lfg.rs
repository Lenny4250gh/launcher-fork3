//! Looking for group: what the launcher holds of the listings of one game, the player's own
//! listings and the requests, and the pure rules the screen leans on (filters, time left).
//!
//! A listing is a short post for one game: the mode, a language, a note and how many players the
//! poster looks for. It closes after an hour. The cloud lists a game's listings only to a player
//! who names the game; that the player owns it is the launcher's word (a filter, not security).

use super::api::{self, AccountId, LfgMine, LfgMode, Listing, Listings};

/// How long the screen waits before it asks the cloud again while it is open, in seconds.
pub const REFRESH_SECS: u64 = 30;

/// The languages a listing can name, as the tags the cloud takes.
pub const LANGUAGES: [&str; 5] = ["en", "tr", "de", "fr", "es"];

/// What the launcher knows of looking for group.
#[derive(Debug, Default)]
pub struct Lfg {
    /// The game the listings below are of.
    pub title_id: Option<String>,
    /// The open listings of that game; `None` before the first answer.
    pub listings: Option<Vec<Listing>>,
    /// The player's own listings and the ones they asked to join.
    pub mine: Option<LfgMine>,
    /// The last fetch of the listings failed.
    pub failed: bool,
    /// When the listings were asked for last (Unix seconds).
    pub asked_at: u64,
    /// The last post was turned away; the screen says why from this text.
    pub post_error: Option<String>,
}

impl Lfg {
    /// Takes the listings of a game. An answer for another game than the one on screen is dropped.
    pub fn keep_listings(&mut self, title_id: &str, answer: Listings) {
        if self.title_id.as_deref() == Some(title_id) {
            self.listings = Some(answer.listings);
            self.failed = false;
        }
    }

    /// The screen opened on a game: forget the listings of an earlier one.
    pub fn open(&mut self, title_id: &str) {
        if self.title_id.as_deref() != Some(title_id) {
            self.listings = None;
        }
        self.title_id = Some(title_id.to_owned());
        self.failed = false;
        self.post_error = None;
    }

    /// The screen was left.
    pub fn close(&mut self) {
        self.title_id = None;
    }

    /// True when a fresh fetch is due: nothing was asked for, or `REFRESH_SECS` have passed.
    pub fn due(&self, now: u64) -> bool {
        self.asked_at == 0 || now >= self.asked_at + REFRESH_SECS
    }

    /// The player's own listing in this game, if they have one open.
    pub fn own_in(&self, title_id: &str) -> Option<&Listing> {
        self.mine
            .as_ref()?
            .posted
            .iter()
            .find(|l| l.title_id == title_id)
    }

    /// A listing by its ID, from the open listings, the player's own or the ones asked.
    pub fn find(&self, id: &str) -> Option<&Listing> {
        self.listings
            .iter()
            .flatten()
            .chain(self.mine.iter().flat_map(|m| m.posted.iter()))
            .chain(self.mine.iter().flat_map(|m| m.asked.iter()))
            .find(|l| l.id == id)
    }

    /// Takes a listing the cloud answered (after a request, an answer) over the copies held.
    pub fn replace(&mut self, listing: &Listing) {
        let swap = |all: &mut Vec<Listing>| {
            if let Some(slot) = all.iter_mut().find(|l| l.id == listing.id) {
                // The poster's view carries the requests, the others' does not: keep them.
                let requests = std::mem::take(&mut slot.requests);
                *slot = listing.clone();
                if slot.requests.is_empty() {
                    slot.requests = requests;
                }
            }
        };
        if let Some(all) = self.listings.as_mut() {
            swap(all);
        }
        if let Some(mine) = self.mine.as_mut() {
            swap(&mut mine.posted);
            swap(&mut mine.asked);
        }
    }

    /// Drops a listing the player closed.
    pub fn remove(&mut self, id: &str) {
        if let Some(all) = self.listings.as_mut() {
            all.retain(|l| l.id != id);
        }
        if let Some(mine) = self.mine.as_mut() {
            mine.posted.retain(|l| l.id != id);
            mine.asked.retain(|l| l.id != id);
        }
    }

    /// Signing out forgets everything.
    pub fn reset(&mut self) {
        *self = Lfg::default();
    }
}

/// The listings that fit the filters, as they came (newest first). A listing with no language
/// fits every language filter; the player's own listings are left out.
pub fn filter<'a>(
    listings: &'a [Listing],
    me: Option<AccountId>,
    mode: Option<LfgMode>,
    language: Option<&str>,
) -> Vec<&'a Listing> {
    listings
        .iter()
        .filter(|l| Some(l.poster.id) != me)
        .filter(|l| mode.is_none_or(|m| l.mode == m))
        .filter(|l| match (language, l.language.as_deref()) {
            (Some(wanted), Some(own)) => wanted == own,
            _ => true,
        })
        .collect()
}

/// Seconds until a listing closes; 0 once it has.
pub fn seconds_left(listing: &Listing, now: u64) -> u64 {
    listing.expires.saturating_sub(now)
}

/// Whole minutes left, rounded up, so a listing never reads "0 min" while it is open.
pub fn minutes_left(listing: &Listing, now: u64) -> u64 {
    seconds_left(listing, now).div_ceil(60)
}

/// How many places are still free.
pub fn places_left(listing: &Listing) -> u8 {
    listing.slots.saturating_sub(listing.filled)
}

/// The name the cloud takes for a mode filter, or none for "all".
pub fn mode_of(index: usize) -> Option<LfgMode> {
    [
        None,
        Some(LfgMode::Online),
        Some(LfgMode::Lan),
        Some(LfgMode::PlayTogether),
    ]
    .get(index)
    .copied()
    .flatten()
}

/// The language tag of a filter index; 0 is any.
pub fn language_of(index: usize) -> Option<&'static str> {
    index.checked_sub(1).and_then(|i| LANGUAGES.get(i)).copied()
}

/// Position of a language tag in the choices (0 is any).
pub fn language_index(tag: Option<&str>) -> usize {
    tag.and_then(|t| LANGUAGES.iter().position(|l| *l == t))
        .map_or(0, |i| i + 1)
}

/// True when the note can be posted: the cloud's own rule.
pub fn note_ok(note: &str) -> bool {
    api::check_lfg_note(note).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use api::Account;

    fn listing(id: &str, poster: u64, mode: LfgMode, language: Option<&str>) -> Listing {
        Listing {
            id: id.into(),
            poster: Account {
                id: AccountId(poster),
                handle: format!("p{poster}"),
            },
            title_id: "GAME00001".into(),
            mode,
            language: language.map(str::to_owned),
            note: String::new(),
            slots: 3,
            filled: 1,
            created: 1000,
            expires: 4600,
            ask: None,
            requests: Vec::new(),
        }
    }

    #[test]
    fn filters_keep_the_mode_and_the_language_and_leave_out_the_players_own() {
        let all = vec![
            listing("a", 1, LfgMode::Online, Some("en")),
            listing("b", 2, LfgMode::Lan, Some("tr")),
            listing("c", 3, LfgMode::Online, None),
        ];
        let ids = |found: Vec<&Listing>| found.iter().map(|l| l.id.clone()).collect::<Vec<_>>();
        assert_eq!(ids(filter(&all, None, None, None)), ["a", "b", "c"]);
        assert_eq!(
            ids(filter(&all, None, Some(LfgMode::Online), None)),
            ["a", "c"]
        );
        // A listing that names no language fits every language.
        assert_eq!(ids(filter(&all, None, None, Some("tr"))), ["b", "c"]);
        assert_eq!(
            ids(filter(&all, Some(AccountId(1)), None, None)),
            ["b", "c"]
        );
    }

    #[test]
    fn time_left_rounds_up_and_stops_at_zero() {
        let l = listing("a", 1, LfgMode::Online, None);
        assert_eq!(minutes_left(&l, 1000), 60);
        assert_eq!(minutes_left(&l, 4541), 1);
        assert_eq!(minutes_left(&l, 4600), 0);
        assert_eq!(minutes_left(&l, 9999), 0);
    }

    #[test]
    fn places_left_never_goes_below_zero() {
        let mut l = listing("a", 1, LfgMode::Online, None);
        assert_eq!(places_left(&l), 2);
        l.filled = 5;
        assert_eq!(places_left(&l), 0);
    }

    #[test]
    fn filter_indexes_map_to_modes_and_languages() {
        assert_eq!(mode_of(0), None);
        assert_eq!(mode_of(2), Some(LfgMode::Lan));
        assert_eq!(mode_of(9), None);
        assert_eq!(language_of(0), None);
        assert_eq!(language_of(2), Some("tr"));
        assert_eq!(language_index(Some("de")), 3);
        assert_eq!(language_index(Some("xx")), 0);
        assert_eq!(language_index(None), 0);
    }

    #[test]
    fn listings_of_another_game_are_dropped_and_the_screen_refreshes_on_a_timer() {
        let mut lfg = Lfg::default();
        lfg.open("GAME00001");
        lfg.keep_listings(
            "GAME00002",
            Listings {
                listings: vec![listing("x", 1, LfgMode::Lan, None)],
            },
        );
        assert!(lfg.listings.is_none());
        lfg.keep_listings(
            "GAME00001",
            Listings {
                listings: vec![listing("a", 1, LfgMode::Lan, None)],
            },
        );
        assert_eq!(lfg.listings.as_ref().map(Vec::len), Some(1));
        assert!(lfg.due(5));
        lfg.asked_at = 100;
        assert!(!lfg.due(129));
        assert!(lfg.due(130));
    }

    #[test]
    fn a_closed_listing_leaves_every_list() {
        let mut lfg = Lfg::default();
        lfg.open("GAME00001");
        lfg.listings = Some(vec![listing("a", 1, LfgMode::Lan, None)]);
        lfg.mine = Some(LfgMine {
            posted: vec![listing("a", 1, LfgMode::Lan, None)],
            asked: vec![],
        });
        assert!(lfg.find("a").is_some());
        lfg.remove("a");
        assert!(lfg.find("a").is_none());
    }
}
