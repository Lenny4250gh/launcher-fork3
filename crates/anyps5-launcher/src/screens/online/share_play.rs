//! Share play in the online panel: the group on the party page and the line on Home
//! (docs/DESIGN.md 17, "Share play"). Rows like any other, so the keyboard, the pad and the
//! screen reader get them from the panel: state is said in words, an icon only repeats it, and
//! nothing here moves. The picture itself is not here; the share worker opens its own window.
//!
//! The group shows only what there is, and one thing at a time: the invitations waiting for the
//! player, else the share they are in (as host or as guest), else a way to start one with each
//! other member of the party. Starting always begins with watching only; the host moves the
//! guest to a pad with the mode row.

use anyps5_core::online::api::{AccountId, ShareMode};
use anyps5_core::online::control::{SharePhase, ShareRole, ShareView};
use anyps5_core::online::{Online, SharePlayCommand};

use super::screen::path_text;
use super::{OnlineAction, OnlineData, OnlinePanel};
use crate::screens::online_share_strings::ShareText as X;
use crate::screens::rows::{Change, Row};
use crate::ui::i18n::{Arg, Language};
use crate::ui::icons::Icon;

/// The other members of the party, in the party's order: whom the player can start a share with.
fn others(online: &Online) -> Vec<(AccountId, String)> {
    let me = online.account.as_ref().map(|a| a.id);
    online
        .social
        .party
        .as_ref()
        .map(|party| {
            party
                .members
                .iter()
                .filter(|m| Some(m.account.id) != me)
                .map(|m| (m.account.id, m.account.handle.clone()))
                .collect()
        })
        .unwrap_or_default()
}

const MODES: [ShareMode; 3] = [ShareMode::Watch, ShareMode::HandOver, ShareMode::Together];

fn mode_index(mode: ShareMode) -> usize {
    MODES.iter().position(|m| *m == mode).unwrap_or(0)
}

fn phase_text(phase: SharePhase) -> X {
    match phase {
        SharePhase::Live => X::PhaseLive,
        SharePhase::Stalled => X::PhaseStalled,
        SharePhase::Invited | SharePhase::Connecting => X::PhaseConnecting,
    }
}

/// What the mode lets the guest do, said to the host.
fn mode_note(mode: ShareMode) -> X {
    match mode {
        ShareMode::HandOver => X::NoteHandOver,
        ShareMode::Together => X::NoteTogether,
        ShareMode::Watch | ShareMode::Unknown => X::NoteWatch,
    }
}

fn host_rows(share: &ShareView, language: Language, group: &mut Vec<Row>) {
    let t = |text: X| text.text(language);
    let name = share.peer.handle.as_str();
    if share.phase == SharePhase::Invited {
        group.push(
            Row::info(
                "shareWaiting",
                X::WaitingFor.format(language, &[Arg::Str(name)]),
                Some(t(X::InvitationSent).to_owned()),
            )
            .icon(Icon::GAME_CONTROLLER),
        );
        group.push(Row::command("shareStop", t(X::CancelInvitation)).icon(Icon::X));
        return;
    }
    group.push(
        Row::info(
            "shareHost",
            X::SeesYourGame.format(language, &[Arg::Str(name)]),
            Some(t(phase_text(share.phase)).to_owned()),
        )
        .icon(Icon::APP_WINDOW)
        .detail(format!(
            "{} · {}",
            share.title,
            path_text(share.link, share.stats.rtt_ms, language)
        )),
    );
    if let Some(asked) = share.requested {
        let text = match asked {
            ShareMode::HandOver => X::AsksHandOver,
            ShareMode::Together => X::AsksTogether,
            ShareMode::Watch | ShareMode::Unknown => X::AsksWatch,
        };
        group.push(Row::info(
            "shareAsks",
            text.format(language, &[Arg::Str(name)]),
            None,
        ));
        group.push(Row::info("shareAsksNote", t(X::AsksDetail), None).attached());
    }
    group.push(
        Row::choice(
            "shareMode",
            t(X::Mode),
            vec![
                t(X::ModeWatch).to_owned(),
                t(X::ModeHandOver).to_owned(),
                t(X::ModeTogether).to_owned(),
            ],
            mode_index(share.mode),
        )
        .icon(Icon::SLIDERS_HORIZONTAL),
    );
    group.push(Row::info("shareModeNote", t(mode_note(share.mode)), None).attached());
    group.push(Row::command("shareStop", t(X::StopSharePlay)).icon(Icon::X));
}

