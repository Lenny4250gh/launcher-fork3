//! Texts of parties and messages in the online area. English and Turkish like the rest. Row
//! titles are in title case, notes and toasts in sentence case, as everywhere in the launcher.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SocialText {
    // Party
    Party,
    /// A row in Home for a player without a party.
    StartParty,
    StartPartyDetail,
    /// `%@` members of `%@`.
    PartySubtitle,
    PartyMembers,
    Leader,
    PartyChat,
    InviteFriends,
    InviteToParty,
    LeaveParty,
    TakeParty,
    MakeLeader,
    RemoveFromParty,
    JoinParty,
    /// The detail of Join Party while the player has a party of their own.
    LeaveFirst,
    /// `%@` = how many members.
    PartyInvitationDetail,
    NobodyToInvite,
    /// `%@` = who.
    MakeLeaderDetail,
    /// `%@` = who.
    RemoveDetail,
    PickFirst,
    PartyFull,
    // Messages
    Messages,
    /// `%@` = how many.
    RequestsCount,
    MessageRequests,
    Conversations,
    NoConversations,
    SendMessage,
    NoMessages,
    /// The line of a conversation in the list that holds no message yet.
    NoPreview,
    /// The tag of a message request in the list.
    RequestTag,
    Loading,
    /// `%@` = how many.
    NewCount,
    WantsToMessage,
    Opening,
    ShowEarlier,
    MessageField,
    Send,
    TooLong,
    LeaveConversation,
    /// `%@` = how many people.
    GroupPeople,
    Seen,
    CopyText,
    Copied,
    RequestNote,
    /// What a screen reader says of a quote: `%@` = who, `%@` = the start of the message.
    InReplyTo,
    /// `%@` = who, `%@` = the start of the message.
    ReplyingTo,
    EditingMessage,
    Reply,
    Edit,
    EditDetail,
    Unsend,
    UnsendDetail,
    Removed,
    Edited,
    Delivered,
    /// `%@` = how many.
    SeenBy,
    /// `%@` = who.
    TypingOne,
    /// `%@` = who, `%@` = who.
    TypingTwo,
    TypingMany,
    Muted,
    Pinned,
    Mute,
    Unmute,
    MuteDetail,
    Pin,
    Unpin,
    PinsFull,
    QuickReplies,
    LoadFailed,
    ListFailed,
    /// `%@` = who.
    BlockedNote,
    Keyboard,
    KeyboardDetail,
    KeyboardEmpty,
    KeyboardDone,
    KeyShift,
    KeySymbols,
    KeyLetters,
    KeySpace,
    KeyDelete,
    ChatOptions,
    QuickRepliesDetail,
    OpenFriends,
    /// `%@` = who, `%@` = the start of the quoted message.
    ReplyTo,
    Quick1,
    Quick2,
    Quick3,
    Quick4,
    Quick5,
    Quick6,
    // Toasts
    /// `%@` = who.
    PartyInvitationArrived,
    /// `%@` = who, `%@` = the start of the message.
    MessageArrived,
    /// `%@` = who.
    MessageRequestArrived,
    /// The profile button's name while messages wait; `%@` = how many.
    ProfileUnread,
    /// The friends button's name while some are online; `%@` = how many.
    FriendsOnline,
}

