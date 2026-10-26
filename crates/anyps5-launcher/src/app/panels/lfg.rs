//! Find Players, Send to a Friend and the files of media messages in the panel host: what the
//! player asks for becomes a command for the net service (`Model::online`).

use anyps5_core::library::Game;
use anyps5_core::online::api::AccountId;
use anyps5_core::online::media;
use anyps5_core::online::{self as net, Command, Purpose, Status, unix_now};
use anyps5_core::platform;

use super::PanelHost;
use crate::app::lfg::Waiting;
use crate::app::model::{Effect, Model};
use crate::app::overlay::{Overlay, OverlayCx};
use crate::screens::lfg::{LfgAction, LfgData, Page};
use crate::screens::lfg_strings::LfgText as T;
use crate::screens::send::{SendAction, SendData};
use crate::ui::i18n::Arg;
use crate::ui::icons::Icon;

/// The folder the files of received media are kept in.
pub fn media_dir(model: &Model) -> std::path::PathBuf {
    model.paths.network_dir().join("media")
}

impl PanelHost {
    pub(super) fn find_players_ui(&mut self, cx: &mut OverlayCx<'_>, wanted: bool) {
        let events = self.take_events(wanted);
        let side_next = cx.model.overlay.is_some_and(Overlay::is_side_panel);
        let Some(game) = cx.model.selected_game().cloned() else {
            if wanted {
                cx.model.dismiss_overlay();
            }
            return;
        };
        let (language, accent) = (cx.model.language(), cx.shell.accent_color());
        if wanted && !self.lfg.is_active() {
            self.lfg.open();
            cx.model.online.open_lfg(&game.title_id);
            self.start_online(cx.model);
        } else if !wanted && self.lfg.is_active() {
            if side_next {
                self.lfg.dismiss();
            } else {
                self.lfg.close();
            }
        }
        if !self.lfg.is_active() {
            return;
        }
        // Listings go stale within the hour: ask again now and then, and when an event said so.
        if wanted && cx.model.online.is_online() && cx.model.online.lfg.due(unix_now()) {
            cx.model.online.refresh_lfg();
        }
        // The cloud took the post: the list shows "Your Listing" now.
        if self.lfg.page() == &Page::Post && cx.model.online.lfg.own_in(&game.title_id).is_some() {
            self.lfg.posted();
        }
        let actions = {
            let data = LfgData {
                game: &game.name,
                title_id: &game.title_id,
                online: &cx.model.online,
                now: unix_now(),
                language,
                accent,
            };
            self.lfg.show(cx.shell, cx.ctx, &events, &data)
        };
        for action in actions {
            Self::lfg_action(cx.model, &game, action);
        }
    }

    /// Opening an area of the network brings the net service up: it is local until the player
    /// signs in.
    fn start_online(&self, model: &mut Model) {
        let cloud = net::cloud(model.prefs.online_cloud.as_deref());
        if cloud.is_some() && !matches!(model.online.status, Status::Running | Status::Missing) {
            model.effects.push(Effect::StartOnline);
        }
    }

    fn lfg_action(model: &mut Model, game: &Game, action: LfgAction) {
        match action {
            LfgAction::Close => {
                model.online.lfg.close();
                model.leave_find_players();
            }
            LfgAction::SignIn | LfgAction::OpenFriends => model.open(Overlay::Friends),
            LfgAction::Refresh => model.online.refresh_lfg(),
            LfgAction::Post {
                mode,
                language,
                note,
                slots,
            } => model
                .online
                .post_lfg(game.title_id.clone(), mode, language, note, slots),
            LfgAction::CloseListing(id) => model.online.close_lfg(&id),
            LfgAction::Ask(id) => model.online.ask_lfg(&id),
            LfgAction::Withdraw(id) => model.online.withdraw_lfg(&id),
            LfgAction::Answer {
                listing,
                account,
                accept,
            } => model.online.answer_lfg(&listing, account, accept),
            LfgAction::Report {
                listing,
                poster,
                reason,
            } => model.online.report_listing(poster, &listing, reason),
            LfgAction::Block(account) => {
                model
                    .online
                    .send(Purpose::Block(account), Command::Block { account });
            }
        }
    }

