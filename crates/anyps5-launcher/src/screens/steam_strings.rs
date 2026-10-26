//! Texts of Add to Steam: the two settings rows and the toasts. English and Turkish like the rest.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SteamText {
    /// Settings row: puts the launcher into the Steam library.
    Add,
    AddDetail,
    /// Settings row: one Steam entry for each game.
    AddGames,
    AddGamesDetail,
    /// Toast: the launcher's entry was written.
    Added,
    /// Toast: the launcher's entry was there and is up to date now.
    Updated,
    /// Toast: one game's entry was written.
    GameAdded,
    /// Toast: `%@` = how many games.
    GamesAdded,
    /// Toast: Steam runs, so nothing was written.
    Running,
    NoSteam,
    /// Toast: Steam has no account folder yet.
    NoUsers,
    /// Toast: the shortcut list is in a form the launcher does not know and was left alone.
    Unreadable,
    Failed,
    NoGames,
}

impl SteamText {
    pub const fn english(self) -> &'static str {
        match self {
            SteamText::Add => "Add to Steam",
            SteamText::AddDetail => {
                "Puts the launcher in your Steam library, so Big Picture and the Steam Deck can start it. Steam has to be closed."
            }
            SteamText::AddGames => "Add My Games to Steam",
            SteamText::AddGamesDetail => {
                "Puts every game from your library in Steam, each with its own artwork. Steam has to be closed."
            }
            SteamText::Added => "Added to Steam. Open Steam again and it is there.",
            SteamText::Updated => "Steam entry updated. Open Steam again to see it.",
            SteamText::GameAdded => "1 game was added to Steam. Open Steam again and it is there.",
            SteamText::GamesAdded => {
                "%@ games were added to Steam. Open Steam again and they are there."
            }
            SteamText::Running => "Steam is still open. Quit it and try once more.",
            SteamText::NoSteam => {
                "No Steam installation was found on this PC. Set Steam up, log in once, and come back."
            }
            SteamText::NoUsers => {
                "Steam has no logged-in account on this PC. Log in to Steam once and come back."
            }
            SteamText::Unreadable => {
                "Steam's shortcut list could not be read. Nothing was changed."
            }
            SteamText::Failed => {
                "Writing to your Steam folder failed. Make sure the folder is not read-only and try once more."
            }
            SteamText::NoGames => "No games in the library yet.",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            SteamText::Add => "Steam'e Ekle",
            SteamText::AddDetail => {
                "Başlatıcıyı Steam kitaplığına koyar; böylece Big Picture ve Steam Deck onu açabilir. Steam kapalı olmalı."
            }
            SteamText::AddGames => "Oyunlarımı Steam'e Ekle",
            SteamText::AddGamesDetail => {
                "Kitaplığındaki her oyunu kendi görseliyle Steam'e koyar. Steam kapalı olmalı."
            }
            SteamText::Added => "Steam'e eklendi. Steam'i yeniden aç, orada olacak.",
            SteamText::Updated => "Steam kaydı güncellendi. Görmek için Steam'i yeniden aç.",
            SteamText::GameAdded => "1 oyun Steam'e eklendi. Steam'i yeniden aç, orada olacak.",
            SteamText::GamesAdded => {
                "%@ oyun Steam'e eklendi. Steam'i yeniden aç, orada olacaklar."
            }
            SteamText::Running => "Steam hâlâ açık. Kapat ve bir kez daha dene.",
            SteamText::NoSteam => {
                "Bu bilgisayarda Steam kurulumu bulunamadı. Steam'i kur, bir kez giriş yap ve geri gel."
            }
            SteamText::NoUsers => {
                "Steam'de bu bilgisayarda giriş yapılmış hesap yok. Steam'e bir kez giriş yap ve geri gel."
            }
            SteamText::Unreadable => {
                "Steam'in kısayol listesi okunamadı. Hiçbir şey değiştirilmedi."
            }
            SteamText::Failed => {
                "Steam klasörüne yazılamadı. Klasörün salt okunur olmadığından emin ol ve bir kez daha dene."
            }
            SteamText::NoGames => "Kitaplıkta henüz oyun yok.",
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            SteamText::Add => "Zu Steam hinzufügen",
            SteamText::AddDetail => {
                "Fügt den Launcher deiner Steam-Bibliothek für Big Picture und das Steam Deck hinzu. Schließe Steam zuerst."
            }
            SteamText::AddGames => "Meine Spiele zu Steam hinzufügen",
            SteamText::AddGamesDetail => {
                "Fügt jedes Spiel deiner Bibliothek mit eigenem Artwork zu Steam hinzu. Schließe Steam zuerst."
            }
            SteamText::Added => "Zu Steam hinzugefügt. Starte Steam neu, um es zu sehen.",
            SteamText::Updated => {
                "In Steam aktualisiert. Starte Steam neu, um die Änderung zu sehen."
            }
            SteamText::GameAdded => "1 Spiel ist in Steam. Starte Steam neu, um es zu sehen.",
            SteamText::GamesAdded => "%@ Spiele sind in Steam. Starte Steam neu, um sie zu sehen.",
            SteamText::Running => "Steam läuft. Schließe Steam und versuche es erneut.",
            SteamText::NoSteam => {
                "Steam wurde auf diesem PC nicht gefunden. Installiere Steam und melde dich einmal an, dann versuche es erneut."
            }
            SteamText::NoUsers => {
                "Steam hat auf diesem PC noch kein Konto. Melde dich einmal bei Steam an, dann versuche es erneut."
            }
            SteamText::Unreadable => {
                "Die Verknüpfungsliste von Steam konnte nicht gelesen werden, deshalb wurde nichts geändert."
            }
            SteamText::Failed => {
                "Im Steam-Ordner konnte nicht gespeichert werden. Prüfe, dass er nicht schreibgeschützt ist, dann versuche es erneut."
            }
            SteamText::NoGames => "Noch keine Spiele in der Bibliothek.",
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

    pub const ALL: [SteamText; 14] = [
        SteamText::Add,
        SteamText::AddDetail,
        SteamText::AddGames,
        SteamText::AddGamesDetail,
        SteamText::Added,
        SteamText::Updated,
        SteamText::GameAdded,
        SteamText::GamesAdded,
        SteamText::Running,
        SteamText::NoSteam,
        SteamText::NoUsers,
        SteamText::Unreadable,
        SteamText::Failed,
        SteamText::NoGames,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_text_exists_in_both_languages_with_the_same_placeholders() {
        for text in SteamText::ALL {
            for language in Language::ALL {
                assert!(!text.text(language).is_empty(), "{text:?} {language:?}");
            }
            assert_eq!(
                text.english().matches("%@").count(),
                text.turkish().matches("%@").count(),
                "{text:?}"
            );
        }
    }
}
