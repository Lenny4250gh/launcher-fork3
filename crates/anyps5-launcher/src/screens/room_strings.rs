//! Texts of the Party room (docs/DESIGN.md 17, "Party room"). English and Turkish like the rest;
//! the Turkish has not been read by a native speaker yet. Buttons are in title case.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RoomText {
    /// Switches the party voice on, where Mute and Deafen would be.
    VoiceOn,
    /// The button of a party invitation's card.
    ViewInvitation,
    /// Under a member the player muted: not their own mute, which nobody reports.
    MutedByYou,
    /// The empty seat of the grid.
    InviteAFriend,
    /// The line of a party of one: the room is open and nobody is in it yet.
    AloneTitle,
    AloneNote,
}

impl RoomText {
    pub const fn english(self) -> &'static str {
        match self {
            RoomText::VoiceOn => "Turn On Voice",
            RoomText::ViewInvitation => "View Invitation",
            RoomText::MutedByYou => "Muted by you",
            RoomText::InviteAFriend => "Invite a Friend",
            RoomText::AloneTitle => "Your party is open",
            RoomText::AloneNote => "Invite friends to talk, chat and play together.",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            RoomText::VoiceOn => "Sesi Aç",
            RoomText::ViewInvitation => "Daveti Görüntüle",
            RoomText::MutedByYou => "Senin tarafından susturuldu",
            RoomText::InviteAFriend => "Arkadaş Davet Et",
            RoomText::AloneTitle => "Partin açık",
            RoomText::AloneNote => "Arkadaşlarını davet et: birlikte konuşun, yazışın ve oynayın.",
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            RoomText::VoiceOn => "Sprache einschalten",
            RoomText::ViewInvitation => "Einladung ansehen",
            RoomText::MutedByYou => "Von dir stummgeschaltet",
            RoomText::InviteAFriend => "Freund einladen",
            RoomText::AloneTitle => "Deine Party ist offen",
            RoomText::AloneNote => {
                "Lade Freunde ein, um zusammen zu sprechen, zu schreiben und zu spielen."
            }
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            RoomText::VoiceOn => "Activar voz",
            RoomText::ViewInvitation => "Ver invitación",
            RoomText::MutedByYou => "Silenciado por ti",
            RoomText::InviteAFriend => "Invitar a un amigo",
            RoomText::AloneTitle => "Tu party está abierta",
            RoomText::AloneNote => "Invita a amigos para hablar, chatear y jugar juntos.",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            RoomText::VoiceOn => "Activer la voix",
            RoomText::ViewInvitation => "Voir l’invitation",
            RoomText::MutedByYou => "Mis en sourdine par toi",
            RoomText::InviteAFriend => "Inviter un ami",
            RoomText::AloneTitle => "Ta party est ouverte",
            RoomText::AloneNote => "Invite des amis pour parler, discuter et jouer ensemble.",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            RoomText::VoiceOn => "Ativar voz",
            RoomText::ViewInvitation => "Ver convite",
            RoomText::MutedByYou => "Silenciado por você",
            RoomText::InviteAFriend => "Convidar um amigo",
            RoomText::AloneTitle => "Sua party está aberta",
            RoomText::AloneNote => "Convide amigos para falar, conversar e jogar juntos.",
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
            Language::Spanish => match self.spanish() {
                Some(text) => text,
                None => self.english(),
            },
            Language::French => match self.french() {
                Some(text) => text,
                None => self.english(),
            },
            Language::Portuguese => match self.portuguese() {
                Some(text) => text,
                None => self.english(),
            },
        }
    }

    pub fn format(self, language: Language, args: &[Arg<'_>]) -> String {
        substitute(self.text(language), args)
    }
}
