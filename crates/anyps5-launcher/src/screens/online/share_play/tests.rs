use anyps5_core::online::api::{Account, Party, PartyMember, ShareInvitation};
use anyps5_core::online::control::{Event, Link, ShareStats};
use anyps5_core::online::{Cloud, Line, Output, SignedIn, Status};
use egui::Color32;

use super::*;
use crate::screens::online::Page;

fn account(id: u64, handle: &str) -> Account {
    Account {
        id: AccountId(id),
        handle: handle.into(),
    }
}

fn online() -> Online {
    let mut online = Online::default();
    online.status = Status::Running;
    online.cloud = Some(Cloud::Online);
    online.account = Some(SignedIn {
        id: AccountId(1),
        handle: "Mira".into(),
        email: "mira@example.org".into(),
        appear_offline: false,
    });
    online.social.apply_party(Some(Party {
        id: "p".into(),
        leader: AccountId(1),
        members: [(1, "Mira"), (2, "Lumen"), (3, "Kit")]
            .iter()
            .map(|(id, handle)| PartyMember {
                account: account(*id, handle),
                joined: 0,
                endpoint: String::new(),
                relay: None,
                voice: false,
            })
            .collect(),
        max_members: 8,
        conversation: "pc".into(),
        created: 0,
        revision: 1,
        discord: None,
    }));
    online
}

fn data(online: &Online) -> OnlineData<'_> {
    OnlineData {
        online,
        cloud: Some("https://cloud.example.org"),
        custom_cloud: false,
        official_cloud: true,
        relay_only: false,
        language: Language::English,
        accent: Color32::WHITE,
        now: 1_792_670_400,
        utc_offset: Some(0),
        game_running: true,
        play_together: None,
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
        stats: ShareStats {
            rtt_ms: Some(24),
            ..ShareStats::default()
        },
        requested: None,
    }
}

fn report(online: &mut Online, can_start: bool, share: Option<ShareView>) {
    online.apply(Line::Output(Output::Event(Event::Share {
        can_start,
        share,
    })));
}

fn invite(online: &mut Online, id: &str, mode: ShareMode) {
    online.apply(Line::Output(Output::Event(Event::ShareInvitation {
        invitation: ShareInvitation {
            id: id.into(),
            from: account(2, "Lumen"),
            share: "s1".into(),
            title_id: "T".into(),
            title: "Astro Bot".into(),
            mode,
            expires: 99,
        },
    })));
}

fn party_rows(online: &Online) -> Vec<Row> {
    let mut panel = OnlinePanel::default();
    panel.go(Page::Party, &data(online));
    panel.rows(&data(online)).2
}

fn ids(rows: &[Row]) -> Vec<&'static str> {
    rows.iter().map(|r| r.id).collect()
}

fn row<'a>(rows: &'a [Row], id: &str) -> &'a Row {
    rows.iter().find(|r| r.id == id).expect(id)
}

fn value(row: &Row) -> Option<&str> {
    match &row.control {
        crate::screens::rows::Control::Info(value) => value.as_deref(),
        _ => None,
    }
}

fn press(online: &Online, id: &'static str, nth: usize) -> Vec<OnlineAction> {
    let mut panel = OnlinePanel::default();
    panel.go(Page::Party, &data(online));
    let rows = panel.rows(&data(online)).2;
    panel.side.nav.row = crate::screens::rows::position(&rows, id).unwrap() + nth;
    let mut actions = Vec::new();
    panel.run(id, &data(online), &mut actions);
    actions
}

#[test]
fn nothing_shows_while_nothing_is_possible() {
    let mut online = online();
    let before = party_rows(&online);
    report(&mut online, false, None);
    assert_eq!(party_rows(&online), before);
    assert!(!ids(&before).contains(&"sharePlayCaption"));
}

#[test]
fn a_game_that_runs_offers_one_row_per_other_member_and_starts_with_watching() {
    let mut online = online();
    report(&mut online, true, None);
    let rows = party_rows(&online);
    let with: Vec<&Row> = rows.iter().filter(|r| r.id == "shareWith").collect();
    assert_eq!(with.len(), 2, "not the player themselves");
    assert_eq!(with[0].title, "Share with Lumen");
    assert_eq!(with[1].title, "Share with Kit");
    assert_eq!(
        with[0].detail.as_deref(),
        Some("They watch first. You decide when they play.")
    );
    assert_eq!(
        press(&online, "shareWith", 1),
        [OnlineAction::SharePlay(SharePlayCommand::Invite(
            AccountId(3)
        ))]
    );
}

#[test]
fn a_host_waiting_for_an_answer_can_take_the_invitation_back() {
    let mut online = online();
    report(
        &mut online,
        false,
        Some(view(ShareRole::Host, ShareMode::Watch, SharePhase::Invited)),
    );
    let rows = party_rows(&online);
    assert!(!ids(&rows).contains(&"shareWith"));
    assert_eq!(row(&rows, "shareWaiting").title, "Waiting for Lumen");
    assert_eq!(
        press(&online, "shareStop", 0),
        [OnlineAction::SharePlay(SharePlayCommand::Stop)]
    );
}

