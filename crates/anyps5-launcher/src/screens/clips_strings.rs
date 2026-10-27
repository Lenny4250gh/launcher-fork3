//! Texts of Clips: the settings rows and the toasts. English and Turkish like the rest.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClipsText {
    /// Settings row: how long a clip is.
    Title,
    Detail,
    Off,
    /// The lengths, in the order of `anyps5_core::clips::LENGTHS` after Off.
    Seconds15,
    Seconds30,
    Minute1,
    Minutes2,
    /// Settings row: the key that saves a clip.
    Key,
    /// Under the key row while it does not wait: what the key does.
    KeyDetail,
    /// Shown on the key row while it waits for a key.
    KeyPrompt,
    /// Settings row: opens the folder. Detail: how many clips are in it.
    Show,
    CountNone,
    CountOne,
    /// `%@` = how many.
    CountMany,
    /// Toast after a clip was written.
    Saved,
    /// Toast: a save did not work.
    SaveFailed,
    /// Toast: pressed before the first seconds were kept.
    NothingYet,
    /// Toast: Create was pressed in a game while Clips is off.
    IsOff,
    /// Toast: the worker has no hardware video encoder.
    NoEncoder,
    NoWindow,
    /// Toast (macOS): Screen Recording is not switched on for AnyPlay.
    NoPermission,
    /// Toast: the recorder stopped in the middle of a game.
    Stopped,
    NoWorker,
    /// Toast: `%@` = the key, which another program has.
    KeyTaken,
}

