//! The Notification Center in the panel host: builds the entries from the model's history, marks
//! them read when the area opens, and runs the buttons (docs/DESIGN.md 9.12). An answer goes the
//! way it goes from the Friends screen (`online_action`), so a notification never has a path of
//! its own.

use anyps5_core::links::SettingsPage;
use anyps5_core::notifications::Act;
use anyps5_core::online::api::AccountId;

use super::PanelHost;
use super::friends::online_action;
use crate::app::clock;
use crate::app::model::Model;
use crate::app::overlay::{Overlay, OverlayCx};
use crate::screens::notification_strings::NotifyText as NT;
use crate::screens::notifications::{Item, NotifyAction, NotifyData};
use crate::screens::online::{OnlineAction, Page};
use crate::ui::icons::Icon;

/// The label of an entry's main button, or none when there is nothing left to do with it: a
/// request that was answered elsewhere, an invitation that ran out.
pub(crate) fn primary_of(act: &Act, model: &Model) -> Option<NT> {
    let online = &model.online;
    match act {
        Act::None => None,
        Act::AcceptFriend(account) => online
            .friends
            .incoming
            .iter()
            .any(|r| r.account.id.0 == *account)
            .then_some(NT::Accept),
        Act::JoinInvitation(id) => online
            .invitations
            .iter()
            .any(|i| i.id == *id)
            .then_some(NT::Join),
        Act::JoinParty(id) => online
            .social
            .party_invitations
            .iter()
            .any(|i| i.id == *id)
            .then_some(NT::Join),
        Act::OpenTrophies(game) => model
            .games
            .iter()
            .any(|g| g.id.as_str() == game)
            .then_some(NT::Open),
        Act::OpenFriends
        | Act::OpenParty
        | Act::OpenMessages(_)
        | Act::OpenUpdate
        | Act::OpenToolkit => Some(NT::Open),
    }
}

impl PanelHost {
    /// Once per frame: the link the player asked for goes to the clipboard as soon as the network
    /// issued the code, or the wait ends with a word.
    pub(super) fn copy_invite_link(&mut self, cx: &mut OverlayCx<'_>) {
        let Some(deadline) = cx.model.invite_link.copy_before else {
            return;
        };
        let language = cx.model.language();
        let link = cx
            .model
            .online
            .join
            .code(anyps5_core::online::unix_now())
            .and_then(anyps5_core::links::join_link);
        if let Some(link) = link {
            cx.model.invite_link.copy_before = None;
            cx.ctx.copy_text(link);
            cx.model
                .toast(NT::InviteLinkCopied.text(language), Icon::CHECK);
        } else if cx.model.now > deadline {
            cx.model.invite_link.copy_before = None;
            cx.model
                .toast(NT::InviteLinkFailed.text(language), Icon::WARNING);
        }
    }

    pub(super) fn notifications_ui(&mut self, cx: &mut OverlayCx<'_>, wanted: bool) {
        let events = self.take_events(wanted);
        if wanted && !self.notifications.is_active() {
            let fresh = cx
                .model
                .notes
                .newest_first()
                .filter(|e| !e.read)
                .map(|e| e.id)
                .collect();
            self.notifications.open(fresh);
        } else if !wanted && self.notifications.is_open() {
            self.notifications.close();
        }
        // What arrives while the area is open is seen at once.
        if wanted {
            cx.model.notes.mark_all_read();
        }
        let language = cx.model.language();
        let items: Vec<Item> = cx
            .model
            .notes
            .newest_first()
            .map(|e| Item {
                id: e.id,
                kind: e.kind,
                text: e.text.clone(),
                at: e.at,
                primary: primary_of(&e.act, cx.model),
            })
            .collect();
        let now = clock::unix_seconds();
        let data = NotifyData {
            items: &items,
            language,
            now,
            utc_offset: crate::screens::format::local_offset_seconds(now.saturating_mul(1_000_000)),
        };
        let actions = self.notifications.show(cx.shell, cx.ctx, &events, &data);
        for action in actions {
            match action {
                NotifyAction::Close => cx.model.dismiss_overlay(),
                NotifyAction::Dismiss(id) => cx.model.notes.remove(id),
                NotifyAction::ClearAll => cx.model.notes.clear(),
                NotifyAction::Run(id) => {
                    let act = cx.model.notes.get(id).map(|e| e.act.clone());
                    if let Some(act) = act {
                        self.run_note(cx.model, id, act);
                    }
                }
            }
        }
    }

    /// The main button of an entry.
    fn run_note(&mut self, model: &mut Model, id: u64, act: Act) {
        match act {
            Act::None => {}
            // An answer takes the entry with it: it is done.
            Act::AcceptFriend(account) => {
                online_action(model, OnlineAction::Accept(AccountId(account)));
                model.notes.remove(id);
            }
            Act::JoinInvitation(invitation) => {
                model.dismiss_overlay();
                online_action(
                    model,
                    OnlineAction::AnswerInvitation {
                        id: invitation,
                        accept: true,
                    },
                );
                model.notes.remove(id);
            }
            Act::JoinParty(invitation) => {
                online_action(
                    model,
                    OnlineAction::PartyAnswer {
                        id: invitation,
                        accept: true,
                    },
                );
                model.notes.remove(id);
            }
            Act::OpenFriends => self.open_friends_on(model, None),
            Act::OpenParty => self.open_friends_on(model, Some(Page::Party)),
            Act::OpenMessages(None) => self.open_friends_on(model, Some(Page::Messages)),
            Act::OpenMessages(Some(conversation)) => {
                self.open_friends_on(model, Some(Page::Conversation(conversation)));
            }
            Act::OpenTrophies(game) => {
                let found = model.games.iter().position(|g| g.id.as_str() == game);
                if let Some(index) = found {
                    model.selected = index;
                    model.switch_overlay(Overlay::Trophies);
                }
            }
            Act::OpenUpdate => model.show_settings(Some(SettingsPage::About)),
            Act::OpenToolkit => model.show_settings(Some(SettingsPage::Toolkit)),
        }
    }

    /// The Friends screen, on a page when one is named.
    fn open_friends_on(&mut self, model: &mut Model, page: Option<Page>) {
        self.online_start = page.map(|page| (page, None));
        model.open(Overlay::Friends);
    }
}
