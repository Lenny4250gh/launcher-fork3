//! Texts and pages of the About page: where AnyPlay lives, its rules and who it builds on.
//! English and Turkish like the rest; row titles in title case.

use crate::ui::i18n::Language;

/// The pages the About page opens. The launcher opens no other address from there.
pub const WEBSITE: &str = "https://anyplaynet.com";
pub const DISCORD: &str = "https://discord.gg/9zYqf8fEsh";
pub const PRIVACY: &str = "https://anyplaynet.com/privacy";
pub const TERMS: &str = "https://anyplaynet.com/terms";
/// Where texts are written and how to improve one (the Texts section of CONTRIBUTING).
pub const TRANSLATIONS: &str =
    "https://github.com/anyplaynet/launcher/blob/main/CONTRIBUTING.md#texts";
pub const PAGES: [&str; 5] = [WEBSITE, DISCORD, PRIVACY, TERMS, TRANSLATIONS];
/// The install guide, which Settings opens from the glibc note (Linux): which systems work.
pub const INSTALL_GUIDE: &str = "https://github.com/anyplaynet/launcher/blob/main/docs/INSTALL.md";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AboutText {
    Website,
    Discord,
    /// Under the Discord row: what it is for.
    DiscordDetail,
    Privacy,
    Terms,
    /// Before an account is made: the pages it means agreeing to. `{terms}` and `{privacy}` stand
    /// for the pages' names, which are links where the text can hold them.
    Consent,
    /// Who the launcher builds on.
    Credits,
    /// Who made LudusCtrl, under the credits.
    LudusCredits,
    /// The row that opens the page on improving a translation.
    Translations,
    /// Under it: Spanish, French and Brazilian Portuguese are waiting for a native speaker's review.
    TranslationsDetail,
}

impl AboutText {
    pub const ALL: [AboutText; 10] = [
        AboutText::Website,
        AboutText::Discord,
        AboutText::DiscordDetail,
        AboutText::Privacy,
        AboutText::Terms,
        AboutText::Consent,
        AboutText::Credits,
        AboutText::LudusCredits,
        AboutText::Translations,
        AboutText::TranslationsDetail,
    ];

