//! Texts of the first start's last step: signing in, or not yet. English and Turkish like the
//! rest; button labels in title case, sentences in sentence case.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OnboardingText {
    OnlineTitle,
    OnlineSubtitle,
    /// Leaves the first start without signing in.
    NotNow,
    /// Under the buttons: where signing in waits later.
    OnlineLater,
    /// Next to the dots: `%@` = this step, `%@` = how many.
    StepOf,
}

impl OnboardingText {
    pub const ALL: [OnboardingText; 5] = [
        OnboardingText::OnlineTitle,
        OnboardingText::OnlineSubtitle,
        OnboardingText::NotNow,
        OnboardingText::OnlineLater,
        OnboardingText::StepOf,
    ];

    pub const fn english(self) -> &'static str {
        match self {
            OnboardingText::OnlineTitle => "Play Together",
            OnboardingText::OnlineSubtitle => {
                "Sign in to see your friends, join their parties and calls, and keep your trophies on your profile. Signing in opens your browser for a moment."
            }
            OnboardingText::NotNow => "Not Now",
            OnboardingText::OnlineLater => "You can sign in any time from the top bar.",
            OnboardingText::StepOf => "Step %@ of %@",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            OnboardingText::OnlineTitle => "Birlikte Oyna",
            OnboardingText::OnlineSubtitle => {
                "Arkadaşlarını görmek, partilerine ve aramalarına katılmak ve kupalarını profilinde tutmak için giriş yap. Giriş için tarayıcın kısa bir süre açılır."
            }
            OnboardingText::NotNow => "Şimdi Değil",
            OnboardingText::OnlineLater => "İstediğin zaman üst çubuktan giriş yapabilirsin.",
            OnboardingText::StepOf => "Adım %@ / %@",
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            OnboardingText::OnlineTitle => "Play Together",
            OnboardingText::OnlineSubtitle => {
                "Melde dich an, um deine Freunde zu sehen, ihren Partys und Anrufen beizutreten und deine Trophäen im Profil zu behalten. Die Anmeldung öffnet kurz deinen Browser."
            }
            OnboardingText::NotNow => "Nicht jetzt",
            OnboardingText::OnlineLater => {
                "Du kannst dich jederzeit in der oberen Leiste anmelden."
            }
            OnboardingText::StepOf => "Schritt %@ von %@",
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
        }
    }

    pub fn format(self, language: Language, args: &[Arg<'_>]) -> String {
        substitute(self.text(language), args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_text_is_there_in_both_languages() {
        for text in OnboardingText::ALL {
            for language in Language::ALL {
                assert!(
                    !text.text(language).trim().is_empty(),
                    "{text:?} {language:?}"
                );
            }
        }
    }
}
