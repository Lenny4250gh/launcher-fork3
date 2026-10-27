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

    pub const fn text(self, language: Language) -> &'static str {
        match language {
            Language::Turkish => self.turkish(),
            Language::English => self.english(),
            Language::German => match self.german() {
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
            for language in [Language::English, Language::Turkish, Language::German] {
                assert!(!text.text(language).is_empty(), "{text:?}");
            }
        }
    }
}
