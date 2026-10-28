//! What the player hears of share play outside the panel: a toast when a member invites them,
//! asks for another mode, changes what they may do, or ends the share, and when a command is
//! refused (docs/DESIGN.md 17, "Share play").

use anyps5_core::notifications::{Act, Kind};
use anyps5_core::online::api::ShareMode;
use anyps5_core::online::{Failure, SharePlayNotice};

use crate::app::model::Model;
use crate::app::online::failure_text;
use crate::app::overlay::Overlay;
use crate::screens::online_share_strings::ShareText as X;
use crate::ui::i18n::Arg;
use crate::ui::icons::Icon;
use crate::ui::sound::Cue;

/// Something about a share changed under the player.
pub fn notice(model: &mut Model, notice: SharePlayNotice) {
    let language = model.language();
    match notice {
        SharePlayNotice::Invited(invitation) => {
            // The history keeps it either way, like a party invitation: the guest may be on
            // another page, and the invitation runs out. The open panel already shows the new
            // rows, so only there the toast stays away.
            let text = X::ToastInvited.format(language, &[Arg::Str(&invitation.from.handle)]);
            let act = Act::JoinShare(invitation.id.clone());
            if model.overlay == Some(Overlay::Friends) {
                model.note(Kind::Invitation, text, act);
            } else {
                model.announce(
                    Kind::Invitation,
                    act,
                    text,
                    Icon::GAME_CONTROLLER,
                    Cue::Invite,
                );
            }
        }
        SharePlayNotice::Requested(_) => {
            if model.overlay != Some(Overlay::Friends) {
                let who = model
                    .online
                    .share_play
                    .share()
                    .map(|share| share.peer.handle.clone())
                    .unwrap_or_else(|| X::SomeoneUnknown.text(language).to_owned());
                let text = X::ToastAsked.format(language, &[Arg::Str(&who)]);
                model.toast(text, Icon::GAME_CONTROLLER);
            }
        }
        SharePlayNotice::ModeChanged(mode) => {
            let text = match mode {
                ShareMode::HandOver => X::ToastNowHandOver,
                ShareMode::Together => X::ToastNowTogether,
                ShareMode::Watch | ShareMode::Unknown => X::ToastNowWatch,
            };
            model.toast(text.text(language), Icon::GAME_CONTROLLER);
        }
        SharePlayNotice::Ended { .. } => {
            model.toast(X::ToastEnded.text(language), Icon::GAME_CONTROLLER);
        }
    }
}

/// A start, answer, mode change, request or stop was refused: a short plain sentence, else the
/// general text.
pub fn failed(model: &mut Model, failure: &Failure) {
    model.online.share_play.forget_expectations();
    let language = model.language();
    let text = match failure.code.as_str() {
        Failure::NO_GAME => X::ErrorNoGame.text(language).to_owned(),
        "not_in_party" => X::ErrorNotInParty.text(language).to_owned(),
        "busy" => X::ErrorBusy.text(language).to_owned(),
        "no_encoder" => X::ErrorNoEncoder.text(language).to_owned(),
        "no_decoder" => X::ErrorNoDecoder.text(language).to_owned(),
        "not_found" => X::ErrorNotFound.text(language).to_owned(),
        "not_allowed" => X::ErrorNotAllowed.text(language).to_owned(),
        _ => failure_text(failure, language),
    };
    model.toast(text, Icon::WARNING);
}

#[cfg(test)]
mod tests {
    use anyps5_core::online::api::{Account, AccountId, ShareInvitation};
    use anyps5_core::online::control::Event;
    use anyps5_core::online::{Line, Output};
    use anyps5_core::paths::AppPaths;
    use anyps5_core::settings::Preferences;

    use super::*;
    use crate::app::panels::notifications::primary_of;
    use crate::screens::notification_strings::NotifyText;

    fn invitation() -> ShareInvitation {
        ShareInvitation {
            id: "i1".into(),
            from: Account {
                id: AccountId(2),
                handle: "Lumen".into(),
            },
            share: "s1".into(),
            title_id: "T".into(),
            title: "Astro Bot".into(),
            mode: ShareMode::Watch,
            expires: 99,
        }
    }

    fn model_with_invitation() -> Model {
        let mut model = Model::new(
            Preferences::default(),
            AppPaths::under(std::path::Path::new("C:/none")),
            false,
        );
        model
            .online
            .apply(Line::Output(Output::Event(Event::ShareInvitation {
                invitation: invitation(),
            })));
        model
    }

    #[test]
    fn an_invitation_is_toasted_and_kept_in_the_history_with_an_accept() {
        let mut model = model_with_invitation();
        notice(&mut model, SharePlayNotice::Invited(invitation()));
        assert_eq!(
            model.toast.as_ref().map(|t| t.text.as_str()),
            Some("Lumen invites you to share play")
        );
        let entry = model.notes.newest_first().next().expect("an entry");
        assert_eq!(entry.kind, Kind::Invitation);
        assert_eq!(entry.act, Act::JoinShare("i1".into()));
        assert_eq!(
            primary_of(&entry.act, &model),
            Some(NotifyText::Accept),
            "the entry has the primary answer while the invitation waits"
        );
    }

    #[test]
    fn with_the_friends_panel_open_only_the_history_gets_it() {
        let mut model = model_with_invitation();
        model.overlay = Some(Overlay::Friends);
        notice(&mut model, SharePlayNotice::Invited(invitation()));
        assert!(model.toast.is_none());
        assert_eq!(model.notes.newest_first().count(), 1);
    }

    #[test]
    fn an_answered_or_expired_invitation_leaves_no_button() {
        let mut model = model_with_invitation();
        model.online.apply(Line::Output(Output::Event(Event::Share {
            can_start: false,
            share: None,
        })));
        let act = Act::JoinShare("gone".into());
        assert_eq!(primary_of(&act, &model), None);
    }
}
