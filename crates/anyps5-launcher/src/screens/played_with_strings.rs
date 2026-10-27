//! Texts of "Played With Recently" in the Friends screen (docs/DESIGN.md 17): the page, its
//! states, the switch, and the question before the list is cleared. The list is private: the
//! words say that only the player sees it. Titles and buttons are in title case, notes and
//! details in sentence case.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlayedWithText {
    /// The name of the page and of its row on the Friends page.
    Title,
    /// The line under the title of the page.
    Subtitle,
    /// The row's detail on the Friends page: `%@` = how many.
    PeopleMany,
    PeopleOne,
    /// The button that sends a friend request to the person.
    AddFriend,
    /// The mark on a person who is a friend already, and on one who was asked.
    Friend,
    RequestSent,
    /// What they played: `%@` = the title as the cloud knows it.
    Played,
    // States
    EmptyTitle,
    EmptyNote,
    LoadingTitle,
    FailedTitle,
    FailedNote,
    // The switch and the foot of the page
    Remember,
    RememberDetail,
    RememberOffNote,
    PrivateNote,
    ClearList,
    // The question before clearing
    ClearTitle,
    ClearNote,
}

impl PlayedWithText {
    pub const fn english(self) -> &'static str {
        match self {
            PlayedWithText::Title => "Played With Recently",
            PlayedWithText::Subtitle => "Only you see this list",
            PlayedWithText::PeopleMany => "%@ people",
            PlayedWithText::PeopleOne => "1 person",
            PlayedWithText::AddFriend => "Add Friend",
            PlayedWithText::Friend => "Friend",
            PlayedWithText::RequestSent => "Request sent",
            PlayedWithText::Played => "Played %@",
            PlayedWithText::EmptyTitle => "No one here yet",
            PlayedWithText::EmptyNote => {
                "People you share a party or a game with for at least 2 minutes show up here."
            }
            PlayedWithText::LoadingTitle => "Loading…",
            PlayedWithText::FailedTitle => "The list did not load",
            PlayedWithText::FailedNote => "Check your connection and try again.",
            PlayedWithText::Remember => "Remember People I Play With",
            PlayedWithText::RememberDetail => "Adds people after a party or a game together",
            PlayedWithText::RememberOffNote => {
                "New people are not added. The people on the list stay until you clear it."
            }
            PlayedWithText::PrivateNote => {
                "Only you see this list, and the people on it are not told. It keeps up to 50 people for 30 days."
            }
            PlayedWithText::ClearList => "Clear the List",
            PlayedWithText::ClearTitle => "Clear the list?",
            PlayedWithText::ClearNote => {
                "Everyone is removed from your list. The lists of other players do not change."
            }
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            PlayedWithText::Title => "Son Oynadıkların",
            PlayedWithText::Subtitle => "Bu listeyi yalnızca sen görürsün",
            PlayedWithText::PeopleMany => "%@ kişi",
            PlayedWithText::PeopleOne => "1 kişi",
            PlayedWithText::AddFriend => "Arkadaş Ekle",
            PlayedWithText::Friend => "Arkadaş",
            PlayedWithText::RequestSent => "İstek gönderildi",
            PlayedWithText::Played => "%@ oynandı",
            PlayedWithText::EmptyTitle => "Henüz kimse yok",
            PlayedWithText::EmptyNote => {
                "En az 2 dakika parti ya da oyun paylaştığın kişiler burada görünür."
            }
            PlayedWithText::LoadingTitle => "Yükleniyor…",
            PlayedWithText::FailedTitle => "Liste yüklenemedi",
            PlayedWithText::FailedNote => "Bağlantını kontrol edip tekrar dene.",
            PlayedWithText::Remember => "Oynadığım Kişileri Hatırla",
            PlayedWithText::RememberDetail => "Birlikte parti ya da oyundan sonra kişileri ekler",
            PlayedWithText::RememberOffNote => {
                "Yeni kişi eklenmez. Listedekiler, sen temizleyene kadar kalır."
            }
            PlayedWithText::PrivateNote => {
                "Bu listeyi yalnızca sen görürsün, listedekilere haber verilmez. 30 gün boyunca en fazla 50 kişi tutar."
            }
            PlayedWithText::ClearList => "Listeyi Temizle",
            PlayedWithText::ClearTitle => "Liste temizlensin mi?",
            PlayedWithText::ClearNote => {
                "Herkes listenden kaldırılır. Diğer oyuncuların listeleri değişmez."
            }
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            PlayedWithText::Title => "Zuletzt gespielt mit",
            PlayedWithText::Subtitle => "Nur du siehst diese Liste",
            PlayedWithText::PeopleMany => "%@ Personen",
            PlayedWithText::PeopleOne => "1 Person",
            PlayedWithText::AddFriend => "Freund hinzufügen",
            PlayedWithText::Friend => "Freund",
            PlayedWithText::RequestSent => "Anfrage gesendet",
            PlayedWithText::Played => "%@ gespielt",
            PlayedWithText::EmptyTitle => "Noch niemand hier",
            PlayedWithText::EmptyNote => {
                "Wen du mindestens 2 Minuten in einer Party oder einem Spiel getroffen hast, erscheint hier."
            }
            PlayedWithText::LoadingTitle => "Wird geladen…",
            PlayedWithText::FailedTitle => "Die Liste wurde nicht geladen",
            PlayedWithText::FailedNote => "Prüfe deine Verbindung und versuche es erneut.",
            PlayedWithText::Remember => "Mitspieler merken",
            PlayedWithText::RememberDetail => "Fügt Leute nach einer Party oder einem Spiel hinzu",
            PlayedWithText::RememberOffNote => {
                "Neue Leute werden nicht hinzugefügt. Die Leute auf der Liste bleiben, bis du sie leerst."
            }
            PlayedWithText::PrivateNote => {
                "Nur du siehst diese Liste, die Leute darauf erfahren nichts davon. Sie hält bis zu 50 Personen für 30 Tage."
            }
            PlayedWithText::ClearList => "Liste leeren",
            PlayedWithText::ClearTitle => "Liste leeren?",
            PlayedWithText::ClearNote => {
                "Alle werden aus deiner Liste entfernt. Die Listen anderer Spieler ändern sich nicht."
            }
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            PlayedWithText::Title => "Jugadores recientes",
            PlayedWithText::Subtitle => "Solo tú ves esta lista",
            PlayedWithText::PeopleMany => "%@ personas",
            PlayedWithText::PeopleOne => "1 persona",
            PlayedWithText::AddFriend => "Añadir amigo",
            PlayedWithText::Friend => "Amigo",
            PlayedWithText::RequestSent => "Solicitud enviada",
            PlayedWithText::Played => "Jugó a %@",
            PlayedWithText::EmptyTitle => "Aún no hay nadie",
            PlayedWithText::EmptyNote => {
                "Aquí aparecen las personas con quienes compartes una party o un juego durante al menos 2 minutos."
            }
            PlayedWithText::LoadingTitle => "Cargando…",
            PlayedWithText::FailedTitle => "La lista no se cargó",
            PlayedWithText::FailedNote => "Comprueba tu conexión e inténtalo de nuevo.",
            PlayedWithText::Remember => "Recordar con quién juego",
            PlayedWithText::RememberDetail => "Añade personas tras una party o un juego juntos",
            PlayedWithText::RememberOffNote => {
                "No se añaden personas nuevas. Las de la lista se quedan hasta que la vacíes."
            }
            PlayedWithText::PrivateNote => {
                "Solo tú ves esta lista y las personas que aparecen no reciben aviso. Guarda hasta 50 personas durante 30 días."
            }
            PlayedWithText::ClearList => "Vaciar la lista",
            PlayedWithText::ClearTitle => "¿Vaciar la lista?",
            PlayedWithText::ClearNote => {
                "Se quita a todos de tu lista. Las listas de otros jugadores no cambian."
            }
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            PlayedWithText::Title => "Joué récemment avec",
            PlayedWithText::Subtitle => "Toi seul vois cette liste",
            PlayedWithText::PeopleMany => "%@ personnes",
            PlayedWithText::PeopleOne => "1 personne",
            PlayedWithText::AddFriend => "Ajouter un ami",
            PlayedWithText::Friend => "Ami",
            PlayedWithText::RequestSent => "Demande envoyée",
            PlayedWithText::Played => "A joué à %@",
            PlayedWithText::EmptyTitle => "Personne pour l’instant",
            PlayedWithText::EmptyNote => {
                "Les personnes avec qui tu partages une party ou un jeu pendant au moins 2 minutes apparaissent ici."
            }
            PlayedWithText::LoadingTitle => "Chargement…",
            PlayedWithText::FailedTitle => "La liste n’a pas été chargée",
            PlayedWithText::FailedNote => "Vérifie ta connexion et réessaie.",
            PlayedWithText::Remember => "Retenir avec qui je joue",
            PlayedWithText::RememberDetail => "Ajoute des gens après une party ou un jeu ensemble",
            PlayedWithText::RememberOffNote => {
                "Personne de nouveau n’est ajouté. Ceux de la liste restent jusqu’à ce que tu la vides."
            }
            PlayedWithText::PrivateNote => {
                "Toi seul vois cette liste et les personnes qui y figurent ne sont pas prévenues. Elle garde jusqu’à 50 personnes pendant 30 jours."
            }
            PlayedWithText::ClearList => "Vider la liste",
            PlayedWithText::ClearTitle => "Vider la liste\u{a0}?",
            PlayedWithText::ClearNote => {
                "Tout le monde est retiré de ta liste. Les listes des autres joueurs ne changent pas."
            }
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            PlayedWithText::Title => "Jogadores recentes",
            PlayedWithText::Subtitle => "Só você vê esta lista",
            PlayedWithText::PeopleMany => "%@ pessoas",
            PlayedWithText::PeopleOne => "1 pessoa",
            PlayedWithText::AddFriend => "Adicionar amigo",
            PlayedWithText::Friend => "Amigo",
            PlayedWithText::RequestSent => "Pedido enviado",
            PlayedWithText::Played => "Jogou %@",
            PlayedWithText::EmptyTitle => "Ninguém por aqui ainda",
            PlayedWithText::EmptyNote => {
                "Quem divide uma party ou um jogo com você por pelo menos 2 minutos aparece aqui."
            }
            PlayedWithText::LoadingTitle => "Carregando…",
            PlayedWithText::FailedTitle => "A lista não carregou",
            PlayedWithText::FailedNote => "Verifique sua conexão e tente de novo.",
            PlayedWithText::Remember => "Lembrar com quem eu jogo",
            PlayedWithText::RememberDetail => {
                "Adiciona pessoas depois de uma party ou um jogo juntos"
            }
            PlayedWithText::RememberOffNote => {
                "Ninguém novo é adicionado. Quem está na lista fica até você limpá-la."
            }
            PlayedWithText::PrivateNote => {
                "Só você vê esta lista e as pessoas nela não são avisadas. Ela guarda até 50 pessoas por 30 dias."
            }
            PlayedWithText::ClearList => "Limpar a lista",
            PlayedWithText::ClearTitle => "Limpar a lista?",
            PlayedWithText::ClearNote => {
                "Todos saem da sua lista. As listas de outros jogadores não mudam."
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
        for text in [PlayedWithText::PeopleMany, PlayedWithText::Played] {
            let count = text.english().matches("%@").count();
            for language in [
                Language::Turkish,
                Language::German,
                Language::Spanish,
                Language::French,
                Language::Portuguese,
            ] {
                assert_eq!(count, text.text(language).matches("%@").count(), "{text:?}");
            }
        }
    }
}
