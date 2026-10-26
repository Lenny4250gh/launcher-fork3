//! The words of the 0.3.0 look-and-feel settings: the Game Colors theme, the volumes by kind of
//! sound, Larger Text and High Contrast (docs/DESIGN.md 3.13, 7.7). English and Turkish like the
//! rest; titles in title case.

use crate::ui::i18n::Language;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FeelText {
    /// The theme that takes its colours from the selected game.
    ThemeGameColors,
    /// Interface Sounds: how loud.
    VolumeInterface,
    /// Home Screen Music: how loud.
    VolumeMusic,
    /// Game Music: how loud.
    VolumeGameMusic,
    /// The switch of the notification sounds.
    NotificationSounds,
    NotificationSoundsDetail,
    /// Notification Sounds: how loud.
    VolumeNotifications,
    /// Settings, Appearance: the type one step bigger.
    LargerText,
    LargerTextDetail,
    /// Settings, Appearance: brighter ink and edges, no blur.
    HighContrast,
    HighContrastDetail,
    /// The value of a volume at zero.
    VolumeOff,
}

impl FeelText {
    pub const ALL: [FeelText; 12] = [
        FeelText::ThemeGameColors,
        FeelText::VolumeInterface,
        FeelText::VolumeMusic,
        FeelText::VolumeGameMusic,
        FeelText::NotificationSounds,
        FeelText::NotificationSoundsDetail,
        FeelText::VolumeNotifications,
        FeelText::LargerText,
        FeelText::LargerTextDetail,
        FeelText::HighContrast,
        FeelText::HighContrastDetail,
        FeelText::VolumeOff,
    ];

    pub const fn english(self) -> &'static str {
        match self {
            FeelText::ThemeGameColors => "Game Colors",
            FeelText::VolumeInterface => "Interface Volume",
            FeelText::VolumeMusic => "Home Screen Music Volume",
            FeelText::VolumeGameMusic => "Game Music Volume",
            FeelText::NotificationSounds => "Notification Sounds",
            FeelText::NotificationSoundsDetail => {
                "A chime for a message, a request, an invitation or a call."
            }
            FeelText::VolumeNotifications => "Notification Volume",
            FeelText::LargerText => "Larger Text",
            FeelText::LargerTextDetail => "Makes all text one step bigger.",
            FeelText::HighContrast => "High Contrast",
            FeelText::HighContrastDetail => "Brighter text and edges, and no frosted glass.",
            FeelText::VolumeOff => "Off",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            FeelText::ThemeGameColors => "Oyun Renkleri",
            FeelText::VolumeInterface => "Arayüz Ses Düzeyi",
            FeelText::VolumeMusic => "Ana Ekran Müziği Düzeyi",
            FeelText::VolumeGameMusic => "Oyun Müziği Düzeyi",
            FeelText::NotificationSounds => "Bildirim Sesleri",
            FeelText::NotificationSoundsDetail => {
                "Bir ileti, istek, davet veya arama için kısa bir çınlama."
            }
            FeelText::VolumeNotifications => "Bildirim Ses Düzeyi",
            FeelText::LargerText => "Daha Büyük Yazı",
            FeelText::LargerTextDetail => "Tüm yazıyı bir basamak büyütür.",
            FeelText::HighContrast => "Yüksek Karşıtlık",
            FeelText::HighContrastDetail => "Daha parlak yazı ve kenarlar, buzlu cam yok.",
            FeelText::VolumeOff => "Kapalı",
        }
    }

    pub const fn text(self, language: Language) -> &'static str {
        match language {
            Language::Turkish => self.turkish(),
            Language::English => self.english(),
        }
    }
}

/// The value of a volume row: `70%`, or Off at zero.
pub fn volume_text(percent: u8, language: Language) -> String {
    if percent == 0 {
        FeelText::VolumeOff.text(language).to_owned()
    } else {
        match language {
            Language::Turkish => format!("%{percent}"),
            Language::English => format!("{percent}%"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_text_is_there_in_both_languages() {
        for text in FeelText::ALL {
            for language in Language::ALL {
                assert!(
                    !text.text(language).trim().is_empty(),
                    "{text:?} {language:?}"
                );
            }
        }
    }

    #[test]
    fn a_volume_reads_as_a_percent_and_zero_as_off() {
        assert_eq!(volume_text(70, Language::English), "70%");
        assert_eq!(volume_text(70, Language::Turkish), "%70");
        assert_eq!(volume_text(0, Language::English), "Off");
    }
}
