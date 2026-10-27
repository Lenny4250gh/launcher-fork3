//! Texts of the confirmations of the online area (docs/DESIGN.md 17, "Confirmations"). English and
//! Turkish like the rest. The headline is the question and ends with a question mark; the line
//! under it says what the action ends and whether it can be taken back, in sentence case. The
//! Turkish was written for this change and has not been read by a native speaker yet.

use crate::ui::i18n::{Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ConfirmText {
    /// `%@` = who.
    BlockTitle,
    /// `%@` = who.
    BlockNote,
    /// `%@` = who.
    BlockRequestNote,
    /// `%@` = who.
    RemoveFriendTitle,
    RemoveFriendNote,
    UnsendTitle,
    UnsendNote,
    LeavePartyTitle,
    LeavePartyNote,
    /// `%@` = who.
    RemoveMemberTitle,
    RemoveMemberNote,
    LeaveConversationTitle,
    LeaveConversationNote,
}

impl ConfirmText {
    pub const fn english(self) -> &'static str {
        match self {
            ConfirmText::BlockTitle => "Block %@?",
            ConfirmText::BlockNote => {
                "%@ can no longer message you or send you friend requests. You can unblock in Settings."
            }
            ConfirmText::BlockRequestNote => {
                "%@ can no longer message you or send you friend requests, and this request is removed. You can unblock in Settings."
            }
            ConfirmText::RemoveFriendTitle => "Remove %@ as a friend?",
            ConfirmText::RemoveFriendNote => {
                "You stop seeing each other's status. Either of you can send a new request later."
            }
            ConfirmText::UnsendTitle => "Unsend this message?",
            ConfirmText::UnsendNote => {
                "It is removed for everyone in the conversation. This cannot be undone."
            }
            ConfirmText::LeavePartyTitle => "Leave the party?",
            ConfirmText::LeavePartyNote => "You need a new invitation to come back.",
            ConfirmText::RemoveMemberTitle => "Remove %@ from the party?",
            ConfirmText::RemoveMemberNote => "They need a new invitation to come back.",
            ConfirmText::LeaveConversationTitle => "Leave this conversation?",
            ConfirmText::LeaveConversationNote => {
                "You stop getting its messages. Someone has to add you again to bring you back."
            }
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            ConfirmText::BlockTitle => "%@ engellensin mi?",
            ConfirmText::BlockNote => {
                "%@ artık sana mesaj ya da arkadaşlık isteği gönderemez. Engeli Ayarlar'dan kaldırabilirsin."
            }
            ConfirmText::BlockRequestNote => {
                "%@ artık sana mesaj ya da arkadaşlık isteği gönderemez ve bu istek kaldırılır. Engeli Ayarlar'dan kaldırabilirsin."
            }
            ConfirmText::RemoveFriendTitle => "%@ arkadaşlıktan çıkarılsın mı?",
            ConfirmText::RemoveFriendNote => {
                "Birbirinizin durumunu görmezsiniz. İkinizden biri sonra yeni istek gönderebilir."
            }
            ConfirmText::UnsendTitle => "Mesaj geri alınsın mı?",
            ConfirmText::UnsendNote => "Konuşmadaki herkes için kaldırılır. Bu geri alınamaz.",
            ConfirmText::LeavePartyTitle => "Partiden ayrılınsın mı?",
            ConfirmText::LeavePartyNote => "Geri dönmek için yeni bir davet gerekir.",
            ConfirmText::RemoveMemberTitle => "%@ partiden çıkarılsın mı?",
            ConfirmText::RemoveMemberNote => "Geri dönmesi için yeni bir davet gerekir.",
            ConfirmText::LeaveConversationTitle => "Sohbetten ayrılınsın mı?",
            ConfirmText::LeaveConversationNote => {
                "Bu sohbetin mesajlarını artık almazsın. Geri dönmen için biri seni yeniden eklemeli."
            }
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            ConfirmText::BlockTitle => "%@ blockieren?",
            ConfirmText::BlockNote => {
                "%@ kann dir keine Nachrichten oder Freundschaftsanfragen mehr senden. Du kannst die Blockierung in den Einstellungen aufheben."
            }
            ConfirmText::BlockRequestNote => {
                "%@ kann dir keine Nachrichten oder Freundschaftsanfragen mehr senden, und diese Anfrage wird entfernt. Du kannst die Blockierung in den Einstellungen aufheben."
            }
            ConfirmText::RemoveFriendTitle => "%@ als Freund entfernen?",
            ConfirmText::RemoveFriendNote => {
                "Ihr seht den Status des anderen nicht mehr. Später kann jeder von euch eine neue Anfrage senden."
            }
            ConfirmText::UnsendTitle => "Diese Nachricht zurückziehen?",
            ConfirmText::UnsendNote => {
                "Sie wird für alle in der Unterhaltung entfernt. Das lässt sich nicht rückgängig machen."
            }
            ConfirmText::LeavePartyTitle => "Die Party verlassen?",
            ConfirmText::LeavePartyNote => "Um zurückzukehren, brauchst du eine neue Einladung.",
            ConfirmText::RemoveMemberTitle => "%@ aus der Party entfernen?",
            ConfirmText::RemoveMemberNote => {
                "Zum Zurückkehren braucht die Person eine neue Einladung."
            }
            ConfirmText::LeaveConversationTitle => "Diese Unterhaltung verlassen?",
            ConfirmText::LeaveConversationNote => {
                "Du bekommst ihre Nachrichten nicht mehr. Jemand muss dich erneut hinzufügen, damit du zurückkommst."
            }
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            ConfirmText::BlockTitle => "¿Bloquear a %@?",
            ConfirmText::BlockNote => {
                "%@ ya no podrá enviarte mensajes ni solicitudes de amistad. Puedes desbloquear en Ajustes."
            }
            ConfirmText::BlockRequestNote => {
                "%@ ya no podrá enviarte mensajes ni solicitudes de amistad, y esta solicitud se elimina. Puedes desbloquear en Ajustes."
            }
            ConfirmText::RemoveFriendTitle => "¿Quitar a %@ de tus amigos?",
            ConfirmText::RemoveFriendNote => {
                "Ya no verán el estado del otro. Cualquiera de los dos puede enviar una solicitud nueva más tarde."
            }
            ConfirmText::UnsendTitle => "¿Retirar este mensaje?",
            ConfirmText::UnsendNote => {
                "Se elimina para todos en la conversación. No se puede deshacer."
            }
            ConfirmText::LeavePartyTitle => "¿Salir de la party?",
            ConfirmText::LeavePartyNote => "Necesitas una invitación nueva para volver.",
            ConfirmText::RemoveMemberTitle => "¿Quitar a %@ de la party?",
            ConfirmText::RemoveMemberNote => "Necesita una invitación nueva para volver.",
            ConfirmText::LeaveConversationTitle => "¿Salir de esta conversación?",
            ConfirmText::LeaveConversationNote => {
                "Dejas de recibir sus mensajes. Alguien tiene que añadirte de nuevo para que vuelvas."
            }
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            ConfirmText::BlockTitle => "Bloquer %@\u{a0}?",
            ConfirmText::BlockNote => {
                "%@ ne pourra plus t’envoyer de messages ni de demandes d’ami. Tu peux débloquer dans les paramètres."
            }
            ConfirmText::BlockRequestNote => {
                "%@ ne pourra plus t’envoyer de messages ni de demandes d’ami, et cette demande est supprimée. Tu peux débloquer dans les paramètres."
            }
            ConfirmText::RemoveFriendTitle => "Retirer %@ de tes amis\u{a0}?",
            ConfirmText::RemoveFriendNote => {
                "Vous ne verrez plus le statut l’un de l’autre. L’un de vous pourra envoyer une nouvelle demande plus tard."
            }
            ConfirmText::UnsendTitle => "Retirer ce message\u{a0}?",
            ConfirmText::UnsendNote => {
                "Il est supprimé pour tout le monde dans la conversation. Impossible d’annuler."
            }
            ConfirmText::LeavePartyTitle => "Quitter la party\u{a0}?",
            ConfirmText::LeavePartyNote => "Il te faut une nouvelle invitation pour revenir.",
            ConfirmText::RemoveMemberTitle => "Retirer %@ de la party\u{a0}?",
            ConfirmText::RemoveMemberNote => {
                "Cette personne a besoin d’une nouvelle invitation pour revenir."
            }
            ConfirmText::LeaveConversationTitle => "Quitter cette conversation\u{a0}?",
            ConfirmText::LeaveConversationNote => {
                "Tu ne reçois plus ses messages. Quelqu’un doit t’ajouter de nouveau pour que tu reviennes."
            }
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            ConfirmText::BlockTitle => "Bloquear %@?",
            ConfirmText::BlockNote => {
                "%@ não poderá mais enviar mensagens nem pedidos de amizade para você. Você pode desbloquear em Configurações."
            }
            ConfirmText::BlockRequestNote => {
                "%@ não poderá mais enviar mensagens nem pedidos de amizade para você, e este pedido é removido. Você pode desbloquear em Configurações."
            }
            ConfirmText::RemoveFriendTitle => "Remover %@ dos amigos?",
            ConfirmText::RemoveFriendNote => {
                "Vocês deixam de ver o status um do outro. Qualquer um dos dois pode enviar um novo pedido depois."
            }
            ConfirmText::UnsendTitle => "Apagar esta mensagem para todos?",
            ConfirmText::UnsendNote => {
                "Ela é removida para todos na conversa. Isso não pode ser desfeito."
            }
            ConfirmText::LeavePartyTitle => "Sair da party?",
            ConfirmText::LeavePartyNote => "Você precisa de um novo convite para voltar.",
            ConfirmText::RemoveMemberTitle => "Remover %@ da party?",
            ConfirmText::RemoveMemberNote => "A pessoa precisa de um novo convite para voltar.",
            ConfirmText::LeaveConversationTitle => "Sair desta conversa?",
            ConfirmText::LeaveConversationNote => {
                "Você deixa de receber as mensagens dela. Alguém precisa adicionar você de novo para você voltar."
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

    pub fn format(self, language: Language, args: &[crate::ui::i18n::Arg<'_>]) -> String {
        substitute(self.text(language), args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::i18n::Arg;

    #[test]
    fn placeholders_match_between_languages() {
        for text in [
            ConfirmText::BlockTitle,
            ConfirmText::BlockNote,
            ConfirmText::BlockRequestNote,
            ConfirmText::RemoveFriendTitle,
            ConfirmText::RemoveMemberTitle,
            ConfirmText::UnsendTitle,
            ConfirmText::LeavePartyTitle,
        ] {
            let count = |s: &str| s.matches("%@").count();
            assert_eq!(count(text.english()), count(text.turkish()), "{text:?}");
            for language in Language::ALL {
                assert_eq!(
                    count(text.english()),
                    count(text.text(language)),
                    "{text:?} {language:?}"
                );
            }
        }
    }

    #[test]
    fn a_title_asks_and_a_note_ends_in_a_full_stop() {
        for title in [
            ConfirmText::BlockTitle,
            ConfirmText::RemoveFriendTitle,
            ConfirmText::UnsendTitle,
            ConfirmText::LeavePartyTitle,
            ConfirmText::RemoveMemberTitle,
            ConfirmText::LeaveConversationTitle,
        ] {
            assert!(title.english().ends_with('?'), "{title:?}");
            assert!(title.turkish().ends_with('?'), "{title:?}");
            for language in Language::ALL {
                assert!(
                    title.text(language).ends_with('?'),
                    "{title:?} {language:?}"
                );
            }
        }
        for note in [
            ConfirmText::BlockNote,
            ConfirmText::BlockRequestNote,
            ConfirmText::RemoveFriendNote,
            ConfirmText::UnsendNote,
            ConfirmText::LeavePartyNote,
            ConfirmText::RemoveMemberNote,
            ConfirmText::LeaveConversationNote,
        ] {
            assert!(note.english().ends_with('.'), "{note:?}");
            assert!(note.turkish().ends_with('.'), "{note:?}");
            for language in Language::ALL {
                assert!(note.text(language).ends_with('.'), "{note:?} {language:?}");
            }
        }
        assert_eq!(
            ConfirmText::BlockTitle.format(Language::English, &[Arg::Str("Lumen")]),
            "Block Lumen?"
        );
    }
}
