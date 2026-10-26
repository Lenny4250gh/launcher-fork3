//! Texts of party voice in the online area (docs/DESIGN.md 17, "Party voice"). English and
//! Turkish like the rest. The Turkish was written for this change and has not been read by a
//! native speaker yet. Row titles are in title case, notes and toasts in sentence case.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VoiceText {
    /// The page, the Home row and the caption on the party page.
    Voice,
    VoiceSubtitle,
    /// The toggle.
    PartyVoice,
    StatusOff,
    StatusNoParty,
    StatusConnecting,
    StatusOn,
    /// The service switched voice off: the cause, in a note.
    FailedGeneric,
    FailedDenied,
    /// The service's own reason; `%@` = what it said.
    FailedReason,
    // The microphone row
    Microphone,
    MicOff,
    /// `%@` = the key.
    MicMuted,
    MicOpen,
    MicListening,
    MicTalking,
    // Mode
    Mode,
    PushToTalk,
    VoiceActivity,
    /// Push to talk with the key (Windows reads it while a game is in front).
    PttNote,
    /// A game is running.
    PttInGameNote,
    /// Systems that cannot read a key behind a game: the pad button still works there.
    PttNoteOther,
    PttInGameNoteOther,
    ActivityNote,
    CallModeNote,
    // The key
    TalkKey,
    /// The detail while the key row waits for a key.
    TalkKeyNote,
    TalkButton,
    TalkButtonNone,
    // Noise
    NoiseReduction,
    NoiseNote,
    // Devices
    Speakers,
    SystemDefault,
    InUse,
    RefreshDevices,
    DevicesNote,
    NoMicrophone,
    NoSound,
    DevicesUnknown,
    /// `%@` = the kind of device the page lists.
    PickMicrophone,
    PickSpeakers,
    DeviceGone,
    // Members
    Volume,
    /// `%@` = the percent.
    VolumeValue,
    Mute,
    /// `%@` = who.
    MuteDetail,
    Speaking,
    MutedByYou,
    NotInVoice,
    VoiceSettings,
    // Toasts
    ToastFailed,
    ToastDenied,
    ToastNoDevice,
}

