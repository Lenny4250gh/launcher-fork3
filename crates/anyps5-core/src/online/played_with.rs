//! "Played with recently": the private list of people the player shared a party or a session with
//! for at least two minutes in the last 30 days (ADR 0038 in anyplaynet/network). The cloud keeps
//! the list; this is what the launcher holds of it, and the small rules the screen leans on.
//!
//! The list is the account's own: nothing here is sent to anyone. Adding a friend, looking at a
//! profile and blocking are the launcher's ordinary actions on the entry's account; what they
//! change shows on the entry at once (`requested`, `befriended`, `hidden`) until the next fetch.

use std::collections::BTreeSet;

use super::api::{Account, AccountId, PlayedWith, PlayedWithList};

/// How long a list stays fresh while the player moves between pages, in seconds.
pub const REFRESH_SECS: u64 = 30;

/// Where a person on the list stands with the player.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Standing {
    /// Friends already.
    Friend,
    /// A friend request is out (sent just now, or earlier).
    Requested,
    /// Not a friend: the player can add them.
    Open,
}

/// What the launcher knows of the list.
#[derive(Debug, Default)]
pub struct PlayedWithState {
    /// The cloud's last answer; `None` before the first one.
    pub list: Option<PlayedWithList>,
    /// The last fetch failed and there is nothing to show.
    pub failed: bool,
    /// When the list was asked for last (Unix seconds).
    pub asked_at: u64,
    /// Friend requests sent from the list since the last fetch.
    requested: BTreeSet<AccountId>,
    /// Requests answered with "friends" since the last fetch.
    befriended: BTreeSet<AccountId>,
    /// People blocked from the list since the last fetch.
    hidden: BTreeSet<AccountId>,
}

impl PlayedWithState {
    /// Takes the cloud's answer: it is the truth again, so the local marks go.
    pub fn keep(&mut self, list: PlayedWithList) {
        self.list = Some(list);
        self.failed = false;
        self.requested.clear();
        self.befriended.clear();
        self.hidden.clear();
    }

    /// The fetch was refused: a list on screen stays, an unloaded one shows the failure.
    pub fn fetch_failed(&mut self) {
        self.failed = self.list.is_none();
    }

    /// Whether the cloud records new people; on until the cloud says otherwise.
    pub fn remember(&self) -> bool {
        self.list.as_ref().is_none_or(|l| l.remember)
    }

    /// The people to show, newest first. A person the player blocked since the fetch is left out
    /// (the cloud leaves them out of the next answer).
    pub fn people(&self) -> impl Iterator<Item = &PlayedWith> {
        self.list
            .iter()
            .flat_map(|l| l.people.iter())
            .filter(|p| !self.hidden.contains(&p.account.id))
    }

    /// How many people the list shows.
    pub fn count(&self) -> usize {
        self.people().count()
    }

    /// The account of a person on the list, blocked since the fetch or not (for names in
    /// sentences).
    pub fn account(&self, id: AccountId) -> Option<&Account> {
        self.list
            .iter()
            .flat_map(|l| l.people.iter())
            .map(|p| &p.account)
            .find(|a| a.id == id)
    }

    /// The player sent a friend request to `handle`; `friends` when it made them friends at once.
    pub fn request_sent(&mut self, handle: &str, friends: bool) {
        let id = self
            .list
            .iter()
            .flat_map(|l| l.people.iter())
            .find(|p| p.account.handle.eq_ignore_ascii_case(handle))
            .map(|p| p.account.id);
        if let Some(id) = id {
            if friends {
                self.befriended.insert(id);
            } else {
                self.requested.insert(id);
            }
        }
    }

    /// The player blocked this account.
    pub fn blocked(&mut self, id: AccountId) {
        self.hidden.insert(id);
    }

    /// Where `person` stands, given whether the launcher holds them as a friend and whether a
    /// request to them is open.
    pub fn standing(&self, person: &PlayedWith, is_friend: bool, request_out: bool) -> Standing {
        let id = person.account.id;
        if person.friend || is_friend || self.befriended.contains(&id) {
            Standing::Friend
        } else if request_out || self.requested.contains(&id) {
            Standing::Requested
        } else {
            Standing::Open
        }
    }

    /// Empties the list on screen at once; returns what was there, to put back on a refusal.
    pub fn take_people(&mut self) -> Vec<PlayedWith> {
        self.list
            .as_mut()
            .map(|l| std::mem::take(&mut l.people))
            .unwrap_or_default()
    }

    /// Puts people back after a refusal to clear, unless a newer answer brought some already.
    pub fn restore_people(&mut self, people: Vec<PlayedWith>) {
        if let Some(list) = self.list.as_mut()
            && list.people.is_empty()
        {
            list.people = people;
        }
    }

    /// Sets the switch as shown; does nothing before the list is known.
    pub fn set_remember(&mut self, on: bool) {
        if let Some(list) = self.list.as_mut() {
            list.remember = on;
        }
    }

    /// True when a fetch is due: nothing was asked for yet, or `REFRESH_SECS` have passed.
    pub fn due(&self, now: u64) -> bool {
        self.asked_at == 0 || now >= self.asked_at + REFRESH_SECS
    }

    /// Signing out forgets everything.
    pub fn reset(&mut self) {
        *self = PlayedWithState::default();
    }
}
