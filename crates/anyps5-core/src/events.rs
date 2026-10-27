//! The one channel from the core's worker threads into the UI, and the cooperative cancel flag.
//!
//! Core code sends a [`CoreEvent`] and then calls the [`Notifier`] so the UI repaints; the UI drains
//! the receiver once per frame. Modules add their own variants here.

use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use crate::{
    library::Game,
    paths::GameId,
    trophies::{Earned, Trophy, TrophySet},
};

/// Called after every send so the UI wakes up (the app passes `egui::Context::request_repaint`).
pub type Notifier = Arc<dyn Fn() + Send + Sync>;

/// Cooperative cancellation: long operations check it between files and chunks.
#[derive(Clone, Debug, Default)]
pub struct CancelToken(Arc<AtomicBool>);

/// Two tokens are equal when they are the same flag.
impl PartialEq for CancelToken {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for CancelToken {}

impl CancelToken {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&self) {
        self.0.store(true, Ordering::Relaxed);
    }

    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

/// Everything a worker thread can tell the UI.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CoreEvent {
    /// A scan finished. `records` maps a game to the toolkit version it was unlocked with.
    LibraryScanned {
        root: PathBuf,
        games: Vec<Game>,
        records: HashMap<GameId, String>,
    },
    /// The watcher saw a change in the games folder and the debounce allows a rescan.
    LibraryChanged,
    /// The size on disk of one game was measured.
    SizeMeasured { game: GameId, bytes: u64 },
    /// The trophy set of a game (re)loaded; `None` when the dump has no trophy data.
    Trophies {
        game: GameId,
        set: Option<TrophySet>,
    },
    /// The running game earned a trophy.
    TrophyUnlocked { game: GameId, trophy: Trophy },
    /// The unlocked games location was looked at (it can sit on a drive that is gone).
    LocationChecked(crate::location::Report),
    /// Moving the unlocked games to a new location: how far it is.
    MoveProgress(crate::relocate::Progress),
    /// The move ended: what moved, what stayed.
    MoveFinished(crate::relocate::Report),
    /// What the player earned in a title, read for the network without texts or pictures.
    TrophyRecords {
        title_id: String,
        earned: Vec<Earned>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_token_is_not_cancelled() {
        let token = CancelToken::new();
        assert!(!token.is_cancelled());
        assert!(!CancelToken::default().is_cancelled());
    }

    #[test]
    fn cancelling_a_token_sets_the_flag_and_repeating_is_harmless() {
        let token = CancelToken::new();
        token.cancel();
        assert!(token.is_cancelled());
        token.cancel();
        assert!(token.is_cancelled());
    }

    #[test]
    fn a_clone_sees_the_cancel_of_the_original_and_the_other_way_round() {
        let original = CancelToken::new();
        let clone = original.clone();
        assert!(!original.is_cancelled());
        assert!(!clone.is_cancelled());

        original.cancel();
        assert!(clone.is_cancelled());
        assert!(original.is_cancelled());

        let original2 = CancelToken::new();
        let clone2 = original2.clone();
        assert!(!original2.is_cancelled());
        assert!(!clone2.is_cancelled());

        clone2.cancel();
        assert!(original2.is_cancelled());
        assert!(clone2.is_cancelled());
    }

    #[test]
    fn a_clone_equals_its_original_and_separate_tokens_are_not_equal() {
        let token1 = CancelToken::new();
        let clone1 = token1.clone();
        let token2 = CancelToken::new();

        assert_eq!(token1, clone1);
        assert_eq!(clone1, token1);

        assert_ne!(token1, token2);
        assert_ne!(token2, token1);

        token1.cancel();
        assert_eq!(token1, clone1);
        assert_ne!(token1, token2);

        token2.cancel();
        assert_ne!(token1, token2);
    }
}