impl VoiceText {
    pub const fn english(self) -> &'static str {
        match self {
            VoiceText::Voice => "Voice",
            VoiceText::VoiceSubtitle => "Talk with your party",
            VoiceText::PartyVoice => "Party Voice",
            VoiceText::StatusOff => "Off",
            VoiceText::StatusNoParty => "Starts when you are in a party",
            VoiceText::StatusConnecting => "Connecting…",
            VoiceText::StatusOn => "On",
            VoiceText::FailedGeneric => {
                "Voice could not start. Check your microphone and speakers, then switch it on again."
            }
            VoiceText::FailedDenied => {
                "The system did not allow microphone access. Allow it in the system's privacy settings, then switch voice on again."
            }
            VoiceText::FailedReason => "Reason: %@",
            VoiceText::Microphone => "Microphone",
            VoiceText::MicOff => "Off",
            VoiceText::MicMuted => "Muted · hold %@ to talk",
            VoiceText::MicOpen => "Open",
            VoiceText::MicListening => "Listening",
            VoiceText::MicTalking => "Talking",
            VoiceText::Mode => "Mode",
            VoiceText::PushToTalk => "Push to Talk",
            VoiceText::VoiceActivity => "Open Mic",
            VoiceText::PttNote => {
                "Hold the talk key to speak. It also works while a game is in front."
            }
            VoiceText::PttInGameNote => {
                "Hold the talk key or the talk button to speak. Pick one that the game does not use."
            }
            VoiceText::PttNoteOther => {
                "Hold the talk key to speak while the launcher is in front. A talk button on your pad also works in a game."
            }
            VoiceText::PttInGameNoteOther => {
                "A game has the keyboard, so the key does not reach the launcher. Use a talk button on your pad, or switch to Open Mic."
            }
            VoiceText::ActivityNote => {
                "The microphone opens whenever you speak. Without headphones, others hear themselves back."
            }
            VoiceText::CallModeNote => {
                "On a call, the microphone opens whenever you speak. Push to Talk is back when the call ends."
            }
            VoiceText::TalkKey => "Talk Key",
            VoiceText::TalkKeyNote => "Press a key. Esc keeps the current one.",
            VoiceText::TalkButton => "Talk Button",
            VoiceText::TalkButtonNone => "None",
            VoiceText::NoiseReduction => "Reduce Background Noise",
            VoiceText::NoiseNote => {
                "Takes keyboard clicks, fans and other steady noise out of your microphone."
            }
            VoiceText::Speakers => "Speakers",
            VoiceText::SystemDefault => "System Default",
            VoiceText::InUse => "In use",
            VoiceText::RefreshDevices => "Refresh Devices",
            VoiceText::DevicesNote => "A change restarts voice for a moment.",
            VoiceText::NoMicrophone => {
                "No microphone found. Connect one or check the system's sound settings, then refresh."
            }
            VoiceText::NoSound => {
                "The net service reports no sound devices. Connect a microphone and speakers, or check the system's sound settings."
            }
            VoiceText::DevicesUnknown => "Devices appear once the net service answers.",
            VoiceText::PickMicrophone => "Choose the microphone",
            VoiceText::PickSpeakers => "Choose the speakers",
            VoiceText::DeviceGone => "This device is not connected right now.",
            VoiceText::Volume => "Volume",
            VoiceText::VolumeValue => "%@%",
            VoiceText::Mute => "Mute",
            VoiceText::MuteDetail => "Only you stop hearing %@",
            VoiceText::Speaking => "Speaking",
            VoiceText::MutedByYou => "Muted",
            VoiceText::NotInVoice => "Not in voice",
            VoiceText::VoiceSettings => "Voice Settings",
            VoiceText::ToastFailed => "Voice could not start. Check your microphone and speakers.",
            VoiceText::ToastDenied => "The system did not allow the microphone.",
            VoiceText::ToastNoDevice => "That device is not available. Refresh the list.",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            VoiceText::Voice => "Ses",
            VoiceText::VoiceSubtitle => "Partinle konuş",
            VoiceText::PartyVoice => "Parti Sesi",
            VoiceText::StatusOff => "Kapalı",
            VoiceText::StatusNoParty => "Bir partiye girince başlar",
            VoiceText::StatusConnecting => "Bağlanıyor…",
            VoiceText::StatusOn => "Açık",
            VoiceText::FailedGeneric => {
                "Ses başlatılamadı. Mikrofonunu ve hoparlörlerini kontrol et, sonra tekrar aç."
            }
            VoiceText::FailedDenied => {
                "Sistem mikrofon erişimine izin vermedi. Sistemin gizlilik ayarlarından izin ver, sonra sesi tekrar aç."
            }
            VoiceText::FailedReason => "Neden: %@",
            VoiceText::Microphone => "Mikrofon",
            VoiceText::MicOff => "Kapalı",
            VoiceText::MicMuted => "Sessiz · konuşmak için %@ tuşuna basılı tut",
            VoiceText::MicOpen => "Açık",
            VoiceText::MicListening => "Dinliyor",
            VoiceText::MicTalking => "Konuşuyor",
            VoiceText::Mode => "Mod",
            VoiceText::PushToTalk => "Bas ve Konuş",
            VoiceText::VoiceActivity => "Açık Mikrofon",
            VoiceText::PttNote => {
                "Konuşmak için konuşma tuşuna basılı tut. Oyun öndeyken de çalışır."
            }
            VoiceText::PttInGameNote => {
                "Konuşmak için konuşma tuşuna ya da konuşma düğmesine basılı tut. Oyunun kullanmadığı birini seç."
            }
            VoiceText::PttNoteOther => {
                "Başlatıcı öndeyken konuşmak için konuşma tuşuna basılı tut. Kumandandaki bir konuşma düğmesi oyunda da çalışır."
            }
            VoiceText::PttInGameNoteOther => {
                "Klavye oyunda olduğu için tuş başlatıcıya ulaşmaz. Kumandada bir konuşma düğmesi kullan ya da Açık Mikrofon yap."
            }
            VoiceText::ActivityNote => {
                "Mikrofon, konuştuğunda açılır. Kulaklık yoksa diğerleri kendi seslerini geri duyar."
            }
            VoiceText::CallModeNote => {
                "Aramada mikrofon, konuştuğunda açılır. Arama bitince Bas ve Konuş geri gelir."
            }
            VoiceText::TalkKey => "Konuşma Tuşu",
            VoiceText::TalkKeyNote => "Bir tuşa bas. Esc mevcut tuşu korur.",
            VoiceText::TalkButton => "Konuşma Düğmesi",
            VoiceText::TalkButtonNone => "Yok",
            VoiceText::NoiseReduction => "Arka Plan Gürültüsünü Azalt",
            VoiceText::NoiseNote => {
                "Klavye tıkırtılarını, fan sesini ve diğer sabit gürültüleri mikrofonundan temizler."
            }
            VoiceText::Speakers => "Hoparlör",
            VoiceText::SystemDefault => "Sistem Varsayılanı",
            VoiceText::InUse => "Kullanımda",
            VoiceText::RefreshDevices => "Cihazları Yenile",
            VoiceText::DevicesNote => "Bir değişiklik sesi kısa süre yeniden başlatır.",
            VoiceText::NoMicrophone => {
                "Mikrofon bulunamadı. Bir mikrofon tak ya da sistemin ses ayarlarına bak, sonra yenile."
            }
            VoiceText::NoSound => {
                "Ağ hizmeti hiç ses cihazı bildirmiyor. Mikrofon ve hoparlör tak ya da sistemin ses ayarlarına bak."
            }
            VoiceText::DevicesUnknown => "Cihazlar, ağ hizmeti yanıt verince görünür.",
            VoiceText::PickMicrophone => "Mikrofonu seç",
            VoiceText::PickSpeakers => "Hoparlörü seç",
            VoiceText::DeviceGone => "Bu cihaz şu an bağlı değil.",
            VoiceText::Volume => "Ses Düzeyi",
            VoiceText::VolumeValue => "%@%",
            VoiceText::Mute => "Sustur",
            VoiceText::MuteDetail => "Yalnızca sen %@ kişisini duymazsın",
            VoiceText::Speaking => "Konuşuyor",
            VoiceText::MutedByYou => "Susturuldu",
            VoiceText::NotInVoice => "Seste değil",
            VoiceText::VoiceSettings => "Ses Ayarları",
            VoiceText::ToastFailed => {
                "Ses başlatılamadı. Mikrofonunu ve hoparlörlerini kontrol et."
            }
            VoiceText::ToastDenied => "Sistem mikrofona izin vermedi.",
            VoiceText::ToastNoDevice => "Bu cihaz kullanılamıyor. Listeyi yenile.",
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            VoiceText::Voice => "Sprache",
            VoiceText::VoiceSubtitle => "Sprich mit deiner Party",
            VoiceText::PartyVoice => "Party-Sprachchat",
            VoiceText::StatusOff => "Aus",
            VoiceText::StatusNoParty => "Startet, sobald du in einer Party bist",
            VoiceText::StatusConnecting => "Verbindet…",
            VoiceText::StatusOn => "An",
            VoiceText::FailedGeneric => {
                "Der Sprachchat konnte nicht starten. Prüfe Mikrofon und Lautsprecher und schalte ihn dann wieder ein."
            }
            VoiceText::FailedDenied => {
                "Das System hat den Mikrofonzugriff nicht erlaubt. Erlaube ihn in den Datenschutzeinstellungen des Systems und schalte den Sprachchat dann wieder ein."
            }
            VoiceText::FailedReason => "Grund: %@",
            VoiceText::Microphone => "Mikrofon",
            VoiceText::MicOff => "Aus",
            VoiceText::MicMuted => "Stumm · zum Sprechen %@ halten",
            VoiceText::MicOpen => "Offen",
            VoiceText::MicListening => "Hört zu",
            VoiceText::MicTalking => "Spricht",
            VoiceText::Mode => "Modus",
            VoiceText::PushToTalk => "Push-to-Talk",
            VoiceText::VoiceActivity => "Offenes Mikrofon",
            VoiceText::PttNote => {
                "Halte die Sprechtaste zum Sprechen. Sie funktioniert auch, wenn ein Spiel vorne ist."
            }
            VoiceText::PttInGameNote => {
                "Halte die Sprechtaste oder die Controller-Taste zum Sprechen. Wähle eine, die das Spiel nicht nutzt."
            }
            VoiceText::PttNoteOther => {
                "Halte die Sprechtaste zum Sprechen, solange der Launcher vorne ist. Eine Sprechtaste am Controller funktioniert auch im Spiel."
            }
            VoiceText::PttInGameNoteOther => {
                "Ein Spiel hat die Tastatur, deshalb erreicht die Taste den Launcher nicht. Nutze eine Sprechtaste am Controller oder wechsle zu Offenes Mikrofon."
            }
            VoiceText::ActivityNote => {
                "Das Mikrofon öffnet sich, sobald du sprichst. Ohne Kopfhörer hören die anderen sich selbst."
            }
            VoiceText::CallModeNote => {
                "Im Anruf öffnet sich das Mikrofon, sobald du sprichst. Push-to-Talk ist nach dem Anruf wieder da."
            }
            VoiceText::TalkKey => "Sprechtaste",
            VoiceText::TalkKeyNote => "Drücke eine Taste. Esc behält die aktuelle.",
            VoiceText::TalkButton => "Sprechtaste am Controller",
            VoiceText::TalkButtonNone => "Keine",
            VoiceText::NoiseReduction => "Hintergrundgeräusche reduzieren",
            VoiceText::NoiseNote => {
                "Filtert Tastaturklicken, Lüfter und andere gleichmäßige Geräusche aus deinem Mikrofon."
            }
            VoiceText::Speakers => "Lautsprecher",
            VoiceText::SystemDefault => "Systemstandard",
            VoiceText::InUse => "In Verwendung",
            VoiceText::RefreshDevices => "Geräte aktualisieren",
            VoiceText::DevicesNote => "Eine Änderung startet den Sprachchat kurz neu.",
            VoiceText::NoMicrophone => {
                "Kein Mikrofon gefunden. Schließe eines an oder prüfe die Soundeinstellungen des Systems und aktualisiere dann."
            }
            VoiceText::NoSound => {
                "Der Netzdienst meldet keine Audiogeräte. Schließe Mikrofon und Lautsprecher an oder prüfe die Soundeinstellungen des Systems."
            }
            VoiceText::DevicesUnknown => "Geräte erscheinen, sobald der Netzdienst antwortet.",
            VoiceText::PickMicrophone => "Mikrofon wählen",
            VoiceText::PickSpeakers => "Lautsprecher wählen",
            VoiceText::DeviceGone => "Dieses Gerät ist gerade nicht verbunden.",
            VoiceText::Volume => "Lautstärke",
            VoiceText::VolumeValue => "%@%",
            VoiceText::Mute => "Stummschalten",
            VoiceText::MuteDetail => "Nur du hörst %@ nicht mehr",
            VoiceText::Speaking => "Spricht",
            VoiceText::MutedByYou => "Stumm",
            VoiceText::NotInVoice => "Nicht im Sprachchat",
            VoiceText::VoiceSettings => "Spracheinstellungen",
            VoiceText::ToastFailed => {
                "Der Sprachchat konnte nicht starten. Prüfe Mikrofon und Lautsprecher."
            }
            VoiceText::ToastDenied => "Das System hat das Mikrofon nicht erlaubt.",
            VoiceText::ToastNoDevice => "Dieses Gerät ist nicht verfügbar. Aktualisiere die Liste.",
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

    const WITH_ARGUMENTS: &[VoiceText] = &[
        VoiceText::FailedReason,
        VoiceText::MicMuted,
        VoiceText::VolumeValue,
        VoiceText::MuteDetail,
    ];

    #[test]
    fn placeholders_match_between_languages() {
        for text in WITH_ARGUMENTS {
            let count = |s: &str| s.matches("%@").count();
            assert_eq!(count(text.english()), 1, "{text:?}");
            assert_eq!(count(text.turkish()), 1, "{text:?}");
        }
    }

    #[test]
    fn row_titles_are_in_title_case() {
        for title in [
            VoiceText::PartyVoice,
            VoiceText::PushToTalk,
            VoiceText::VoiceActivity,
            VoiceText::TalkKey,
            VoiceText::TalkButton,
            VoiceText::NoiseReduction,
            VoiceText::SystemDefault,
            VoiceText::RefreshDevices,
            VoiceText::VoiceSettings,
        ] {
            for word in title.english().split(' ') {
                let first = word.chars().next().unwrap();
                assert!(first.is_uppercase() || word == "to", "{title:?}: {word}");
            }
        }
    }

    #[test]
    fn the_values_read_in_a_row() {
        assert_eq!(
            VoiceText::MicMuted.format(Language::English, &[Arg::Str("V")]),
            "Muted · hold V to talk"
        );
        assert_eq!(
            VoiceText::VolumeValue.format(Language::English, &[Arg::Int(150)]),
            "150%"
        );
    }
}
