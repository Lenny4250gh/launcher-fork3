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

    /// The German text; `None` falls back to English.
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            NotifyText::Title => "Benachrichtigungen",
            NotifyText::BellUnread => "Benachrichtigungen, %@ neu",
            NotifyText::ClearAll => "Alle löschen",
            NotifyText::KindFriendRequest => "Freundschaftsanfrage",
            NotifyText::KindInvitation => "Einladung",
            NotifyText::KindMessage => "Nachricht",
            NotifyText::KindTrophy => "Trophäe",
            NotifyText::KindUpdate => "Update",
            NotifyText::KindTogether => "Gemeinsam spielen",
            NotifyText::KindSystem => "Hinweis",
            NotifyText::Accept => "Annehmen",
            NotifyText::Join => "Beitreten",
            NotifyText::Open => "Öffnen",
            NotifyText::Dismiss => "Schließen",
            NotifyText::EntryName => "%@, %@, %@",
            NotifyText::EmptyTitle => "Keine Benachrichtigungen",
            NotifyText::EmptyBody => {
                "Freundschaftsanfragen, Einladungen, Nachrichten, Trophäen und Updates erscheinen hier."
            }
            NotifyText::CopyInviteLink => "Einladungslink kopieren",
            NotifyText::CopyInviteLinkDetail => {
                "Ein Link, der zum Beitreten einlädt; er tritt nie von selbst bei"
            }
            NotifyText::InviteLinkCopied => "Einladungslink kopiert",
            NotifyText::InviteLinkWaiting => "Dein Einladungslink wird erstellt…",
            NotifyText::InviteLinkOffline => {
                "Melde dich im AnyPlay-Netzwerk an, um einen Einladungslink zu erhalten."
            }
            NotifyText::InviteLinkFailed => {
                "Der Einladungslink konnte nicht erstellt werden. Versuche es gleich noch einmal."
            }
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            NotifyText::Title => "Notificaciones",
            NotifyText::BellUnread => "Notificaciones, %@ nuevas",
            NotifyText::ClearAll => "Borrar todo",
            NotifyText::KindFriendRequest => "Solicitud de amistad",
            NotifyText::KindInvitation => "Invitación",
            NotifyText::KindMessage => "Mensaje",
            NotifyText::KindTrophy => "Trofeo",
            NotifyText::KindUpdate => "Actualización",
            NotifyText::KindTogether => "Play Together",
            NotifyText::KindSystem => "Aviso",
            NotifyText::Accept => "Aceptar",
            NotifyText::Join => "Unirse",
            NotifyText::Open => "Abrir",
            NotifyText::Dismiss => "Descartar",
            NotifyText::EntryName => "%@, %@, %@",
            NotifyText::EmptyTitle => "Sin notificaciones",
            NotifyText::EmptyBody => {
                "Aquí aparecen las solicitudes de amistad, invitaciones, mensajes, trofeos y actualizaciones."
            }
            NotifyText::CopyInviteLink => "Copiar enlace de invitación",
            NotifyText::CopyInviteLinkDetail => {
                "Un enlace que invita a unirse; nunca se une por sí solo"
            }
            NotifyText::InviteLinkCopied => "Enlace de invitación copiado",
            NotifyText::InviteLinkWaiting => "Obteniendo tu enlace de invitación…",
            NotifyText::InviteLinkOffline => {
                "Inicia sesión en la red de AnyPlay para obtener un enlace de invitación."
            }
            NotifyText::InviteLinkFailed => {
                "No se pudo obtener el enlace de invitación. Inténtalo de nuevo en un momento."
            }
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            NotifyText::Title => "Notifications",
            NotifyText::BellUnread => "Notifications, %@ nouvelles",
            NotifyText::ClearAll => "Tout effacer",
            NotifyText::KindFriendRequest => "Demande d’ami",
            NotifyText::KindInvitation => "Invitation",
            NotifyText::KindMessage => "Message",
            NotifyText::KindTrophy => "Trophée",
            NotifyText::KindUpdate => "Mise à jour",
            NotifyText::KindTogether => "Play Together",
            NotifyText::KindSystem => "Avis",
            NotifyText::Accept => "Accepter",
            NotifyText::Join => "Rejoindre",
            NotifyText::Open => "Ouvrir",
            NotifyText::Dismiss => "Ignorer",
            NotifyText::EntryName => "%@, %@, %@",
            NotifyText::EmptyTitle => "Aucune notification",
            NotifyText::EmptyBody => {
                "Les demandes d’ami, invitations, messages, trophées et mises à jour apparaissent ici."
            }
            NotifyText::CopyInviteLink => "Copier le lien d’invitation",
            NotifyText::CopyInviteLinkDetail => {
                "Un lien qui invite à rejoindre\u{a0}; il ne rejoint jamais tout seul"
            }
            NotifyText::InviteLinkCopied => "Lien d’invitation copié",
            NotifyText::InviteLinkWaiting => "Récupération de ton lien d’invitation…",
            NotifyText::InviteLinkOffline => {
                "Connecte-toi au réseau AnyPlay pour obtenir un lien d’invitation."
            }
            NotifyText::InviteLinkFailed => {
                "Impossible d’obtenir le lien d’invitation. Réessaie dans un instant."
            }
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            NotifyText::Title => "Notificações",
            NotifyText::BellUnread => "Notificações, %@ novas",
            NotifyText::ClearAll => "Limpar tudo",
            NotifyText::KindFriendRequest => "Pedido de amizade",
            NotifyText::KindInvitation => "Convite",
            NotifyText::KindMessage => "Mensagem",
            NotifyText::KindTrophy => "Troféu",
            NotifyText::KindUpdate => "Atualização",
            NotifyText::KindTogether => "Play Together",
            NotifyText::KindSystem => "Aviso",
            NotifyText::Accept => "Aceitar",
            NotifyText::Join => "Entrar",
            NotifyText::Open => "Abrir",
            NotifyText::Dismiss => "Dispensar",
            NotifyText::EntryName => "%@, %@, %@",
            NotifyText::EmptyTitle => "Nenhuma notificação",
            NotifyText::EmptyBody => {
                "Pedidos de amizade, convites, mensagens, troféus e atualizações aparecem aqui."
            }
            NotifyText::CopyInviteLink => "Copiar link de convite",
            NotifyText::CopyInviteLinkDetail => {
                "Um link que convida a entrar; ele nunca entra sozinho"
            }
            NotifyText::InviteLinkCopied => "Link de convite copiado",
            NotifyText::InviteLinkWaiting => "Obtendo seu link de convite…",
            NotifyText::InviteLinkOffline => {
                "Entre na rede do AnyPlay para obter um link de convite."
            }
            NotifyText::InviteLinkFailed => {
                "Não foi possível obter o link de convite. Tente de novo em instantes."
            }
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
