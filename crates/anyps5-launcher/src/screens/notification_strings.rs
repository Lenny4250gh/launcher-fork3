//! Texts of the Notification Center (docs/DESIGN.md 9.12) and of the invite link. English and
//! Turkish like the rest. Buttons and titles are in title case, sentences in sentence case.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NotifyText {
    /// The area's name: the title, the top bar button, the Home button panel's row.
    Title,
    /// The accessible name of the bell with `%@` unread entries.
    BellUnread,
    ClearAll,
    /// The small label over an entry, by what it is about.
    KindFriendRequest,
    KindInvitation,
    KindMessage,
    KindTrophy,
    KindUpdate,
    KindTogether,
    KindSystem,
    /// The buttons of an entry.
    Accept,
    Join,
    Open,
    Dismiss,
    /// The accessible name of an entry: `%@` = what it is, `%@` = the sentence, `%@` = when.
    EntryName,
    /// The empty state: what lives here and how it fills.
    EmptyTitle,
    EmptyBody,
    /// The invite link: the row, its detail, and the toasts.
    CopyInviteLink,
    CopyInviteLinkDetail,
    InviteLinkCopied,
    InviteLinkWaiting,
    InviteLinkOffline,
    InviteLinkFailed,
}

impl NotifyText {
    pub const ALL: [NotifyText; 23] = [
        NotifyText::Title,
        NotifyText::BellUnread,
        NotifyText::ClearAll,
        NotifyText::KindFriendRequest,
        NotifyText::KindInvitation,
        NotifyText::KindMessage,
        NotifyText::KindTrophy,
        NotifyText::KindUpdate,
        NotifyText::KindTogether,
        NotifyText::KindSystem,
        NotifyText::Accept,
        NotifyText::Join,
        NotifyText::Open,
        NotifyText::Dismiss,
        NotifyText::EntryName,
        NotifyText::EmptyTitle,
        NotifyText::EmptyBody,
        NotifyText::CopyInviteLink,
        NotifyText::CopyInviteLinkDetail,
        NotifyText::InviteLinkCopied,
        NotifyText::InviteLinkWaiting,
        NotifyText::InviteLinkOffline,
        NotifyText::InviteLinkFailed,
    ];

    pub const fn english(self) -> &'static str {
        match self {
            NotifyText::Title => "Notifications",
            NotifyText::BellUnread => "Notifications, %@ new",
            NotifyText::ClearAll => "Clear All",
            NotifyText::KindFriendRequest => "Friend request",
            NotifyText::KindInvitation => "Invitation",
            NotifyText::KindMessage => "Message",
            NotifyText::KindTrophy => "Trophy",
            NotifyText::KindUpdate => "Update",
            NotifyText::KindTogether => "Play Together",
            NotifyText::KindSystem => "Notice",
            NotifyText::Accept => "Accept",
            NotifyText::Join => "Join",
            NotifyText::Open => "Open",
            NotifyText::Dismiss => "Dismiss",
            NotifyText::EntryName => "%@, %@, %@",
            NotifyText::EmptyTitle => "No notifications",
            NotifyText::EmptyBody => {
                "Friend requests, invitations, messages, trophies and updates show up here."
            }
            NotifyText::CopyInviteLink => "Copy Invite Link",
            NotifyText::CopyInviteLinkDetail => {
                "A link that asks you to join; it never joins by itself"
            }
            NotifyText::InviteLinkCopied => "Invite link copied",
            NotifyText::InviteLinkWaiting => "Getting your invite link…",
            NotifyText::InviteLinkOffline => {
                "Sign in to the AnyPlay network to get an invite link."
            }
            NotifyText::InviteLinkFailed => "Unable to get an invite link. Try again in a moment.",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            NotifyText::Title => "Bildirimler",
            NotifyText::BellUnread => "Bildirimler, %@ yeni",
            NotifyText::ClearAll => "Tümünü Temizle",
            NotifyText::KindFriendRequest => "Arkadaşlık isteği",
            NotifyText::KindInvitation => "Davet",
            NotifyText::KindMessage => "Mesaj",
            NotifyText::KindTrophy => "Kupa",
            NotifyText::KindUpdate => "Güncelleme",
            NotifyText::KindTogether => "Birlikte Oyna",
            NotifyText::KindSystem => "Bildirim",
            NotifyText::Accept => "Kabul Et",
            NotifyText::Join => "Katıl",
            NotifyText::Open => "Aç",
            NotifyText::Dismiss => "Kapat",
            NotifyText::EntryName => "%@, %@, %@",
            NotifyText::EmptyTitle => "Bildirim yok",
            NotifyText::EmptyBody => {
                "Arkadaşlık istekleri, davetler, mesajlar, kupalar ve güncellemeler burada görünür."
            }
            NotifyText::CopyInviteLink => "Davet Bağlantısını Kopyala",
            NotifyText::CopyInviteLinkDetail => {
                "Katılmanı isteyen bir bağlantı; kendiliğinden katılmaz"
            }
            NotifyText::InviteLinkCopied => "Davet bağlantısı kopyalandı",
            NotifyText::InviteLinkWaiting => "Davet bağlantın hazırlanıyor…",
            NotifyText::InviteLinkOffline => "Davet bağlantısı için AnyPlay ağında oturum aç.",
            NotifyText::InviteLinkFailed => "Davet bağlantısı alınamadı. Birazdan tekrar dene.",
        }
    }

    pub const fn text(self, language: Language) -> &'static str {
        match language {
            Language::Turkish => self.turkish(),
            Language::English => self.english(),
        }
    }

    pub fn format(self, language: Language, args: &[Arg<'_>]) -> String {
        substitute(self.text(language), args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholders_match_between_languages() {
        for text in NotifyText::ALL {
            assert_eq!(
                text.english().matches("%@").count(),
                text.turkish().matches("%@").count(),
                "{text:?}"
            );
        }
    }
}
