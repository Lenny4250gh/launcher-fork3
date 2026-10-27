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
                "Legt den Launcher in deiner Steam-Bibliothek ab, damit Big Picture und das Steam Deck ihn starten können. Steam muss geschlossen sein."
            }
            SteamText::AddGames => "Meine Spiele zu Steam hinzufügen",
            SteamText::AddGamesDetail => {
                "Legt jedes Spiel deiner Bibliothek mit eigenem Artwork in Steam ab. Steam muss geschlossen sein."
            }
            SteamText::Added => "Zu Steam hinzugefügt. Öffne Steam erneut, dann ist er dort.",
            SteamText::Updated => {
                "Steam-Eintrag aktualisiert. Öffne Steam erneut, um ihn zu sehen."
            }
            SteamText::GameAdded => {
                "1 Spiel wurde zu Steam hinzugefügt. Öffne Steam erneut, dann ist es dort."
            }
            SteamText::GamesAdded => {
                "%@ Spiele wurden zu Steam hinzugefügt. Öffne Steam erneut, dann sind sie dort."
            }
            SteamText::Running => "Steam ist noch offen. Beende es und versuche es noch einmal.",
            SteamText::NoSteam => {
                "Auf diesem PC wurde keine Steam-Installation gefunden. Richte Steam ein, melde dich einmal an und komm zurück."
            }
            SteamText::NoUsers => {
                "Auf diesem PC ist in Steam kein Konto angemeldet. Melde dich einmal bei Steam an und komm zurück."
            }
            SteamText::Unreadable => {
                "Die Verknüpfungsliste von Steam konnte nicht gelesen werden. Es wurde nichts geändert."
            }
            SteamText::Failed => {
                "Das Schreiben in deinen Steam-Ordner ist fehlgeschlagen. Prüfe, dass der Ordner nicht schreibgeschützt ist, und versuche es noch einmal."
            }
            SteamText::NoGames => "Noch keine Spiele in der Bibliothek.",
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            SteamText::Add => "Añadir a Steam",
            SteamText::AddDetail => {
                "Pone el launcher en tu biblioteca de Steam para que Big Picture y la Steam Deck puedan iniciarlo. Steam debe estar cerrado."
            }
            SteamText::AddGames => "Añadir mis juegos a Steam",
            SteamText::AddGamesDetail => {
                "Pone en Steam cada juego de tu biblioteca, cada uno con su propia imagen. Steam debe estar cerrado."
            }
            SteamText::Added => "Añadido a Steam. Abre Steam de nuevo y estará ahí.",
            SteamText::Updated => "Entrada de Steam actualizada. Abre Steam de nuevo para verla.",
            SteamText::GameAdded => "Se añadió 1 juego a Steam. Abre Steam de nuevo y estará ahí.",
            SteamText::GamesAdded => {
                "Se añadieron %@ juegos a Steam. Abre Steam de nuevo y estarán ahí."
            }
            SteamText::Running => "Steam sigue abierto. Ciérralo e inténtalo de nuevo.",
            SteamText::NoSteam => {
                "No se encontró ninguna instalación de Steam en este PC. Configura Steam, inicia sesión una vez y vuelve."
            }
            SteamText::NoUsers => {
                "Steam no tiene ninguna cuenta con sesión iniciada en este PC. Inicia sesión en Steam una vez y vuelve."
            }
            SteamText::Unreadable => {
                "No se pudo leer la lista de accesos directos de Steam. No se cambió nada."
            }
            SteamText::Failed => {
                "No se pudo escribir en tu carpeta de Steam. Comprueba que la carpeta no sea de solo lectura e inténtalo de nuevo."
            }
            SteamText::NoGames => "Aún no hay juegos en la biblioteca.",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            SteamText::Add => "Ajouter à Steam",
            SteamText::AddDetail => {
                "Place le launcher dans ta bibliothèque Steam pour que Big Picture et la Steam Deck puissent le lancer. Steam doit être fermé."
            }
            SteamText::AddGames => "Ajouter mes jeux à Steam",
            SteamText::AddGamesDetail => {
                "Place chaque jeu de ta bibliothèque dans Steam, chacun avec son propre visuel. Steam doit être fermé."
            }
            SteamText::Added => "Ajouté à Steam. Rouvre Steam et il sera là.",
            SteamText::Updated => "Entrée Steam mise à jour. Rouvre Steam pour la voir.",
            SteamText::GameAdded => "1 jeu a été ajouté à Steam. Rouvre Steam et il sera là.",
            SteamText::GamesAdded => {
                "%@ jeux ont été ajoutés à Steam. Rouvre Steam et ils seront là."
            }
            SteamText::Running => "Steam est encore ouvert. Ferme-le et réessaie.",
            SteamText::NoSteam => {
                "Aucune installation de Steam n’a été trouvée sur ce PC. Installe Steam, connecte-toi une fois, puis reviens."
            }
            SteamText::NoUsers => {
                "Aucun compte n’est connecté à Steam sur ce PC. Connecte-toi une fois à Steam, puis reviens."
            }
            SteamText::Unreadable => {
                "La liste des raccourcis de Steam n’a pas pu être lue. Rien n’a été modifié."
            }
            SteamText::Failed => {
                "L’écriture dans ton dossier Steam a échoué. Vérifie que le dossier n’est pas en lecture seule, puis réessaie."
            }
            SteamText::NoGames => "Pas encore de jeux dans la bibliothèque.",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            SteamText::Add => "Adicionar ao Steam",
            SteamText::AddDetail => {
                "Coloca o launcher na sua biblioteca do Steam para que o Big Picture e o Steam Deck possam iniciá-lo. O Steam precisa estar fechado."
            }
            SteamText::AddGames => "Adicionar meus jogos ao Steam",
            SteamText::AddGamesDetail => {
                "Coloca no Steam cada jogo da sua biblioteca, cada um com sua própria arte. O Steam precisa estar fechado."
            }
            SteamText::Added => "Adicionado ao Steam. Abra o Steam de novo e ele estará lá.",
            SteamText::Updated => "Entrada do Steam atualizada. Abra o Steam de novo para vê-la.",
            SteamText::GameAdded => {
                "1 jogo foi adicionado ao Steam. Abra o Steam de novo e ele estará lá."
            }
            SteamText::GamesAdded => {
                "%@ jogos foram adicionados ao Steam. Abra o Steam de novo e eles estarão lá."
            }
            SteamText::Running => "O Steam ainda está aberto. Feche-o e tente de novo.",
            SteamText::NoSteam => {
                "Nenhuma instalação do Steam foi encontrada neste PC. Configure o Steam, entre uma vez e volte."
            }
            SteamText::NoUsers => {
                "O Steam não tem nenhuma conta conectada neste PC. Entre no Steam uma vez e volte."
            }
            SteamText::Unreadable => {
                "Não foi possível ler a lista de atalhos do Steam. Nada foi alterado."
            }
            SteamText::Failed => {
                "Falha ao gravar na sua pasta do Steam. Verifique se a pasta não é somente leitura e tente de novo."
            }
            SteamText::NoGames => "Ainda não há jogos na biblioteca.",
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
