//! Texts of Console Mode, Start with Windows and Open in Full Screen: the settings rows and one
//! toast (docs/DESIGN.md 9.10). Every row says what happens when it is on.

use crate::ui::i18n::Language;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ConsoleText {
    /// Row: the launcher becomes the PC's home.
    Console,
    ConsoleDetail,
    /// The detail when this copy cannot open itself at sign-in.
    ConsoleDetailManual,
    /// Row on Windows.
    AutostartWindows,
    /// Row on Linux and macOS.
    AutostartLogin,
    AutostartDetail,
    /// The detail when the entry starts an AppImage file.
    AutostartDetailImage,
    /// Note in place of the row when this copy has nothing to point the entry at.
    AutostartNone,
    Fullscreen,
    FullscreenDetail,
    /// Row: closes the launcher.
    Desktop,
    DesktopDetail,
    /// Toast: the entry could not be written or removed.
    Failed,
}

impl ConsoleText {
    pub const fn english(self) -> &'static str {
        match self {
            ConsoleText::Console => "Console Mode",
            ConsoleText::ConsoleDetail => {
                "Opens AnyPlay when you sign in, fills the screen and puts the controller first. Return to Desktop closes it."
            }
            ConsoleText::ConsoleDetailManual => {
                "Fills the screen and puts the controller first. This copy cannot open itself at sign-in, so start AnyPlay yourself. Return to Desktop closes it."
            }
            ConsoleText::AutostartWindows => "Start with Windows",
            ConsoleText::AutostartLogin => "Start at Login",
            ConsoleText::AutostartDetail => "Opens AnyPlay by itself when you sign in.",
            ConsoleText::AutostartDetailImage => {
                "Opens AnyPlay by itself when you sign in. If you move the AppImage file, start it once from its new place."
            }
            ConsoleText::AutostartNone => {
                "This copy cannot start at login because it runs from a temporary place. Start the AppImage from its own file to turn this on."
            }
            ConsoleText::Fullscreen => "Open in Full Screen",
            ConsoleText::FullscreenDetail => {
                "AnyPlay fills the whole screen when it starts. F11 switches while it runs."
            }
            ConsoleText::Desktop => "Return to Desktop",
            ConsoleText::DesktopDetail => "Closes AnyPlay and shows your desktop.",
            ConsoleText::Failed => {
                "The start at login could not be changed. Make sure the folder is not read-only and try once more."
            }
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            ConsoleText::Console => "Konsol Modu",
            ConsoleText::ConsoleDetail => {
                "Oturum açtığında AnyPlay'i açar, ekranı doldurur ve kontrolcüyü öne alır. Masaüstüne Dön onu kapatır."
            }
            ConsoleText::ConsoleDetailManual => {
                "Ekranı doldurur ve kontrolcüyü öne alır. Bu kopya oturum açılırken kendini başlatamaz; AnyPlay'i kendin aç. Masaüstüne Dön onu kapatır."
            }
            ConsoleText::AutostartWindows => "Windows ile Başlat",
            ConsoleText::AutostartLogin => "Oturum Açılınca Başlat",
            ConsoleText::AutostartDetail => "Oturum açtığında AnyPlay'i kendiliğinden açar.",
            ConsoleText::AutostartDetailImage => {
                "Oturum açtığında AnyPlay'i kendiliğinden açar. AppImage dosyasını taşırsan onu yeni yerinden bir kez aç."
            }
            ConsoleText::AutostartNone => {
                "Bu kopya geçici bir yerden çalıştığı için oturum açılınca başlayamaz. Açmak için AppImage'ı kendi dosyasından başlat."
            }
            ConsoleText::Fullscreen => "Tam Ekranda Aç",
            ConsoleText::FullscreenDetail => {
                "AnyPlay başlarken tüm ekranı kaplar. Çalışırken F11 ile değişir."
            }
            ConsoleText::Desktop => "Masaüstüne Dön",
            ConsoleText::DesktopDetail => "AnyPlay'i kapatır ve masaüstünü gösterir.",
            ConsoleText::Failed => {
                "Oturum açılışta başlatma değiştirilemedi. Klasörün salt okunur olmadığından emin ol ve bir kez daha dene."
            }
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            ConsoleText::Console => "Konsolenmodus",
            ConsoleText::ConsoleDetail => {
                "Öffnet AnyPlay bei der Anmeldung, füllt den Bildschirm und stellt den Controller an die erste Stelle. Mit Zurück zum Desktop beendest du es."
            }
            ConsoleText::ConsoleDetailManual => {
                "Füllt den Bildschirm und stellt den Controller an die erste Stelle. Diese Kopie kann sich bei der Anmeldung nicht selbst öffnen, starte AnyPlay also selbst. Mit Zurück zum Desktop beendest du es."
            }
            ConsoleText::AutostartWindows => "Mit Windows starten",
            ConsoleText::AutostartLogin => "Bei der Anmeldung starten",
            ConsoleText::AutostartDetail => "Öffnet AnyPlay von selbst, wenn du dich anmeldest.",
            ConsoleText::AutostartDetailImage => {
                "Öffnet AnyPlay von selbst, wenn du dich anmeldest. Wenn du die AppImage-Datei verschiebst, starte sie einmal von ihrem neuen Ort."
            }
            ConsoleText::AutostartNone => {
                "Diese Kopie kann sich bei der Anmeldung nicht starten, weil sie von einem temporären Ort läuft. Starte das AppImage aus seiner eigenen Datei, um das einzuschalten."
            }
            ConsoleText::Fullscreen => "Im Vollbild öffnen",
            ConsoleText::FullscreenDetail => {
                "AnyPlay füllt beim Start den ganzen Bildschirm. Mit F11 wechselst du, während es läuft."
            }
            ConsoleText::Desktop => "Zurück zum Desktop",
            ConsoleText::DesktopDetail => "Beendet AnyPlay und zeigt deinen Desktop.",
            ConsoleText::Failed => {
                "Der Start bei der Anmeldung konnte nicht geändert werden. Prüfe, dass der Ordner nicht schreibgeschützt ist, und versuche es noch einmal."
            }
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            ConsoleText::Console => "Modo consola",
            ConsoleText::ConsoleDetail => {
                "Abre AnyPlay al iniciar sesión, llena la pantalla y pone el mando primero. Volver al escritorio lo cierra."
            }
            ConsoleText::ConsoleDetailManual => {
                "Llena la pantalla y pone el mando primero. Esta copia no puede abrirse sola al iniciar sesión, así que abre AnyPlay tú mismo. Volver al escritorio lo cierra."
            }
            ConsoleText::AutostartWindows => "Iniciar con Windows",
            ConsoleText::AutostartLogin => "Iniciar al abrir sesión",
            ConsoleText::AutostartDetail => "Abre AnyPlay solo cuando inicias sesión.",
            ConsoleText::AutostartDetailImage => {
                "Abre AnyPlay solo cuando inicias sesión. Si mueves el archivo AppImage, ábrelo una vez desde su nuevo lugar."
            }
            ConsoleText::AutostartNone => {
                "Esta copia no puede iniciarse al abrir sesión porque se ejecuta desde un lugar temporal. Abre el AppImage desde su propio archivo para activarlo."
            }
            ConsoleText::Fullscreen => "Abrir a pantalla completa",
            ConsoleText::FullscreenDetail => {
                "AnyPlay llena toda la pantalla al iniciar. Con F11 cambias mientras se ejecuta."
            }
            ConsoleText::Desktop => "Volver al escritorio",
            ConsoleText::DesktopDetail => "Cierra AnyPlay y muestra tu escritorio.",
            ConsoleText::Failed => {
                "No se pudo cambiar el inicio al abrir sesión. Comprueba que la carpeta no sea de solo lectura e inténtalo de nuevo."
            }
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            ConsoleText::Console => "Mode console",
            ConsoleText::ConsoleDetail => {
                "Ouvre AnyPlay à l’ouverture de session, remplit l’écran et met la manette en premier. Retour au bureau le ferme."
            }
            ConsoleText::ConsoleDetailManual => {
                "Remplit l’écran et met la manette en premier. Cette copie ne peut pas s’ouvrir seule à l’ouverture de session\u{a0}: lance AnyPlay toi-même. Retour au bureau le ferme."
            }
            ConsoleText::AutostartWindows => "Démarrer avec Windows",
            ConsoleText::AutostartLogin => "Démarrer à la connexion",
            ConsoleText::AutostartDetail => "Ouvre AnyPlay tout seul quand tu ouvres ta session.",
            ConsoleText::AutostartDetailImage => {
                "Ouvre AnyPlay tout seul quand tu ouvres ta session. Si tu déplaces le fichier AppImage, lance-le une fois depuis son nouvel emplacement."
            }
            ConsoleText::AutostartNone => {
                "Cette copie ne peut pas démarrer à la connexion, car elle s’exécute depuis un emplacement temporaire. Lance l’AppImage depuis son propre fichier pour l’activer."
            }
            ConsoleText::Fullscreen => "Ouvrir en plein écran",
            ConsoleText::FullscreenDetail => {
                "AnyPlay remplit tout l’écran au démarrage. F11 bascule pendant l’exécution."
            }
            ConsoleText::Desktop => "Retour au bureau",
            ConsoleText::DesktopDetail => "Ferme AnyPlay et affiche ton bureau.",
            ConsoleText::Failed => {
                "Impossible de modifier le démarrage à la connexion. Vérifie que le dossier n’est pas en lecture seule et réessaie."
            }
            _ => return None,
        })
    }

    /// The Brazilian Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            ConsoleText::Console => "Modo console",
            ConsoleText::ConsoleDetail => {
                "Abre o AnyPlay quando você entra, ocupa a tela e coloca o controle em primeiro lugar. Voltar para a área de trabalho fecha o AnyPlay."
            }
            ConsoleText::ConsoleDetailManual => {
                "Ocupa a tela e coloca o controle em primeiro lugar. Esta cópia não consegue abrir sozinha ao entrar, então abra o AnyPlay você mesmo. Voltar para a área de trabalho fecha o AnyPlay."
            }
            ConsoleText::AutostartWindows => "Iniciar com o Windows",
            ConsoleText::AutostartLogin => "Iniciar ao entrar",
            ConsoleText::AutostartDetail => "Abre o AnyPlay sozinho quando você entra.",
            ConsoleText::AutostartDetailImage => {
                "Abre o AnyPlay sozinho quando você entra. Se você mover o arquivo AppImage, abra-o uma vez do novo local."
            }
            ConsoleText::AutostartNone => {
                "Esta cópia não pode iniciar ao entrar porque roda de um local temporário. Abra o AppImage pelo próprio arquivo para ativar."
            }
            ConsoleText::Fullscreen => "Abrir em tela cheia",
            ConsoleText::FullscreenDetail => {
                "O AnyPlay ocupa a tela toda ao iniciar. F11 alterna enquanto ele roda."
            }
            ConsoleText::Desktop => "Voltar para a área de trabalho",
            ConsoleText::DesktopDetail => "Fecha o AnyPlay e mostra a área de trabalho.",
            ConsoleText::Failed => {
                "Não foi possível alterar o início ao entrar. Verifique se a pasta não é somente leitura e tente de novo."
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

    pub const ALL: [ConsoleText; 13] = [
        ConsoleText::Console,
        ConsoleText::ConsoleDetail,
        ConsoleText::ConsoleDetailManual,
        ConsoleText::AutostartWindows,
        ConsoleText::AutostartLogin,
        ConsoleText::AutostartDetail,
        ConsoleText::AutostartDetailImage,
        ConsoleText::AutostartNone,
        ConsoleText::Fullscreen,
        ConsoleText::FullscreenDetail,
        ConsoleText::Desktop,
        ConsoleText::DesktopDetail,
        ConsoleText::Failed,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_text_exists_in_every_language() {
        for text in ConsoleText::ALL {
            for language in Language::ALL {
                assert!(!text.text(language).is_empty(), "{text:?} {language:?}");
            }
        }
    }
}
