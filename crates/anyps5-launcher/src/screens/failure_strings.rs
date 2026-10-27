//! The words of a game that stopped before it was ready (the failed unlock on the game page):
//! what happened and what to do, in plain words; the technical message stays as a quiet line
//! under it. English and Turkish like the rest.

use crate::ui::i18n::Language;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FailureText {
    /// Headline of a failed unlock: what happened and how to go on.
    UnlockStopped,
}

impl FailureText {
    pub const ALL: [FailureText; 1] = [FailureText::UnlockStopped];

    pub const fn english(self) -> &'static str {
        match self {
            FailureText::UnlockStopped => {
                "Unlocking stopped. Try again. If it stops again, Options > Log shows why, and Report a Problem sends it to us."
            }
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            FailureText::UnlockStopped => {
                "Kilit a\u{e7}ma durdu. Yeniden dene. Yine durursa Se\u{e7}enekler > G\u{fc}nl\u{fc}k nedenini g\u{f6}sterir; Sorun bildir bunu bize g\u{f6}nderir."
            }
        }
    }

    pub const fn german(self) -> &'static str {
        match self {
            FailureText::UnlockStopped => {
                "Das Freischalten wurde angehalten. Versuch es noch einmal. Wenn es wieder anhält, zeigt dir Optionen > Protokoll den Grund, und Problem melden schickt ihn an uns."
            }
        }
    }

    pub const fn spanish(self) -> &'static str {
        match self {
            FailureText::UnlockStopped => {
                "Se detuvo el desbloqueo. Inténtalo de nuevo. Si vuelve a detenerse, Opciones > Registro muestra el motivo e Informar de un problema nos lo envía."
            }
        }
    }

    pub const fn french(self) -> &'static str {
        match self {
            FailureText::UnlockStopped => {
                "Le déverrouillage s’est arrêté. Réessaie. S’il s’arrête encore, Options > Journal montre pourquoi, et Signaler un problème nous l’envoie."
            }
        }
    }

    pub const fn portuguese(self) -> &'static str {
        match self {
            FailureText::UnlockStopped => {
                "O desbloqueio parou. Tente de novo. Se parar outra vez, Opções > Registro mostra o motivo, e Informar um problema envia para nós."
            }
        }
    }

    pub fn text(self, language: Language) -> &'static str {
        match language {
            Language::English => self.english(),
            Language::Turkish => self.turkish(),
            Language::German => self.german(),
            Language::Spanish => self.spanish(),
            Language::French => self.french(),
            Language::Portuguese => self.portuguese(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_text_exists_in_both_languages() {
        for text in FailureText::ALL {
            for language in Language::ALL {
                assert!(!text.text(language).is_empty(), "{text:?} {language:?}");
            }
        }
    }
}
