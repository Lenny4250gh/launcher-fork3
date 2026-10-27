//! Texts of calls and of the voice provider in the online area (docs/DESIGN.md 17, "Calls").
//! English and Turkish like the rest. The Turkish was written for this change and has not been
//! read by a native speaker yet. Row titles are in title case, notes, details and toasts in
//! sentence case.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CallText {
    // The ringing call
    /// `%@` = who. The accessible name of the card.
    IncomingName,
    /// Under the caller's name.
    IncomingLine,
    /// Under the caller's name while the player's voice is on Discord.
    IncomingDiscordLine,
    Accept,
    Decline,
    // The call in progress
    InCall,
    /// `%@` = who.
    Calling,
    Cancel,
    Mute,
    Unmute,
    Deafen,
    Undeafen,
    Leave,
    /// After a name that is speaking.
    SpeakingSuffix,
    /// The state line: your microphone is muted.
    YouMuted,
    YouDeafened,
    /// Shown while nobody else is in the call yet.
    WaitingForOthers,
    // Discord
    VoiceOnDiscord,
    /// Accepting a call with the voice set to Discord and the party has a link.
    AnswerOnDiscord,
    OpenInDiscord,
    /// No link: tell the player where the talking happens.
    DiscordSetNote,
    // Missed calls
    /// `%@` = how long ago.
    MissedAgo,
    MissedNow,
    // The friend page
    CallFriend,
    CallFriendDetail,
    AddToCall,
    CallNeedsLeader,
    CallFriendBusy,
    // The voice provider
    VoiceOn,
    ProviderAnyPlay,
    ProviderDiscord,
    ProviderDiscordNote,
    ProviderAnyPlayNote,
    StatusDiscord,
    // The party's Discord link
    DiscordLink,
    DiscordLinkNone,
    DiscordLinkField,
    SetLink,
    RemoveLink,
    LinkNote,
    LinkInvalid,
    // Toasts
    /// `%@` = who.
    ToastMissed,
    ToastNoAnswer,
    ToastDeclined,
    ToastUnreachable,
    ToastNeedsLeader,
}

