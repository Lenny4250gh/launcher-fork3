//! Texts of the Join button on Discord and of the join links that open the launcher
//! (docs/DESIGN.md 7.4 and 9.6). English and Turkish like the rest.

use anyps5_core::online::Failure;
use anyps5_core::online::api::ErrorCode;

use crate::ui::i18n::Language;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DiscordText {
    /// Settings row: the button on the Discord status, and what it does.
    Button,
    ButtonDetail,
    /// Toast: a join link arrived while nobody is signed in.
    SignIn,
    /// Toasts for the answers to a join link.
    Gone,
    Full,
    Already,
    Own,
}

impl DiscordText {
    pub const fn english(self) -> &'static str {
        match self {
            DiscordText::Button => "Show a Join Button on Discord",
            DiscordText::ButtonDetail => {
                "While you play online, people who see your Discord status can get an invitation. They choose whether to join."
            }
            DiscordText::SignIn => "Sign in to AnyPlay to use this join link.",
            DiscordText::Gone => {
                "This join link has expired, or the player left. Ask for a new one."
            }
            DiscordText::Full => "That game or party is full. Try again later.",
            DiscordText::Already => "You are already there, or in another party.",
            DiscordText::Own => "This is your own join link. Send it to a friend.",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            DiscordText::Button => "Discord'da Katıl Düğmesini Göster",
            DiscordText::ButtonDetail => {
                "Çevrimiçi oynarken Discord durumunu görenler davet alabilir. Katılıp katılmayacaklarına kendileri karar verir."
            }
            DiscordText::SignIn => "Bu katılma bağlantısı için önce AnyPlay'e giriş yap.",
            DiscordText::Gone => {
                "Bu katılma bağlantısının süresi dolmuş ya da oyuncu ayrılmış. Yenisini iste."
            }
            DiscordText::Full => "Bu oyun ya da parti dolu. Sonra tekrar dene.",
            DiscordText::Already => "Zaten buradasın ya da başka bir partidesin.",
            DiscordText::Own => "Bu senin kendi katılma bağlantın. Bir arkadaşına gönder.",
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            DiscordText::Button => "Beitreten-Button auf Discord zeigen",
            DiscordText::ButtonDetail => {
                "Während du online spielst, können Leute, die deinen Discord-Status sehen, eine Einladung erhalten. Sie entscheiden selbst, ob sie beitreten."
            }
            DiscordText::SignIn => "Melde dich bei AnyPlay an, um diesen Beitrittslink zu nutzen.",
            DiscordText::Gone => {
                "Dieser Beitrittslink ist abgelaufen oder die Person hat das Spiel verlassen. Bitte um einen neuen."
            }
            DiscordText::Full => {
                "Dieses Spiel oder diese Party ist voll. Versuche es später erneut."
            }
            DiscordText::Already => "Du bist schon dort oder in einer anderen Party.",
            DiscordText::Own => "Das ist dein eigener Beitrittslink. Sende ihn an einen Freund.",
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            DiscordText::Button => "Mostrar un botón para unirse en Discord",
            DiscordText::ButtonDetail => {
                "Mientras juegas en línea, quienes vean tu estado de Discord pueden recibir una invitación. Ellos deciden si se unen."
            }
            DiscordText::SignIn => "Inicia sesión en AnyPlay para usar este enlace de invitación.",
            DiscordText::Gone => {
                "Este enlace de invitación ha caducado o el jugador se ha ido. Pide uno nuevo."
            }
            DiscordText::Full => "No quedan plazas en ese juego o party. Inténtalo más tarde.",
            DiscordText::Already => "Ya estás ahí o en otra party.",
            DiscordText::Own => "Este es tu propio enlace de invitación. Envíaselo a un amigo.",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            DiscordText::Button => "Afficher un bouton Rejoindre sur Discord",
            DiscordText::ButtonDetail => {
                "Pendant que tu joues en ligne, les personnes qui voient ton statut Discord peuvent recevoir une invitation. Elles choisissent de te rejoindre ou non."
            }
            DiscordText::SignIn => "Connecte-toi à AnyPlay pour utiliser ce lien d’invitation.",
            DiscordText::Gone => {
                "Ce lien d’invitation a expiré, ou le joueur est parti. Demande-en un nouveau."
            }
            DiscordText::Full => {
                "Il n’y a plus de place dans ce jeu ou cette party. Réessaie plus tard."
            }
            DiscordText::Already => "Tu y es déjà, ou tu es dans une autre party.",
            DiscordText::Own => "C’est ton propre lien d’invitation. Envoie-le à un ami.",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            DiscordText::Button => "Mostrar um botão Participar no Discord",
            DiscordText::ButtonDetail => {
                "Enquanto você joga online, quem vê seu status do Discord pode receber um convite. Cada pessoa decide se quer participar."
            }
            DiscordText::SignIn => "Entre na AnyPlay para usar este link de convite.",
            DiscordText::Gone => "Este link de convite expirou ou o jogador saiu. Peça um novo.",
            DiscordText::Full => "Não há mais vagas nesse jogo ou party. Tente de novo mais tarde.",
            DiscordText::Already => "Você já está lá ou em outra party.",
            DiscordText::Own => "Este é o seu próprio link de convite. Envie para um amigo.",
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

    pub const ALL: [DiscordText; 7] = [
        DiscordText::Button,
        DiscordText::ButtonDetail,
        DiscordText::SignIn,
        DiscordText::Gone,
        DiscordText::Full,
        DiscordText::Already,
        DiscordText::Own,
    ];
}

/// What a refused join link tells the player, or none for a failure with no text of its own
/// (the caller then uses the general one: offline, rate limited, restricted).
pub fn redeem_failure(failure: &Failure, language: Language) -> Option<&'static str> {
    let text = if failure.is(ErrorCode::NotFound) {
        DiscordText::Gone
    } else if failure.is(ErrorCode::Full) {
        DiscordText::Full
    } else if failure.is(ErrorCode::Limit) {
        DiscordText::Already
    } else if failure.is(ErrorCode::BadRequest) {
        DiscordText::Own
    } else {
        return None;
    };
    Some(text.text(language))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_text_exists_in_both_languages() {
        for text in DiscordText::ALL {
            for language in Language::ALL {
                assert!(!text.text(language).is_empty(), "{text:?} {language:?}");
            }
        }
    }

    #[test]
    fn the_names_are_ours() {
        for text in DiscordText::ALL {
            for language in Language::ALL {
                let shown = text.text(language);
                assert!(
                    !shown.contains("PlayStation") && !shown.contains("PS5"),
                    "{shown}"
                );
            }
        }
    }

    #[test]
    fn each_answer_of_the_cloud_has_a_sentence() {
        for (code, expected) in [
            ("not_found", DiscordText::Gone),
            ("full", DiscordText::Full),
            ("limit", DiscordText::Already),
            ("bad_request", DiscordText::Own),
        ] {
            let failure = Failure::new(code, "x");
            assert_eq!(
                redeem_failure(&failure, Language::English),
                Some(expected.english()),
                "{code}"
            );
        }
        for code in ["rate_limited", Failure::OFFLINE, "internal"] {
            assert_eq!(
                redeem_failure(&Failure::new(code, "x"), Language::English),
                None,
                "{code}"
            );
        }
    }
}
