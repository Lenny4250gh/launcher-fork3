//! Texts the sign-in page adds for a player with a pad (docs/DESIGN.md 9.14).

use crate::ui::i18n::Language;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SignInText {
    /// Under the note while the browser has the sign-in page: what the pad can and cannot do there.
    BrowserPad,
}

impl SignInText {
    pub const fn english(self) -> &'static str {
        match self {
            SignInText::BrowserPad => {
                "A controller cannot steer the browser, so finish there with the mouse or keyboard. AnyPlay comes back to the front by itself."
            }
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            SignInText::BrowserPad => {
                "Kontrolcü tarayıcıyı yönetemez; orada fare veya klavyeyle bitir. AnyPlay kendiliğinden öne gelir."
            }
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            SignInText::BrowserPad => {
                "Ein Controller kann den Browser nicht steuern, also schließe dort mit Maus oder Tastatur ab. AnyPlay kommt von selbst wieder nach vorn."
            }
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            SignInText::BrowserPad => {
                "Un mando no puede manejar el navegador, así que termina allí con el ratón o el teclado. AnyPlay vuelve al frente solo."
            }
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            SignInText::BrowserPad => {
                "Une manette ne peut pas piloter le navigateur\u{a0}: termine là-bas à la souris ou au clavier. AnyPlay revient tout seul au premier plan."
            }
            _ => return None,
        })
    }

    /// The Brazilian Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            SignInText::BrowserPad => {
                "Um controle não consegue usar o navegador, então termine lá com o mouse ou o teclado. O AnyPlay volta para a frente sozinho."
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
