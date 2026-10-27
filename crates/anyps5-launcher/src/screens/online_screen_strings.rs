//! Texts of share screen in the online area (docs/DESIGN.md 17, "Share screen"). English and
//! Turkish like the rest. The Turkish was written for this change and has not been read by a
//! native speaker yet. Row titles are in title case, notes and toasts in sentence case. The
//! numbers are said in plain words: where the picture comes from and how long it takes, never
//! a protocol's names.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScreenText {
    /// The caption of the group on the party page and the title of the Home row.
    Screen,
    /// The button that starts a stream, and what it does.
    ShareScreen,
    ShareDetail,
    /// The sentence over the player's own stream.
    SharedNobody,
    SharedOne,
    /// `%@` = how many.
    SharedMany,
    /// The detail of the sentence: `%@` = the game.
    HostStarting,
    HostLive,
    HostStalled,
    StopSharing,
    // Those who watch
    ViewerNotWatching,
    ViewerLosing,
    // Offers
    /// `%@` = who.
    Watch,
    /// `%@` = the game.
    SharingTitle,
    /// `%@` = who, `%@` = the game.
    IsSharing,
    CannotShow,
    // The stream being watched
    /// `%@` = who.
    Watching,
    PhaseConnecting,
    PhaseLive,
    PhaseStalled,
    StopWatching,
    WatchNote,
    // Where the picture comes from
    /// `%@` = milliseconds.
    DirectMs,
    RelayMs,
    // Home row
    /// `%@` = who.
    HomeWatching,
    // Toasts
    /// `%@` = who, `%@` = the game.
    ToastOffered,
    /// `%@` = who.
    ToastWatchEnded,
    ToastShareEnded,
    ErrorNoGame,
    ErrorNotInParty,
    ErrorBusy,
    ErrorNoEncoder,
    ErrorNotFound,
    /// A party member whose name is not known.
    SomeoneUnknown,
}

