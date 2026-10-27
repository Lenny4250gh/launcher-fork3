//! Share play as the launcher knows it (network `spec/share-play.md`, "Control protocol
//! additions"). A host streams the game they play to one member of their party, who watches,
//! plays on the host's pad, or plays beside them. The host invites and changes the mode; the
//! guest accepts, leaves, or asks for a mode. The share worker, a process the net service
//! starts, captures, encodes, decodes and draws; the launcher only chooses and says in words
//! what the `share` event reports.
//!
//! - [`SharePlay`] is what the service reported: whether the player could invite someone now,
//!   the share they are in and the invitations waiting for an answer.
//! - [`SharePlayCommand`] is what the player can ask for; [`Online::share_play_send`] sends it.
//! - [`SharePlayNotice`] is what the player should hear of whatever they are looking at.

use anyplay_control::{Command, ShareRole, ShareView};

use super::api::{AccountId, ShareInvitation, ShareMode};
use super::{Event, Notice, Online, Purpose};

/// What the player asks of share play.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SharePlayCommand {
    /// The host invites a member of the party. The share starts with watching only: the guest
    /// gets a pad when the host says so.
    Invite(AccountId),
    /// The guest answers an invitation.
    Answer { id: String, accept: bool },
    /// The host changes the mode.
    Mode(ShareMode),
    /// The guest asks the host for a mode.
    Request(ShareMode),
    /// The host stops, the guest leaves, or a host still waiting for an answer takes it back.
    Stop,
}

/// Something about share play the player should hear of, whatever they are looking at.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SharePlayNotice {
    /// A party member invited the player.
    Invited(ShareInvitation),
    /// The guest asks the host for a mode.
    Requested(ShareMode),
    /// The host changed what the guest may do.
    ModeChanged(ShareMode),
    /// The share ended without the player's ending it.
    Ended { was_host: bool },
}

/// What the net service reported about share play.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SharePlay {
    can_start: bool,
    share: Option<ShareView>,
    invitations: Vec<ShareInvitation>,
    /// The player asked to stop; the end of the share is no news to them.
    stopping: bool,
}

impl SharePlay {
    /// Forgets everything: signed out, or the service ended.
    pub fn reset(&mut self) {
        *self = SharePlay::default();
    }

    /// True when anything about share play belongs on the screen: a button may show, or there
    /// is a share or an invitation to show.
    pub fn visible(&self) -> bool {
        self.can_start || self.share.is_some() || !self.invitations.is_empty()
    }

    /// The player could invite a party member now: a game runs through AnyPlay in a party, the
    /// worker can encode, the cloud offers share play and the player is in no share.
    pub fn can_start(&self) -> bool {
        self.can_start && self.share.is_none()
    }

    /// The share the player is in.
    pub fn share(&self) -> Option<&ShareView> {
        self.share.as_ref()
    }

    /// True while the player is the host of a share.
    pub fn hosting(&self) -> bool {
        self.share
            .as_ref()
            .is_some_and(|s| s.role == ShareRole::Host)
    }

    /// The invitations waiting for an answer, the newest last.
    pub fn invitations(&self) -> &[ShareInvitation] {
        &self.invitations
    }

    /// Takes over one `share` event and tells what changed that the player should hear of.
    pub fn changed(&mut self, can_start: bool, share: Option<ShareView>) -> Vec<SharePlayNotice> {
        let mut notices = Vec::new();
        if let (Some(was), Some(now)) = (&self.share, &share)
            && was.id == now.id
            && was.mode != now.mode
            && now.role == ShareRole::Guest
        {
            notices.push(SharePlayNotice::ModeChanged(now.mode));
        }
        if let (Some(was), None) = (&self.share, &share)
            && !self.stopping
        {
            notices.push(SharePlayNotice::Ended {
                was_host: was.role == ShareRole::Host,
            });
        }
        if share.is_none() {
            self.stopping = false;
        }
        self.can_start = can_start;
        self.share = share;
        notices
    }