    pub const fn english(self) -> &'static str {
        match self {
            AboutText::Website => "AnyPlay Website",
            AboutText::Discord => "AnyPlay Discord",
            AboutText::DiscordDetail => "News, help and other players",
            AboutText::Privacy => "Privacy Policy",
            AboutText::Terms => "Terms of Service",
            AboutText::Consent => {
                "By creating an account you agree to the {terms} and the {privacy}."
            }
            AboutText::Credits => {
                "Built on AnyPS5 by boykopovar. The launcher began as utkuhalis' AnyPS5-ARM launcher for macOS."
            }
            AboutText::LudusCredits => "LudusCtrl protocol and app by Josip Parlov (@parl0v)",
            AboutText::Translations => "Improve a Translation",
            AboutText::TranslationsDetail => {
                "Spanish, French and Portuguese still need a native speaker's review"
            }
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            AboutText::Website => "AnyPlay Web Sitesi",
            AboutText::Discord => "AnyPlay Discord",
            AboutText::DiscordDetail => "Haberler, yardım ve diğer oyuncular",
            AboutText::Privacy => "Gizlilik Politikası",
            AboutText::Terms => "Hizmet Şartları",
            AboutText::Consent => {
                "Hesap oluşturarak {terms} ve {privacy} metinlerini kabul edersin."
            }
            AboutText::Credits => {
                "boykopovar'ın AnyPS5'i üzerine kurulu. Launcher, utkuhalis'in macOS için AnyPS5-ARM launcher'ı olarak başladı."
            }
            AboutText::LudusCredits => "LudusCtrl protokolü ve uygulaması: Josip Parlov (@parl0v)",
            AboutText::Translations => "Bir Çeviriyi Geliştir",
            AboutText::TranslationsDetail => {
                "İspanyolca, Fransızca ve Portekizce'yi henüz anadili olan biri gözden geçirmedi"
            }
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            AboutText::Website => "AnyPlay-Website",
            AboutText::Discord => "AnyPlay Discord",
            AboutText::DiscordDetail => "Neuigkeiten, Hilfe und andere Spieler",
            AboutText::Privacy => "Datenschutzerklärung",
            AboutText::Terms => "Nutzungsbedingungen",
            AboutText::Consent => {
                "Mit dem Erstellen eines Kontos stimmst du den {terms} und der {privacy} zu."
            }
            AboutText::Credits => {
                "Aufgebaut auf AnyPS5 von boykopovar. Der Launcher begann als AnyPS5-ARM-Launcher von utkuhalis für macOS."
            }
            AboutText::LudusCredits => "LudusCtrl-Protokoll und App von Josip Parlov (@parl0v)",
            AboutText::Translations => "Eine Übersetzung verbessern",
            AboutText::TranslationsDetail => {
                "Spanisch, Französisch und Portugiesisch brauchen noch die Prüfung durch Muttersprachler"
            }
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            AboutText::Website => "Sitio web de AnyPlay",
            AboutText::Discord => "AnyPlay Discord",
            AboutText::DiscordDetail => "Novedades, ayuda y otros jugadores",
            AboutText::Privacy => "Política de privacidad",
            AboutText::Terms => "Términos del servicio",
            AboutText::Consent => "Al crear una cuenta aceptas los {terms} y la {privacy}.",
            AboutText::Credits => {
                "Creado sobre AnyPS5 de boykopovar. El launcher nació como el launcher AnyPS5-ARM de utkuhalis para macOS."
            }
            AboutText::LudusCredits => "Protocolo y app LudusCtrl de Josip Parlov (@parl0v)",
            AboutText::Translations => "Mejorar una traducción",
            AboutText::TranslationsDetail => {
                "Español, francés y portugués aún necesitan la revisión de un hablante nativo"
            }
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            AboutText::Website => "Site web AnyPlay",
            AboutText::Discord => "AnyPlay Discord",
            AboutText::DiscordDetail => "Actualités, aide et autres joueurs",
            AboutText::Privacy => "Politique de confidentialité",
            AboutText::Terms => "Conditions d’utilisation",
            AboutText::Consent => "En créant un compte, tu acceptes les {terms} et la {privacy}.",
            AboutText::Credits => {
                "Construit sur AnyPS5 de boykopovar. Le launcher est né du launcher AnyPS5-ARM d’utkuhalis pour macOS."
            }
            AboutText::LudusCredits => "Protocole et app LudusCtrl par Josip Parlov (@parl0v)",
            AboutText::Translations => "Améliorer une traduction",
            AboutText::TranslationsDetail => {
                "L’espagnol, le français et le portugais attendent encore la relecture d’un locuteur natif"
            }
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            AboutText::Website => "Site da AnyPlay",
            AboutText::Discord => "AnyPlay Discord",
            AboutText::DiscordDetail => "Novidades, ajuda e outros jogadores",
            AboutText::Privacy => "Política de privacidade",
            AboutText::Terms => "Termos de serviço",
            AboutText::Consent => {
                "Ao criar uma conta, você concorda com os {terms} e com a {privacy}."
            }
            AboutText::Credits => {
                "Criado sobre o AnyPS5 de boykopovar. O launcher começou como o launcher AnyPS5-ARM de utkuhalis para macOS."
            }
            AboutText::LudusCredits => "Protocolo e app LudusCtrl de Josip Parlov (@parl0v)",
            AboutText::Translations => "Melhorar uma tradução",
            AboutText::TranslationsDetail => {
                "Espanhol, francês e português ainda precisam da revisão de um falante nativo"
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_text_is_there_in_both_languages() {
        for text in AboutText::ALL {
            for language in Language::ALL {
                assert!(
                    !text.text(language).trim().is_empty(),
                    "{text:?} {language:?}"
                );
            }
        }
    }

    #[test]
    fn ludusctrl_is_credited_word_for_word() {
        assert_eq!(
            AboutText::LudusCredits.english(),
            "LudusCtrl protocol and app by Josip Parlov (@parl0v)"
        );
        assert!(
            AboutText::LudusCredits
                .turkish()
                .contains("Josip Parlov (@parl0v)")
        );
    }

    #[test]
    fn the_install_guide_is_ours_and_secure() {
        assert!(INSTALL_GUIDE.starts_with("https://github.com/anyplaynet/"));
    }

    #[test]
    fn the_pages_are_ours_and_secure() {
        for page in PAGES {
            assert!(page.starts_with("https://"), "{page}");
            assert!(
                page.starts_with("https://anyplaynet.com")
                    || page.starts_with("https://discord.gg/")
                    || page.starts_with("https://github.com/anyplaynet/"),
                "{page}"
            );
        }
    }
}