fn guest_rows(share: &ShareView, language: Language, group: &mut Vec<Row>) {
    let t = |text: X| text.text(language);
    let name = share.peer.handle.as_str();
    group.push(
        Row::info(
            "shareGuest",
            X::GuestOf.format(language, &[Arg::Str(name), Arg::Str(&share.title)]),
            Some(t(phase_text(share.phase)).to_owned()),
        )
        .icon(Icon::APP_WINDOW)
        .detail(path_text(share.link, share.stats.rtt_ms, language)),
    );
    group.push(Row::info("shareWindow", t(X::WindowNote), None).attached());
    let part = match share.mode {
        ShareMode::HandOver => X::PartHandOver,
        ShareMode::Together => X::PartTogether,
        ShareMode::Watch | ShareMode::Unknown => X::PartWatching,
    };
    group.push(
        Row::info("shareYou", t(X::YourPart), Some(t(part).to_owned())).icon(Icon::GAME_CONTROLLER),
    );
    if share.mode != ShareMode::HandOver {
        group.push(
            Row::command("shareAskHandOver", t(X::AskHandOver))
                .icon(Icon::GAME_CONTROLLER)
                .detail(t(X::AskDetail)),
        );
    }
    if share.mode != ShareMode::Together {
        group.push(
            Row::command("shareAskTogether", t(X::AskTogether))
                .icon(Icon::GAME_CONTROLLER)
                .detail(t(X::AskDetail)),
        );
    }
    group.push(Row::command("shareStop", t(X::LeaveSharePlay)).icon(Icon::X));
}

