//! Captions and the few lines of the global settings that have no other home: the group headings
//! that tell a long page apart (docs/DESIGN.md 5.7). English and Turkish like the rest; captions
//! in title case.

use crate::ui::i18n::Language;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SettingsText {
    /// Broadcast: the page on a system without the recorder.
    NoBroadcast,
    /// Connected Apps: nothing to connect on this system.
    NoApps,
    /// The tab: start-up, the Home button, updates.
    General,
    /// The tab: account, privacy, Discord status.
    Online,
    /// The tab: sounds and music.
    Sound,
    /// The tab: screenshots and clips.
    Captures,
    /// The tab: the pad.
    Controllers,
    /// The tab: Steam and links from websites.
    ConnectedApps,
    /// Language: the fonts games use.
    Fonts,
    /// Advanced (Linux): the video codec.
    ShareScreen,
    /// Toolkit: the update check.
    Updates,
    /// Toolkit: the releases nothing uses.
    Unused,
    /// Toolkit: the header row.
    Installed,
    /// Toolkit: the toolkit was never looked up.
    NotCheckedYet,
    /// Advanced: under the screenshots rows.
    ScreenshotHint,
}

impl SettingsText {
    pub const ALL: [SettingsText; 15] = [
        SettingsText::General,
        SettingsText::Online,
        SettingsText::NoApps,
        SettingsText::NoBroadcast,
        SettingsText::Sound,
        SettingsText::Captures,
        SettingsText::Controllers,
        SettingsText::ConnectedApps,
        SettingsText::Fonts,
        SettingsText::ShareScreen,
        SettingsText::Updates,
        SettingsText::Unused,
        SettingsText::Installed,
        SettingsText::NotCheckedYet,
        SettingsText::ScreenshotHint,
    ];

    pub const fn english(self) -> &'static str {
        match self {
            SettingsText::General => "General",
            SettingsText::Online => "Online",
            SettingsText::NoApps => "Nothing to connect on this system.",
            SettingsText::NoBroadcast => "Live streaming needs Windows.",
            SettingsText::Sound => "Sound",
            SettingsText::Captures => "Captures",
            SettingsText::Controllers => "Controllers",
            SettingsText::ConnectedApps => "Connected Apps",
            SettingsText::Fonts => "Fonts",
            SettingsText::ShareScreen => "Share Screen",
            SettingsText::Updates => "Updates",
            SettingsText::Unused => "Not Used by Any Game",
            SettingsText::Installed => "Installed Toolkit",
            SettingsText::NotCheckedYet => "Not checked yet",
            SettingsText::ScreenshotHint => "F12 in a game saves a screenshot.",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            SettingsText::General => "Genel",
            SettingsText::Online => "Çevrim İçi",
            SettingsText::NoApps => "Bu sistemde bağlanacak bir uygulama yok.",
            SettingsText::NoBroadcast => "Canlı yayın yalnızca Windows'ta var.",
            SettingsText::Sound => "Ses",
            SettingsText::Captures => "Yakalamalar",
            SettingsText::Controllers => "Kollar",
            SettingsText::ConnectedApps => "Bağlı Uygulamalar",
            SettingsText::Fonts => "Yazı Tipleri",
            SettingsText::ShareScreen => "Ekran Paylaşımı",
            SettingsText::Updates => "Güncellemeler",
            SettingsText::Unused => "Hiçbir Oyunun Kullanmadığı Sürümler",
            SettingsText::Installed => "Kurulu Araç Seti",
            SettingsText::NotCheckedYet => "Henüz denetlenmedi",
            SettingsText::ScreenshotHint => "Oyunda F12 bir ekran görüntüsü kaydeder.",
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            SettingsText::Sound => "Ton",
            SettingsText::Captures => "Aufnahmen",
            SettingsText::General => "Allgemein",
            SettingsText::Online => "Online",
            SettingsText::Controllers => "Controller",
            SettingsText::NoApps => "Auf diesem System gibt es nichts zu verbinden.",
            SettingsText::NoBroadcast => "Live-Übertragung gibt es nur unter Windows.",
            SettingsText::ConnectedApps => "Verbundene Apps",
            SettingsText::Fonts => "Schriften",
            SettingsText::ShareScreen => "Bildschirm teilen",
            SettingsText::Updates => "Updates",
            SettingsText::Unused => "Von keinem Spiel genutzt",
            SettingsText::Installed => "Installiertes Toolkit",
            SettingsText::NotCheckedYet => "Noch nicht geprüft",
            SettingsText::ScreenshotHint => "F12 im Spiel speichert einen Screenshot.",
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            SettingsText::General => "General",
            SettingsText::Online => "En línea",
            SettingsText::NoApps => "No hay nada que conectar en este sistema.",
            SettingsText::NoBroadcast => {
                "La transmisión en directo solo está disponible en Windows."
            }
            SettingsText::Sound => "Sonido",
            SettingsText::Captures => "Capturas",
            SettingsText::Controllers => "Mandos",
            SettingsText::ConnectedApps => "Apps conectadas",
            SettingsText::Fonts => "Fuentes",
            SettingsText::ShareScreen => "Compartir pantalla",
            SettingsText::Updates => "Actualizaciones",
            SettingsText::Unused => "Sin uso en ningún juego",
            SettingsText::Installed => "Toolkit instalado",
            SettingsText::NotCheckedYet => "Aún sin comprobar",
            SettingsText::ScreenshotHint => "F12 en un juego guarda una captura de pantalla.",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            SettingsText::General => "Général",
            SettingsText::Online => "En ligne",
            SettingsText::NoApps => "Rien à connecter sur ce système.",
            SettingsText::NoBroadcast => "La diffusion en direct nécessite Windows.",
            SettingsText::Sound => "Son",
            SettingsText::Captures => "Captures",
            SettingsText::Controllers => "Manettes",
            SettingsText::ConnectedApps => "Apps connectées",
            SettingsText::Fonts => "Polices",
            SettingsText::ShareScreen => "Partage d’écran",
            SettingsText::Updates => "Mises à jour",
            SettingsText::Unused => "Utilisé par aucun jeu",
            SettingsText::Installed => "Toolkit installé",
            SettingsText::NotCheckedYet => "Pas encore vérifié",
            SettingsText::ScreenshotHint => "F12 en jeu enregistre une capture d’écran.",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            SettingsText::General => "Geral",
            SettingsText::Online => "Online",
            SettingsText::NoApps => "Não há nada para conectar neste sistema.",
            SettingsText::NoBroadcast => "A transmissão ao vivo só funciona no Windows.",
            SettingsText::Sound => "Som",
            SettingsText::Captures => "Capturas",
            SettingsText::Controllers => "Controles",
            SettingsText::ConnectedApps => "Apps conectados",
            SettingsText::Fonts => "Fontes",
            SettingsText::ShareScreen => "Compartilhar tela",
            SettingsText::Updates => "Atualizações",
            SettingsText::Unused => "Não usado por nenhum jogo",
            SettingsText::Installed => "Toolkit instalado",
            SettingsText::NotCheckedYet => "Ainda não verificado",
            SettingsText::ScreenshotHint => "F12 em um jogo salva uma captura de tela.",
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_text_is_there_in_both_languages() {
        for text in SettingsText::ALL {
            for language in Language::ALL {
                assert!(
                    !text.text(language).trim().is_empty(),
                    "{text:?} {language:?}"
                );
            }
        }
    }
}