    fn invited(&mut self, invitation: ShareInvitation) -> Option<SharePlayNotice> {
        if self.invitations.iter().any(|i| i.id == invitation.id) {
            return None;
        }
        self.invitations.push(invitation.clone());
        Some(SharePlayNotice::Invited(invitation))
    }

    fn invitation_gone(&mut self, id: &str) {
        self.invitations.retain(|i| i.id != id);
    }

    /// A stop failed: the share did not end by the player's choice after all.
    pub fn forget_expectations(&mut self) {
        self.stopping = false;
    }
}

impl Online {
    /// Sends what the player asked of share play.
    pub fn share_play_send(&mut self, command: SharePlayCommand) {
        match command {
            SharePlayCommand::Invite(account) => {
                self.send(
                    Purpose::ShareStart,
                    Command::ShareStart {
                        account,
                        mode: ShareMode::Watch,
                    },
                );
            }
            SharePlayCommand::Answer { id, accept } => {
                // A declined invitation leaves the list at once.
                if !accept {
                    self.share_play.invitation_gone(&id);
                }
                self.send(
                    Purpose::ShareAnswer { accept },
                    Command::ShareAnswer { id, accept },
                );
            }
            SharePlayCommand::Mode(mode) => {
                self.send(Purpose::ShareMode, Command::ShareMode { mode });
            }
            SharePlayCommand::Request(mode) => {
                self.send(Purpose::ShareRequest, Command::ShareRequest { mode });
            }
            SharePlayCommand::Stop => {
                self.share_play.stopping = self.share_play.share.is_some();
                self.send(Purpose::ShareStop, Command::ShareStop);
            }
        }
    }