/// The same things as cards of the room: one thing at a time, each with its one button.
pub(super) fn cards(data: &OnlineData<'_>) -> Vec<super::room::Card> {
    use super::room::{Card, CardAction};
    let online = data.online;
    let play = &online.share_play;
    let language = data.language;
    let t = |text: X| text.text(language).to_owned();
    let card = |icon, title: String, detail: String, label: String, command| Card {
        icon,
        title,
        detail,
        action: Some((label, CardAction::Share(command))),
    };
    let mut cards = Vec::new();
    if let Some(share) = play.share() {
        let name = share.peer.handle.as_str();
        if share.role == ShareRole::Host {
            if share.phase == SharePhase::Invited {
                cards.push(card(
                    Icon::GAME_CONTROLLER,
                    X::WaitingFor.format(language, &[Arg::Str(name)]),
                    t(X::InvitationSent),
                    t(X::CancelInvitation),
                    SharePlayCommand::Stop,
                ));
                return cards;
            }
            let mut detail = format!(
                "{} · {}",
                t(phase_text(share.phase)),
                path_text(share.link, share.stats.rtt_ms, language)
            );
            if let Some(asked) = share.requested {
                let text = match asked {
                    ShareMode::HandOver => X::AsksHandOver,
                    ShareMode::Together => X::AsksTogether,
                    ShareMode::Watch | ShareMode::Unknown => X::AsksWatch,
                };
                detail = text.format(language, &[Arg::Str(name)]);
            }
            cards.push(card(
                Icon::APP_WINDOW,
                X::SeesYourGame.format(language, &[Arg::Str(name)]),
                detail,
                t(X::StopSharePlay),
                SharePlayCommand::Stop,
            ));
            for (mode, label, note) in [
                (ShareMode::Watch, X::ModeWatch, X::NoteWatch),
                (ShareMode::HandOver, X::ModeHandOver, X::NoteHandOver),
                (ShareMode::Together, X::ModeTogether, X::NoteTogether),
            ] {
                if mode != share.mode {
                    cards.push(card(
                        Icon::GAME_CONTROLLER,
                        t(label),
                        t(note),
                        t(X::Switch),
                        SharePlayCommand::Mode(mode),
                    ));
                }
            }
        } else {
            let part = match share.mode {
                ShareMode::HandOver => X::PartHandOver,
                ShareMode::Together => X::PartTogether,
                ShareMode::Watch | ShareMode::Unknown => X::PartWatching,
            };
            cards.push(card(
                Icon::APP_WINDOW,
                X::GuestOf.format(language, &[Arg::Str(name), Arg::Str(&share.title)]),
                format!("{} · {}", t(part), t(X::WindowNote)),
                t(X::LeaveSharePlay),
                SharePlayCommand::Stop,
            ));
            for (mode, title) in [
                (ShareMode::HandOver, X::AskHandOver),
                (ShareMode::Together, X::AskTogether),
            ] {
                if mode != share.mode {
                    cards.push(card(
                        Icon::GAME_CONTROLLER,
                        t(title),
                        t(X::AskDetail),
                        t(X::Ask),
                        SharePlayCommand::Request(mode),
                    ));
                }
            }
        }
        return cards;
    }
    for invitation in play.invitations() {
        let text = match invitation.mode {
            ShareMode::HandOver => X::InvitedHandOver,
            ShareMode::Together => X::InvitedTogether,
            ShareMode::Watch | ShareMode::Unknown => X::InvitedWatch,
        };
        let title = text.format(
            language,
            &[
                Arg::Str(&invitation.from.handle),
                Arg::Str(&invitation.title),
            ],
        );
        cards.push(card(
            Icon::GAME_CONTROLLER,
            title,
            t(X::SharePlay),
            t(X::Accept),
            SharePlayCommand::Answer {
                id: invitation.id.clone(),
                accept: true,
            },
        ));
        cards.push(card(
            Icon::X,
            t(X::Decline),
            invitation.from.handle.clone(),
            t(X::Decline),
            SharePlayCommand::Answer {
                id: invitation.id.clone(),
                accept: false,
            },
        ));
    }
    if cards.is_empty() && play.can_start() {
        for (account, name) in others(online) {
            cards.push(card(
                Icon::GAME_CONTROLLER,
                X::ShareWith.format(language, &[Arg::Str(&name)]),
                t(X::ShareWithDetail),
                t(X::Invite),
                SharePlayCommand::Invite(account),
            ));
        }
    }
    cards
}

/// The group on the party page. Adds nothing when there is nothing to show.
pub(super) fn rows(data: &OnlineData<'_>, rows: &mut Vec<Row>) {
    let online = data.online;
    let play = &online.share_play;
    let language = data.language;
    if !play.visible() {
        return;
    }
    let t = |text: X| text.text(language);
    let mut group: Vec<Row> = Vec::new();

    if let Some(share) = play.share() {
        match share.role {
            ShareRole::Host => host_rows(share, language, &mut group),
            ShareRole::Guest => guest_rows(share, language, &mut group),
        }
    } else {
        for invitation in play.invitations() {
            let text = match invitation.mode {
                ShareMode::HandOver => X::InvitedHandOver,
                ShareMode::Together => X::InvitedTogether,
                ShareMode::Watch | ShareMode::Unknown => X::InvitedWatch,
            };
            group.push(
                Row::info(
                    "shareInvited",
                    text.format(
                        language,
                        &[
                            Arg::Str(&invitation.from.handle),
                            Arg::Str(&invitation.title),
                        ],
                    ),
                    None,
                )
                .icon(Icon::GAME_CONTROLLER),
            );
            group.push(Row::command("shareAccept", t(X::Accept)).icon(Icon::CHECK));
            group.push(Row::command("shareDecline", t(X::Decline)).icon(Icon::X));
        }
        if group.is_empty() && play.can_start() {
            for (_, name) in others(online) {
                group.push(
                    Row::command(
                        "shareWith",
                        X::ShareWith.format(language, &[Arg::Str(&name)]),
                    )
                    .icon(Icon::GAME_CONTROLLER)
                    .detail(t(X::ShareWithDetail)),
                );
            }
        }
    }

    if group.is_empty() {
        return;
    }
    rows.push(Row::caption("sharePlayCaption", t(X::SharePlay)).group_start());
    rows.extend(group);
}

