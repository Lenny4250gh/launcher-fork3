//! Texts of share play in the online area (docs/DESIGN.md 17, "Share play"). English, Turkish and
//! German like the rest; Spanish, French and Portuguese follow with the languages pull request.
//! The Turkish was written for this change and has not been read by a native speaker yet. Row
//! titles are in title case, notes and toasts in sentence case. What a mode lets the guest do is
//! said in plain words, never as a protocol's name.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShareText {
    /// The caption of the group on the party page and the title of the Home row.
    SharePlay,
    // Starting
    /// `%@` = who.
    ShareWith,
    ShareWithDetail,
    // The host waits for an answer
    /// `%@` = who.
    WaitingFor,
    InvitationSent,
    CancelInvitation,
    // The host's running share
    /// `%@` = who.
    SeesYourGame,
    PhaseConnecting,
    PhaseLive,
    PhaseStalled,
    Mode,
    ModeWatch,
    ModeHandOver,
    ModeTogether,
    NoteWatch,
    NoteHandOver,
    NoteTogether,
    /// `%@` = who.
    AsksWatch,
    AsksHandOver,
    AsksTogether,
    AsksDetail,
    StopSharePlay,
    // The guest
    /// `%@` = who, `%@` = the game.
    InvitedWatch,
    InvitedHandOver,
    InvitedTogether,
    Accept,
    Decline,
    /// The button of a card that asks the host for a mode.
    Ask,
    /// The button of a card that changes the mode.
    Switch,
    /// The button of a card that invites a member.
    Invite,
    /// `%@` = who, `%@` = the game.
    GuestOf,
    YourPart,
    PartWatching,
    PartHandOver,
    PartTogether,
    AskHandOver,
    AskTogether,
    AskDetail,
    WindowNote,
    LeaveSharePlay,
    // Where the picture comes from
    /// `%@` = milliseconds.
    DirectMs,
    RelayMs,
    // Toasts
    /// `%@` = who.
    ToastInvited,
    /// `%@` = who.
    ToastAsked,
    ToastNowWatch,
    ToastNowHandOver,
    ToastNowTogether,
    ToastEnded,
    // Failures
    ErrorNoGame,
    ErrorNotInParty,
    ErrorBusy,
    ErrorNoEncoder,
    ErrorNoDecoder,
    ErrorNotFound,
    ErrorNotAllowed,
    /// A party member whose name is not known.
    SomeoneUnknown,
}

