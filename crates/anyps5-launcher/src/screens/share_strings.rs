//! Texts of the sharing features: layouts (export, import, community) and "Report to AnyPS5".
//! English and Turkish like the rest; what a report holds (the form's fields) stays English, as
//! the upstream form is.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Share {
    /// Layout choice: `%@` = the layout's name.
    LayoutCommunity,
    LayoutExport,
    LayoutImport,
    /// Toast after the file was written.
    LayoutExported,
    LayoutNotSaved,
    LayoutImported,
    /// Toast: `%@` = the reason (a line of the file, or that it cannot be read).
    LayoutImportFailed,
    LayoutUnreadable,
    /// Toast: the file was read, but lines of it have no row here.
    LayoutImportedPartly,
    /// Game settings row.
    Report,
    ReportDetail,
    /// The review: what happens, and what to do when the report is long.
    ReportHint,
    ReportPasteHint,
    ReportOpen,
    ReportOpenedPaste,
    ReportOpened,
    ReportOpenFailed,
}

impl Share {
    pub const fn english(self) -> &'static str {
        match self {
            Share::LayoutCommunity => "Community: %@",
            Share::LayoutExport => "Export Layout",
            Share::LayoutImport => "Import Layout",
            Share::LayoutExported => "Layout saved",
            Share::LayoutNotSaved => "Could not save the layout.",
            Share::LayoutImported => "Layout imported",
            Share::LayoutImportFailed => "Layout not imported. %@",
            Share::LayoutUnreadable => "The file cannot be read.",
            Share::LayoutImportedPartly => {
                "Layout imported. Some actions are not shown in this list."
            }
            Share::Report => "Report a Problem",
            Share::ReportDetail => "Review the report, then save it or open it on GitHub",
            Share::ReportHint => {
                "This is the report. Save it as a file, or open it on GitHub with the form filled in. Nothing is sent until you submit it there."
            }
            Share::ReportPasteHint => {
                "The log is too long for the link. Open on GitHub copies the full report; paste it into the form."
            }
            Share::ReportOpen => "Open on GitHub",
            Share::ReportOpenedPaste => "Report copied. Paste it into the GitHub form.",
            Share::ReportOpened => "The form is open in your browser",
            Share::ReportOpenFailed => {
                "Could not open the browser. The link was copied. Paste it into your browser."
            }
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            Share::LayoutCommunity => "Topluluk: %@",
            Share::LayoutExport => "Düzeni Dışa Aktar",
            Share::LayoutImport => "Düzeni İçe Aktar",
            Share::LayoutExported => "Düzen kaydedildi",
            Share::LayoutNotSaved => "Düzen kaydedilemedi.",
            Share::LayoutImported => "Düzen içe aktarıldı",
            Share::LayoutImportFailed => "Düzen içe aktarılamadı. %@",
            Share::LayoutUnreadable => "Dosya okunamıyor.",
            Share::LayoutImportedPartly => {
                "Düzen içe aktarıldı. Bazı eylemler bu listede gösterilmiyor."
            }
            Share::Report => "Sorun Bildir",
            Share::ReportDetail => "Raporu gözden geçir, sonra kaydet ya da GitHub'da aç",
            Share::ReportHint => {
                "Bu rapordur. Dosya olarak kaydet ya da doldurulmuş hâliyle GitHub'da aç. GitHub'da sen gönderene kadar hiçbir şey gönderilmez."
            }
            Share::ReportPasteHint => {
                "Günlük bağlantı için çok uzun. GitHub'da aç, raporun tamamını panoya kopyalar; forma yapıştır."
            }
            Share::ReportOpen => "GitHub'da Aç",
            Share::ReportOpenedPaste => "Rapor kopyalandı. GitHub formuna yapıştır.",
            Share::ReportOpened => "Form tarayıcında açıldı",
            Share::ReportOpenFailed => {
                "Tarayıcı açılamadı. Bağlantı panoya kopyalandı. Tarayıcına yapıştır."
            }
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            Share::LayoutCommunity => "Community: %@",
            Share::LayoutExport => "Layout exportieren",
            Share::LayoutImport => "Layout importieren",
            Share::LayoutExported => "Layout gespeichert",
            Share::LayoutNotSaved => "Das Layout konnte nicht gespeichert werden.",
            Share::LayoutImported => "Layout importiert",
            Share::LayoutImportFailed => "Layout nicht importiert. %@",
            Share::LayoutUnreadable => "Die Datei kann nicht gelesen werden.",
            Share::LayoutImportedPartly => {
                "Layout importiert. Einige Aktionen werden in dieser Liste nicht angezeigt."
            }
            Share::Report => "Problem melden",
            Share::ReportDetail => {
                "Prüfe den Bericht, dann speichere ihn oder öffne ihn auf GitHub"
            }
            Share::ReportHint => {
                "Das ist der Bericht. Speichere ihn als Datei oder öffne ihn mit ausgefülltem Formular auf GitHub. Nichts wird gesendet, bevor du es dort absendest."
            }
            Share::ReportPasteHint => {
                "Das Protokoll ist für den Link zu lang. Öffnen auf GitHub kopiert den ganzen Bericht; füge ihn ins Formular ein."
            }
            Share::ReportOpen => "Auf GitHub öffnen",
            Share::ReportOpenedPaste => "Bericht kopiert. Füge ihn ins GitHub-Formular ein.",
            Share::ReportOpened => "Das Formular ist in deinem Browser geöffnet",
            Share::ReportOpenFailed => {
                "Der Browser konnte nicht geöffnet werden. Der Link wurde kopiert. Füge ihn in deinen Browser ein."
            }
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            Share::LayoutCommunity => "Comunidad: %@",
            Share::LayoutExport => "Exportar diseño",
            Share::LayoutImport => "Importar diseño",
            Share::LayoutExported => "Diseño guardado",
            Share::LayoutNotSaved => "No se pudo guardar el diseño.",
            Share::LayoutImported => "Diseño importado",
            Share::LayoutImportFailed => "Diseño no importado. %@",
            Share::LayoutUnreadable => "No se puede leer el archivo.",
            Share::LayoutImportedPartly => {
                "Diseño importado. Algunas acciones no aparecen en esta lista."
            }
            Share::Report => "Informar de un problema",
            Share::ReportDetail => "Revisa el informe y guárdalo o ábrelo en GitHub",
            Share::ReportHint => {
                "Este es el informe. Guárdalo como archivo o ábrelo en GitHub con el formulario rellenado. No se envía nada hasta que lo envíes allí."
            }
            Share::ReportPasteHint => {
                "El registro es demasiado largo para el enlace. Abrir en GitHub copia el informe completo; pégalo en el formulario."
            }
            Share::ReportOpen => "Abrir en GitHub",
            Share::ReportOpenedPaste => "Informe copiado. Pégalo en el formulario de GitHub.",
            Share::ReportOpened => "El formulario está abierto en tu navegador",
            Share::ReportOpenFailed => {
                "No se pudo abrir el navegador. Se copió el enlace. Pégalo en tu navegador."
            }
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            Share::LayoutCommunity => "Communauté\u{a0}: %@",
            Share::LayoutExport => "Exporter la disposition",
            Share::LayoutImport => "Importer une disposition",
            Share::LayoutExported => "Disposition enregistrée",
            Share::LayoutNotSaved => "Impossible d’enregistrer la disposition.",
            Share::LayoutImported => "Disposition importée",
            Share::LayoutImportFailed => "Disposition non importée. %@",
            Share::LayoutUnreadable => "Le fichier est illisible.",
            Share::LayoutImportedPartly => {
                "Disposition importée. Certaines actions ne figurent pas dans cette liste."
            }
            Share::Report => "Signaler un problème",
            Share::ReportDetail => "Relis le rapport, puis enregistre-le ou ouvre-le sur GitHub",
            Share::ReportHint => {
                "Voici le rapport. Enregistre-le dans un fichier, ou ouvre-le sur GitHub avec le formulaire rempli. Rien n’est envoyé tant que tu ne l’envoies pas là-bas."
            }
            Share::ReportPasteHint => {
                "Le journal est trop long pour le lien. Ouvrir sur GitHub copie le rapport complet\u{a0}; colle-le dans le formulaire."
            }
            Share::ReportOpen => "Ouvrir sur GitHub",
            Share::ReportOpenedPaste => "Rapport copié. Colle-le dans le formulaire GitHub.",
            Share::ReportOpened => "Le formulaire est ouvert dans ton navigateur",
            Share::ReportOpenFailed => {
                "Impossible d’ouvrir le navigateur. Le lien a été copié. Colle-le dans ton navigateur."
            }
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            Share::LayoutCommunity => "Comunidade: %@",
            Share::LayoutExport => "Exportar layout",
            Share::LayoutImport => "Importar layout",
            Share::LayoutExported => "Layout salvo",
            Share::LayoutNotSaved => "Não foi possível salvar o layout.",
            Share::LayoutImported => "Layout importado",
            Share::LayoutImportFailed => "Layout não importado. %@",
            Share::LayoutUnreadable => "Não foi possível ler o arquivo.",
            Share::LayoutImportedPartly => {
                "Layout importado. Algumas ações não aparecem nesta lista."
            }
            Share::Report => "Informar um problema",
            Share::ReportDetail => "Revise o relatório e salve-o ou abra-o no GitHub",
            Share::ReportHint => {
                "Este é o relatório. Salve-o como arquivo ou abra-o no GitHub com o formulário preenchido. Nada é enviado até você enviá-lo lá."
            }
            Share::ReportPasteHint => {
                "O registro é longo demais para o link. Abrir no GitHub copia o relatório completo; cole-o no formulário."
            }
            Share::ReportOpen => "Abrir no GitHub",
            Share::ReportOpenedPaste => "Relatório copiado. Cole-o no formulário do GitHub.",
            Share::ReportOpened => "O formulário está aberto no seu navegador",
            Share::ReportOpenFailed => {
                "Não foi possível abrir o navegador. O link foi copiado. Cole-o no seu navegador."
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

    pub const ALL: [Share; 17] = [
        Share::LayoutCommunity,
        Share::LayoutExport,
        Share::LayoutImport,
        Share::LayoutExported,
        Share::LayoutNotSaved,
        Share::LayoutImported,
        Share::LayoutImportFailed,
        Share::LayoutUnreadable,
        Share::LayoutImportedPartly,
        Share::Report,
        Share::ReportDetail,
        Share::ReportHint,
        Share::ReportPasteHint,
        Share::ReportOpen,
        Share::ReportOpenedPaste,
        Share::ReportOpened,
        Share::ReportOpenFailed,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_text_exists_in_both_languages_with_the_same_placeholders() {
        for share in Share::ALL {
            for language in Language::ALL {
                assert!(!share.text(language).is_empty(), "{share:?} {language:?}");
            }
            for language in Language::ALL {
                assert_eq!(
                    share.english().matches("%@").count(),
                    share.text(language).matches("%@").count(),
                    "{share:?} {language:?}"
                );
            }
        }
    }
}