/// The line on Home: only while there is something to say. An invitation shows even before
/// the player is in a party: it must be seen.
pub(super) fn home_row(data: &OnlineData<'_>, rows: &mut Vec<Row>) {
    let online = data.online;
    let play = &online.share_play;
    let language = data.language;
    let detail = if let Some(share) = play.share() {
        let name = share.peer.handle.as_str();
        match share.role {
            ShareRole::Host => X::SeesYourGame.format(language, &[Arg::Str(name)]),
            ShareRole::Guest => {
                X::GuestOf.format(language, &[Arg::Str(name), Arg::Str(&share.title)])
            }
        }
    } else if let Some(invitation) = play.invitations().first() {
        let text = match invitation.mode {
            ShareMode::HandOver => X::InvitedHandOver,
            ShareMode::Together => X::InvitedTogether,
            ShareMode::Watch | ShareMode::Unknown => X::InvitedWatch,
        };
        text.format(
            language,
            &[
                Arg::Str(&invitation.from.handle),
                Arg::Str(&invitation.title),
            ],
        )
    } else {
        return;
    };
    rows.push(
        Row::action("sharePlay", X::SharePlay.text(language))
            .icon(Icon::GAME_CONTROLLER)
            .detail(detail),
    );
}

impl OnlinePanel {
    /// What the rows of the group do. True when `row` was one of them.
    pub(super) fn share_play_run(
        &mut self,
        row: &str,
        data: &OnlineData<'_>,
        actions: &mut Vec<OnlineAction>,
    ) -> bool {
        let play = &data.online.share_play;
        match row {
            "shareWith" => {
                let index = self.nth(data, "shareWith");
                if let Some((account, _)) = others(data.online).get(index) {
                    actions.push(OnlineAction::SharePlay(SharePlayCommand::Invite(*account)));
                }
            }
            "shareAccept" | "shareDecline" => {
                let index = self.nth(
                    data,
                    if row == "shareAccept" {
                        "shareAccept"
                    } else {
                        "shareDecline"
                    },
                );
                if let Some(invitation) = play.invitations().get(index) {
                    actions.push(OnlineAction::SharePlay(SharePlayCommand::Answer {
                        id: invitation.id.clone(),
                        accept: row == "shareAccept",
                    }));
                }
            }
            "shareStop" => actions.push(OnlineAction::SharePlay(SharePlayCommand::Stop)),
            "shareAskHandOver" => actions.push(OnlineAction::SharePlay(SharePlayCommand::Request(
                ShareMode::HandOver,
            ))),
            "shareAskTogether" => actions.push(OnlineAction::SharePlay(SharePlayCommand::Request(
                ShareMode::Together,
            ))),
            // The Home line leads to where the share is handled.
            "sharePlay" => self.go(super::Page::Party, data),
            _ => return false,
        }
        true
    }

    /// The choice of the mode row. True when `change` was one of the group's.
    pub(super) fn share_play_change(
        &mut self,
        change: &Change,
        actions: &mut Vec<OnlineAction>,
    ) -> bool {
        match change {
            Change::Choice {
                row: "shareMode",
                index,
            } => {
                if let Some(mode) = MODES.get(*index) {
                    actions.push(OnlineAction::SharePlay(SharePlayCommand::Mode(*mode)));
                }
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests;