impl ShareText {
    pub const fn english(self) -> &'static str {
        match self {
            ShareText::SharePlay => "Share Play",
            ShareText::ShareWith => "Share with %@",
            ShareText::ShareWithDetail => "They watch first. You decide when they play.",
            ShareText::WaitingFor => "Waiting for %@",
            ShareText::InvitationSent => "Invitation sent",
            ShareText::CancelInvitation => "Cancel Invitation",
            ShareText::SeesYourGame => "%@ sees your game",
            ShareText::PhaseConnecting => "Connecting…",
            ShareText::PhaseLive => "Live",
            ShareText::PhaseStalled => "The picture has stopped",
            ShareText::Mode => "Mode",
            ShareText::ModeWatch => "Watch",
            ShareText::ModeHandOver => "Take over",
            ShareText::ModeTogether => "Together",
            ShareText::NoteWatch => "They see and hear your game.",
            ShareText::NoteHandOver => {
                "They play with your pad. Hold Select and Start for a second to take it back."
            }
            ShareText::NoteTogether => {
                "They play as the second player. Not every game has room for one."
            }
            ShareText::AsksWatch => "%@ would like to only watch",
            ShareText::AsksHandOver => "%@ would like to play with your pad",
            ShareText::AsksTogether => "%@ would like to play beside you",
            ShareText::AsksDetail => "Choose a mode below to agree.",
            ShareText::StopSharePlay => "Stop Sharing",
            ShareText::InvitedWatch => "%@ invites you to watch %@",
            ShareText::InvitedHandOver => "%@ invites you to play %@ with their pad",
            ShareText::InvitedTogether => "%@ invites you to play %@ beside them",
            ShareText::Ask => "Ask",
            ShareText::Switch => "Switch",
            ShareText::Invite => "Invite",
            ShareText::Accept => "Accept",
            ShareText::Decline => "Decline",
            ShareText::GuestOf => "%@ shares %@",
            ShareText::YourPart => "You",
            ShareText::PartWatching => "Watch",
            ShareText::PartHandOver => "Play with their pad",
            ShareText::PartTogether => "Play as the second player",
            ShareText::AskHandOver => "Ask to Take Over",
            ShareText::AskTogether => "Ask to Play Together",
            ShareText::AskDetail => "The host decides.",
            ShareText::WindowNote => "The picture opens in its own window.",
            ShareText::LeaveSharePlay => "Leave",
            ShareText::DirectMs => "Direct, %@ ms",
            ShareText::RelayMs => "Through a relay, %@ ms",
            ShareText::ToastInvited => "%@ invites you to share play",
            ShareText::ToastAsked => "%@ would like another mode.",
            ShareText::ToastNowWatch => "You can only watch now.",
            ShareText::ToastNowHandOver => "You play with the host's pad now.",
            ShareText::ToastNowTogether => "You are the second player now.",
            ShareText::ToastEnded => "Share play ended.",
            ShareText::ErrorNoGame => "Start a game first, then share it.",
            ShareText::ErrorNotInParty => "Share play goes to someone in your party.",
            ShareText::ErrorBusy => "You are in a share already.",
            ShareText::ErrorNoEncoder => "This PC cannot share a game yet.",
            ShareText::ErrorNoDecoder => "This PC cannot show a shared game yet.",
            ShareText::ErrorNotFound => "That invitation is not there any more.",
            ShareText::ErrorNotAllowed => "This cloud does not offer share play.",
            ShareText::SomeoneUnknown => "Someone",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            ShareText::SharePlay => "Birlikte Oyna",
            ShareText::ShareWith => "%@ ile paylaş",
            ShareText::ShareWithDetail => "Önce izler. Ne zaman oynayacağına sen karar verirsin.",
            ShareText::WaitingFor => "%@ bekleniyor",
            ShareText::InvitationSent => "Davet gönderildi",
            ShareText::CancelInvitation => "Daveti İptal Et",
            ShareText::SeesYourGame => "%@ oyununu görüyor",
            ShareText::PhaseConnecting => "Bağlanıyor…",
            ShareText::PhaseLive => "Canlı",
            ShareText::PhaseStalled => "Görüntü gitmiyor",
            ShareText::Mode => "Mod",
            ShareText::ModeWatch => "İzle",
            ShareText::ModeHandOver => "Devral",
            ShareText::ModeTogether => "Birlikte",
            ShareText::NoteWatch => "Oyununu görür ve duyar.",
            ShareText::NoteHandOver => {
                "Oyun kolunla oynar. Geri almak için Select ve Start tuşlarını bir saniye basılı tut."
            }
            ShareText::NoteTogether => {
                "İkinci oyuncu olarak oynar. Her oyunda ikinci oyuncuya yer yoktur."
            }
            ShareText::AsksWatch => "%@ yalnızca izlemek istiyor",
            ShareText::AsksHandOver => "%@ oyun kolunla oynamak istiyor",
            ShareText::AsksTogether => "%@ yanında oynamak istiyor",
            ShareText::AsksDetail => "Kabul etmek için aşağıdan bir mod seç.",
            ShareText::StopSharePlay => "Paylaşımı Durdur",
            ShareText::InvitedWatch => "%@ seni %@ oyununu izlemeye çağırıyor",
            ShareText::InvitedHandOver => {
                "%@ seni %@ oyununu kendi oyun koluyla oynamaya çağırıyor"
            }
            ShareText::InvitedTogether => "%@ seni %@ oyununu yanında oynamaya çağırıyor",
            ShareText::Ask => "İste",
            ShareText::Switch => "Geç",
            ShareText::Invite => "Davet Et",
            ShareText::Accept => "Kabul Et",
            ShareText::Decline => "Reddet",
            ShareText::GuestOf => "%@ şunu paylaşıyor: %@",
            ShareText::YourPart => "Sen",
            ShareText::PartWatching => "İzliyorsun",
            ShareText::PartHandOver => "Onun oyun koluyla oynuyorsun",
            ShareText::PartTogether => "İkinci oyuncu olarak oynuyorsun",
            ShareText::AskHandOver => "Devralmayı İste",
            ShareText::AskTogether => "Birlikte Oynamayı İste",
            ShareText::AskDetail => "Karar ev sahibinin.",
            ShareText::WindowNote => "Görüntü kendi penceresinde açılır.",
            ShareText::LeaveSharePlay => "Ayrıl",
            ShareText::DirectMs => "Doğrudan, %@ ms",
            ShareText::RelayMs => "Aktarıcı üzerinden, %@ ms",
            ShareText::ToastInvited => "%@ seni birlikte oynamaya çağırıyor",
            ShareText::ToastAsked => "%@ başka bir mod istiyor.",
            ShareText::ToastNowWatch => "Artık yalnızca izleyebilirsin.",
            ShareText::ToastNowHandOver => "Artık ev sahibinin oyun koluyla oynuyorsun.",
            ShareText::ToastNowTogether => "Artık ikinci oyuncusun.",
            ShareText::ToastEnded => "Paylaşım bitti.",
            ShareText::ErrorNoGame => "Önce bir oyun başlat, sonra paylaş.",
            ShareText::ErrorNotInParty => "Paylaşım partindeki birine gider.",
            ShareText::ErrorBusy => "Zaten bir paylaşımdasın.",
            ShareText::ErrorNoEncoder => "Bu bilgisayar henüz oyun paylaşamıyor.",
            ShareText::ErrorNoDecoder => "Bu bilgisayar paylaşılan oyunu henüz gösteremiyor.",
            ShareText::ErrorNotFound => "Bu davet artık yok.",
            ShareText::ErrorNotAllowed => "Bu bulut birlikte oynamayı sunmuyor.",
            ShareText::SomeoneUnknown => "Biri",
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            ShareText::SharePlay => "Share Play",
            ShareText::ShareWith => "Mit %@ teilen",
            ShareText::ShareWithDetail => {
                "Zuerst schaut die Person zu. Du entscheidest, wann sie spielt."
            }
            ShareText::WaitingFor => "Warten auf %@",
            ShareText::InvitationSent => "Einladung gesendet",
            ShareText::CancelInvitation => "Einladung zurückziehen",
            ShareText::SeesYourGame => "%@ sieht dein Spiel",
            ShareText::PhaseConnecting => "Verbinden…",
            ShareText::PhaseLive => "Live",
            ShareText::PhaseStalled => "Das Bild ist stehen geblieben",
            ShareText::Mode => "Modus",
            ShareText::ModeWatch => "Zuschauen",
            ShareText::ModeHandOver => "Übernehmen",
            ShareText::ModeTogether => "Zusammen",
            ShareText::NoteWatch => "Die Person sieht und hört dein Spiel.",
            ShareText::NoteHandOver => {
                "Die Person spielt mit deinem Controller. Halte Select und Start eine Sekunde gedrückt, um ihn zurückzunehmen."
            }
            ShareText::NoteTogether => {
                "Die Person spielt als zweite Spielerin oder zweiter Spieler. Nicht jedes Spiel hat Platz für eine zweite Person."
            }
            ShareText::AsksWatch => "%@ möchte nur zuschauen",
            ShareText::AsksHandOver => "%@ möchte mit deinem Controller spielen",
            ShareText::AsksTogether => "%@ möchte neben dir spielen",
            ShareText::AsksDetail => "Wähle unten einen Modus, um zuzustimmen.",
            ShareText::StopSharePlay => "Teilen beenden",
            ShareText::InvitedWatch => "%@ lädt dich ein, %@ zuzuschauen",
            ShareText::InvitedHandOver => "%@ lädt dich ein, %@ mit dem Controller zu spielen",
            ShareText::InvitedTogether => "%@ lädt dich ein, %@ neben ihr oder ihm zu spielen",
            ShareText::Ask => "Anfragen",
            ShareText::Switch => "Wechseln",
            ShareText::Invite => "Einladen",
            ShareText::Accept => "Annehmen",
            ShareText::Decline => "Ablehnen",
            ShareText::GuestOf => "%@ teilt %@",
            ShareText::YourPart => "Du",
            ShareText::PartWatching => "Zuschauen",
            ShareText::PartHandOver => "Mit seinem oder ihrem Controller spielen",
            ShareText::PartTogether => "Als zweite Person spielen",
            ShareText::AskHandOver => "Übernehmen anfragen",
            ShareText::AskTogether => "Zusammenspielen anfragen",
            ShareText::AskDetail => "Der Host entscheidet.",
            ShareText::WindowNote => "Das Bild öffnet sich in einem eigenen Fenster.",
            ShareText::LeaveSharePlay => "Verlassen",
            ShareText::DirectMs => "Direkt, %@ ms",
            ShareText::RelayMs => "Über ein Relay, %@ ms",
            ShareText::ToastInvited => "%@ lädt dich zu Share Play ein",
            ShareText::ToastAsked => "%@ möchte einen anderen Modus.",
            ShareText::ToastNowWatch => "Du kannst jetzt nur zuschauen.",
            ShareText::ToastNowHandOver => "Du spielst jetzt mit dem Controller des Hosts.",
            ShareText::ToastNowTogether => "Du bist jetzt die zweite Person im Spiel.",
            ShareText::ToastEnded => "Share Play ist beendet.",
            ShareText::ErrorNoGame => "Starte zuerst ein Spiel, dann teile es.",
            ShareText::ErrorNotInParty => "Share Play geht an jemanden aus deiner Party.",
            ShareText::ErrorBusy => "Du bist schon in einem Share.",
            ShareText::ErrorNoEncoder => "Dieser PC kann noch kein Spiel teilen.",
            ShareText::ErrorNoDecoder => "Dieser PC kann ein geteiltes Spiel noch nicht anzeigen.",
            ShareText::ErrorNotFound => "Diese Einladung gibt es nicht mehr.",
            ShareText::ErrorNotAllowed => "Diese Cloud bietet Share Play nicht an.",
            ShareText::SomeoneUnknown => "Jemand",
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

    const ONE_ARGUMENT: &[ShareText] = &[
        ShareText::ShareWith,
        ShareText::WaitingFor,
        ShareText::SeesYourGame,
        ShareText::AsksWatch,
        ShareText::AsksHandOver,
        ShareText::AsksTogether,
        ShareText::DirectMs,
        ShareText::RelayMs,
        ShareText::ToastInvited,
        ShareText::ToastAsked,
    ];
    const TWO_ARGUMENTS: &[ShareText] = &[
        ShareText::InvitedWatch,
        ShareText::InvitedHandOver,
        ShareText::InvitedTogether,
        ShareText::GuestOf,
    ];

    #[test]
    fn placeholders_match_between_languages() {
        let count = |s: &str| s.matches("%@").count();
        for text in ONE_ARGUMENT {
            for language in [Language::English, Language::Turkish, Language::German] {
                assert_eq!(count(text.text(language)), 1, "{text:?} {language:?}");
            }
        }
        for text in TWO_ARGUMENTS {
            for language in [Language::English, Language::Turkish, Language::German] {
                assert_eq!(count(text.text(language)), 2, "{text:?} {language:?}");
            }
        }
    }

    #[test]
    fn row_titles_are_in_title_case() {
        for title in [
            ShareText::SharePlay,
            ShareText::CancelInvitation,
            ShareText::StopSharePlay,
            ShareText::AskHandOver,
            ShareText::AskTogether,
        ] {
            let english = title.english();
            for word in english.split(' ').filter(|w| *w != "to") {
                assert!(
                    word.chars().next().is_some_and(char::is_uppercase),
                    "{english}"
                );
            }
        }
    }

    #[test]
    fn the_modes_have_three_distinct_short_labels() {
        let labels = [
            ShareText::ModeWatch,
            ShareText::ModeHandOver,
            ShareText::ModeTogether,
        ];
        for language in [Language::English, Language::Turkish, Language::German] {
            let texts: Vec<&str> = labels.iter().map(|l| l.text(language)).collect();
            assert!(texts[0] != texts[1] && texts[1] != texts[2] && texts[0] != texts[2]);
            assert!(texts.iter().all(|t| t.chars().count() <= 12), "{texts:?}");
        }
    }
}
