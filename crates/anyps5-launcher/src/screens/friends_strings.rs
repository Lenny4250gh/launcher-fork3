//! Texts of the Friends screen (docs/DESIGN.md 17, "Friends screen"): the sections of its rail,
//! the sections of the friends list and the states around it. English and Turkish like the rest.
//! Titles of sections and buttons are in title case, notes and details in sentence case.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FriendsText {
    Activity,
    Settings,
    /// The title of the page behind Settings.
    OnlineSettings,
    // Sections of the friends list
    PlayingNow,
    OnlineCaption,
    OfflineCaption,
    // The line under the handle
    /// `%@` = how many.
    CountMany,
    CountOne,
    CountNone,
    // Actions on a friend's row
    ActionJoin,
    ActionMessage,
    /// The accessible name of the row's own press: the friend's page.
    MoreAbout,
    // States
    ConnectingNote,
    OfflineNote,
    TryAgain,
    /// A friend request while offline: it cannot be answered yet.
    RequestOffline,
    /// The bar where the composer would be, while offline.
    ComposerOffline,
    NotInParty,
    NotInPartyNote,
    /// The empty Friends page: what it is, and how to fill it.
    NoFriendsTitle,
    NoFriendsNote,
    /// The empty Messages page.
    NoConversationsTitle,
    NoConversationsNote,
    /// The state title of a page that needs the cloud while it is out.
    OfflineTitle,
    PartyOfflineNote,
    /// The rail's name for an item with something waiting: `%@` = the item, `%@` = how many.
    WaitingName,
}

