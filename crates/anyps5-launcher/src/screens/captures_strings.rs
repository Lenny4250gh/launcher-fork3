//! Texts of the Captures area (docs/DESIGN.md 9.10). English and Turkish like the rest.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CapturesText {
    /// The area's name: the title, the top bar button, the Home button panel's row.
    Title,
    FilterAll,
    FilterScreenshots,
    FilterClips,
    /// Accessible name of a tile: `%@` = the game, `%@` = when.
    Screenshot,
    Clip,
    /// The first line of the loading state.
    Loading,
    /// Nothing in the folder.
    EmptyTitle,
    /// How to take a screenshot, and a clip where the player can.
    EmptyBody,
    EmptyBodyShotsOnly,
    OpenClipsSettings,
    /// The filter hides everything.
    NoScreenshots,
    NoClips,
    NoClipsBody,
    ShowAll,
    /// The folder is not there. `%@` = its path.
    MissingTitle,
    MissingBody,
    OpenFolderSettings,
    /// The folder is there and cannot be read. `%@` = its path.
    FailedTitle,
    FailedBody,
    TryAgain,
    /// A game's group when the library does not know the game.
    OtherGame,
    /// The full view.
    OpenInPlayer,
    ShowInFolder,
    CopyImage,
    /// The button that opens the friend picker.
    SendToFriend,
    Delete,
    Previous,
    Next,
    /// `%@` = place, `%@` = how many.
    Position,
    /// Under a clip that has no picture.
    ClipNote,
    /// The question before a delete.
    DeleteScreenshotTitle,
    DeleteClipTitle,
    /// `%@` = the file name.
    DeleteBody,
    DeleteScreenshot,
    DeleteClip,
    Cancel,
    /// Toasts. `%@` = the file name.
    Deleted,
    DeleteFailed,
    Copied,
    CopyFailed,
    PlayerFailed,
    FolderFailed,
}