    /// The events of share play.
    pub(super) fn share_play_event(&mut self, event: Event, notices: &mut Vec<Notice>) {
        match event {
            Event::Share { can_start, share } => notices.extend(
                self.share_play
                    .changed(can_start, share)
                    .into_iter()
                    .map(Notice::SharePlay),
            ),
            Event::ShareInvitation { invitation } => {
                notices.extend(self.share_play.invited(invitation).map(Notice::SharePlay));
            }
            Event::ShareInvitationGone { id } => self.share_play.invitation_gone(&id),
            Event::ShareRequest { mode } => {
                notices.push(Notice::SharePlay(SharePlayNotice::Requested(mode)));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use anyplay_control::{Link, SharePhase, ShareStats};

    use super::*;
    use crate::online::api::Account;
    use crate::online::{Line, Output};

    fn account(id: u64, handle: &str) -> Account {
        Account {
            id: AccountId(id),
            handle: handle.into(),
        }
    }

    fn view(role: ShareRole, mode: ShareMode, phase: SharePhase) -> ShareView {
        ShareView {
            id: "s1".into(),
            role,
            peer: account(2, "Lumen"),
            title: "Astro Bot".into(),
            mode,
            phase,
            link: Some(Link::Direct),
            codec: None,
            stats: ShareStats::default(),
            requested: None,
        }
    }

    fn invitation(id: &str) -> ShareInvitation {
        ShareInvitation {
            id: id.into(),
            from: account(2, "Lumen"),
            share: "s1".into(),
            title_id: "T".into(),
            title: "Astro Bot".into(),
            mode: ShareMode::Watch,
            expires: 99,
        }
    }

    fn event(online: &mut Online, event: Event) -> Vec<Notice> {
        online.apply(Line::Output(Output::Event(event)))
    }

    #[test]
    fn nothing_shows_until_there_is_something_to_say() {
        let mut share = SharePlay::default();
        assert!(!share.visible());
        share.changed(true, None);
        assert!(share.visible() && share.can_start());
        share.changed(
            true,
            Some(view(ShareRole::Host, ShareMode::Watch, SharePhase::Invited)),
        );
        assert!(
            !share.can_start(),
            "a player in a share cannot start another"
        );
        assert!(share.hosting());
    }

    #[test]
    fn an_invitation_is_heard_once_and_gone_when_the_service_says() {
        let mut online = Online::default();
        let notices = event(
            &mut online,
            Event::ShareInvitation {
                invitation: invitation("i1"),
            },
        );
        assert_eq!(
            notices,
            vec![Notice::SharePlay(SharePlayNotice::Invited(invitation(
                "i1"
            )))]
        );
        let again = event(
            &mut online,
            Event::ShareInvitation {
                invitation: invitation("i1"),
            },
        );
        assert!(again.is_empty());
        assert_eq!(online.share_play.invitations().len(), 1);
        event(&mut online, Event::ShareInvitationGone { id: "i1".into() });
        assert!(online.share_play.invitations().is_empty());
    }

    #[test]
    fn a_guest_hears_the_mode_change_and_the_end() {
        let mut online = Online::default();
        event(
            &mut online,
            Event::Share {
                can_start: false,
                share: Some(view(ShareRole::Guest, ShareMode::Watch, SharePhase::Live)),
            },
        );
        let notices = event(
            &mut online,
            Event::Share {
                can_start: false,
                share: Some(view(
                    ShareRole::Guest,
                    ShareMode::HandOver,
                    SharePhase::Live,
                )),
            },
        );
        assert_eq!(
            notices,
            vec![Notice::SharePlay(SharePlayNotice::ModeChanged(
                ShareMode::HandOver
            ))]
        );
        let notices = event(
            &mut online,
            Event::Share {
                can_start: false,
                share: None,
            },
        );
        assert_eq!(
            notices,
            vec![Notice::SharePlay(SharePlayNotice::Ended {
                was_host: false
            })]
        );
    }

    #[test]
    fn a_stop_the_player_asked_for_is_no_news() {
        let mut online = Online::default();
        event(
            &mut online,
            Event::Share {
                can_start: false,
                share: Some(view(ShareRole::Host, ShareMode::Watch, SharePhase::Live)),
            },
        );
        online.share_play_send(SharePlayCommand::Stop);
        let notices = event(
            &mut online,
            Event::Share {
                can_start: true,
                share: None,
            },
        );
        assert!(notices.is_empty());
        // The host's own mode change is not announced to the host either.
        event(
            &mut online,
            Event::Share {
                can_start: false,
                share: Some(view(ShareRole::Host, ShareMode::Watch, SharePhase::Live)),
            },
        );
        let notices = event(
            &mut online,
            Event::Share {
                can_start: false,
                share: Some(view(ShareRole::Host, ShareMode::Together, SharePhase::Live)),
            },
        );
        assert!(notices.is_empty());
    }

    #[test]
    fn the_request_notice_and_the_commands_are_what_the_spec_names() {
        let mut online = Online::default();
        let notices = event(
            &mut online,
            Event::ShareRequest {
                mode: ShareMode::Together,
            },
        );
        assert_eq!(
            notices,
            vec![Notice::SharePlay(SharePlayNotice::Requested(
                ShareMode::Together
            ))]
        );
        online.share_play_send(SharePlayCommand::Invite(AccountId(7)));
        online.share_play_send(SharePlayCommand::Answer {
            id: "i1".into(),
            accept: true,
        });
        online.share_play_send(SharePlayCommand::Mode(ShareMode::HandOver));
        online.share_play_send(SharePlayCommand::Request(ShareMode::Together));
        online.share_play_send(SharePlayCommand::Stop);
        let sent = online.take_outbox();
        assert!(sent[0].contains("\"do\":\"share_start\"") && sent[0].contains("\"watch\""));
        assert!(
            sent[1].contains("\"do\":\"share_answer\"")
                && sent[1].contains("\"invitation\":\"i1\"")
        );
        assert!(sent[2].contains("\"do\":\"share_mode\"") && sent[2].contains("hand_over"));
        assert!(sent[3].contains("\"do\":\"share_request\""));
        assert!(sent[4].contains("\"do\":\"share_stop\""));
    }
}
