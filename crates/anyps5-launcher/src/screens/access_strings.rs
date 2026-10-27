//! Names only a screen reader hears: the controls that carry no visible label (the window's caption
//! buttons are glyphs). English, Turkish and German like the rest; titles are in title case.

use crate::ui::i18n::Language;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AccessText {
    /// The caption button that minimises the window.
    Minimize,
    /// The caption button that maximises the window.
    Maximize,
    /// The same button while the window is maximised or full screen.
    Restore,
}

impl AccessText {
    pub const ALL: [AccessText; 3] = [
        AccessText::Minimize,
        AccessText::Maximize,
        AccessText::Restore,
    ];

    pub const fn english(self) -> &'static str {
        match self {
            AccessText::Minimize => "Minimize",
            AccessText::Maximize => "Maximize",
            AccessText::Restore => "Restore",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            AccessText::Minimize => "Küçült",
            AccessText::Maximize => "Büyüt",
            AccessText::Restore => "Geri Yükle",
        }
    }

    /// The German text; `None` falls back to English.
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            AccessText::Minimize => "Minimieren",
            AccessText::Maximize => "Maximieren",
            AccessText::Restore => "Wiederherstellen",
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            AccessText::Minimize => "Minimizar",
            AccessText::Maximize => "Maximizar",
            AccessText::Restore => "Restaurar",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            AccessText::Minimize => "Réduire",
            AccessText::Maximize => "Agrandir",
            AccessText::Restore => "Restaurer",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            AccessText::Minimize => "Minimizar",
            AccessText::Maximize => "Maximizar",
            AccessText::Restore => "Restaurar",
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
    fn every_name_is_there_in_every_language() {
        for text in AccessText::ALL {
            for language in Language::ALL {
                assert!(!text.text(language).is_empty(), "{text:?}");
            }
        }
    }
}