impl ClipsText {
    pub const fn english(self) -> &'static str {
        match self {
            ClipsText::Title => "Clips",
            ClipsText::Detail => "Keeps the last seconds of a running game, without sound",
            ClipsText::Off => "Off",
            ClipsText::Seconds15 => "15 seconds",
            ClipsText::Seconds30 => "30 seconds",
            ClipsText::Minute1 => "1 minute",
            ClipsText::Minutes2 => "2 minutes",
            ClipsText::Key => "Clip Key",
            ClipsText::KeyDetail => "Saves the kept seconds as a clip. The Create button does too",
            ClipsText::KeyPrompt => "Press the keys you want to use. Esc keeps the current ones.",
            ClipsText::Show => "Show Clips",
            ClipsText::CountNone => "No clips yet",
            ClipsText::CountOne => "1 clip",
            ClipsText::CountMany => "%@ clips",
            ClipsText::Saved => "Clip saved",
            ClipsText::SaveFailed => "The clip could not be saved.",
            ClipsText::NothingYet => "Nothing recorded yet. Try again in a moment.",
            ClipsText::IsOff => "Clips are off. Turn them on in Settings.",
            ClipsText::NoEncoder => "Clips need a graphics card with a video encoder.",
            ClipsText::NoWindow => "Clips could not find the game's window.",
            ClipsText::NoPermission => {
                "Clips need Screen Recording. Turn on AnyPlay in System Settings, then reopen AnyPlay."
            }
            ClipsText::Stopped => "Clips stopped. Start the game again to record.",
            ClipsText::NoWorker => "Clips are not part of this build.",
            ClipsText::KeyTaken => "Another program uses %@. Choose another clip key.",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            ClipsText::Title => "Klipler",
            ClipsText::Detail => "Çalışan oyunun son saniyelerini sessiz olarak tutar",
            ClipsText::Off => "Kapalı",
            ClipsText::Seconds15 => "15 saniye",
            ClipsText::Seconds30 => "30 saniye",
            ClipsText::Minute1 => "1 dakika",
            ClipsText::Minutes2 => "2 dakika",
            ClipsText::Key => "Klip tuşu",
            ClipsText::KeyDetail => {
                "Tutulan saniyeleri klip olarak kaydeder. Create düğmesi de kaydeder"
            }
            ClipsText::KeyPrompt => "Kullanmak istediğin tuşlara bas. Esc mevcut tuşları korur.",
            ClipsText::Show => "Klipleri göster",
            ClipsText::CountNone => "Henüz klip yok",
            ClipsText::CountOne => "1 klip",
            ClipsText::CountMany => "%@ klip",
            ClipsText::Saved => "Klip kaydedildi",
            ClipsText::SaveFailed => "Klip kaydedilemedi.",
            ClipsText::NothingYet => "Henüz bir şey kaydedilmedi. Biraz sonra tekrar dene.",
            ClipsText::IsOff => "Klipler kapalı. Ayarlar'dan aç.",
            ClipsText::NoEncoder => "Klipler için video kodlayıcısı olan bir ekran kartı gerekir.",
            ClipsText::NoWindow => "Klipler oyunun penceresini bulamadı.",
            ClipsText::NoPermission => {
                "Klipler için Ekran Kaydı gerekir. Sistem Ayarları'nda AnyPlay'i aç, sonra AnyPlay'i yeniden aç."
            }
            ClipsText::Stopped => "Klipler durdu. Kayıt için oyunu yeniden başlat.",
            ClipsText::NoWorker => "Klipler bu sürümde yok.",
            ClipsText::KeyTaken => {
                "%@ tuşunu başka bir program kullanıyor. Başka bir klip tuşu seç."
            }
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            ClipsText::Title => "Clips",
            ClipsText::Detail => "Behält die letzten Sekunden eines laufenden Spiels, ohne Ton",
            ClipsText::Off => "Aus",
            ClipsText::Seconds15 => "15 Sekunden",
            ClipsText::Seconds30 => "30 Sekunden",
            ClipsText::Minute1 => "1 Minute",
            ClipsText::Minutes2 => "2 Minuten",
            ClipsText::Key => "Clip-Taste",
            ClipsText::KeyDetail => {
                "Speichert die behaltenen Sekunden als Clip. Die Create-Taste tut das auch"
            }
            ClipsText::KeyPrompt => {
                "Drücke die Tasten, die du nutzen willst. Esc behält die aktuellen."
            }
            ClipsText::Show => "Clips anzeigen",
            ClipsText::CountNone => "Noch keine Clips",
            ClipsText::CountOne => "1 Clip",
            ClipsText::CountMany => "%@ Clips",
            ClipsText::Saved => "Clip gespeichert",
            ClipsText::SaveFailed => "Der Clip konnte nicht gespeichert werden.",
            ClipsText::NothingYet => "Noch nichts aufgenommen. Versuche es gleich noch einmal.",
            ClipsText::IsOff => "Clips sind aus. Schalte sie in den Einstellungen ein.",
            ClipsText::NoEncoder => "Clips brauchen eine Grafikkarte mit Video-Encoder.",
            ClipsText::NoWindow => "Clips konnten das Spielfenster nicht finden.",
            ClipsText::NoPermission => {
                "Clips brauchen Bildschirmaufnahme. Schalte AnyPlay in den Systemeinstellungen ein und öffne AnyPlay dann neu."
            }
            ClipsText::Stopped => "Clips wurden gestoppt. Starte das Spiel neu, um aufzunehmen.",
            ClipsText::NoWorker => "Clips sind nicht Teil dieser Version.",
            ClipsText::KeyTaken => "Ein anderes Programm nutzt %@. Wähle eine andere Clip-Taste.",
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            ClipsText::Title => "Clips",
            ClipsText::Detail => "Guarda los últimos segundos de un juego en ejecución, sin sonido",
            ClipsText::Off => "Desactivado",
            ClipsText::Seconds15 => "15 segundos",
            ClipsText::Seconds30 => "30 segundos",
            ClipsText::Minute1 => "1 minuto",
            ClipsText::Minutes2 => "2 minutos",
            ClipsText::Key => "Tecla de clip",
            ClipsText::KeyDetail => {
                "Guarda como clip los segundos conservados. El botón Create también lo hace"
            }
            ClipsText::KeyPrompt => "Pulsa las teclas que quieras usar. Esc conserva las actuales.",
            ClipsText::Show => "Mostrar clips",
            ClipsText::CountNone => "Aún no hay clips",
            ClipsText::CountOne => "1 clip",
            ClipsText::CountMany => "%@ clips",
            ClipsText::Saved => "Clip guardado",
            ClipsText::SaveFailed => "No se pudo guardar el clip.",
            ClipsText::NothingYet => "Aún no se ha grabado nada. Inténtalo de nuevo en un momento.",
            ClipsText::IsOff => "Los clips están desactivados. Actívalos en Ajustes.",
            ClipsText::NoEncoder => {
                "Los clips necesitan una tarjeta gráfica con codificador de video."
            }
            ClipsText::NoWindow => "Los clips no encontraron la ventana del juego.",
            ClipsText::NoPermission => {
                "Los clips necesitan Grabación de pantalla. Activa AnyPlay en Ajustes del Sistema y vuelve a abrir AnyPlay."
            }
            ClipsText::Stopped => "Los clips se detuvieron. Reinicia el juego para grabar.",
            ClipsText::NoWorker => "Los clips no forman parte de esta versión.",
            ClipsText::KeyTaken => "Otro programa usa %@. Elige otra tecla de clip.",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            ClipsText::Title => "Clips",
            ClipsText::Detail => "Garde les dernières secondes d’un jeu en cours, sans le son",
            ClipsText::Off => "Désactivé",
            ClipsText::Seconds15 => "15 secondes",
            ClipsText::Seconds30 => "30 secondes",
            ClipsText::Minute1 => "1 minute",
            ClipsText::Minutes2 => "2 minutes",
            ClipsText::Key => "Touche de clip",
            ClipsText::KeyDetail => {
                "Enregistre en clip les secondes gardées. Le bouton Create le fait aussi"
            }
            ClipsText::KeyPrompt => {
                "Appuie sur les touches à utiliser. Échap conserve les actuelles."
            }
            ClipsText::Show => "Afficher les clips",
            ClipsText::CountNone => "Aucun clip pour l’instant",
            ClipsText::CountOne => "1 clip",
            ClipsText::CountMany => "%@ clips",
            ClipsText::Saved => "Clip enregistré",
            ClipsText::SaveFailed => "Impossible d’enregistrer le clip.",
            ClipsText::NothingYet => "Rien n’est encore enregistré. Réessaie dans un instant.",
            ClipsText::IsOff => "Les clips sont désactivés. Active-les dans Paramètres.",
            ClipsText::NoEncoder => {
                "Les clips nécessitent une carte graphique avec encodeur vidéo."
            }
            ClipsText::NoWindow => "Les clips n’ont pas trouvé la fenêtre du jeu.",
            ClipsText::NoPermission => {
                "Les clips nécessitent l’Enregistrement de l’écran. Active AnyPlay dans Réglages Système, puis rouvre AnyPlay."
            }
            ClipsText::Stopped => "Les clips se sont arrêtés. Relance le jeu pour enregistrer.",
            ClipsText::NoWorker => "Les clips ne sont pas inclus dans cette version.",
            ClipsText::KeyTaken => {
                "Un autre programme utilise %@. Choisis une autre touche de clip."
            }
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            ClipsText::Title => "Clipes",
            ClipsText::Detail => "Guarda os últimos segundos de um jogo em execução, sem som",
            ClipsText::Off => "Desativado",
            ClipsText::Seconds15 => "15 segundos",
            ClipsText::Seconds30 => "30 segundos",
            ClipsText::Minute1 => "1 minuto",
            ClipsText::Minutes2 => "2 minutos",
            ClipsText::Key => "Tecla de clipe",
            ClipsText::KeyDetail => {
                "Salva como clipe os segundos guardados. O botão Create também faz isso"
            }
            ClipsText::KeyPrompt => "Pressione as teclas que quer usar. Esc mantém as atuais.",
            ClipsText::Show => "Mostrar clipes",
            ClipsText::CountNone => "Nenhum clipe ainda",
            ClipsText::CountOne => "1 clipe",
            ClipsText::CountMany => "%@ clipes",
            ClipsText::Saved => "Clipe salvo",
            ClipsText::SaveFailed => "Não foi possível salvar o clipe.",
            ClipsText::NothingYet => "Nada foi gravado ainda. Tente de novo daqui a pouco.",
            ClipsText::IsOff => "Os clipes estão desativados. Ative-os em Configurações.",
            ClipsText::NoEncoder => {
                "Os clipes precisam de uma placa de vídeo com codificador de vídeo."
            }
            ClipsText::NoWindow => "Os clipes não encontraram a janela do jogo.",
            ClipsText::NoPermission => {
                "Os clipes precisam de Gravação de Tela. Ative o AnyPlay em Ajustes do Sistema e abra o AnyPlay de novo."
            }
            ClipsText::Stopped => "Os clipes pararam. Reinicie o jogo para gravar.",
            ClipsText::NoWorker => "Os clipes não fazem parte desta versão.",
            ClipsText::KeyTaken => "Outro programa usa %@. Escolha outra tecla de clipe.",
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

    pub const ALL: [ClipsText; 24] = [
        ClipsText::Title,
        ClipsText::Detail,
        ClipsText::Off,
        ClipsText::Seconds15,
        ClipsText::Seconds30,
        ClipsText::Minute1,
        ClipsText::Minutes2,
        ClipsText::Key,
        ClipsText::KeyDetail,
        ClipsText::KeyPrompt,
        ClipsText::Show,
        ClipsText::CountNone,
        ClipsText::CountOne,
        ClipsText::CountMany,
        ClipsText::Saved,
        ClipsText::SaveFailed,
        ClipsText::NothingYet,
        ClipsText::IsOff,
        ClipsText::NoEncoder,
        ClipsText::NoWindow,
        ClipsText::NoPermission,
        ClipsText::Stopped,
        ClipsText::NoWorker,
        ClipsText::KeyTaken,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_text_exists_in_both_languages_with_the_same_placeholders() {
        for text in ClipsText::ALL {
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

    #[test]
    fn no_visible_text_names_a_platform() {
        for text in ClipsText::ALL {
            for language in Language::ALL {
                let shown = text.text(language).to_lowercase();
                assert!(
                    !shown.contains("psn") && !shown.contains("playstation"),
                    "{text:?}"
                );
            }
        }
    }
}