impl CapturesText {
    pub const fn english(self) -> &'static str {
        match self {
            CapturesText::Title => "Captures",
            CapturesText::FilterAll => "All",
            CapturesText::FilterScreenshots => "Screenshots",
            CapturesText::FilterClips => "Clips",
            CapturesText::Screenshot => "Screenshot",
            CapturesText::Clip => "Clip",
            CapturesText::Loading => "Loading captures",
            CapturesText::EmptyTitle => "No captures yet",
            CapturesText::EmptyBody => {
                "Take a screenshot in a game (F12 by default). Turn on Clips to keep the last seconds of play as a video."
            }
            CapturesText::EmptyBodyShotsOnly => "Take a screenshot in a game (F12 by default).",
            CapturesText::OpenClipsSettings => "Open Clips Settings",
            CapturesText::NoScreenshots => "No screenshots yet",
            CapturesText::NoClips => "No clips yet",
            CapturesText::NoClipsBody => {
                "Turn on Clips in Settings to keep the last seconds of a game as a video."
            }
            CapturesText::ShowAll => "Show All Captures",
            CapturesText::MissingTitle => "Screenshots folder not found",
            CapturesText::MissingBody => {
                "%@ is missing. Take a screenshot in a game to create it, or choose another folder in Settings."
            }
            CapturesText::OpenFolderSettings => "Open Folder Settings",
            CapturesText::FailedTitle => "Captures could not be read",
            CapturesText::FailedBody => {
                "AnyPlay could not read %@. Check that you have permission to open the folder, then try again."
            }
            CapturesText::TryAgain => "Try Again",
            CapturesText::OtherGame => "Other Captures",
            CapturesText::OpenInPlayer => "Open in Player",
            CapturesText::ShowInFolder => "Show in Folder",
            CapturesText::CopyImage => "Copy Image",
            CapturesText::SendToFriend => "Send to a Friend",
            CapturesText::Delete => "Delete",
            CapturesText::Previous => "Previous capture",
            CapturesText::Next => "Next capture",
            CapturesText::Position => "%@ of %@",
            CapturesText::ClipNote => "Plays in your video player",
            CapturesText::DeleteScreenshotTitle => "Delete this screenshot?",
            CapturesText::DeleteClipTitle => "Delete this clip?",
            CapturesText::DeleteBody => "%@ is removed from your PC. This cannot be undone.",
            CapturesText::DeleteScreenshot => "Delete Screenshot",
            CapturesText::DeleteClip => "Delete Clip",
            CapturesText::Cancel => "Cancel",
            CapturesText::Deleted => "Deleted %@",
            CapturesText::DeleteFailed => "%@ could not be deleted.",
            CapturesText::Copied => "Image copied",
            CapturesText::CopyFailed => "The image could not be copied.",
            CapturesText::PlayerFailed => "The video player could not be opened.",
            CapturesText::FolderFailed => "The folder could not be opened.",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            CapturesText::Title => "Yakalamalar",
            CapturesText::FilterAll => "Tümü",
            CapturesText::FilterScreenshots => "Ekran Görüntüleri",
            CapturesText::FilterClips => "Klipler",
            CapturesText::Screenshot => "Ekran görüntüsü",
            CapturesText::Clip => "Klip",
            CapturesText::Loading => "Yakalamalar yükleniyor",
            CapturesText::EmptyTitle => "Henüz yakalama yok",
            CapturesText::EmptyBody => {
                "Oyunda ekran görüntüsü al (varsayılan F12). Oynayışın son saniyelerini video olarak tutmak için Klipleri aç."
            }
            CapturesText::EmptyBodyShotsOnly => "Oyunda ekran görüntüsü al (varsayılan F12).",
            CapturesText::OpenClipsSettings => "Klip Ayarlarını Aç",
            CapturesText::NoScreenshots => "Henüz ekran görüntüsü yok",
            CapturesText::NoClips => "Henüz klip yok",
            CapturesText::NoClipsBody => {
                "Bir oyunun son saniyelerini video olarak tutmak için Ayarlar'dan Klipleri aç."
            }
            CapturesText::ShowAll => "Tüm Yakalamaları Göster",
            CapturesText::MissingTitle => "Ekran görüntüsü klasörü bulunamadı",
            CapturesText::MissingBody => {
                "%@ yok. Oluşturmak için bir oyunda ekran görüntüsü al ya da Ayarlar'dan başka bir klasör seç."
            }
            CapturesText::OpenFolderSettings => "Klasör Ayarlarını Aç",
            CapturesText::FailedTitle => "Yakalamalar okunamadı",
            CapturesText::FailedBody => {
                "AnyPlay %@ klasörünü okuyamadı. Klasörü açma iznin olduğundan emin ol, sonra tekrar dene."
            }
            CapturesText::TryAgain => "Tekrar Dene",
            CapturesText::OtherGame => "Diğer Yakalamalar",
            CapturesText::OpenInPlayer => "Oynatıcıda Aç",
            CapturesText::ShowInFolder => "Klasörde Göster",
            CapturesText::CopyImage => "Görüntüyü Kopyala",
            CapturesText::SendToFriend => "Bir Arkadaşa Gönder",
            CapturesText::Delete => "Sil",
            CapturesText::Previous => "Önceki yakalama",
            CapturesText::Next => "Sonraki yakalama",
            CapturesText::Position => "%@ / %@",
            CapturesText::ClipNote => "Video oynatıcında açılır",
            CapturesText::DeleteScreenshotTitle => "Bu ekran görüntüsü silinsin mi?",
            CapturesText::DeleteClipTitle => "Bu klip silinsin mi?",
            CapturesText::DeleteBody => "%@ bilgisayarından kaldırılır. Bu geri alınamaz.",
            CapturesText::DeleteScreenshot => "Ekran Görüntüsünü Sil",
            CapturesText::DeleteClip => "Klibi Sil",
            CapturesText::Cancel => "Vazgeç",
            CapturesText::Deleted => "%@ silindi",
            CapturesText::DeleteFailed => "%@ silinemedi.",
            CapturesText::Copied => "Görüntü kopyalandı",
            CapturesText::CopyFailed => "Görüntü kopyalanamadı.",
            CapturesText::PlayerFailed => "Video oynatıcısı açılamadı.",
            CapturesText::FolderFailed => "Klasör açılamadı.",
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            CapturesText::Title => "Aufnahmen",
            CapturesText::FilterAll => "Alle",
            CapturesText::FilterScreenshots => "Screenshots",
            CapturesText::FilterClips => "Clips",
            CapturesText::Screenshot => "Screenshot",
            CapturesText::Clip => "Clip",
            CapturesText::Loading => "Aufnahmen werden geladen",
            CapturesText::EmptyTitle => "Noch keine Aufnahmen",
            CapturesText::EmptyBody => {
                "Mach in einem Spiel einen Screenshot (standardmäßig F12). Schalte Clips ein, um die letzten Sekunden als Video zu behalten."
            }
            CapturesText::EmptyBodyShotsOnly => {
                "Mach in einem Spiel einen Screenshot (standardmäßig F12)."
            }
            CapturesText::OpenClipsSettings => "Clip-Einstellungen öffnen",
            CapturesText::NoScreenshots => "Noch keine Screenshots",
            CapturesText::NoClips => "Noch keine Clips",
            CapturesText::NoClipsBody => {
                "Schalte Clips in den Einstellungen ein, um die letzten Sekunden eines Spiels als Video zu behalten."
            }
            CapturesText::ShowAll => "Alle Aufnahmen anzeigen",
            CapturesText::MissingTitle => "Screenshot-Ordner nicht gefunden",
            CapturesText::MissingBody => {
                "%@ fehlt. Mach in einem Spiel einen Screenshot, um den Ordner zu erstellen, oder wähle in den Einstellungen einen anderen."
            }
            CapturesText::OpenFolderSettings => "Ordner-Einstellungen öffnen",
            CapturesText::FailedTitle => "Aufnahmen konnten nicht gelesen werden",
            CapturesText::FailedBody => {
                "AnyPlay konnte %@ nicht lesen. Prüfe, ob du den Ordner öffnen darfst, und versuche es dann erneut."
            }
            CapturesText::TryAgain => "Erneut versuchen",
            CapturesText::OtherGame => "Weitere Aufnahmen",
            CapturesText::OpenInPlayer => "Im Player öffnen",
            CapturesText::ShowInFolder => "Im Ordner anzeigen",
            CapturesText::CopyImage => "Bild kopieren",
            CapturesText::Delete => "Löschen",
            CapturesText::Previous => "Vorherige Aufnahme",
            CapturesText::Next => "Nächste Aufnahme",
            CapturesText::Position => "%@ von %@",
            CapturesText::ClipNote => "Wird in deinem Videoplayer abgespielt",
            CapturesText::DeleteScreenshotTitle => "Diesen Screenshot löschen?",
            CapturesText::DeleteClipTitle => "Diesen Clip löschen?",
            CapturesText::DeleteBody => {
                "%@ wird von deinem PC entfernt. Das lässt sich nicht rückgängig machen."
            }
            CapturesText::DeleteScreenshot => "Screenshot löschen",
            CapturesText::DeleteClip => "Clip löschen",
            CapturesText::Cancel => "Abbrechen",
            CapturesText::Deleted => "%@ gelöscht",
            CapturesText::DeleteFailed => "%@ konnte nicht gelöscht werden.",
            CapturesText::Copied => "Bild kopiert",
            CapturesText::CopyFailed => "Das Bild konnte nicht kopiert werden.",
            CapturesText::PlayerFailed => "Der Videoplayer konnte nicht geöffnet werden.",
            CapturesText::FolderFailed => "Der Ordner konnte nicht geöffnet werden.",
            CapturesText::SendToFriend => "An einen Freund senden",
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            CapturesText::Title => "Capturas",
            CapturesText::FilterAll => "Todas",
            CapturesText::FilterScreenshots => "Capturas de pantalla",
            CapturesText::FilterClips => "Clips",
            CapturesText::Screenshot => "Captura de pantalla",
            CapturesText::Clip => "Clip",
            CapturesText::Loading => "Cargando capturas",
            CapturesText::EmptyTitle => "Aún no hay capturas",
            CapturesText::EmptyBody => {
                "Haz una captura de pantalla en un juego (F12 por defecto). Activa los clips para guardar los últimos segundos de juego como video."
            }
            CapturesText::EmptyBodyShotsOnly => {
                "Haz una captura de pantalla en un juego (F12 por defecto)."
            }
            CapturesText::OpenClipsSettings => "Abrir ajustes de clips",
            CapturesText::NoScreenshots => "Aún no hay capturas de pantalla",
            CapturesText::NoClips => "Aún no hay clips",
            CapturesText::NoClipsBody => {
                "Activa los clips en Ajustes para guardar los últimos segundos de un juego como video."
            }
            CapturesText::ShowAll => "Mostrar todas las capturas",
            CapturesText::MissingTitle => "No se encontró la carpeta de capturas",
            CapturesText::MissingBody => {
                "Falta %@. Haz una captura de pantalla en un juego para crearla o elige otra carpeta en Ajustes."
            }
            CapturesText::OpenFolderSettings => "Abrir ajustes de carpeta",
            CapturesText::FailedTitle => "No se pudieron leer las capturas",
            CapturesText::FailedBody => {
                "AnyPlay no pudo leer %@. Comprueba que tienes permiso para abrir la carpeta y vuelve a intentarlo."
            }
            CapturesText::TryAgain => "Reintentar",
            CapturesText::OtherGame => "Otras capturas",
            CapturesText::OpenInPlayer => "Abrir en el reproductor",
            CapturesText::ShowInFolder => "Mostrar en la carpeta",
            CapturesText::CopyImage => "Copiar imagen",
            CapturesText::SendToFriend => "Enviar a un amigo",
            CapturesText::Delete => "Eliminar",
            CapturesText::Previous => "Captura anterior",
            CapturesText::Next => "Captura siguiente",
            CapturesText::Position => "%@ de %@",
            CapturesText::ClipNote => "Se reproduce en tu reproductor de video",
            CapturesText::DeleteScreenshotTitle => "¿Eliminar esta captura de pantalla?",
            CapturesText::DeleteClipTitle => "¿Eliminar este clip?",
            CapturesText::DeleteBody => "%@ se eliminará de tu PC. No se puede deshacer.",
            CapturesText::DeleteScreenshot => "Eliminar captura",
            CapturesText::DeleteClip => "Eliminar clip",
            CapturesText::Cancel => "Cancelar",
            CapturesText::Deleted => "%@ eliminado",
            CapturesText::DeleteFailed => "No se pudo eliminar %@.",
            CapturesText::Copied => "Imagen copiada",
            CapturesText::CopyFailed => "No se pudo copiar la imagen.",
            CapturesText::PlayerFailed => "No se pudo abrir el reproductor de video.",
            CapturesText::FolderFailed => "No se pudo abrir la carpeta.",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            CapturesText::Title => "Captures",
            CapturesText::FilterAll => "Toutes",
            CapturesText::FilterScreenshots => "Captures d’écran",
            CapturesText::FilterClips => "Clips",
            CapturesText::Screenshot => "Capture d’écran",
            CapturesText::Clip => "Clip",
            CapturesText::Loading => "Chargement des captures",
            CapturesText::EmptyTitle => "Pas encore de captures",
            CapturesText::EmptyBody => {
                "Fais une capture d’écran dans un jeu (F12 par défaut). Active les clips pour garder les dernières secondes de jeu en vidéo."
            }
            CapturesText::EmptyBodyShotsOnly => {
                "Fais une capture d’écran dans un jeu (F12 par défaut)."
            }
            CapturesText::OpenClipsSettings => "Ouvrir les paramètres des clips",
            CapturesText::NoScreenshots => "Pas encore de captures d’écran",
            CapturesText::NoClips => "Pas encore de clips",
            CapturesText::NoClipsBody => {
                "Active les clips dans Paramètres pour garder les dernières secondes d’un jeu en vidéo."
            }
            CapturesText::ShowAll => "Afficher toutes les captures",
            CapturesText::MissingTitle => "Dossier des captures introuvable",
            CapturesText::MissingBody => {
                "%@ est introuvable. Fais une capture d’écran dans un jeu pour le créer, ou choisis un autre dossier dans Paramètres."
            }
            CapturesText::OpenFolderSettings => "Ouvrir les paramètres du dossier",
            CapturesText::FailedTitle => "Lecture des captures impossible",
            CapturesText::FailedBody => {
                "AnyPlay n’a pas pu lire %@. Vérifie que tu as l’autorisation d’ouvrir le dossier, puis réessaie."
            }
            CapturesText::TryAgain => "Réessayer",
            CapturesText::OtherGame => "Autres captures",
            CapturesText::OpenInPlayer => "Ouvrir dans le lecteur",
            CapturesText::ShowInFolder => "Afficher dans le dossier",
            CapturesText::CopyImage => "Copier l’image",
            CapturesText::SendToFriend => "Envoyer à un ami",
            CapturesText::Delete => "Supprimer",
            CapturesText::Previous => "Capture précédente",
            CapturesText::Next => "Capture suivante",
            CapturesText::Position => "%@ sur %@",
            CapturesText::ClipNote => "Se lit dans ton lecteur vidéo",
            CapturesText::DeleteScreenshotTitle => "Supprimer cette capture d’écran\u{a0}?",
            CapturesText::DeleteClipTitle => "Supprimer ce clip\u{a0}?",
            CapturesText::DeleteBody => {
                "%@ sera supprimé de ton PC. Cette action est irréversible."
            }
            CapturesText::DeleteScreenshot => "Supprimer la capture",
            CapturesText::DeleteClip => "Supprimer le clip",
            CapturesText::Cancel => "Annuler",
            CapturesText::Deleted => "%@ supprimé",
            CapturesText::DeleteFailed => "Impossible de supprimer %@.",
            CapturesText::Copied => "Image copiée",
            CapturesText::CopyFailed => "Impossible de copier l’image.",
            CapturesText::PlayerFailed => "Impossible d’ouvrir le lecteur vidéo.",
            CapturesText::FolderFailed => "Impossible d’ouvrir le dossier.",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            CapturesText::Title => "Capturas",
            CapturesText::FilterAll => "Todas",
            CapturesText::FilterScreenshots => "Capturas de tela",
            CapturesText::FilterClips => "Clipes",
            CapturesText::Screenshot => "Captura de tela",
            CapturesText::Clip => "Clipe",
            CapturesText::Loading => "Carregando capturas",
            CapturesText::EmptyTitle => "Ainda não há capturas",
            CapturesText::EmptyBody => {
                "Faça uma captura de tela em um jogo (F12 por padrão). Ative os clipes para guardar os últimos segundos de jogo como vídeo."
            }
            CapturesText::EmptyBodyShotsOnly => {
                "Faça uma captura de tela em um jogo (F12 por padrão)."
            }
            CapturesText::OpenClipsSettings => "Abrir configurações de clipes",
            CapturesText::NoScreenshots => "Ainda não há capturas de tela",
            CapturesText::NoClips => "Ainda não há clipes",
            CapturesText::NoClipsBody => {
                "Ative os clipes em Configurações para guardar os últimos segundos de um jogo como vídeo."
            }
            CapturesText::ShowAll => "Mostrar todas as capturas",
            CapturesText::MissingTitle => "Pasta de capturas não encontrada",
            CapturesText::MissingBody => {
                "%@ não existe. Faça uma captura de tela em um jogo para criar a pasta ou escolha outra em Configurações."
            }
            CapturesText::OpenFolderSettings => "Abrir configurações da pasta",
            CapturesText::FailedTitle => "Não foi possível ler as capturas",
            CapturesText::FailedBody => {
                "O AnyPlay não conseguiu ler %@. Verifique se você tem permissão para abrir a pasta e tente de novo."
            }
            CapturesText::TryAgain => "Tentar de novo",
            CapturesText::OtherGame => "Outras capturas",
            CapturesText::OpenInPlayer => "Abrir no reprodutor",
            CapturesText::ShowInFolder => "Mostrar na pasta",
            CapturesText::CopyImage => "Copiar imagem",
            CapturesText::SendToFriend => "Enviar a um amigo",
            CapturesText::Delete => "Excluir",
            CapturesText::Previous => "Captura anterior",
            CapturesText::Next => "Próxima captura",
            CapturesText::Position => "%@ de %@",
            CapturesText::ClipNote => "Reproduz no seu reprodutor de vídeo",
            CapturesText::DeleteScreenshotTitle => "Excluir esta captura de tela?",
            CapturesText::DeleteClipTitle => "Excluir este clipe?",
            CapturesText::DeleteBody => "%@ será excluído do seu PC. Isso não pode ser desfeito.",
            CapturesText::DeleteScreenshot => "Excluir captura",
            CapturesText::DeleteClip => "Excluir clipe",
            CapturesText::Cancel => "Cancelar",
            CapturesText::Deleted => "%@ excluído",
            CapturesText::DeleteFailed => "Não foi possível excluir %@.",
            CapturesText::Copied => "Imagem copiada",
            CapturesText::CopyFailed => "Não foi possível copiar a imagem.",
            CapturesText::PlayerFailed => "Não foi possível abrir o reprodutor de vídeo.",
            CapturesText::FolderFailed => "Não foi possível abrir a pasta.",
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

    pub const ALL: [CapturesText; 43] = [
        CapturesText::Title,
        CapturesText::FilterAll,
        CapturesText::FilterScreenshots,
        CapturesText::FilterClips,
        CapturesText::Screenshot,
        CapturesText::Clip,
        CapturesText::Loading,
        CapturesText::EmptyTitle,
        CapturesText::EmptyBody,
        CapturesText::EmptyBodyShotsOnly,
        CapturesText::OpenClipsSettings,
        CapturesText::NoScreenshots,
        CapturesText::NoClips,
        CapturesText::NoClipsBody,
        CapturesText::ShowAll,
        CapturesText::MissingTitle,
        CapturesText::MissingBody,
        CapturesText::OpenFolderSettings,
        CapturesText::FailedTitle,
        CapturesText::FailedBody,
        CapturesText::TryAgain,
        CapturesText::OtherGame,
        CapturesText::OpenInPlayer,
        CapturesText::ShowInFolder,
        CapturesText::CopyImage,
        CapturesText::SendToFriend,
        CapturesText::Delete,
        CapturesText::Previous,
        CapturesText::Next,
        CapturesText::Position,
        CapturesText::ClipNote,
        CapturesText::DeleteScreenshotTitle,
        CapturesText::DeleteClipTitle,
        CapturesText::DeleteBody,
        CapturesText::DeleteScreenshot,
        CapturesText::DeleteClip,
        CapturesText::Cancel,
        CapturesText::Deleted,
        CapturesText::DeleteFailed,
        CapturesText::Copied,
        CapturesText::CopyFailed,
        CapturesText::PlayerFailed,
        CapturesText::FolderFailed,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_text_has_both_languages() {
        for text in CapturesText::ALL {
            assert!(!text.english().is_empty());
            assert!(!text.turkish().is_empty());
            assert_eq!(
                text.english().matches("%@").count(),
                text.turkish().matches("%@").count(),
                "{text:?}"
            );
        }
    }
}
