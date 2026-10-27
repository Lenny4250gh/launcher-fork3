//! Texts of the trophy comparison (docs/DESIGN.md 17, "Trophy compare"). English and Turkish like
//! the rest. Titles are in title case, notes and details in sentence case. Grades, "Hidden Trophy"
//! and "Show" come from the shared strings; this file holds only what the screen adds.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CompareText {
    /// The small caption over the game's name.
    Overline,
    /// Under the game's name: `%@` = the other player's handle.
    With,
    /// The player's own column.
    You,
    /// `%@` = the level.
    Level,
    /// `%@` = the points.
    Points,
    /// `%@` = the points of the trophies in this game.
    GamePoints,
    /// A trophy the player has not earned.
    NotYet,
    /// `%@` = the trophy number, for a game the player does not have.
    NumberedTrophy,
    /// Under the header when the game is not in the player's library.
    NotOwnedNote,
    LoadingNote,
    OfflineTitle,
    OfflineNote,
    FailedTitle,
    FailedNote,
    HiddenTitle,
    /// `%@` = the handle.
    HiddenNote,
    EmptyTitle,
    EmptyNote,
    /// A column with no trophies in the game, in its accessible name.
    NoneInGame,
    /// The accessible name of a column: `%@` handle, `%@` level, `%@` points in all, `%@` the
    /// game's trophies by grade, `%@` points in this game.
    SideName,
    /// The accessible name of a row: `%@` trophy, `%@` your state, `%@` handle, `%@` their state.
    RowName,
    /// `%@` = the date.
    EarnedOn,
    /// The accessible name of the screen: `%@` = the game, `%@` = the handle.
    ScreenName,
}

