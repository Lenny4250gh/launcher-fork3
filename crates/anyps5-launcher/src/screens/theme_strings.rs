//! The names of the three looks added in 0.3.5: Ember, Tide and Daylight (docs/DESIGN.md 3.16).
//! Short, one or two words, title case; the older three live in the string table.

use crate::ui::i18n::Language;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ThemeText {
    /// The warm look: slow embers rising from a low hearth.
    Ember,
    /// The deep, cool look: light moving through water.
    Tide,
    /// The light look: soft daylight.
    Daylight,
}

impl ThemeText {
    pub const ALL: [ThemeText; 3] = [ThemeText::Ember, ThemeText::Tide, ThemeText::Daylight];

    pub const fn english(self) -> &'static str {
        match self {
            ThemeText::Ember => "Ember",
            ThemeText::Tide => "Tide",
            ThemeText::Daylight => "Daylight",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            ThemeText::Ember => "Köz",
            ThemeText::Tide => "Gelgit",
            ThemeText::Daylight => "Gün Işığı",
        }
    }

    pub const fn german(self) -> &'static str {
        match self {
            ThemeText::Ember => "Glut",
            ThemeText::Tide => "Gezeiten",
            ThemeText::Daylight => "Tageslicht",
        }
    }

    pub const fn spanish(self) -> &'static str {
        match self {
            ThemeText::Ember => "Brasa",
            ThemeText::Tide => "Marea",
            ThemeText::Daylight => "Luz del Día",
        }
    }

    pub const fn french(self) -> &'static str {
        match self {
            ThemeText::Ember => "Braise",
            ThemeText::Tide => "Marée",
            ThemeText::Daylight => "Plein Jour",
        }
    }

    pub const fn portuguese(self) -> &'static str {
        match self {
            ThemeText::Ember => "Brasa",
            ThemeText::Tide => "Maré",
            ThemeText::Daylight => "Luz do Dia",
        }
    }

    pub const fn text(self, language: Language) -> &'static str {
        match language {
            Language::German => self.german(),
            Language::Spanish => self.spanish(),
            Language::French => self.french(),
            Language::Portuguese => self.portuguese(),
            Language::Turkish => self.turkish(),
            Language::English => self.english(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_is_short_and_there_in_every_language() {
        for text in ThemeText::ALL {
            for language in Language::ALL {
                let name = text.text(language);
                assert!(!name.trim().is_empty(), "{text:?} {language:?}");
                assert!(name.chars().count() <= 12, "{text:?} {language:?}: {name}");
            }
        }
    }
}
