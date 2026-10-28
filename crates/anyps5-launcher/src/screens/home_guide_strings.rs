//! Texts of "Use AnyPlay as Your Console Home", the steps at the end of Connected Apps
//! (docs/DESIGN.md 9.13). Each names the rows it points to through `%@`, so a step always says
//! what the row is called in the player's language. The names of other programs' menus (Steam,
//! Windows) stay as those programs show them in English, and the text says so.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HomeGuide {
    /// The caption over the steps.
    Caption,
    /// Windows: `%@` = Console Mode, `%@` = the page it is on.
    ConsoleWindows,
    /// Linux and macOS: the same two names.
    ConsoleLogin,
    /// Steam's Big Picture at sign-in (Windows and macOS): `%@` = Add to Steam.
    BigPicture,
    /// Windows: Xbox mode cannot take AnyPlay as its home app.
    Xbox,
    /// Steam Deck and Bazzite Deck: `%@` = Add to Steam.
    Deck,
    /// Linux with a Steam Gaming Mode session.
    Session,
}

impl HomeGuide {
    pub const fn english(self) -> &'static str {
        match self {
            HomeGuide::Caption => "Use AnyPlay as Your Console Home",
            HomeGuide::ConsoleWindows => {
                "1. Turn on %@ (on the %@ page). AnyPlay then opens full screen when you sign in to Windows."
            }
            HomeGuide::ConsoleLogin => {
                "1. Turn on %@ (on the %@ page). AnyPlay then opens full screen when you log in."
            }
            HomeGuide::BigPicture => {
                "2. To start in Steam's Big Picture instead: choose %@ on this page, open Steam, go to Steam > Settings > Interface and turn on Run Steam when my computer starts and Start Steam in Big Picture Mode."
            }
            HomeGuide::Xbox => {
                "3. Xbox mode (Settings > Gaming > Xbox mode) starts a home app from a list of apps that Windows supports, and AnyPlay is not one of them. Press Win + F11 to leave Xbox mode and return to your desktop."
            }
            HomeGuide::Deck => {
                "2. On a Steam Deck or Bazzite Deck, stay in Desktop Mode and choose %@ on this page. Then select Return to Gaming Mode on the desktop. AnyPlay is in your Library under Non-Steam."
            }
            HomeGuide::Session => {
                "3. On Linux with a Steam Gaming Mode session, choose that session in the session menu on the login screen, where your system offers one. Then add AnyPlay to Steam the same way."
            }
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            HomeGuide::Caption => "AnyPlay'i Konsol Ana Ekranın Olarak Kullan",
            HomeGuide::ConsoleWindows => {
                "1. %@ seçeneğini aç (%@ sayfasında). AnyPlay, Windows'ta oturum açtığında tam ekranda açılır."
            }
            HomeGuide::ConsoleLogin => {
                "1. %@ seçeneğini aç (%@ sayfasında). AnyPlay, oturum açtığında tam ekranda açılır."
            }
            HomeGuide::BigPicture => {
                "2. Bunun yerine Steam'in Big Picture'ında başlamak için: bu sayfada %@ seçeneğini seç, Steam'i aç, Steam > Settings > Interface'e git ve Run Steam when my computer starts ile Start Steam in Big Picture Mode'u aç. (Adlar Steam'in İngilizce arayüzündeki gibidir.)"
            }
            HomeGuide::Xbox => {
                "3. Xbox mode (Settings > Gaming > Xbox mode), Windows'un desteklediği uygulamalar listesinden bir ana uygulama başlatır; AnyPlay bunlardan biri değil. Xbox mode'dan çıkıp masaüstüne dönmek için Win + F11'e bas. (Adlar Windows'un İngilizce arayüzündeki gibidir.)"
            }
            HomeGuide::Deck => {
                "2. Steam Deck veya Bazzite Deck'te Masaüstü Modu'nda kal ve bu sayfada %@ seçeneğini seç. Sonra masaüstündeki Return to Gaming Mode'u seç. AnyPlay, Kitaplık'ta Non-Steam altında olur."
            }
            HomeGuide::Session => {
                "3. Steam Gaming Mode oturumu olan bir Linux'ta, sistemin sunduğu yerde giriş ekranındaki oturum menüsünden o oturumu seç. Sonra AnyPlay'i aynı şekilde Steam'e ekle."
            }
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            HomeGuide::Caption => "AnyPlay als Konsolen-Startseite nutzen",
            HomeGuide::ConsoleWindows => {
                "1. Schalte %@ ein (Seite %@). AnyPlay öffnet sich dann im Vollbild, wenn du dich bei Windows anmeldest."
            }
            HomeGuide::ConsoleLogin => {
                "1. Schalte %@ ein (Seite %@). AnyPlay öffnet sich dann im Vollbild, wenn du dich anmeldest."
            }
            HomeGuide::BigPicture => {
                "2. Willst du stattdessen in Steams Big Picture starten: Wähle auf dieser Seite %@, öffne Steam, gehe zu Steam > Settings > Interface und schalte Run Steam when my computer starts und Start Steam in Big Picture Mode ein. (Die Namen stehen so in der englischen Steam-Oberfläche.)"
            }
            HomeGuide::Xbox => {
                "3. Der Xbox-Modus (Settings > Gaming > Xbox mode) startet eine Home-App aus einer Liste von Apps, die Windows unterstützt, und AnyPlay gehört nicht dazu. Mit Win + F11 verlässt du den Xbox-Modus und kehrst zum Desktop zurück. (Die Namen stehen so in der englischen Windows-Oberfläche.)"
            }
            HomeGuide::Deck => {
                "2. Bleibe auf einem Steam Deck oder Bazzite Deck im Desktop-Modus und wähle auf dieser Seite %@. Wähle dann auf dem Desktop Return to Gaming Mode. AnyPlay findest du in deiner Bibliothek unter Non-Steam."
            }
            HomeGuide::Session => {
                "3. Auf Linux mit einer Steam-Gaming-Mode-Sitzung wählst du diese Sitzung im Sitzungsmenü des Anmeldebildschirms, wo dein System eines anbietet. Füge AnyPlay danach genauso zu Steam hinzu."
            }
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            HomeGuide::Caption => "Usa AnyPlay como tu inicio de consola",
            HomeGuide::ConsoleWindows => {
                "1. Activa %@ (en la página %@). AnyPlay se abrirá a pantalla completa cuando inicies sesión en Windows."
            }
            HomeGuide::ConsoleLogin => {
                "1. Activa %@ (en la página %@). AnyPlay se abrirá a pantalla completa cuando inicies sesión."
            }
            HomeGuide::BigPicture => {
                "2. Para empezar en el Big Picture de Steam: elige %@ en esta página, abre Steam, ve a Steam > Settings > Interface y activa Run Steam when my computer starts y Start Steam in Big Picture Mode. (Los nombres son los de la interfaz de Steam en inglés.)"
            }
            HomeGuide::Xbox => {
                "3. El modo Xbox (Settings > Gaming > Xbox mode) inicia una app de inicio de una lista de apps que Windows admite, y AnyPlay no es una de ellas. Pulsa Win + F11 para salir del modo Xbox y volver al escritorio. (Los nombres son los de la interfaz de Windows en inglés.)"
            }
            HomeGuide::Deck => {
                "2. En una Steam Deck o Bazzite Deck, quédate en el modo escritorio y elige %@ en esta página. Después selecciona Return to Gaming Mode en el escritorio. AnyPlay está en tu biblioteca, en Non-Steam."
            }
            HomeGuide::Session => {
                "3. En Linux con una sesión Steam Gaming Mode, elige esa sesión en el menú de sesiones de la pantalla de inicio de sesión, donde tu sistema lo ofrezca. Luego añade AnyPlay a Steam de la misma forma."
            }
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            HomeGuide::Caption => "Utiliser AnyPlay comme accueil de console",
            HomeGuide::ConsoleWindows => {
                "1. Active %@ (page %@). AnyPlay s’ouvre alors en plein écran quand tu ouvres ta session Windows."
            }
            HomeGuide::ConsoleLogin => {
                "1. Active %@ (page %@). AnyPlay s’ouvre alors en plein écran quand tu ouvres ta session."
            }
            HomeGuide::BigPicture => {
                "2. Pour démarrer plutôt dans le Big Picture de Steam\u{a0}: choisis %@ sur cette page, ouvre Steam, va dans Steam > Settings > Interface et active Run Steam when my computer starts et Start Steam in Big Picture Mode. (Les noms sont ceux de l’interface anglaise de Steam.)"
            }
            HomeGuide::Xbox => {
                "3. Le mode Xbox (Settings > Gaming > Xbox mode) démarre une app d’accueil choisie dans une liste d’apps prises en charge par Windows, et AnyPlay n’en fait pas partie. Appuie sur Win + F11 pour quitter le mode Xbox et revenir au bureau. (Les noms sont ceux de l’interface anglaise de Windows.)"
            }
            HomeGuide::Deck => {
                "2. Sur un Steam Deck ou un Bazzite Deck, reste en mode bureau et choisis %@ sur cette page. Sélectionne ensuite Return to Gaming Mode sur le bureau. AnyPlay se trouve dans ta bibliothèque, sous Non-Steam."
            }
            HomeGuide::Session => {
                "3. Sous Linux avec une session Steam Gaming Mode, choisis cette session dans le menu des sessions de l’écran de connexion, là où ton système en propose un. Ajoute ensuite AnyPlay à Steam de la même façon."
            }
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            HomeGuide::Caption => "Use o AnyPlay como a tela inicial do seu console",
            HomeGuide::ConsoleWindows => {
                "1. Ative %@ (na página %@). O AnyPlay passa a abrir em tela cheia quando você entra no Windows."
            }
            HomeGuide::ConsoleLogin => {
                "1. Ative %@ (na página %@). O AnyPlay passa a abrir em tela cheia quando você entra."
            }
            HomeGuide::BigPicture => {
                "2. Para começar no Big Picture do Steam: escolha %@ nesta página, abra o Steam, vá em Steam > Settings > Interface e ative Run Steam when my computer starts e Start Steam in Big Picture Mode. (Os nomes são os da interface do Steam em inglês.)"
            }
            HomeGuide::Xbox => {
                "3. O modo Xbox (Settings > Gaming > Xbox mode) inicia um app inicial de uma lista de apps compatíveis com o Windows, e o AnyPlay não está nela. Pressione Win + F11 para sair do modo Xbox e voltar à área de trabalho. (Os nomes são os da interface do Windows em inglês.)"
            }
            HomeGuide::Deck => {
                "2. Em um Steam Deck ou Bazzite Deck, fique no Modo Desktop e escolha %@ nesta página. Depois selecione Return to Gaming Mode na área de trabalho. O AnyPlay fica na sua biblioteca, em Non-Steam."
            }
            HomeGuide::Session => {
                "3. No Linux com uma sessão Steam Gaming Mode, escolha essa sessão no menu de sessões da tela de login, onde seu sistema oferecer um. Depois adicione o AnyPlay ao Steam do mesmo jeito."
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

    pub const ALL: [HomeGuide; 7] = [
        HomeGuide::Caption,
        HomeGuide::ConsoleWindows,
        HomeGuide::ConsoleLogin,
        HomeGuide::BigPicture,
        HomeGuide::Xbox,
        HomeGuide::Deck,
        HomeGuide::Session,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_step_exists_in_every_language_with_the_same_names() {
        for text in HomeGuide::ALL {
            let names = text.english().matches("%@").count();
            for language in Language::ALL {
                assert!(!text.text(language).is_empty(), "{text:?} {language:?}");
                assert_eq!(
                    text.text(language).matches("%@").count(),
                    names,
                    "{text:?} {language:?}"
                );
            }
        }
    }
}
