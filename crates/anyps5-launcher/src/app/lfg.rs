//! Find Players and Send to a Friend on the model's side (docs/DESIGN.md 17).
//!
//! Find Players is a panel of the game frame for the game the home screen has selected; whoever
//! opens it says where Back leads ([`Model::open_find_players`]). Send to a Friend holds the
//! capture the player chose while the friend picker is open, and then, until the friend's
//! conversation exists and the upload has started, the capture that waits for it.

use std::path::{Path, PathBuf};

use anyps5_core::online::api::AccountId;
use anyps5_core::online::media::{self, Refusal};

use crate::app::model::Model;
use crate::app::overlay::Overlay;
use crate::screens::lfg_strings::LfgText;
use crate::ui::i18n::Language;

/// A capture the player wants to send: a file the checks passed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Outgoing {
    pub path: PathBuf,
    pub name: String,
}

/// A capture that waits for the conversation with its friend.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Waiting {
    pub friend: AccountId,
    pub handle: String,
    pub outgoing: Outgoing,
    /// The conversation was asked for.
    pub asked: bool,
}

/// What the two areas keep between frames.
#[derive(Debug, Default)]
pub struct State {
    /// Where Back from Find Players leads; none closes the overlay.
    pub back: Option<Overlay>,
    /// The capture the friend picker is for.
    pub outgoing: Option<Outgoing>,
    /// The capture that was given to a friend and waits for the conversation.
    pub waiting: Option<Waiting>,
}

impl Model {
    /// Opens Find Players for the selected game. `back` is the overlay Back returns to: the game
    /// settings, or the game's page. Callable from anywhere that has a game selected.
    pub fn open_find_players(&mut self, back: Overlay) {
        self.lfg.back = Some(back);
        self.switch_overlay(Overlay::FindPlayers);
    }

    /// Leaves Find Players for where it was opened from.
    pub fn leave_find_players(&mut self) {
        match self.lfg.back.take() {
            Some(back) => self.switch_overlay(back),
            None => self.dismiss_overlay(),
        }
    }

    /// Opens the friend picker for a capture.
    pub fn open_send_capture(&mut self, outgoing: Outgoing) {
        self.lfg.outgoing = Some(outgoing);
        self.switch_overlay(Overlay::SendCapture);
    }
}

/// Checks a capture against the cloud's limits before anything is sent. `duration_ms` is a clip's
/// length when the launcher knows it.
pub fn prepare(path: &Path, size: u64, duration_ms: Option<u64>) -> Result<Outgoing, Refusal> {
    let kind = media::kind_of(path).ok_or(Refusal::Unsupported)?;
    media::check(kind, size, duration_ms)?;
    Ok(Outgoing {
        path: path.to_path_buf(),
        name: path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default(),
    })
}

/// The words for a refusal.
pub fn refusal_text(refusal: Refusal, language: Language) -> &'static str {
    match refusal {
        Refusal::Unsupported => LfgText::Unsupported,
        Refusal::Empty => LfgText::EmptyFile,
        Refusal::ImageTooLarge => LfgText::TooLargeImage,
        Refusal::ClipTooLarge => LfgText::TooLargeClip,
        Refusal::ClipTooLong => LfgText::TooLongClip,
    }
    .text(language)
}

/// The words for a send the cloud turned away, by its error code.
pub fn send_failure_text(code: &str, language: Language) -> &'static str {
    match code {
        "forbidden" | "not_found" => LfgText::SendFriendsOnly,
        "restricted" | "banned" => LfgText::SendRestricted,
        "rate_limited" => LfgText::SendRate,
        "bad_request" => LfgText::Unsupported,
        _ => LfgText::SendFailed,
    }
    .text(language)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyps5_core::paths::AppPaths;
    use anyps5_core::settings::Preferences;

    fn model() -> Model {
        let prefs = Preferences {
            onboarded: true,
            ..Preferences::default()
        };
        Model::new(
            prefs,
            AppPaths::under(std::path::Path::new("C:/none")),
            false,
        )
    }

    #[test]
    fn back_from_find_players_leads_where_it_was_opened_from() {
        let mut m = model();
        m.open_find_players(Overlay::GameSettings);
        assert_eq!(m.overlay, Some(Overlay::FindPlayers));
        m.leave_find_players();
        assert_eq!(m.overlay, Some(Overlay::GameSettings));
        m.switch_overlay(Overlay::FindPlayers);
        m.leave_find_players();
        assert_eq!(m.overlay, None, "nothing remembered: the area closes");
    }

    #[test]
    fn a_capture_is_checked_before_the_picker_opens() {
        assert!(prepare(Path::new("a/shot.png"), 1024, None).is_ok());
        assert_eq!(
            prepare(Path::new("a/shot.bmp"), 1024, None),
            Err(Refusal::Unsupported)
        );
        assert_eq!(
            prepare(Path::new("a/clip.mp4"), 1024, Some(61_000)),
            Err(Refusal::ClipTooLong)
        );
        let out = prepare(Path::new("a/clip.mp4"), 1024, Some(30_000)).expect("a clip");
        assert_eq!(out.name, "clip.mp4");
    }

    #[test]
    fn a_refused_send_says_why_in_words() {
        assert_eq!(
            send_failure_text("forbidden", Language::English),
            "Captures go to friends only."
        );
        assert_eq!(
            send_failure_text("rate_limited", Language::English),
            "You sent a lot of captures. Try again in a few minutes."
        );
        assert_eq!(
            send_failure_text("internal", Language::English),
            "The capture could not be sent. Try again."
        );
    }

    #[test]
    fn every_refusal_has_words_in_both_languages() {
        for refusal in [
            Refusal::Unsupported,
            Refusal::Empty,
            Refusal::ImageTooLarge,
            Refusal::ClipTooLarge,
            Refusal::ClipTooLong,
        ] {
            for language in Language::ALL {
                assert!(!refusal_text(refusal, language).is_empty());
            }
        }
    }

    #[test]
    fn the_picker_holds_the_capture_it_was_opened_for() {
        let mut m = model();
        let out = prepare(Path::new("a/shot.png"), 10, None).expect("a screenshot");
        m.open_send_capture(out.clone());
        assert_eq!(m.overlay, Some(Overlay::SendCapture));
        assert_eq!(m.lfg.outgoing, Some(out));
    }
}