impl FriendsText {
    pub const fn english(self) -> &'static str {
        match self {
            FriendsText::Activity => "Activity",
            FriendsText::Settings => "Settings",
            FriendsText::OnlineSettings => "Online Settings",
            FriendsText::PlayingNow => "Playing Now",
            FriendsText::OnlineCaption => "Online",
            FriendsText::OfflineCaption => "Offline",
            FriendsText::CountMany => "%@ friends active",
            FriendsText::CountOne => "1 friend active",
            FriendsText::CountNone => "No friends active",
            FriendsText::ActionJoin => "Join",
            FriendsText::ActionMessage => "Message",
            FriendsText::MoreAbout => "Options",
            FriendsText::ConnectingNote => "Connecting to the network…",
            FriendsText::OfflineNote => {
                "You are offline. Friends and messages update when the connection is back."
            }
            FriendsText::TryAgain => "Try Again",
            FriendsText::RequestOffline => "Wants to be your friend. Answer when you are online.",
            FriendsText::ComposerOffline => {
                "You are offline. You can write again when the connection is back."
            }
            FriendsText::NoFriendsTitle => "No friends yet",
            FriendsText::NoFriendsNote => {
                "Friends show up here with what they play. Add one by the name they chose."
            }
            FriendsText::NoConversationsTitle => "No conversations yet",
            FriendsText::NoConversationsNote => {
                "Messages with friends and groups are kept here. Open a friend's page and choose Send Message."
            }
            FriendsText::OfflineTitle => "You are offline",
            FriendsText::PartyOfflineNote => {
                "A party needs a connection. Try again when it is back."
            }
            FriendsText::NotInParty => "You are not in a party",
            FriendsText::NotInPartyNote => {
                "A party keeps you together across games, with chat and voice."
            }
            FriendsText::WaitingName => "%@, %@ new",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            FriendsText::Activity => "Etkinlik",
            FriendsText::Settings => "Ayarlar",
            FriendsText::OnlineSettings => "Çevrimiçi Ayarları",
            FriendsText::PlayingNow => "Şimdi Oynayanlar",
            FriendsText::OnlineCaption => "Çevrimiçi",
            FriendsText::OfflineCaption => "Çevrimdışı",
            FriendsText::CountMany => "%@ arkadaş aktif",
            FriendsText::CountOne => "1 arkadaş aktif",
            FriendsText::CountNone => "Aktif arkadaş yok",
            FriendsText::ActionJoin => "Katıl",
            FriendsText::ActionMessage => "Mesaj",
            FriendsText::MoreAbout => "Seçenekler",
            FriendsText::ConnectingNote => "Ağa bağlanılıyor…",
            FriendsText::OfflineNote => {
                "Çevrimdışısın. Arkadaşlar ve mesajlar bağlantı gelince güncellenir."
            }
            FriendsText::TryAgain => "Tekrar Dene",
            FriendsText::RequestOffline => "Arkadaşın olmak istiyor. Çevrimiçi olunca yanıtla.",
            FriendsText::ComposerOffline => "Çevrimdışısın. Bağlantı gelince yine yazabilirsin.",
            FriendsText::NoFriendsTitle => "Henüz arkadaşın yok",
            FriendsText::NoFriendsNote => {
                "Arkadaşlar oynadıklarıyla burada görünür. Seçtikleri adla birini ekle."
            }
            FriendsText::NoConversationsTitle => "Henüz sohbet yok",
            FriendsText::NoConversationsNote => {
                "Arkadaşlarınla ve gruplarla yazışmalar burada durur. Bir arkadaşın sayfasını aç ve Mesaj Gönder'i seç."
            }
            FriendsText::OfflineTitle => "Çevrimdışısın",
            FriendsText::PartyOfflineNote => {
                "Parti için bağlantı gerekir. Geri gelince tekrar dene."
            }
            FriendsText::NotInParty => "Bir partide değilsin",
            FriendsText::NotInPartyNote => {
                "Parti, sohbet ve sesle oyunlar arasında seni arkadaşlarınla bir arada tutar."
            }
            FriendsText::WaitingName => "%@, %@ yeni",
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            FriendsText::Activity => "Aktivität",
            FriendsText::Settings => "Einstellungen",
            FriendsText::OnlineSettings => "Online-Einstellungen",
            FriendsText::PlayingNow => "Spielen gerade",
            FriendsText::OnlineCaption => "Online",
            FriendsText::OfflineCaption => "Offline",
            FriendsText::CountMany => "%@ Freunde aktiv",
            FriendsText::CountOne => "1 Freund aktiv",
            FriendsText::CountNone => "Keine Freunde aktiv",
            FriendsText::ActionJoin => "Beitreten",
            FriendsText::ActionMessage => "Nachricht",
            FriendsText::MoreAbout => "Optionen",
            FriendsText::ConnectingNote => "Verbinde mit dem Netzwerk…",
            FriendsText::OfflineNote => {
                "Du bist offline. Freunde und Nachrichten aktualisieren sich, sobald die Verbindung wieder da ist."
            }
            FriendsText::TryAgain => "Erneut versuchen",
            FriendsText::RequestOffline => {
                "Möchte dein Freund sein. Antworte, wenn du online bist."
            }
            FriendsText::ComposerOffline => {
                "Du bist offline. Schreiben geht wieder, sobald die Verbindung da ist."
            }
            FriendsText::NoFriendsTitle => "Noch keine Freunde",
            FriendsText::NoFriendsNote => {
                "Freunde erscheinen hier mit dem, was sie spielen. Füge jemanden über den gewählten Namen hinzu."
            }
            FriendsText::NoConversationsTitle => "Noch keine Unterhaltungen",
            FriendsText::NoConversationsNote => {
                "Nachrichten mit Freunden und Gruppen findest du hier. Öffne die Seite eines Freundes und wähle Nachricht senden."
            }
            FriendsText::OfflineTitle => "Du bist offline",
            FriendsText::PartyOfflineNote => {
                "Eine Party braucht eine Verbindung. Versuche es erneut, sobald sie wieder da ist."
            }
            FriendsText::NotInParty => "Du bist in keiner Party",
            FriendsText::NotInPartyNote => {
                "Eine Party hält dich über Spiele hinweg mit Chat und Sprache zusammen."
            }
            FriendsText::WaitingName => "%@, %@ neu",
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            FriendsText::Activity => "Actividad",
            FriendsText::Settings => "Ajustes",
            FriendsText::OnlineSettings => "Ajustes en línea",
            FriendsText::PlayingNow => "Jugando ahora",
            FriendsText::OnlineCaption => "En línea",
            FriendsText::OfflineCaption => "Sin conexión",
            FriendsText::CountMany => "%@ amigos activos",
            FriendsText::CountOne => "1 amigo activo",
            FriendsText::CountNone => "Ningún amigo activo",
            FriendsText::ActionJoin => "Unirse",
            FriendsText::ActionMessage => "Mensaje",
            FriendsText::MoreAbout => "Opciones",
            FriendsText::ConnectingNote => "Conectando con la red…",
            FriendsText::OfflineNote => {
                "No tienes conexión. Los amigos y mensajes se actualizan cuando vuelva."
            }
            FriendsText::TryAgain => "Reintentar",
            FriendsText::RequestOffline => "Quiere ser tu amigo. Responde cuando estés en línea.",
            FriendsText::ComposerOffline => {
                "No tienes conexión. Podrás escribir de nuevo cuando vuelva."
            }
            FriendsText::NoFriendsTitle => "Aún no tienes amigos",
            FriendsText::NoFriendsNote => {
                "Los amigos aparecen aquí con lo que juegan. Añade a alguien por el nombre que eligió."
            }
            FriendsText::NoConversationsTitle => "Aún no hay conversaciones",
            FriendsText::NoConversationsNote => {
                "Aquí se guardan los mensajes con amigos y grupos. Abre la página de un amigo y elige Enviar mensaje."
            }
            FriendsText::OfflineTitle => "No tienes conexión",
            FriendsText::PartyOfflineNote => {
                "Una party necesita conexión. Inténtalo de nuevo cuando vuelva."
            }
            FriendsText::NotInParty => "No estás en ninguna party",
            FriendsText::NotInPartyNote => {
                "Una party te mantiene con tus amigos entre juegos, con chat y voz."
            }
            FriendsText::WaitingName => "%@, nuevos: %@",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            FriendsText::Activity => "Activité",
            FriendsText::Settings => "Paramètres",
            FriendsText::OnlineSettings => "Paramètres en ligne",
            FriendsText::PlayingNow => "En train de jouer",
            FriendsText::OnlineCaption => "En ligne",
            FriendsText::OfflineCaption => "Hors ligne",
            FriendsText::CountMany => "%@ amis actifs",
            FriendsText::CountOne => "1 ami actif",
            FriendsText::CountNone => "Aucun ami actif",
            FriendsText::ActionJoin => "Rejoindre",
            FriendsText::ActionMessage => "Message",
            FriendsText::MoreAbout => "Options",
            FriendsText::ConnectingNote => "Connexion au réseau…",
            FriendsText::OfflineNote => {
                "Tu es hors ligne. Les amis et les messages se mettent à jour au retour de la connexion."
            }
            FriendsText::TryAgain => "Réessayer",
            FriendsText::RequestOffline => "Veut être ton ami. Réponds quand tu seras en ligne.",
            FriendsText::ComposerOffline => {
                "Tu es hors ligne. Tu pourras réécrire au retour de la connexion."
            }
            FriendsText::NoFriendsTitle => "Pas encore d’amis",
            FriendsText::NoFriendsNote => {
                "Les amis apparaissent ici avec ce à quoi ils jouent. Ajoute quelqu’un grâce au nom qu’il a choisi."
            }
            FriendsText::NoConversationsTitle => "Pas encore de conversations",
            FriendsText::NoConversationsNote => {
                "Les messages avec tes amis et tes groupes sont conservés ici. Ouvre la page d’un ami et choisis Envoyer un message."
            }
            FriendsText::OfflineTitle => "Tu es hors ligne",
            FriendsText::PartyOfflineNote => {
                "Une party a besoin d’une connexion. Réessaie quand elle sera de retour."
            }
            FriendsText::NotInParty => "Tu n’es dans aucune party",
            FriendsText::NotInPartyNote => {
                "Une party te garde avec tes amis d’un jeu à l’autre, avec chat et voix."
            }
            FriendsText::WaitingName => "%@, nouveaux\u{a0}: %@",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            FriendsText::Activity => "Atividade",
            FriendsText::Settings => "Configurações",
            FriendsText::OnlineSettings => "Configurações online",
            FriendsText::PlayingNow => "Jogando agora",
            FriendsText::OnlineCaption => "Online",
            FriendsText::OfflineCaption => "Offline",
            FriendsText::CountMany => "%@ amigos ativos",
            FriendsText::CountOne => "1 amigo ativo",
            FriendsText::CountNone => "Nenhum amigo ativo",
            FriendsText::ActionJoin => "Entrar",
            FriendsText::ActionMessage => "Mensagem",
            FriendsText::MoreAbout => "Opções",
            FriendsText::ConnectingNote => "Conectando à rede…",
            FriendsText::OfflineNote => {
                "Você está offline. Amigos e mensagens se atualizam quando a conexão voltar."
            }
            FriendsText::TryAgain => "Tentar de novo",
            FriendsText::RequestOffline => "Quer ser seu amigo. Responda quando estiver online.",
            FriendsText::ComposerOffline => {
                "Você está offline. Poderá escrever de novo quando a conexão voltar."
            }
            FriendsText::NoFriendsTitle => "Nenhum amigo ainda",
            FriendsText::NoFriendsNote => {
                "Os amigos aparecem aqui com o que estão jogando. Adicione alguém pelo nome que escolheu."
            }
            FriendsText::NoConversationsTitle => "Nenhuma conversa ainda",
            FriendsText::NoConversationsNote => {
                "As mensagens com amigos e grupos ficam aqui. Abra a página de um amigo e escolha Enviar mensagem."
            }
            FriendsText::OfflineTitle => "Você está offline",
            FriendsText::PartyOfflineNote => {
                "Uma party precisa de conexão. Tente de novo quando ela voltar."
            }
            FriendsText::NotInParty => "Você não está em nenhuma party",
            FriendsText::NotInPartyNote => {
                "Uma party mantém você com seus amigos entre jogos, com chat e voz."
            }
            FriendsText::WaitingName => "%@, novos: %@",
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
        for text in [
            FriendsText::CountMany,
            FriendsText::CountOne,
            FriendsText::WaitingName,
            FriendsText::OfflineNote,
        ] {
            assert_eq!(
                text.english().matches("%@").count(),
                text.turkish().matches("%@").count(),
                "{text:?}"
            );
        }
    }
}