impl ScreenText {
    pub const fn english(self) -> &'static str {
        match self {
            ScreenText::Screen => "Screen",
            ScreenText::ShareScreen => "Share Screen",
            ScreenText::ShareDetail => "Show your game to the party",
            ScreenText::SharedNobody => "Your screen is shared",
            ScreenText::SharedOne => "Your screen is shared with 1 person",
            ScreenText::SharedMany => "Your screen is shared with %@ people",
            ScreenText::HostStarting => "Starting…",
            ScreenText::HostLive => "Live",
            ScreenText::HostStalled => "The picture has stopped",
            ScreenText::StopSharing => "Stop Sharing",
            ScreenText::ViewerNotWatching => "Not watching",
            ScreenText::ViewerLosing => "Some of the picture is lost",
            ScreenText::Watch => "Watch %@",
            ScreenText::SharingTitle => "Sharing %@",
            ScreenText::IsSharing => "%@ is sharing %@",
            ScreenText::CannotShow => "This version cannot show it yet.",
            ScreenText::Watching => "Watching %@",
            ScreenText::PhaseConnecting => "Connecting…",
            ScreenText::PhaseLive => "Live",
            ScreenText::PhaseStalled => "Waiting for the picture",
            ScreenText::StopWatching => "Stop Watching",
            ScreenText::WatchNote => "The picture opens in its own window.",
            ScreenText::DirectMs => "Direct, %@ ms",
            ScreenText::RelayMs => "Through a relay, %@ ms",
            ScreenText::HomeWatching => "Watching %@",
            ScreenText::ToastOffered => "%@ is sharing %@",
            ScreenText::ToastWatchEnded => "%@ stopped sharing.",
            ScreenText::ToastShareEnded => "Your screen is no longer shared.",
            ScreenText::ErrorNoGame => "Start a game first, then share its screen.",
            ScreenText::ErrorNotInParty => "Join a party to share your screen.",
            ScreenText::ErrorBusy => "You are already sharing or watching a screen.",
            ScreenText::ErrorNoEncoder => "This PC cannot share its screen.",
            ScreenText::ErrorNotFound => "That screen is not shared any more.",
            ScreenText::SomeoneUnknown => "Someone",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            ScreenText::Screen => "Ekran",
            ScreenText::ShareScreen => "Ekranı Paylaş",
            ScreenText::ShareDetail => "Oyununu partiye göster",
            ScreenText::SharedNobody => "Ekranın paylaşılıyor",
            ScreenText::SharedOne => "Ekranın 1 kişiyle paylaşılıyor",
            ScreenText::SharedMany => "Ekranın %@ kişiyle paylaşılıyor",
            ScreenText::HostStarting => "Başlıyor…",
            ScreenText::HostLive => "Canlı",
            ScreenText::HostStalled => "Görüntü gitmiyor",
            ScreenText::StopSharing => "Paylaşımı Durdur",
            ScreenText::ViewerNotWatching => "İzlemiyor",
            ScreenText::ViewerLosing => "Görüntünün bir kısmı kayboluyor",
            ScreenText::Watch => "İzle: %@",
            ScreenText::SharingTitle => "Paylaşılan: %@",
            ScreenText::IsSharing => "%@ şunu paylaşıyor: %@",
            ScreenText::CannotShow => "Bu sürüm bunu henüz gösteremiyor.",
            ScreenText::Watching => "İzlenen: %@",
            ScreenText::PhaseConnecting => "Bağlanıyor…",
            ScreenText::PhaseLive => "Canlı",
            ScreenText::PhaseStalled => "Görüntü bekleniyor",
            ScreenText::StopWatching => "İzlemeyi Bırak",
            ScreenText::WatchNote => "Görüntü kendi penceresinde açılır.",
            ScreenText::DirectMs => "Doğrudan, %@ ms",
            ScreenText::RelayMs => "Aktarıcı üzerinden, %@ ms",
            ScreenText::HomeWatching => "İzlenen: %@",
            ScreenText::ToastOffered => "%@ şunu paylaşıyor: %@",
            ScreenText::ToastWatchEnded => "%@ paylaşımı bıraktı.",
            ScreenText::ToastShareEnded => "Ekranın artık paylaşılmıyor.",
            ScreenText::ErrorNoGame => "Önce bir oyun başlat, sonra ekranını paylaş.",
            ScreenText::ErrorNotInParty => "Ekranını paylaşmak için bir partiye katıl.",
            ScreenText::ErrorBusy => "Zaten bir ekran paylaşıyor ya da izliyorsun.",
            ScreenText::ErrorNoEncoder => "Bu bilgisayar ekranını paylaşamıyor.",
            ScreenText::ErrorNotFound => "Bu ekran artık paylaşılmıyor.",
            ScreenText::SomeoneUnknown => "Biri",
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            ScreenText::Screen => "Bildschirm",
            ScreenText::ShareScreen => "Bildschirm teilen",
            ScreenText::ShareDetail => "Zeige dein Spiel der Party",
            ScreenText::SharedNobody => "Dein Bildschirm wird geteilt",
            ScreenText::SharedOne => "Dein Bildschirm wird mit 1 Person geteilt",
            ScreenText::SharedMany => "Dein Bildschirm wird mit %@ Personen geteilt",
            ScreenText::HostStarting => "Startet…",
            ScreenText::HostLive => "Live",
            ScreenText::HostStalled => "Das Bild ist stehen geblieben",
            ScreenText::StopSharing => "Teilen beenden",
            ScreenText::ViewerNotWatching => "Schaut nicht zu",
            ScreenText::ViewerLosing => "Ein Teil des Bilds geht verloren",
            ScreenText::Watch => "%@ ansehen",
            ScreenText::SharingTitle => "%@ wird geteilt",
            ScreenText::IsSharing => "%@ teilt %@",
            ScreenText::CannotShow => "Diese Version kann es noch nicht anzeigen.",
            ScreenText::Watching => "Du siehst %@",
            ScreenText::PhaseConnecting => "Verbinden…",
            ScreenText::PhaseLive => "Live",
            ScreenText::PhaseStalled => "Warten auf das Bild",
            ScreenText::StopWatching => "Zuschauen beenden",
            ScreenText::WatchNote => "Das Bild öffnet sich in einem eigenen Fenster.",
            ScreenText::DirectMs => "Direkt, %@ ms",
            ScreenText::RelayMs => "Über ein Relay, %@ ms",
            ScreenText::HomeWatching => "Du siehst %@",
            ScreenText::ToastOffered => "%@ teilt %@",
            ScreenText::ToastWatchEnded => "%@ hat das Teilen beendet.",
            ScreenText::ToastShareEnded => "Dein Bildschirm wird nicht mehr geteilt.",
            ScreenText::ErrorNoGame => "Starte zuerst ein Spiel, dann teile seinen Bildschirm.",
            ScreenText::ErrorNotInParty => "Tritt einer Party bei, um deinen Bildschirm zu teilen.",
            ScreenText::ErrorBusy => "Du teilst oder siehst bereits einen Bildschirm.",
            ScreenText::ErrorNoEncoder => "Dieser PC kann seinen Bildschirm nicht teilen.",
            ScreenText::ErrorNotFound => "Dieser Bildschirm wird nicht mehr geteilt.",
            ScreenText::SomeoneUnknown => "Jemand",
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            ScreenText::Screen => "Pantalla",
            ScreenText::ShareScreen => "Compartir pantalla",
            ScreenText::ShareDetail => "Muestra tu juego a la party",
            ScreenText::SharedNobody => "Tu pantalla está compartida",
            ScreenText::SharedOne => "Tu pantalla está compartida con 1 persona",
            ScreenText::SharedMany => "Tu pantalla está compartida con %@ personas",
            ScreenText::HostStarting => "Iniciando…",
            ScreenText::HostLive => "En directo",
            ScreenText::HostStalled => "La imagen se ha detenido",
            ScreenText::StopSharing => "Dejar de compartir",
            ScreenText::ViewerNotWatching => "No está viendo",
            ScreenText::ViewerLosing => "Se pierde parte de la imagen",
            ScreenText::Watch => "Ver %@",
            ScreenText::SharingTitle => "Compartiendo %@",
            ScreenText::IsSharing => "%@ comparte %@",
            ScreenText::CannotShow => "Esta versión aún no puede mostrarlo.",
            ScreenText::Watching => "Viendo %@",
            ScreenText::PhaseConnecting => "Conectando…",
            ScreenText::PhaseLive => "En directo",
            ScreenText::PhaseStalled => "Esperando la imagen",
            ScreenText::StopWatching => "Dejar de ver",
            ScreenText::WatchNote => "La imagen se abre en su propia ventana.",
            ScreenText::DirectMs => "Directo, %@ ms",
            ScreenText::RelayMs => "Mediante un relé, %@ ms",
            ScreenText::HomeWatching => "Viendo %@",
            ScreenText::ToastOffered => "%@ comparte %@",
            ScreenText::ToastWatchEnded => "%@ dejó de compartir.",
            ScreenText::ToastShareEnded => "Tu pantalla ya no se comparte.",
            ScreenText::ErrorNoGame => "Inicia primero un juego y luego comparte su pantalla.",
            ScreenText::ErrorNotInParty => "Únete a una party para compartir tu pantalla.",
            ScreenText::ErrorBusy => "Ya estás compartiendo o viendo una pantalla.",
            ScreenText::ErrorNoEncoder => "Este PC no puede compartir su pantalla.",
            ScreenText::ErrorNotFound => "Esa pantalla ya no se comparte.",
            ScreenText::SomeoneUnknown => "Alguien",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            ScreenText::Screen => "Écran",
            ScreenText::ShareScreen => "Partager l’écran",
            ScreenText::ShareDetail => "Montre ton jeu à la party",
            ScreenText::SharedNobody => "Ton écran est partagé",
            ScreenText::SharedOne => "Ton écran est partagé avec 1 personne",
            ScreenText::SharedMany => "Ton écran est partagé avec %@ personnes",
            ScreenText::HostStarting => "Démarrage…",
            ScreenText::HostLive => "En direct",
            ScreenText::HostStalled => "L’image s’est arrêtée",
            ScreenText::StopSharing => "Arrêter le partage",
            ScreenText::ViewerNotWatching => "Ne regarde pas",
            ScreenText::ViewerLosing => "Une partie de l’image est perdue",
            ScreenText::Watch => "Regarder %@",
            ScreenText::SharingTitle => "Partage de %@",
            ScreenText::IsSharing => "%@ partage %@",
            ScreenText::CannotShow => "Cette version ne peut pas encore l’afficher.",
            ScreenText::Watching => "Tu regardes %@",
            ScreenText::PhaseConnecting => "Connexion…",
            ScreenText::PhaseLive => "En direct",
            ScreenText::PhaseStalled => "En attente de l’image",
            ScreenText::StopWatching => "Arrêter de regarder",
            ScreenText::WatchNote => "L’image s’ouvre dans sa propre fenêtre.",
            ScreenText::DirectMs => "Direct, %@\u{a0}ms",
            ScreenText::RelayMs => "Via un relais, %@\u{a0}ms",
            ScreenText::HomeWatching => "Tu regardes %@",
            ScreenText::ToastOffered => "%@ partage %@",
            ScreenText::ToastWatchEnded => "%@ a arrêté le partage.",
            ScreenText::ToastShareEnded => "Ton écran n’est plus partagé.",
            ScreenText::ErrorNoGame => "Lance d’abord un jeu, puis partage son écran.",
            ScreenText::ErrorNotInParty => "Rejoins une party pour partager ton écran.",
            ScreenText::ErrorBusy => "Tu partages ou regardes déjà un écran.",
            ScreenText::ErrorNoEncoder => "Ce PC ne peut pas partager son écran.",
            ScreenText::ErrorNotFound => "Cet écran n’est plus partagé.",
            ScreenText::SomeoneUnknown => "Quelqu’un",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            ScreenText::Screen => "Tela",
            ScreenText::ShareScreen => "Compartilhar tela",
            ScreenText::ShareDetail => "Mostre seu jogo para a party",
            ScreenText::SharedNobody => "Sua tela está compartilhada",
            ScreenText::SharedOne => "Sua tela está compartilhada com 1 pessoa",
            ScreenText::SharedMany => "Sua tela está compartilhada com %@ pessoas",
            ScreenText::HostStarting => "Iniciando…",
            ScreenText::HostLive => "Ao vivo",
            ScreenText::HostStalled => "A imagem parou",
            ScreenText::StopSharing => "Parar de compartilhar",
            ScreenText::ViewerNotWatching => "Não está assistindo",
            ScreenText::ViewerLosing => "Parte da imagem está se perdendo",
            ScreenText::Watch => "Assistir a %@",
            ScreenText::SharingTitle => "Compartilhando %@",
            ScreenText::IsSharing => "%@ está compartilhando %@",
            ScreenText::CannotShow => "Esta versão ainda não consegue exibir isso.",
            ScreenText::Watching => "Assistindo a %@",
            ScreenText::PhaseConnecting => "Conectando…",
            ScreenText::PhaseLive => "Ao vivo",
            ScreenText::PhaseStalled => "Aguardando a imagem",
            ScreenText::StopWatching => "Parar de assistir",
            ScreenText::WatchNote => "A imagem abre em uma janela própria.",
            ScreenText::DirectMs => "Direto, %@ ms",
            ScreenText::RelayMs => "Por retransmissão, %@ ms",
            ScreenText::HomeWatching => "Assistindo a %@",
            ScreenText::ToastOffered => "%@ está compartilhando %@",
            ScreenText::ToastWatchEnded => "%@ parou de compartilhar.",
            ScreenText::ToastShareEnded => "Sua tela não está mais compartilhada.",
            ScreenText::ErrorNoGame => "Inicie um jogo primeiro e depois compartilhe a tela dele.",
            ScreenText::ErrorNotInParty => "Entre em uma party para compartilhar sua tela.",
            ScreenText::ErrorBusy => "Você já está compartilhando ou assistindo a uma tela.",
            ScreenText::ErrorNoEncoder => "Este PC não consegue compartilhar a tela.",
            ScreenText::ErrorNotFound => "Essa tela não está mais compartilhada.",
            ScreenText::SomeoneUnknown => "Alguém",
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

    const ONE_ARGUMENT: &[ScreenText] = &[
        ScreenText::SharedMany,
        ScreenText::Watch,
        ScreenText::SharingTitle,
        ScreenText::Watching,
        ScreenText::DirectMs,
        ScreenText::RelayMs,
        ScreenText::HomeWatching,
        ScreenText::ToastWatchEnded,
    ];
    const TWO_ARGUMENTS: &[ScreenText] = &[ScreenText::IsSharing, ScreenText::ToastOffered];

    #[test]
    fn placeholders_match_between_languages() {
        let count = |s: &str| s.matches("%@").count();
        for text in ONE_ARGUMENT {
            assert_eq!(count(text.english()), 1, "{text:?}");
            for language in Language::ALL {
                assert_eq!(count(text.text(language)), 1, "{text:?} {language:?}");
            }
        }
        for text in TWO_ARGUMENTS {
            assert_eq!(count(text.english()), 2, "{text:?}");
            for language in Language::ALL {
                assert_eq!(count(text.text(language)), 2, "{text:?} {language:?}");
            }
        }
    }

    #[test]
    fn row_titles_are_in_title_case() {
        for title in [
            ScreenText::Screen,
            ScreenText::ShareScreen,
            ScreenText::StopSharing,
            ScreenText::StopWatching,
        ] {
            for word in title.english().split(' ') {
                assert!(word.chars().next().unwrap().is_uppercase(), "{title:?}");
            }
        }
    }

    #[test]
    fn the_sentences_read_in_plain_words() {
        let en = Language::English;
        assert_eq!(
            ScreenText::SharedMany.format(en, &[Arg::Int(2)]),
            "Your screen is shared with 2 people"
        );
        assert_eq!(
            ScreenText::IsSharing.format(en, &[Arg::Str("Alec"), Arg::Str("Astro Bot")]),
            "Alec is sharing Astro Bot"
        );
        assert_eq!(
            ScreenText::DirectMs.format(en, &[Arg::Int(24)]),
            "Direct, 24 ms"
        );
        assert_eq!(
            ScreenText::RelayMs.format(en, &[Arg::Int(80)]),
            "Through a relay, 80 ms"
        );
    }

    #[test]
    fn no_text_uses_the_protocol_s_words() {
        for text in [
            ScreenText::DirectMs,
            ScreenText::RelayMs,
            ScreenText::PhaseStalled,
            ScreenText::HostStalled,
            ScreenText::ViewerLosing,
        ] {
            let said = text.english().to_lowercase();
            for jargon in ["rtt", "quic", "packet", "latency", "stalled"] {
                assert!(!said.contains(jargon), "{text:?}: {jargon}");
            }
        }
    }
}