impl CompareText {
    pub const fn english(self) -> &'static str {
        match self {
            CompareText::Overline => "Compare Trophies",
            CompareText::With => "With %@",
            CompareText::You => "You",
            CompareText::Level => "Level %@",
            CompareText::Points => "%@ points",
            CompareText::GamePoints => "%@ points in this game",
            CompareText::NotYet => "Not yet",
            CompareText::NoneInGame => "no trophies",
            CompareText::NumberedTrophy => "Trophy %@",
            CompareText::NotOwnedNote => {
                "This game is not in your library, so its trophies show by number."
            }
            CompareText::LoadingNote => "Loading the comparison…",
            CompareText::OfflineTitle => "You are offline",
            CompareText::OfflineNote => "Trophies are compared when you are back online.",
            CompareText::FailedTitle => "The comparison did not load",
            CompareText::FailedNote => "Check your connection and try again.",
            CompareText::HiddenTitle => "Their trophies are hidden",
            CompareText::HiddenNote => "%@ does not share trophies with you.",
            CompareText::EmptyTitle => "Nothing to compare yet",
            CompareText::EmptyNote => "Neither of you has earned a trophy in this game.",
            CompareText::SideName => "%@: level %@, %@ points, %@, %@ points in this game",
            CompareText::RowName => "%@. %@: %@. %@: %@",
            CompareText::EarnedOn => "Earned %@",
            CompareText::ScreenName => "Trophies of %@ compared with %@",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            CompareText::Overline => "Kupaları Karşılaştır",
            CompareText::With => "%@ ile",
            CompareText::You => "Sen",
            CompareText::Level => "Seviye %@",
            CompareText::Points => "%@ puan",
            CompareText::GamePoints => "Bu oyunda %@ puan",
            CompareText::NotYet => "Henüz yok",
            CompareText::NoneInGame => "kupa yok",
            CompareText::NumberedTrophy => "%@. kupa",
            CompareText::NotOwnedNote => {
                "Bu oyun kitaplığında yok, bu yüzden kupalar numarayla görünür."
            }
            CompareText::LoadingNote => "Karşılaştırma yükleniyor…",
            CompareText::OfflineTitle => "Çevrimdışısın",
            CompareText::OfflineNote => "Kupalar tekrar çevrimiçi olunca karşılaştırılır.",
            CompareText::FailedTitle => "Karşılaştırma yüklenemedi",
            CompareText::FailedNote => "Bağlantını kontrol edip tekrar dene.",
            CompareText::HiddenTitle => "Onun kupaları gizli",
            CompareText::HiddenNote => "%@ kupalarını seninle paylaşmıyor.",
            CompareText::EmptyTitle => "Henüz karşılaştıracak bir şey yok",
            CompareText::EmptyNote => "İkinizin de bu oyunda kazandığı bir kupa yok.",
            CompareText::SideName => "%@: seviye %@, %@ puan, %@, bu oyunda %@ puan",
            CompareText::RowName => "%@. %@: %@. %@: %@",
            CompareText::EarnedOn => "%@ tarihinde kazanıldı",
            CompareText::ScreenName => "%@ kupaları, %@ ile karşılaştırma",
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            CompareText::Overline => "Trophäen vergleichen",
            CompareText::With => "Mit %@",
            CompareText::You => "Du",
            CompareText::Level => "Level %@",
            CompareText::Points => "%@ Punkte",
            CompareText::GamePoints => "%@ Punkte in diesem Spiel",
            CompareText::NotYet => "Noch nicht",
            CompareText::NoneInGame => "keine Trophäen",
            CompareText::NumberedTrophy => "Trophäe %@",
            CompareText::NotOwnedNote => {
                "Dieses Spiel ist nicht in deiner Bibliothek, deshalb erscheinen die Trophäen mit Nummer."
            }
            CompareText::LoadingNote => "Lade den Vergleich…",
            CompareText::OfflineTitle => "Du bist offline",
            CompareText::OfflineNote => "Trophäen werden verglichen, sobald du wieder online bist.",
            CompareText::FailedTitle => "Der Vergleich wurde nicht geladen",
            CompareText::FailedNote => "Prüfe deine Verbindung und versuche es erneut.",
            CompareText::HiddenTitle => "Die Trophäen sind verborgen",
            CompareText::HiddenNote => "%@ teilt keine Trophäen mit dir.",
            CompareText::EmptyTitle => "Noch nichts zu vergleichen",
            CompareText::EmptyNote => "Keiner von euch hat in diesem Spiel eine Trophäe erhalten.",
            CompareText::SideName => "%@: Level %@, %@ Punkte, %@, %@ Punkte in diesem Spiel",
            CompareText::RowName => "%@. %@: %@. %@: %@",
            CompareText::EarnedOn => "Erhalten am %@",
            CompareText::ScreenName => "Trophäen von %@ im Vergleich mit %@",
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            CompareText::Overline => "Comparar trofeos",
            CompareText::With => "Con %@",
            CompareText::You => "Tú",
            CompareText::Level => "Nivel %@",
            CompareText::Points => "%@ puntos",
            CompareText::GamePoints => "%@ puntos en este juego",
            CompareText::NotYet => "Aún no",
            CompareText::NoneInGame => "sin trofeos",
            CompareText::NumberedTrophy => "Trofeo %@",
            CompareText::NotOwnedNote => {
                "Este juego no está en tu biblioteca, así que sus trofeos se muestran por número."
            }
            CompareText::LoadingNote => "Cargando la comparación…",
            CompareText::OfflineTitle => "Estás sin conexión",
            CompareText::OfflineNote => "Los trofeos se comparan cuando vuelves a estar en línea.",
            CompareText::FailedTitle => "No se cargó la comparación",
            CompareText::FailedNote => "Revisa tu conexión e inténtalo de nuevo.",
            CompareText::HiddenTitle => "Sus trofeos están ocultos",
            CompareText::HiddenNote => "%@ no comparte sus trofeos contigo.",
            CompareText::EmptyTitle => "Aún no hay nada que comparar",
            CompareText::EmptyNote => "Ninguno de los dos ha conseguido un trofeo en este juego.",
            CompareText::SideName => "%@: nivel %@, %@ puntos, %@, %@ puntos en este juego",
            CompareText::RowName => "%@. %@: %@. %@: %@",
            CompareText::EarnedOn => "Conseguido el %@",
            CompareText::ScreenName => "Trofeos de %@ comparados con %@",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            CompareText::Overline => "Comparer les trophées",
            CompareText::With => "Avec %@",
            CompareText::You => "Toi",
            CompareText::Level => "Niveau %@",
            CompareText::Points => "%@ points",
            CompareText::GamePoints => "%@ points dans ce jeu",
            CompareText::NotYet => "Pas encore",
            CompareText::NoneInGame => "aucun trophée",
            CompareText::NumberedTrophy => "Trophée %@",
            CompareText::NotOwnedNote => {
                "Ce jeu n’est pas dans ta bibliothèque, ses trophées s’affichent donc par numéro."
            }
            CompareText::LoadingNote => "Chargement de la comparaison…",
            CompareText::OfflineTitle => "Tu es hors ligne",
            CompareText::OfflineNote => {
                "Les trophées sont comparés quand tu es de nouveau en ligne."
            }
            CompareText::FailedTitle => "La comparaison ne s’est pas chargée",
            CompareText::FailedNote => "Vérifie ta connexion et réessaie.",
            CompareText::HiddenTitle => "Ses trophées sont masqués",
            CompareText::HiddenNote => "%@ ne partage pas ses trophées avec toi.",
            CompareText::EmptyTitle => "Rien à comparer pour l’instant",
            CompareText::EmptyNote => "Aucun de vous n’a encore obtenu de trophée dans ce jeu.",
            CompareText::SideName => "%@\u{a0}: niveau %@, %@ points, %@, %@ points dans ce jeu",
            CompareText::RowName => "%@. %@\u{a0}: %@. %@\u{a0}: %@",
            CompareText::EarnedOn => "Obtenu le %@",
            CompareText::ScreenName => "Trophées de %@ comparés avec %@",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            CompareText::Overline => "Comparar troféus",
            CompareText::With => "Com %@",
            CompareText::You => "Você",
            CompareText::Level => "Nível %@",
            CompareText::Points => "%@ pontos",
            CompareText::GamePoints => "%@ pontos neste jogo",
            CompareText::NotYet => "Ainda não",
            CompareText::NoneInGame => "sem troféus",
            CompareText::NumberedTrophy => "Troféu %@",
            CompareText::NotOwnedNote => {
                "Este jogo não está na sua biblioteca, então os troféus aparecem por número."
            }
            CompareText::LoadingNote => "Carregando a comparação…",
            CompareText::OfflineTitle => "Você está offline",
            CompareText::OfflineNote => {
                "Os troféus são comparados quando você voltar a ficar online."
            }
            CompareText::FailedTitle => "A comparação não carregou",
            CompareText::FailedNote => "Verifique sua conexão e tente de novo.",
            CompareText::HiddenTitle => "Os troféus estão ocultos",
            CompareText::HiddenNote => "%@ não compartilha troféus com você.",
            CompareText::EmptyTitle => "Ainda não há nada para comparar",
            CompareText::EmptyNote => "Nenhum dos dois conquistou um troféu neste jogo.",
            CompareText::SideName => "%@: nível %@, %@ pontos, %@, %@ pontos neste jogo",
            CompareText::RowName => "%@. %@: %@. %@: %@",
            CompareText::EarnedOn => "Conquistado em %@",
            CompareText::ScreenName => "Troféus de %@ comparados com %@",
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
            CompareText::With,
            CompareText::Level,
            CompareText::Points,
            CompareText::GamePoints,
            CompareText::NumberedTrophy,
            CompareText::HiddenNote,
            CompareText::SideName,
            CompareText::RowName,
            CompareText::EarnedOn,
            CompareText::ScreenName,
            CompareText::NotOwnedNote,
        ] {
            assert_eq!(
                text.english().matches("%@").count(),
                text.turkish().matches("%@").count(),
                "{text:?}"
            );
        }
    }

    #[test]
    fn no_text_names_the_platform() {
        for text in [
            CompareText::Overline,
            CompareText::NotOwnedNote,
            CompareText::HiddenTitle,
            CompareText::EmptyNote,
        ] {
            for words in [text.english(), text.turkish()] {
                assert!(!words.contains("PSN") && !words.contains("PlayStation"));
            }
        }
    }
}