impl CallText {
    pub const fn english(self) -> &'static str {
        match self {
            CallText::IncomingName => "Incoming call from %@",
            CallText::IncomingLine => "is calling you",
            CallText::IncomingDiscordLine => "Your voice is set to Discord",
            CallText::Accept => "Accept",
            CallText::Decline => "Decline",
            CallText::InCall => "In call",
            CallText::Calling => "Calling %@",
            CallText::Cancel => "Cancel",
            CallText::Mute => "Mute",
            CallText::Unmute => "Unmute",
            CallText::Deafen => "Deafen",
            CallText::Undeafen => "Undeafen",
            CallText::Leave => "Leave Call",
            CallText::SpeakingSuffix => "speaking",
            CallText::YouMuted => "You are muted",
            CallText::YouDeafened => "You are deafened",
            CallText::WaitingForOthers => "Waiting for others",
            CallText::VoiceOnDiscord => "Voice on Discord",
            CallText::AnswerOnDiscord => "Answer on Discord",
            CallText::OpenInDiscord => "Open in Discord",
            CallText::DiscordSetNote => {
                "Your voice is set to Discord. Join the call there; AnyPlay does not open your microphone."
            }
            CallText::MissedAgo => "Missed call · %@",
            CallText::MissedNow => "Missed call · just now",
            CallText::CallFriend => "Call",
            CallText::CallFriendDetail => "Rings for 30 seconds",
            CallText::AddToCall => "Add to Call",
            CallText::CallNeedsLeader => "Only the one who started the call can add people.",
            CallText::CallFriendBusy => "In a party already",
            CallText::VoiceOn => "Voice Service",
            CallText::ProviderAnyPlay => "AnyPlay",
            CallText::ProviderDiscord => "Discord",
            CallText::ProviderDiscordNote => {
                "On Discord, AnyPlay never opens your microphone for parties or calls. Calls still ring here."
            }
            CallText::ProviderAnyPlayNote => "Parties and calls use AnyPlay's own voice.",
            CallText::StatusDiscord => "On Discord",
            CallText::DiscordLink => "Discord Link",
            CallText::DiscordLinkNone => "Not set",
            CallText::DiscordLinkField => "discord.com/channels/… or discord.gg/…",
            CallText::SetLink => "Save Link",
            CallText::RemoveLink => "Remove Link",
            CallText::LinkNote => {
                "Everyone in the party sees this link. Only the leader can change it."
            }
            CallText::LinkInvalid => {
                "That is not a Discord link. Use an address from discord.com/channels or discord.gg."
            }
            CallText::ToastMissed => "Missed call from %@",
            CallText::ToastNoAnswer => "%@ did not answer.",
            CallText::ToastDeclined => "%@ declined the call.",
            CallText::ToastUnreachable => "%@ cannot take a call right now.",
            CallText::ToastNeedsLeader => "Only the one who started the call can add people.",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            CallText::IncomingName => "%@ seni arıyor",
            CallText::IncomingLine => "seni arıyor",
            CallText::IncomingDiscordLine => "Sesin Discord'a ayarlı",
            CallText::Accept => "Kabul Et",
            CallText::Decline => "Reddet",
            CallText::InCall => "Aramada",
            CallText::Calling => "%@ aranıyor",
            CallText::Cancel => "Vazgeç",
            CallText::Mute => "Sessize Al",
            CallText::Unmute => "Sesi Aç",
            CallText::Deafen => "Kulaklığı Kapat",
            CallText::Undeafen => "Kulaklığı Aç",
            CallText::Leave => "Aramadan Çık",
            CallText::SpeakingSuffix => "konuşuyor",
            CallText::YouMuted => "Mikrofonun kapalı",
            CallText::YouDeafened => "Kimseyi duymuyorsun",
            CallText::WaitingForOthers => "Diğerleri bekleniyor",
            CallText::VoiceOnDiscord => "Ses Discord'da",
            CallText::AnswerOnDiscord => "Discord'da Yanıtla",
            CallText::OpenInDiscord => "Discord'da Aç",
            CallText::DiscordSetNote => {
                "Sesin Discord'a ayarlı. Aramaya orada katıl; AnyPlay mikrofonunu açmaz."
            }
            CallText::MissedAgo => "Cevapsız arama · %@",
            CallText::MissedNow => "Cevapsız arama · az önce",
            CallText::CallFriend => "Ara",
            CallText::CallFriendDetail => "30 saniye çalar",
            CallText::AddToCall => "Aramaya Ekle",
            CallText::CallNeedsLeader => "Yalnızca aramayı başlatan kişi ekleyebilir.",
            CallText::CallFriendBusy => "Zaten bir partide",
            CallText::VoiceOn => "Ses Hizmeti",
            CallText::ProviderAnyPlay => "AnyPlay",
            CallText::ProviderDiscord => "Discord",
            CallText::ProviderDiscordNote => {
                "Discord seçiliyken AnyPlay parti ve aramalar için mikrofonunu hiç açmaz. Aramalar yine burada çalar."
            }
            CallText::ProviderAnyPlayNote => {
                "Partiler ve aramalar AnyPlay'in kendi sesini kullanır."
            }
            CallText::StatusDiscord => "Discord'da",
            CallText::DiscordLink => "Discord Bağlantısı",
            CallText::DiscordLinkNone => "Ayarlanmadı",
            CallText::DiscordLinkField => "discord.com/channels/… veya discord.gg/…",
            CallText::SetLink => "Bağlantıyı Kaydet",
            CallText::RemoveLink => "Bağlantıyı Kaldır",
            CallText::LinkNote => {
                "Partideki herkes bu bağlantıyı görür. Yalnızca lider değiştirebilir."
            }
            CallText::LinkInvalid => {
                "Bu bir Discord bağlantısı değil. discord.com/channels veya discord.gg adresi kullan."
            }
            CallText::ToastMissed => "%@ seni aradı, açamadın",
            CallText::ToastNoAnswer => "%@ yanıt vermedi.",
            CallText::ToastDeclined => "%@ aramayı reddetti.",
            CallText::ToastUnreachable => "%@ şu an arama alamıyor.",
            CallText::ToastNeedsLeader => "Yalnızca aramayı başlatan kişi ekleyebilir.",
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            CallText::IncomingName => "Anruf von %@",
            CallText::IncomingLine => "ruft dich an",
            CallText::IncomingDiscordLine => "Dein Sprachchat ist auf Discord eingestellt",
            CallText::Accept => "Annehmen",
            CallText::Decline => "Ablehnen",
            CallText::InCall => "Im Anruf",
            CallText::Calling => "%@ wird angerufen",
            CallText::Cancel => "Abbrechen",
            CallText::Mute => "Stummschalten",
            CallText::Unmute => "Stummschaltung aufheben",
            CallText::Deafen => "Ton ausschalten",
            CallText::Undeafen => "Ton einschalten",
            CallText::Leave => "Anruf verlassen",
            CallText::SpeakingSuffix => "spricht",
            CallText::YouMuted => "Du bist stummgeschaltet",
            CallText::YouDeafened => "Du hörst niemanden",
            CallText::WaitingForOthers => "Warten auf die anderen",
            CallText::VoiceOnDiscord => "Sprachchat auf Discord",
            CallText::AnswerOnDiscord => "Auf Discord annehmen",
            CallText::OpenInDiscord => "In Discord öffnen",
            CallText::DiscordSetNote => {
                "Dein Sprachchat ist auf Discord eingestellt. Tritt dem Anruf dort bei; AnyPlay öffnet dein Mikrofon nicht."
            }
            CallText::MissedAgo => "Verpasster Anruf · %@",
            CallText::MissedNow => "Verpasster Anruf · gerade eben",
            CallText::CallFriend => "Anrufen",
            CallText::CallFriendDetail => "Klingelt 30 Sekunden",
            CallText::AddToCall => "Zum Anruf hinzufügen",
            CallText::CallNeedsLeader => {
                "Nur wer den Anruf gestartet hat, kann Personen hinzufügen."
            }
            CallText::CallFriendBusy => "Bereits in einer Party",
            CallText::VoiceOn => "Sprachdienst",
            CallText::ProviderAnyPlay => "AnyPlay",
            CallText::ProviderDiscord => "Discord",
            CallText::ProviderDiscordNote => {
                "Bei Discord öffnet AnyPlay dein Mikrofon nie für Partys oder Anrufe. Anrufe klingeln trotzdem hier."
            }
            CallText::ProviderAnyPlayNote => {
                "Partys und Anrufe nutzen den eigenen Sprachchat von AnyPlay."
            }
            CallText::StatusDiscord => "Auf Discord",
            CallText::DiscordLink => "Discord-Link",
            CallText::DiscordLinkNone => "Nicht festgelegt",
            CallText::DiscordLinkField => "discord.com/channels/… oder discord.gg/…",
            CallText::SetLink => "Link speichern",
            CallText::RemoveLink => "Link entfernen",
            CallText::LinkNote => {
                "Alle in der Party sehen diesen Link. Nur die Leitung kann ihn ändern."
            }
            CallText::LinkInvalid => {
                "Das ist kein Discord-Link. Nutze eine Adresse von discord.com/channels oder discord.gg."
            }
            CallText::ToastMissed => "Verpasster Anruf von %@",
            CallText::ToastNoAnswer => "%@ hat nicht geantwortet.",
            CallText::ToastDeclined => "%@ hat den Anruf abgelehnt.",
            CallText::ToastUnreachable => "%@ kann gerade keinen Anruf annehmen.",
            CallText::ToastNeedsLeader => {
                "Nur wer den Anruf gestartet hat, kann Personen hinzufügen."
            }
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            CallText::IncomingName => "Llamada entrante de %@",
            CallText::IncomingLine => "te está llamando",
            CallText::IncomingDiscordLine => "Tu voz está configurada en Discord",
            CallText::Accept => "Aceptar",
            CallText::Decline => "Rechazar",
            CallText::InCall => "En llamada",
            CallText::Calling => "Llamando a %@",
            CallText::Cancel => "Cancelar",
            CallText::Mute => "Silenciar",
            CallText::Unmute => "Quitar silencio",
            CallText::Deafen => "Desactivar sonido",
            CallText::Undeafen => "Activar sonido",
            CallText::Leave => "Salir de la llamada",
            CallText::SpeakingSuffix => "hablando",
            CallText::YouMuted => "Tu micrófono está silenciado",
            CallText::YouDeafened => "No oyes a nadie",
            CallText::WaitingForOthers => "Esperando a los demás",
            CallText::VoiceOnDiscord => "Voz en Discord",
            CallText::AnswerOnDiscord => "Responder en Discord",
            CallText::OpenInDiscord => "Abrir en Discord",
            CallText::DiscordSetNote => {
                "Tu voz está configurada en Discord. Únete a la llamada allí; AnyPlay no abre tu micrófono."
            }
            CallText::MissedAgo => "Llamada perdida · %@",
            CallText::MissedNow => "Llamada perdida · ahora mismo",
            CallText::CallFriend => "Llamar",
            CallText::CallFriendDetail => "Suena durante 30 segundos",
            CallText::AddToCall => "Añadir a la llamada",
            CallText::CallNeedsLeader => "Solo quien inició la llamada puede añadir personas.",
            CallText::CallFriendBusy => "Ya está en una party",
            CallText::VoiceOn => "Servicio de voz",
            CallText::ProviderAnyPlay => "AnyPlay",
            CallText::ProviderDiscord => "Discord",
            CallText::ProviderDiscordNote => {
                "Con Discord, AnyPlay nunca abre tu micrófono en parties ni llamadas. Las llamadas siguen sonando aquí."
            }
            CallText::ProviderAnyPlayNote => {
                "Las parties y las llamadas usan el chat de voz propio de AnyPlay."
            }
            CallText::StatusDiscord => "En Discord",
            CallText::DiscordLink => "Enlace de Discord",
            CallText::DiscordLinkNone => "Sin configurar",
            CallText::DiscordLinkField => "discord.com/channels/… o discord.gg/…",
            CallText::SetLink => "Guardar enlace",
            CallText::RemoveLink => "Quitar enlace",
            CallText::LinkNote => {
                "Todos los de la party ven este enlace. Solo el líder puede cambiarlo."
            }
            CallText::LinkInvalid => {
                "Eso no es un enlace de Discord. Usa una dirección de discord.com/channels o discord.gg."
            }
            CallText::ToastMissed => "Llamada perdida de %@",
            CallText::ToastNoAnswer => "%@ no respondió.",
            CallText::ToastDeclined => "%@ rechazó la llamada.",
            CallText::ToastUnreachable => "%@ no puede recibir una llamada ahora mismo.",
            CallText::ToastNeedsLeader => "Solo quien inició la llamada puede añadir personas.",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            CallText::IncomingName => "Appel entrant de %@",
            CallText::IncomingLine => "t’appelle",
            CallText::IncomingDiscordLine => "Ta voix est réglée sur Discord",
            CallText::Accept => "Accepter",
            CallText::Decline => "Refuser",
            CallText::InCall => "En appel",
            CallText::Calling => "Appel vers %@",
            CallText::Cancel => "Annuler",
            CallText::Mute => "Couper le micro",
            CallText::Unmute => "Réactiver le micro",
            CallText::Deafen => "Couper le son",
            CallText::Undeafen => "Rétablir le son",
            CallText::Leave => "Quitter l’appel",
            CallText::SpeakingSuffix => "parle",
            CallText::YouMuted => "Ton micro est coupé",
            CallText::YouDeafened => "Tu n’entends personne",
            CallText::WaitingForOthers => "En attente des autres",
            CallText::VoiceOnDiscord => "Voix sur Discord",
            CallText::AnswerOnDiscord => "Répondre sur Discord",
            CallText::OpenInDiscord => "Ouvrir dans Discord",
            CallText::DiscordSetNote => {
                "Ta voix est réglée sur Discord. Rejoins l’appel là-bas\u{a0}; AnyPlay n’active pas ton micro."
            }
            CallText::MissedAgo => "Appel manqué · %@",
            CallText::MissedNow => "Appel manqué · à l’instant",
            CallText::CallFriend => "Appeler",
            CallText::CallFriendDetail => "Sonne pendant 30 secondes",
            CallText::AddToCall => "Ajouter à l’appel",
            CallText::CallNeedsLeader => {
                "Seule la personne qui a lancé l’appel peut ajouter des gens."
            }
            CallText::CallFriendBusy => "Déjà dans une party",
            CallText::VoiceOn => "Service vocal",
            CallText::ProviderAnyPlay => "AnyPlay",
            CallText::ProviderDiscord => "Discord",
            CallText::ProviderDiscordNote => {
                "Avec Discord, AnyPlay n’active jamais ton micro pour les party ou les appels. Les appels sonnent toujours ici."
            }
            CallText::ProviderAnyPlayNote => {
                "Les party et les appels utilisent le chat vocal propre d’AnyPlay."
            }
            CallText::StatusDiscord => "Sur Discord",
            CallText::DiscordLink => "Lien Discord",
            CallText::DiscordLinkNone => "Non défini",
            CallText::DiscordLinkField => "discord.com/channels/… ou discord.gg/…",
            CallText::SetLink => "Enregistrer le lien",
            CallText::RemoveLink => "Supprimer le lien",
            CallText::LinkNote => {
                "Tous les membres de la party voient ce lien. Seul le chef peut le modifier."
            }
            CallText::LinkInvalid => {
                "Ce n’est pas un lien Discord. Utilise une adresse de discord.com/channels ou discord.gg."
            }
            CallText::ToastMissed => "Appel manqué de %@",
            CallText::ToastNoAnswer => "%@ n’a pas répondu.",
            CallText::ToastDeclined => "%@ a refusé l’appel.",
            CallText::ToastUnreachable => "%@ ne peut pas recevoir d’appel pour l’instant.",
            CallText::ToastNeedsLeader => {
                "Seule la personne qui a lancé l’appel peut ajouter des gens."
            }
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            CallText::IncomingName => "Chamada recebida de %@",
            CallText::IncomingLine => "está ligando para você",
            CallText::IncomingDiscordLine => "Sua voz está configurada no Discord",
            CallText::Accept => "Aceitar",
            CallText::Decline => "Recusar",
            CallText::InCall => "Em chamada",
            CallText::Calling => "Chamando %@",
            CallText::Cancel => "Cancelar",
            CallText::Mute => "Silenciar",
            CallText::Unmute => "Reativar microfone",
            CallText::Deafen => "Desativar áudio",
            CallText::Undeafen => "Ativar áudio",
            CallText::Leave => "Sair da chamada",
            CallText::SpeakingSuffix => "falando",
            CallText::YouMuted => "Seu microfone está silenciado",
            CallText::YouDeafened => "Você não ouve ninguém",
            CallText::WaitingForOthers => "Aguardando os outros",
            CallText::VoiceOnDiscord => "Voz no Discord",
            CallText::AnswerOnDiscord => "Atender no Discord",
            CallText::OpenInDiscord => "Abrir no Discord",
            CallText::DiscordSetNote => {
                "Sua voz está configurada no Discord. Entre na chamada por lá; o AnyPlay não abre seu microfone."
            }
            CallText::MissedAgo => "Chamada perdida · %@",
            CallText::MissedNow => "Chamada perdida · agora mesmo",
            CallText::CallFriend => "Ligar",
            CallText::CallFriendDetail => "Toca por 30 segundos",
            CallText::AddToCall => "Adicionar à chamada",
            CallText::CallNeedsLeader => "Só quem iniciou a chamada pode adicionar pessoas.",
            CallText::CallFriendBusy => "Já está em uma party",
            CallText::VoiceOn => "Serviço de voz",
            CallText::ProviderAnyPlay => "AnyPlay",
            CallText::ProviderDiscord => "Discord",
            CallText::ProviderDiscordNote => {
                "Com o Discord, o AnyPlay nunca abre seu microfone em parties ou chamadas. As chamadas continuam tocando aqui."
            }
            CallText::ProviderAnyPlayNote => {
                "Parties e chamadas usam o chat de voz próprio do AnyPlay."
            }
            CallText::StatusDiscord => "No Discord",
            CallText::DiscordLink => "Link do Discord",
            CallText::DiscordLinkNone => "Não definido",
            CallText::DiscordLinkField => "discord.com/channels/… ou discord.gg/…",
            CallText::SetLink => "Salvar link",
            CallText::RemoveLink => "Remover link",
            CallText::LinkNote => "Todos na party veem este link. Só o líder pode alterá-lo.",
            CallText::LinkInvalid => {
                "Isso não é um link do Discord. Use um endereço de discord.com/channels ou discord.gg."
            }
            CallText::ToastMissed => "Chamada perdida de %@",
            CallText::ToastNoAnswer => "%@ não atendeu.",
            CallText::ToastDeclined => "%@ recusou a chamada.",
            CallText::ToastUnreachable => "%@ não pode receber uma chamada agora.",
            CallText::ToastNeedsLeader => "Só quem iniciou a chamada pode adicionar pessoas.",
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

/// How long ago a missed call was: "just now", "5 min ago", "2 h ago", "3 d ago".
pub fn ago(seconds: u64, language: Language) -> String {
    let (n, unit) = match seconds {
        0..=59 => return CallText::MissedNow.text(language).to_owned(),
        60..=3599 => (seconds / 60, "min"),
        3600..=86_399 => (seconds / 3600, "h"),
        _ => (seconds / 86_400, "d"),
    };
    let how = match language {
        Language::English => format!("{n} {unit} ago"),
        Language::German => format!(
            "vor {n} {}",
            match unit {
                "min" => "Min.",
                "h" => "Std.",
                _ => "Tg.",
            }
        ),
        Language::Turkish => format!(
            "{n} {} önce",
            match unit {
                "min" => "dk",
                "h" => "sa",
                _ => "gün",
            }
        ),
        Language::Spanish => format!("hace {n} {unit}"),
        Language::French => format!("il y a {n}\u{a0}{}", if unit == "d" { "j" } else { unit }),
        Language::Portuguese => format!("há {n} {unit}"),
    };
    CallText::MissedAgo.format(language, &[Arg::Str(&how)])
}

#[cfg(test)]
mod tests {
    use super::*;

    const WITH_ARGUMENT: [CallText; 7] = [
        CallText::IncomingName,
        CallText::Calling,
        CallText::MissedAgo,
        CallText::ToastMissed,
        CallText::ToastNoAnswer,
        CallText::ToastDeclined,
        CallText::ToastUnreachable,
    ];

    #[test]
    fn placeholders_match_between_languages() {
        for text in WITH_ARGUMENT {
            let count = |s: &str| s.matches("%@").count();
            assert_eq!(count(text.english()), 1, "{text:?}");
            assert_eq!(count(text.turkish()), 1, "{text:?}");
            for language in Language::ALL {
                assert_eq!(count(text.text(language)), 1, "{text:?} {language:?}");
            }
        }
    }

    #[test]
    fn a_missed_call_says_how_long_ago() {
        assert_eq!(ago(10, Language::English), "Missed call · just now");
        assert_eq!(ago(300, Language::English), "Missed call · 5 min ago");
        assert_eq!(ago(7300, Language::English), "Missed call · 2 h ago");
        assert_eq!(ago(3 * 86_400, Language::English), "Missed call · 3 d ago");
        assert_eq!(ago(300, Language::German), "Verpasster Anruf · vor 5 Min.");
        assert_eq!(ago(300, Language::Turkish), "Cevapsız arama · 5 dk önce");
        assert_eq!(ago(300, Language::Spanish), "Llamada perdida · hace 5 min");
        assert_eq!(
            ago(7300, Language::French),
            "Appel manqué · il y a 2\u{a0}h"
        );
        assert_eq!(
            ago(3 * 86_400, Language::French),
            "Appel manqué · il y a 3\u{a0}j"
        );
        assert_eq!(
            ago(3 * 86_400, Language::Portuguese),
            "Chamada perdida · há 3 d"
        );
        assert_eq!(ago(10, Language::French), "Appel manqué · à l’instant");
    }
}