impl SocialText {
    pub const fn english(self) -> &'static str {
        match self {
            SocialText::Party => "Party",
            SocialText::StartParty => "Start a Party",
            SocialText::StartPartyDetail => "Stay together across games",
            SocialText::PartySubtitle => "%@ of %@ members",
            SocialText::PartyMembers => "Members",
            SocialText::Leader => "Leader",
            SocialText::PartyChat => "Party Chat",
            SocialText::InviteFriends => "Invite Friends",
            SocialText::InviteToParty => "Invite to Party",
            SocialText::LeaveParty => "Leave Party",
            SocialText::TakeParty => "Take the Party to My Game",
            SocialText::MakeLeader => "Make Leader",
            SocialText::RemoveFromParty => "Remove from Party",
            SocialText::JoinParty => "Join Party",
            SocialText::LeaveFirst => "Leave your party first to join this one.",
            SocialText::PartyInvitationDetail => "Party of %@",
            SocialText::NobodyToInvite => "None of your friends is online and free to invite.",
            SocialText::MakeLeaderDetail => "%@ will lead the party",
            SocialText::RemoveDetail => "%@ is taken out of the party",
            SocialText::PickFirst => "Pick a friend first",
            SocialText::PartyFull => "The party is full",
            SocialText::Messages => "Messages",
            SocialText::RequestsCount => "Requests: %@",
            SocialText::MessageRequests => "Message Requests",
            SocialText::Conversations => "Conversations",
            SocialText::NoConversations => {
                "No conversations yet. Open a friend's page and choose Send Message."
            }
            SocialText::SendMessage => "Send Message",
            SocialText::NoMessages => "No messages yet. Say hello.",
            SocialText::NoPreview => "No messages yet",
            SocialText::RequestTag => "Request",
            SocialText::Loading => "Loading…",
            SocialText::NewCount => "%@ new",
            SocialText::WantsToMessage => "Wants to message you",
            SocialText::Opening => "Opening…",
            SocialText::ShowEarlier => "Show Earlier Messages",
            SocialText::MessageField => "Message",
            SocialText::Send => "Send",
            SocialText::TooLong => "Shorten it to 2,000 characters",
            SocialText::LeaveConversation => "Leave Conversation",
            SocialText::GroupPeople => "%@ people",
            SocialText::Seen => "Seen",
            SocialText::CopyText => "Copy Text",
            SocialText::Copied => "Copied",
            SocialText::RequestNote => {
                "They cannot see that you read this. Links in messages do not open."
            }
            SocialText::ReplyingTo => "Replying to %@: %@",
            SocialText::EditingMessage => "Editing your message",
            SocialText::Reply => "Reply",
            SocialText::Edit => "Edit",
            SocialText::EditDetail => "For 15 minutes after sending",
            SocialText::Unsend => "Unsend",
            SocialText::UnsendDetail => "Removes the text for everyone",
            SocialText::Removed => "Message removed",
            SocialText::Edited => "Edited",
            SocialText::Delivered => "Delivered",
            SocialText::SeenBy => "Seen by %@",
            SocialText::TypingOne => "%@ is typing…",
            SocialText::TypingTwo => "%@ and %@ are typing…",
            SocialText::TypingMany => "Several people are typing…",
            SocialText::Muted => "Muted",
            SocialText::Pinned => "Pinned",
            SocialText::Mute => "Mute Conversation",
            SocialText::Unmute => "Unmute Conversation",
            SocialText::MuteDetail => "No toasts or badge for it",
            SocialText::Pin => "Pin to Top",
            SocialText::Unpin => "Unpin",
            SocialText::PinsFull => "Unpin another conversation first",
            SocialText::QuickReplies => "Quick Replies",
            SocialText::LoadFailed => "The messages could not be loaded.",
            SocialText::ListFailed => "Your conversations could not be loaded.",
            SocialText::BlockedNote => "You blocked %@. Unblock them to write.",
            SocialText::Keyboard => "On-Screen Keyboard",
            SocialText::KeyboardDetail => "Type with a pad",
            SocialText::KeyboardEmpty => "Nothing typed yet",
            SocialText::KeyboardDone => "Done",
            SocialText::KeyShift => "Shift",
            SocialText::KeySymbols => "123",
            SocialText::KeyLetters => "ABC",
            SocialText::KeySpace => "Space",
            SocialText::KeyDelete => "Delete",
            SocialText::ChatOptions => "Conversation Options",
            SocialText::QuickRepliesDetail => "Send a short answer without typing",
            SocialText::OpenFriends => "Open Friends",
            SocialText::ReplyTo => "%@: %@",
            SocialText::InReplyTo => "In reply to %@: %@",
            SocialText::Quick1 => "On my way",
            SocialText::Quick2 => "One moment",
            SocialText::Quick3 => "Ready when you are",
            SocialText::Quick4 => "Thanks",
            SocialText::Quick5 => "GG",
            SocialText::Quick6 => "Can't talk now",
            SocialText::PartyInvitationArrived => "%@ invites you to a party",
            SocialText::MessageArrived => "%@: %@",
            SocialText::MessageRequestArrived => "%@ wants to message you",
            SocialText::ProfileUnread => "%@ unread",
            SocialText::FriendsOnline => "Friends, %@ online",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            SocialText::Party => "Parti",
            SocialText::StartParty => "Parti Kur",
            SocialText::StartPartyDetail => "Oyunlar arasında bir arada kal",
            SocialText::PartySubtitle => "%@ / %@ üye",
            SocialText::PartyMembers => "Üyeler",
            SocialText::Leader => "Lider",
            SocialText::PartyChat => "Parti Sohbeti",
            SocialText::InviteFriends => "Arkadaşları Davet Et",
            SocialText::InviteToParty => "Partiye Davet Et",
            SocialText::LeaveParty => "Partiden Ayrıl",
            SocialText::TakeParty => "Partiyi Oyunuma Götür",
            SocialText::MakeLeader => "Lider Yap",
            SocialText::RemoveFromParty => "Partiden Çıkar",
            SocialText::JoinParty => "Partiye Katıl",
            SocialText::LeaveFirst => "Bu partiye katılmak için önce kendi partinden ayrıl.",
            SocialText::PartyInvitationDetail => "%@ kişilik parti",
            SocialText::MakeLeaderDetail => "%@ partiyi yönetecek",
            SocialText::RemoveDetail => "%@ partiden çıkarılır",
            SocialText::PickFirst => "Önce bir arkadaş seç",
            SocialText::PartyFull => "Parti dolu",
            SocialText::NobodyToInvite => {
                "Arkadaşlarından hiçbiri çevrimiçi değil ya da davet edilebilecek durumda değil."
            }
            SocialText::Messages => "Mesajlar",
            SocialText::RequestsCount => "İstekler: %@",
            SocialText::MessageRequests => "Mesaj İstekleri",
            SocialText::Conversations => "Sohbetler",
            SocialText::NoConversations => {
                "Henüz sohbet yok. Bir arkadaşının sayfasını aç ve Mesaj Gönder'i seç."
            }
            SocialText::SendMessage => "Mesaj Gönder",
            SocialText::NoMessages => "Henüz mesaj yok. Merhaba de.",
            SocialText::NoPreview => "Henüz mesaj yok",
            SocialText::RequestTag => "İstek",
            SocialText::Loading => "Yükleniyor…",
            SocialText::NewCount => "%@ yeni",
            SocialText::WantsToMessage => "Sana mesaj yazmak istiyor",
            SocialText::Opening => "Açılıyor…",
            SocialText::ShowEarlier => "Önceki Mesajları Göster",
            SocialText::MessageField => "Mesaj",
            SocialText::Send => "Gönder",
            SocialText::TooLong => "En fazla 2.000 karakter olsun",
            SocialText::LeaveConversation => "Sohbetten Ayrıl",
            SocialText::GroupPeople => "%@ kişi",
            SocialText::Seen => "Görüldü",
            SocialText::CopyText => "Metni Kopyala",
            SocialText::Copied => "Kopyalandı",
            SocialText::RequestNote => "Okuduğunu göremezler. Mesajlardaki bağlantılar açılmaz.",
            SocialText::ReplyingTo => "%@ kişisine yanıt: %@",
            SocialText::EditingMessage => "Mesajını düzenliyorsun",
            SocialText::Reply => "Yanıtla",
            SocialText::Edit => "Düzenle",
            SocialText::EditDetail => "Gönderdikten sonra 15 dakika",
            SocialText::Unsend => "Geri Al",
            SocialText::UnsendDetail => "Metni herkesten kaldırır",
            SocialText::Removed => "Mesaj kaldırıldı",
            SocialText::Edited => "Düzenlendi",
            SocialText::Delivered => "İletildi",
            SocialText::SeenBy => "%@ kişi gördü",
            SocialText::TypingOne => "%@ yazıyor…",
            SocialText::TypingTwo => "%@ ve %@ yazıyor…",
            SocialText::TypingMany => "Birkaç kişi yazıyor…",
            SocialText::Muted => "Sessiz",
            SocialText::Pinned => "Sabitlendi",
            SocialText::Mute => "Sohbeti Sessize Al",
            SocialText::Unmute => "Sohbetin Sesini Aç",
            SocialText::MuteDetail => "Bunun için bildirim ve rozet yok",
            SocialText::Pin => "En Üste Sabitle",
            SocialText::Unpin => "Sabitlemeyi Kaldır",
            SocialText::PinsFull => "Önce başka bir sohbetin sabitlemesini kaldır",
            SocialText::QuickReplies => "Hızlı Yanıtlar",
            SocialText::LoadFailed => "Mesajlar yüklenemedi.",
            SocialText::ListFailed => "Sohbetlerin yüklenemedi.",
            SocialText::BlockedNote => "%@ kişisini engelledin. Yazmak için engeli kaldır.",
            SocialText::Keyboard => "Ekran Klavyesi",
            SocialText::KeyboardDetail => "Oyun koluyla yaz",
            SocialText::KeyboardEmpty => "Henüz bir şey yazılmadı",
            SocialText::KeyboardDone => "Bitti",
            SocialText::KeyShift => "Büyük",
            SocialText::KeySymbols => "123",
            SocialText::KeyLetters => "ABC",
            SocialText::KeySpace => "Boşluk",
            SocialText::KeyDelete => "Sil",
            SocialText::ChatOptions => "Sohbet Seçenekleri",
            SocialText::QuickRepliesDetail => "Yazmadan kısa bir yanıt gönder",
            SocialText::OpenFriends => "Arkadaşları Aç",
            SocialText::ReplyTo => "%@: %@",
            SocialText::InReplyTo => "%@ kişisine yanıt: %@",
            SocialText::Quick1 => "Geliyorum",
            SocialText::Quick2 => "Bir dakika",
            SocialText::Quick3 => "Sen hazır olunca hazırım",
            SocialText::Quick4 => "Teşekkürler",
            SocialText::Quick5 => "GG",
            SocialText::Quick6 => "Şimdi konuşamam",
            SocialText::PartyInvitationArrived => "%@ seni bir partiye davet ediyor",
            SocialText::MessageArrived => "%@: %@",
            SocialText::MessageRequestArrived => "%@ sana mesaj yazmak istiyor",
            SocialText::ProfileUnread => "%@ okunmamış",
            SocialText::FriendsOnline => "Arkadaşlar, %@ çevrimiçi",
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            SocialText::Party => "Party",
            SocialText::StartParty => "Party starten",
            SocialText::StartPartyDetail => "Bleibt über Spiele hinweg zusammen",
            SocialText::PartySubtitle => "%@ von %@ Mitgliedern",
            SocialText::PartyMembers => "Mitglieder",
            SocialText::Leader => "Anführer",
            SocialText::PartyChat => "Party-Chat",
            SocialText::InviteFriends => "Freunde einladen",
            SocialText::InviteToParty => "In die Party einladen",
            SocialText::LeaveParty => "Party verlassen",
            SocialText::TakeParty => "Party in mein Spiel mitnehmen",
            SocialText::MakeLeader => "Zum Anführer machen",
            SocialText::RemoveFromParty => "Aus der Party entfernen",
            SocialText::JoinParty => "Party beitreten",
            SocialText::LeaveFirst => "Verlasse zuerst deine Party, um dieser beizutreten.",
            SocialText::PartyInvitationDetail => "Party mit %@",
            SocialText::NobodyToInvite => "Keiner deiner Freunde ist online und frei zum Einladen.",
            SocialText::MakeLeaderDetail => "%@ führt die Party an",
            SocialText::RemoveDetail => "%@ wird aus der Party entfernt",
            SocialText::PickFirst => "Wähle zuerst einen Freund",
            SocialText::PartyFull => "Die Party ist voll",
            SocialText::Messages => "Nachrichten",
            SocialText::RequestsCount => "Anfragen: %@",
            SocialText::MessageRequests => "Nachrichtenanfragen",
            SocialText::Conversations => "Unterhaltungen",
            SocialText::NoConversations => {
                "Noch keine Unterhaltungen. Öffne die Seite eines Freundes und wähle Nachricht senden."
            }
            SocialText::SendMessage => "Nachricht senden",
            SocialText::NoMessages => "Noch keine Nachrichten. Sag Hallo.",
            SocialText::NoPreview => "Noch keine Nachrichten",
            SocialText::RequestTag => "Anfrage",
            SocialText::Loading => "Wird geladen…",
            SocialText::NewCount => "%@ neu",
            SocialText::WantsToMessage => "Möchte dir schreiben",
            SocialText::Opening => "Wird geöffnet…",
            SocialText::ShowEarlier => "Frühere Nachrichten anzeigen",
            SocialText::MessageField => "Nachricht",
            SocialText::Send => "Senden",
            SocialText::TooLong => "Kürze auf 2.000 Zeichen",
            SocialText::LeaveConversation => "Unterhaltung verlassen",
            SocialText::GroupPeople => "%@ Personen",
            SocialText::Seen => "Gesehen",
            SocialText::CopyText => "Text kopieren",
            SocialText::Copied => "Kopiert",
            SocialText::RequestNote => {
                "Die Person sieht nicht, dass du das gelesen hast. Links in Nachrichten öffnen sich nicht."
            }
            SocialText::ReplyingTo => "Antwort an %@: %@",
            SocialText::EditingMessage => "Du bearbeitest deine Nachricht",
            SocialText::Reply => "Antworten",
            SocialText::Edit => "Bearbeiten",
            SocialText::EditDetail => "15 Minuten nach dem Senden",
            SocialText::Unsend => "Zurückziehen",
            SocialText::UnsendDetail => "Entfernt den Text für alle",
            SocialText::Removed => "Nachricht entfernt",
            SocialText::Edited => "Bearbeitet",
            SocialText::Delivered => "Zugestellt",
            SocialText::SeenBy => "Gesehen von %@",
            SocialText::TypingOne => "%@ schreibt…",
            SocialText::TypingTwo => "%@ und %@ schreiben…",
            SocialText::TypingMany => "Mehrere Personen schreiben…",
            SocialText::Muted => "Stumm",
            SocialText::Pinned => "Angeheftet",
            SocialText::Mute => "Unterhaltung stummschalten",
            SocialText::Unmute => "Stummschaltung aufheben",
            SocialText::MuteDetail => "Keine Hinweise und kein Zähler dafür",
            SocialText::Pin => "Oben anheften",
            SocialText::Unpin => "Lösen",
            SocialText::PinsFull => "Löse zuerst eine andere Unterhaltung",
            SocialText::QuickReplies => "Schnellantworten",
            SocialText::LoadFailed => "Die Nachrichten konnten nicht geladen werden.",
            SocialText::ListFailed => "Deine Unterhaltungen konnten nicht geladen werden.",
            SocialText::BlockedNote => {
                "Du hast %@ blockiert. Hebe die Blockierung auf, um zu schreiben."
            }
            SocialText::Keyboard => "Bildschirmtastatur",
            SocialText::KeyboardDetail => "Mit dem Controller tippen",
            SocialText::KeyboardEmpty => "Noch nichts getippt",
            SocialText::KeyboardDone => "Fertig",
            SocialText::KeyShift => "Umschalt",
            SocialText::KeySymbols => "123",
            SocialText::KeyLetters => "ABC",
            SocialText::KeySpace => "Leertaste",
            SocialText::KeyDelete => "Löschen",
            SocialText::ChatOptions => "Optionen der Unterhaltung",
            SocialText::QuickRepliesDetail => "Kurze Antwort senden, ohne zu tippen",
            SocialText::OpenFriends => "Freunde öffnen",
            SocialText::ReplyTo => "%@: %@",
            SocialText::InReplyTo => "Antwort an %@: %@",
            SocialText::Quick1 => "Bin unterwegs",
            SocialText::Quick2 => "Einen Moment",
            SocialText::Quick3 => "Bereit, wenn du es bist",
            SocialText::Quick4 => "Danke",
            SocialText::Quick5 => "GG",
            SocialText::Quick6 => "Kann gerade nicht reden",
            SocialText::PartyInvitationArrived => "%@ lädt dich in eine Party ein",
            SocialText::MessageArrived => "%@: %@",
            SocialText::MessageRequestArrived => "%@ möchte dir schreiben",
            SocialText::ProfileUnread => "%@ ungelesen",
            SocialText::FriendsOnline => "Freunde, %@ online",
            _ => return None,
        })
    }

    pub const fn text(self, language: Language) -> &'static str {
        match language {
            Language::Turkish => self.turkish(),
            Language::English => self.english(),
            Language::German => match self.german() {
                Some(text) => text,
                None => self.english(),
            },
        }
    }

    pub fn format(self, language: Language, args: &[Arg<'_>]) -> String {
        substitute(self.text(language), args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WITH_ARGUMENTS: &[SocialText] = &[
        SocialText::PartySubtitle,
        SocialText::PartyInvitationDetail,
        SocialText::MakeLeaderDetail,
        SocialText::RemoveDetail,
        SocialText::RequestsCount,
        SocialText::NewCount,
        SocialText::GroupPeople,
        SocialText::PartyInvitationArrived,
        SocialText::MessageArrived,
        SocialText::MessageRequestArrived,
        SocialText::ProfileUnread,
        SocialText::FriendsOnline,
        SocialText::ReplyingTo,
        SocialText::BlockedNote,
        SocialText::SeenBy,
        SocialText::TypingOne,
        SocialText::TypingTwo,
        SocialText::ReplyTo,
        SocialText::InReplyTo,
    ];

    #[test]
    fn placeholders_match_between_languages() {
        for text in WITH_ARGUMENTS {
            let count = |s: &str| s.matches("%@").count();
            assert_eq!(count(text.english()), count(text.turkish()), "{text:?}");
        }
    }

    #[test]
    fn row_titles_are_in_title_case_and_notes_in_sentence_case() {
        for title in [
            SocialText::StartParty,
            SocialText::InviteFriends,
            SocialText::TakeParty,
            SocialText::ShowEarlier,
            SocialText::LeaveConversation,
            SocialText::MessageRequests,
            SocialText::QuickReplies,
            SocialText::Keyboard,
            SocialText::ChatOptions,
            SocialText::Mute,
            SocialText::Unmute,
            SocialText::Pin,
        ] {
            for word in title.english().split(' ') {
                let first = word.chars().next().unwrap();
                assert!(
                    first.is_uppercase() || matches!(word, "the" | "to" | "a"),
                    "{title:?}: {word}"
                );
            }
        }
        assert_eq!(
            SocialText::PartySubtitle.format(Language::English, &[Arg::Int(3), Arg::Int(8)]),
            "3 of 8 members"
        );
    }
}
