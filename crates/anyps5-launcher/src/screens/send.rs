//! "Send to a Friend" (docs/DESIGN.md 17, "Send to a Friend"): the friend picker that opens from a
//! screenshot or a clip in Captures. A panel of the game frame with one row per friend; a press on
//! a friend sends the capture, and the panel closes. Friends only: a stranger or a message request
//! is not listed, and the cloud refuses them as well.

use anyps5_core::online::Online;
use anyps5_core::online::api::{AccountId, MediaInfo, MediaKind};
use anyps5_core::online::media;
use egui::{Color32, Context};

use crate::screens::lfg::item_at;
use crate::screens::lfg_strings::LfgText as T;
use crate::screens::rows::{self, Row};
use crate::screens::side::{Frame, Leave, SideState};
use crate::ui::Shell;
use crate::ui::i18n::Language;
use crate::ui::icons::Icon;
use crate::ui::input::InputEvent;

/// What the host carries out.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SendAction {
    /// Back, or the scrim: return to Captures.
    Close,
    /// Opens the sign-in.
    SignIn,
    /// Send the capture to this friend.
    Friend(AccountId),
}

/// What a media message reads as in one line: "Screenshot · 1.2 MB", "Clip · 0:23 · 14.0 MB".
pub fn media_label(info: &MediaInfo, language: Language) -> String {
    let size = media::size_text(info.bytes);
    match (info.kind, info.duration_ms) {
        (MediaKind::Clip, Some(ms)) => format!(
            "{} · {} · {size}",
            T::Clip.text(language),
            media::duration_text(u64::from(ms))
        ),
        (MediaKind::Clip, None) => format!("{} · {size}", T::Clip.text(language)),
        _ => format!("{} · {size}", T::Screenshot.text(language)),
    }
}

/// What the picker reads each frame.
pub struct SendData<'a> {
    /// The file's name.
    pub name: &'a str,
    pub online: &'a Online,
    pub language: Language,
    pub accent: Color32,
}

/// The picker.
#[derive(Default)]
pub struct SendPanel {
    side: SideState,
}

impl SendPanel {
    pub fn open(&mut self) {
        self.side.open_at(0);
    }

    pub fn close(&mut self) {
        self.side.close();
    }

    pub fn dismiss(&mut self) {
        self.side.dismiss();
    }

    pub fn is_active(&self) -> bool {
        self.side.is_active()
    }

    pub fn show(
        &mut self,
        shell: &mut Shell,
        ctx: &Context,
        events: &[InputEvent],
        data: &SendData<'_>,
    ) -> Vec<SendAction> {
        let mut actions = Vec::new();
        let frame = Frame {
            title: T::SendTitle.text(data.language),
            subtitle: Some(data.name),
            list_id: "send-rows",
            accent: data.accent,
            language: data.language,
        };
        self.side.show_rows(
            shell,
            ctx,
            events,
            &mut { data },
            &frame,
            Leave::Slide,
            |data| build_rows(data),
            |side, _, data, change, actions| {
                if let rows::Change::Run { row } = change {
                    match *row {
                        "signIn" => {
                            side.close();
                            actions.push(SendAction::SignIn);
                        }
                        "friend" => {
                            let rows = build_rows(data);
                            let friends = friends_of(data.online);
                            if let Some(n) = item_at(&rows, "friend", side.nav.row)
                                && let Some(friend) = friends.get(n)
                            {
                                side.close();
                                actions.push(SendAction::Friend(*friend));
                            }
                        }
                        _ => {}
                    }
                }
            },
            SendAction::Close,
            &mut actions,
        );
        actions
    }
}

/// The friends the picker lists, in the order of the Friends list: those who play, then the
/// others online, then the rest.
pub fn friends_of(online: &Online) -> Vec<AccountId> {
    online
        .sorted_friends()
        .into_iter()
        .map(|f| f.account.id)
        .collect()
}

/// The rows: a friend each, or why there is none.
pub fn build_rows(data: &SendData<'_>) -> Vec<Row> {
    let language = data.language;
    let t = |text: T| text.text(language).to_owned();
    let mut rows = Vec::new();
    if !data.online.is_signed_in() {
        rows.push(Row::info("signedOut", t(T::SendSignedOut), None));
        rows.push(Row::command("signIn", t(T::SignIn)).icon(Icon::SIGN_IN));
        return rows;
    }
    let friends = data.online.sorted_friends();
    if friends.is_empty() {
        rows.push(Row::info("none", t(T::NoFriends), None));
        return rows;
    }
    for friend in friends {
        rows.push(
            Row::command("friend", friend.account.handle.clone()).icon(Icon::PAPER_PLANE_RIGHT),
        );
    }
    rows.push(Row::info("note", t(T::SendNote), None).group_start());
    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyps5_core::online::api::{Account, Friend, Presence};

    fn friend(id: u64, handle: &str) -> Friend {
        Friend {
            account: Account {
                id: AccountId(id),
                handle: handle.into(),
            },
            since: 0,
            presence: Presence::default(),
        }
    }

    fn data<'a>(online: &'a Online) -> SendData<'a> {
        SendData {
            name: "shot.png",
            online,
            language: Language::English,
            accent: Color32::WHITE,
        }
    }

    #[test]
    fn a_media_message_reads_as_its_kind_its_length_and_its_size() {
        let mut info = MediaInfo {
            id: "m".into(),
            kind: MediaKind::Image,
            content_type: "image/png".into(),
            bytes: 1_300_000,
            width: None,
            height: None,
            duration_ms: None,
            expires: 0,
        };
        assert_eq!(media_label(&info, Language::English), "Screenshot · 1.2 MB");
        info.kind = MediaKind::Clip;
        info.duration_ms = Some(23_000);
        info.bytes = 14 * 1024 * 1024;
        assert_eq!(
            media_label(&info, Language::English),
            "Clip · 0:23 · 14.0 MB"
        );
        info.duration_ms = None;
        assert_eq!(media_label(&info, Language::Turkish), "Klip · 14.0 MB");
    }

    #[test]
    fn signed_out_the_picker_offers_the_sign_in() {
        let online = Online::default();
        let ids: Vec<_> = build_rows(&data(&online)).iter().map(|r| r.id).collect();
        assert_eq!(ids, ["signedOut", "signIn"]);
    }

    #[test]
    fn friends_are_the_rows_and_the_limits_are_said_under_them() {
        let mut online = Online::default();
        let account = anyps5_core::online::SignedIn {
            id: AccountId(1),
            handle: "me".into(),
            email: String::new(),
            appear_offline: false,
        };
        online.account = Some(account);
        let ids: Vec<_> = build_rows(&data(&online)).iter().map(|r| r.id).collect();
        assert_eq!(ids, ["none"]);
        online.friends.friends = vec![friend(2, "Lumen"), friend(3, "Aya")];
        let rows = build_rows(&data(&online));
        let ids: Vec<_> = rows.iter().map(|r| r.id).collect();
        assert_eq!(ids, ["friend", "friend", "note"]);
        assert_eq!(friends_of(&online).len(), 2);
    }
}