#[test]
fn the_host_sees_the_guest_the_path_and_one_mode_row_with_what_it_means() {
    let mut online = online();
    report(
        &mut online,
        false,
        Some(view(ShareRole::Host, ShareMode::HandOver, SharePhase::Live)),
    );
    let rows = party_rows(&online);
    let host = row(&rows, "shareHost");
    assert_eq!(host.title, "Lumen sees your game");
    assert_eq!(host.detail.as_deref(), Some("Astro Bot · Direct, 24 ms"));
    match &row(&rows, "shareMode").control {
        crate::screens::rows::Control::Choice { options, selected } => {
            assert_eq!(options, &["Watch", "Take over", "Together"]);
            assert_eq!(*selected, 1);
        }
        other => panic!("a choice, got {other:?}"),
    }
    assert!(
        row(&rows, "shareModeNote")
            .title
            .contains("Select and Start")
    );
    assert!(row(&rows, "shareModeNote").attached);

    let mut panel = OnlinePanel::default();
    let mut actions = Vec::new();
    assert!(panel.share_play_change(
        &Change::Choice {
            row: "shareMode",
            index: 2
        },
        &mut actions
    ));
    assert_eq!(
        actions,
        [OnlineAction::SharePlay(SharePlayCommand::Mode(
            ShareMode::Together
        ))]
    );
}

#[test]
fn a_request_is_said_with_a_note_and_changes_nothing() {
    let mut online = online();
    let mut share = view(ShareRole::Host, ShareMode::Watch, SharePhase::Live);
    share.requested = Some(ShareMode::HandOver);
    report(&mut online, false, Some(share));
    let rows = party_rows(&online);
    assert_eq!(
        row(&rows, "shareAsks").title,
        "Lumen would like to play with your pad"
    );
    assert_eq!(
        row(&rows, "shareAsksNote").title,
        "Choose a mode below to agree."
    );
    assert!(row(&rows, "shareAsksNote").attached);
}

#[test]
fn an_invitation_has_a_sentence_and_two_answers_and_is_answered_by_its_id() {
    let mut online = online();
    invite(&mut online, "i1", ShareMode::Watch);
    let rows = party_rows(&online);
    assert_eq!(
        row(&rows, "shareInvited").title,
        "Lumen invites you to watch Astro Bot"
    );
    assert_eq!(
        press(&online, "shareAccept", 0),
        [OnlineAction::SharePlay(SharePlayCommand::Answer {
            id: "i1".into(),
            accept: true
        })]
    );
    assert_eq!(
        press(&online, "shareDecline", 0),
        [OnlineAction::SharePlay(SharePlayCommand::Answer {
            id: "i1".into(),
            accept: false
        })]
    );
    // The Home line leads to the party page.
    let mut panel = OnlinePanel::default();
    panel.go(Page::Home, &data(&online));
    let home = panel.rows(&data(&online)).2;
    assert!(ids(&home).contains(&"sharePlay"));
}

#[test]
fn a_guest_sees_their_part_and_can_ask_for_the_modes_they_do_not_have() {
    let mut online = online();
    report(
        &mut online,
        false,
        Some(view(ShareRole::Guest, ShareMode::Watch, SharePhase::Live)),
    );
    let rows = party_rows(&online);
    assert_eq!(row(&rows, "shareGuest").title, "Lumen shares Astro Bot");
    assert_eq!(value(row(&rows, "shareYou")), Some("Watch"));
    assert!(ids(&rows).contains(&"shareAskHandOver"));
    assert!(ids(&rows).contains(&"shareAskTogether"));
    assert_eq!(
        press(&online, "shareAskTogether", 0),
        [OnlineAction::SharePlay(SharePlayCommand::Request(
            ShareMode::Together
        ))]
    );

    report(
        &mut online,
        false,
        Some(view(
            ShareRole::Guest,
            ShareMode::HandOver,
            SharePhase::Live,
        )),
    );
    let rows = party_rows(&online);
    assert!(!ids(&rows).contains(&"shareAskHandOver"));
    assert_eq!(value(row(&rows, "shareYou")), Some("Play with their pad"));
    assert_eq!(
        press(&online, "shareStop", 0),
        [OnlineAction::SharePlay(SharePlayCommand::Stop)]
    );
}

#[test]
fn a_stalled_picture_is_said_in_words() {
    let mut online = online();
    report(
        &mut online,
        false,
        Some(view(ShareRole::Host, ShareMode::Watch, SharePhase::Stalled)),
    );
    let rows = party_rows(&online);
    assert_eq!(
        value(row(&rows, "shareHost")),
        Some("The picture has stopped")
    );
}