    pub(super) fn send_capture_ui(&mut self, cx: &mut OverlayCx<'_>, wanted: bool) {
        let events = self.take_events(wanted);
        let side_next = cx.model.overlay.is_some_and(Overlay::is_side_panel);
        let Some(outgoing) = cx.model.lfg.outgoing.clone() else {
            if wanted {
                cx.model.dismiss_overlay();
            }
            return;
        };
        let (language, accent) = (cx.model.language(), cx.shell.accent_color());
        if wanted && !self.send.is_active() {
            self.send.open();
            self.start_online(cx.model);
        } else if !wanted && self.send.is_active() {
            if side_next {
                self.send.dismiss();
            } else {
                self.send.close();
            }
        }
        if !self.send.is_active() {
            return;
        }
        let actions = {
            let data = SendData {
                name: &outgoing.name,
                online: &cx.model.online,
                language,
                accent,
            };
            self.send.show(cx.shell, cx.ctx, &events, &data)
        };
        for action in actions {
            match action {
                SendAction::Close => cx.model.switch_overlay(Overlay::Captures),
                SendAction::SignIn => cx.model.open(Overlay::Friends),
                SendAction::Friend(friend) => {
                    start_send(cx.model, friend, outgoing.clone());
                    cx.model.switch_overlay(Overlay::Captures);
                }
            }
        }
    }

    /// Once per frame: a capture handed to a friend goes up as soon as the conversation with them
    /// exists, which is asked for when the player never wrote to them.
    pub(super) fn continue_media_send(&mut self, model: &mut Model) {
        let Some(waiting) = model.lfg.waiting.clone() else {
            return;
        };
        let language = model.language();
        if !model.online.is_online() {
            model.lfg.waiting = None;
            model.toast_failure(T::SendFailed.text(language));
            return;
        }
        let conversation = model
            .online
            .social
            .direct_with(waiting.friend)
            .map(|c| c.id.clone());
        if let Some(conversation) = conversation {
            model.lfg.waiting = None;
            model
                .online
                .send_media(&conversation, &waiting.outgoing.path);
            let text = T::Sending.format(language, &[Arg::Str(&waiting.handle)]);
            model.toast(text, Icon::PAPER_PLANE_RIGHT);
        } else if !waiting.asked {
            if let Some(w) = model.lfg.waiting.as_mut() {
                w.asked = true;
            }
            model.online.send(
                Purpose::OpenConversation(waiting.friend),
                Command::OpenConversation {
                    accounts: vec![waiting.friend],
                    name: None,
                },
            );
        } else if !model
            .online
            .is_waiting(|p| matches!(p, Purpose::OpenConversation(_)))
        {
            // The conversation was refused or never came.
            model.lfg.waiting = None;
            model.toast_failure(T::SendFailed.text(language));
        }
    }
}

/// The friend was chosen: the capture waits for the conversation with them.
fn start_send(model: &mut Model, friend: AccountId, outgoing: crate::app::lfg::Outgoing) {
    let handle = model
        .online
        .known(friend)
        .map(|a| a.handle.clone())
        .unwrap_or_default();
    model.lfg.waiting = Some(Waiting {
        friend,
        handle,
        outgoing,
        asked: false,
    });
}

/// Opens the file of a media message: at once when this PC holds it, else after it was downloaded.
pub(super) fn open_media(model: &mut Model, conversation: &str, seq: u64) {
    let language = model.language();
    let Some(info) = model
        .online
        .social
        .messages(conversation)
        .iter()
        .find(|m| m.seq == seq)
        .and_then(|m| m.media.clone())
    else {
        return;
    };
    let dir = media_dir(model);
    let Some(path) = media::cache_file(&dir, &info.id, &info.content_type) else {
        return;
    };
    if path.exists() {
        open_path(model, &path);
        return;
    }
    if std::fs::create_dir_all(&dir).is_err() {
        model.toast_failure(T::OpenFailed.text(language));
        return;
    }
    model.online.media.failed.remove(&info.id);
    model
        .online
        .media
        .open
        .insert(info.id.clone(), path.clone());
    model.online.fetch_media(&info.id, &path);
    model.toast(T::Downloading.text(language), Icon::DOWNLOAD_SIMPLE);
}

/// Opens a downloaded file with the program the system has for it.
pub fn open_path(model: &mut Model, path: &std::path::Path) {
    if let Err(error) = platform::open_file(path) {
        tracing::warn!("media: {} did not open: {error}", path.display());
        let text = T::OpenFailed.text(model.language());
        model.toast_failure(text);
    }
}
