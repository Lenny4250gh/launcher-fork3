//! What the player hears of share play outside the panel: a toast when a member invites them,
//! asks for another mode, changes what they may do, or ends the share, and when a command is
//! refused (docs/DESIGN.md 17, "Share play").

use anyps5_core::online::api::ShareMode;
use anyps5_core::online::{Failure, SharePlayNotice};

use crate::app::model::Model;
use crate::app::online::failure_text;
use crate::app::overlay::Overlay;
use crate::screens::online_share_strings::ShareText as X;
use crate::ui::i18n::Arg;
use crate::ui::icons::Icon;

/// Something about a share changed under the player.
pub fn notice(model: &mut Model, notice: SharePlayNotice) {
    let language = model.language();
    match notice {
        SharePlayNotice::Invited(invitation) => {
            // The open panel already shows the new rows.
            if model.overlay != Some(Overlay::Friends) {
                let text = X::ToastInvited.format(language, &[Arg::Str(&invitation.from.handle)]);
                model.toast(text, Icon::GAME_CONTROLLER);
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
