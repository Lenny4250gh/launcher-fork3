//! The notification history of the launcher (docs/DESIGN.md 9.12): everything the launcher tells
//! the player by a toast that arrives on its own (a friend request, an invitation, a message, a
//! trophy, an update), kept in one list so a missed toast is not lost. The list is local, capped
//! and written next to the preferences; nothing of it leaves the PC.
//!
//! This module holds the data and its rules only. The launcher records entries, draws the list
//! and runs the action of an entry.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// The most entries kept; the oldest go first.
pub const CAP: usize = 200;

/// What an entry is about; it decides the icon and the small label.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    FriendRequest,
    /// A party or a game invitation.
    Invitation,
    Message,
    Trophy,
    /// A launcher or toolkit update.
    Update,
    /// A Play Together session or an invitation to one.
    Together,
    System,
}

/// What the main button of an entry does.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "do", content = "arg")]
pub enum Act {
    /// Nothing to do: the entry only tells.
    None,
    /// Accept the friend request of this account.
    AcceptFriend(u64),
    /// Join the Play Together session or game this invitation (its ID) is for.
    JoinInvitation(String),
    /// Join the party this invitation (its ID) is for.
    JoinParty(String),
    /// Accept the share play invitation (its ID) from a party member.
    JoinShare(String),
    /// Open the friends list on its first page.
    OpenFriends,
    /// Open the party page.
    OpenParty,
    /// Open a conversation, or the list of messages.
    OpenMessages(Option<String>),
    /// Open the trophies of a game (its ID in the library).
    OpenTrophies(String),
    /// Open the settings on the page that holds the launcher's update.
    OpenUpdate,
    /// Open the settings on the toolkit page.
    OpenToolkit,
}

impl Act {
    /// The entry has a main button.
    pub fn has_button(&self) -> bool {
        *self != Act::None
    }
}

/// One entry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    pub id: u64,
    pub kind: Kind,
    /// The sentence as it was shown, in the language of that day.
    pub text: String,
    /// Seconds since the epoch.
    pub at: u64,
    pub read: bool,
    pub act: Act,
}

#[derive(Default, Serialize, Deserialize)]
struct File {
    next: u64,
    entries: Vec<Entry>,
}

/// The list, newest last, and where it lives.
#[derive(Debug, Default)]
pub struct Notifications {
    entries: Vec<Entry>,
    next: u64,
    file: Option<PathBuf>,
    dirty: bool,
}

impl Notifications {
    /// The history in `file`; a missing or broken file is an empty history. With `persist` off
    /// (previews, tests) nothing is read or written.
    pub fn open(file: PathBuf, persist: bool) -> Self {
        if !persist {
            return Self::default();
        }
        let loaded: File = std::fs::read(&file)
            .ok()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
            .unwrap_or_default();
        let mut this = Self {
            next: loaded.next,
            entries: loaded.entries,
            file: Some(file),
            dirty: false,
        };
        this.trim();
        this.next = this
            .next
            .max(this.entries.iter().map(|e| e.id + 1).max().unwrap_or(0));
        this
    }

    /// Records an entry at `at` (seconds since the epoch). Returns its ID.
    pub fn push(&mut self, kind: Kind, text: String, act: Act, at: u64) -> u64 {
        let id = self.next;
        self.next += 1;
        self.entries.push(Entry {
            id,
            kind,
            text,
            at,
            read: false,
            act,
        });
        self.trim();
        self.dirty = true;
        id
    }

    fn trim(&mut self) {
        if self.entries.len() > CAP {
            let extra = self.entries.len() - CAP;
            self.entries.drain(..extra);
        }
    }

    /// Newest first.
    pub fn newest_first(&self) -> impl Iterator<Item = &Entry> {
        self.entries.iter().rev()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn unread(&self) -> usize {
        self.entries.iter().filter(|e| !e.read).count()
    }

    pub fn get(&self, id: u64) -> Option<&Entry> {
        self.entries.iter().find(|e| e.id == id)
    }

    /// Marks everything read.
    pub fn mark_all_read(&mut self) {
        let mut changed = false;
        for entry in self.entries.iter_mut().filter(|e| !e.read) {
            entry.read = true;
            changed = true;
        }
        self.dirty |= changed;
    }

    pub fn remove(&mut self, id: u64) {
        let before = self.entries.len();
        self.entries.retain(|e| e.id != id);
        self.dirty |= self.entries.len() != before;
    }

    pub fn clear(&mut self) {
        self.dirty |= !self.entries.is_empty();
        self.entries.clear();
    }

    /// Writes the list when it changed. A write that fails is logged and tried again with the next
    /// change.
    pub fn save_if_dirty(&mut self) {
        if !std::mem::take(&mut self.dirty) {
            return;
        }
        let Some(file) = self.file.as_deref() else {
            return;
        };
        if let Err(error) = write(file, self.next, &self.entries) {
            tracing::warn!("notification history: {error}");
        }
    }
}

fn write(file: &Path, next: u64, entries: &[Entry]) -> std::io::Result<()> {
    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let bytes = serde_json::to_vec(&serde_json::json!({"next": next, "entries": entries}))
        .map_err(std::io::Error::other)?;
    let partial = file.with_extension("partial");
    std::fs::write(&partial, bytes)?;
    std::fs::rename(&partial, file)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note(list: &mut Notifications, n: u64) -> u64 {
        list.push(Kind::System, format!("note {n}"), Act::None, n)
    }

    #[test]
    fn the_list_is_capped_and_drops_the_oldest() {
        let mut list = Notifications::default();
        for n in 0..(CAP as u64 + 5) {
            note(&mut list, n);
        }
        assert_eq!(list.len(), CAP);
        assert_eq!(list.newest_first().next().unwrap().text, "note 204");
        assert_eq!(list.newest_first().last().unwrap().text, "note 5");
    }

    #[test]
    fn reading_removing_and_clearing() {
        let mut list = Notifications::default();
        let a = note(&mut list, 1);
        note(&mut list, 2);
        assert_eq!(list.unread(), 2);
        list.mark_all_read();
        assert_eq!(list.unread(), 0);
        note(&mut list, 3);
        assert_eq!(list.unread(), 1);
        list.remove(a);
        assert!(list.get(a).is_none());
        assert_eq!(list.len(), 2);
        list.clear();
        assert!(list.is_empty());
    }

    #[test]
    fn the_history_survives_a_restart_and_ids_do_not_repeat() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("Data").join("notifications.json");
        let mut list = Notifications::open(file.clone(), true);
        list.push(Kind::FriendRequest, "Mira".into(), Act::AcceptFriend(7), 10);
        let last = list.push(Kind::Message, "Hi".into(), Act::OpenMessages(None), 11);
        list.mark_all_read();
        list.save_if_dirty();
        let mut again = Notifications::open(file, true);
        assert_eq!(again.len(), 2);
        assert_eq!(again.unread(), 0);
        let next = again.push(Kind::System, "x".into(), Act::None, 12);
        assert!(next > last);
        assert_eq!(
            again.get(0).map(|e| &e.act),
            Some(&Act::AcceptFriend(7)),
            "the action is kept"
        );
    }

    #[test]
    fn a_broken_file_is_an_empty_history() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("notifications.json");
        std::fs::write(&file, b"not json").unwrap();
        assert!(Notifications::open(file, true).is_empty());
    }

    #[test]
    fn without_persisting_nothing_is_written() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("notifications.json");
        let mut list = Notifications::open(file.clone(), false);
        note(&mut list, 1);
        list.save_if_dirty();
        assert!(!file.exists());
    }
}
