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

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            OnboardingText::OnlineTitle => "Play Together",
            OnboardingText::OnlineSubtitle => {
                "Inicia sesión para ver a tus amigos, unirte a sus parties y llamadas, y guardar tus trofeos en tu perfil. Al iniciar sesión se abre el navegador un momento."
            }
            OnboardingText::NotNow => "Ahora no",
            OnboardingText::OnlineLater => {
                "Puedes iniciar sesión cuando quieras desde la barra superior."
            }
            OnboardingText::StepOf => "Paso %@ de %@",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            OnboardingText::OnlineTitle => "Play Together",
            OnboardingText::OnlineSubtitle => {
                "Connecte-toi pour voir tes amis, rejoindre leurs parties et leurs appels, et garder tes trophées sur ton profil. La connexion ouvre brièvement ton navigateur."
            }
            OnboardingText::NotNow => "Pas maintenant",
            OnboardingText::OnlineLater => {
                "Tu peux te connecter à tout moment depuis la barre du haut."
            }
            OnboardingText::StepOf => "Étape %@ sur %@",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            OnboardingText::OnlineTitle => "Play Together",
            OnboardingText::OnlineSubtitle => {
                "Entre para ver seus amigos, participar das parties e chamadas deles e manter seus troféus no perfil. Ao entrar, seu navegador abre por um instante."
            }
            OnboardingText::NotNow => "Agora não",
            OnboardingText::OnlineLater => {
                "Você pode entrar a qualquer momento pela barra superior."
            }
            OnboardingText::StepOf => "Etapa %@ de %@",
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
