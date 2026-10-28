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
    StatusFailed,
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
    // Echo
    EchoCancellation,
    EchoNote,
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
            VoiceText::StatusFailed => "Not working",
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
            VoiceText::EchoCancellation => "Echo Cancellation",
            VoiceText::EchoNote => {
                "For speakers. Turn off with a headset if your voice sounds cut off."
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
            VoiceText::StatusFailed => "Çalışmıyor",
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
            VoiceText::EchoCancellation => "Yankı Giderme",
            VoiceText::EchoNote => "Hoparlör için. Kulaklıkla sesin kesiliyorsa kapat.",
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
            VoiceText::StatusFailed => "Funktioniert nicht",
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
            VoiceText::EchoCancellation => "Echounterdrückung",
            VoiceText::EchoNote => {
                "Für Lautsprecher. Mit Headset ausschalten, wenn deine Stimme abgehackt klingt."
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

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            VoiceText::Voice => "Voz",
            VoiceText::VoiceSubtitle => "Habla con tu party",
            VoiceText::PartyVoice => "Voz de la party",
            VoiceText::StatusFailed => "No funciona",
            VoiceText::StatusOff => "Desactivada",
            VoiceText::StatusNoParty => "Se inicia cuando estás en una party",
            VoiceText::StatusConnecting => "Conectando…",
            VoiceText::StatusOn => "Activada",
            VoiceText::FailedGeneric => {
                "No se pudo iniciar la voz. Revisa el micrófono y los altavoces y vuelve a activarla."
            }
            VoiceText::FailedDenied => {
                "El sistema no permitió el acceso al micrófono. Permítelo en los ajustes de privacidad del sistema y vuelve a activar la voz."
            }
            VoiceText::FailedReason => "Motivo: %@",
            VoiceText::Microphone => "Micrófono",
            VoiceText::MicOff => "Desactivado",
            VoiceText::MicMuted => "Silenciado · mantén %@ para hablar",
            VoiceText::MicOpen => "Abierto",
            VoiceText::MicListening => "Escuchando",
            VoiceText::MicTalking => "Hablando",
            VoiceText::Mode => "Modo",
            VoiceText::PushToTalk => "Pulsar para hablar",
            VoiceText::VoiceActivity => "Micrófono abierto",
            VoiceText::PttNote => {
                "Mantén pulsada la tecla de hablar. También funciona con un juego en primer plano."
            }
            VoiceText::PttInGameNote => {
                "Mantén pulsada la tecla o el botón de hablar. Elige uno que el juego no use."
            }
            VoiceText::PttNoteOther => {
                "Mantén pulsada la tecla de hablar mientras el launcher está en primer plano. Un botón de hablar del mando también funciona en un juego."
            }
            VoiceText::PttInGameNoteOther => {
                "Un juego tiene el teclado, así que la tecla no llega al launcher. Usa un botón de hablar del mando o cambia a Micrófono abierto."
            }
            VoiceText::ActivityNote => {
                "El micrófono se abre cuando hablas. Sin auriculares, los demás se oyen a sí mismos."
            }
            VoiceText::CallModeNote => {
                "En una llamada, el micrófono se abre cuando hablas. Pulsar para hablar vuelve al terminar la llamada."
            }
            VoiceText::TalkKey => "Tecla de hablar",
            VoiceText::TalkKeyNote => "Pulsa una tecla. Esc mantiene la actual.",
            VoiceText::TalkButton => "Botón de hablar",
            VoiceText::TalkButtonNone => "Ninguno",
            VoiceText::NoiseReduction => "Reducir ruido de fondo",
            VoiceText::NoiseNote => {
                "Quita de tu micrófono los clics del teclado, los ventiladores y otros ruidos constantes."
            }
            VoiceText::EchoCancellation => "Cancelación de eco",
            VoiceText::EchoNote => {
                "Para altavoces. Desactívala con auriculares si tu voz suena cortada."
            }
            VoiceText::Speakers => "Altavoces",
            VoiceText::SystemDefault => "Predeterminado del sistema",
            VoiceText::InUse => "En uso",
            VoiceText::RefreshDevices => "Actualizar dispositivos",
            VoiceText::DevicesNote => "Un cambio reinicia la voz un momento.",
            VoiceText::NoMicrophone => {
                "No se encontró ningún micrófono. Conecta uno o revisa los ajustes de sonido del sistema y actualiza."
            }
            VoiceText::NoSound => {
                "El servicio de red no informa de ningún dispositivo de sonido. Conecta un micrófono y altavoces, o revisa los ajustes de sonido del sistema."
            }
            VoiceText::DevicesUnknown => {
                "Los dispositivos aparecen cuando el servicio de red responde."
            }
            VoiceText::PickMicrophone => "Elegir micrófono",
            VoiceText::PickSpeakers => "Elegir altavoces",
            VoiceText::DeviceGone => "Este dispositivo no está conectado ahora.",
            VoiceText::Volume => "Volumen",
            VoiceText::VolumeValue => "%@\u{a0}%",
            VoiceText::Mute => "Silenciar",
            VoiceText::MuteDetail => "Solo tú dejas de oír a %@",
            VoiceText::Speaking => "Hablando",
            VoiceText::MutedByYou => "Silenciado",
            VoiceText::NotInVoice => "No está en la voz",
            VoiceText::VoiceSettings => "Ajustes de voz",
            VoiceText::ToastFailed => {
                "No se pudo iniciar la voz. Revisa el micrófono y los altavoces."
            }
            VoiceText::ToastDenied => "El sistema no permitió el micrófono.",
            VoiceText::ToastNoDevice => "Ese dispositivo no está disponible. Actualiza la lista.",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            VoiceText::Voice => "Voix",
            VoiceText::VoiceSubtitle => "Parle avec ta party",
            VoiceText::PartyVoice => "Voix de la party",
            VoiceText::StatusFailed => "Ne fonctionne pas",
            VoiceText::StatusOff => "Désactivée",
            VoiceText::StatusNoParty => "Démarre quand tu es dans une party",
            VoiceText::StatusConnecting => "Connexion…",
            VoiceText::StatusOn => "Activée",
            VoiceText::FailedGeneric => {
                "Impossible de démarrer la voix. Vérifie ton micro et tes haut-parleurs, puis réactive-la."
            }
            VoiceText::FailedDenied => {
                "Le système n’a pas autorisé l’accès au micro. Autorise-le dans les paramètres de confidentialité du système, puis réactive la voix."
            }
            VoiceText::FailedReason => "Raison\u{a0}: %@",
            VoiceText::Microphone => "Micro",
            VoiceText::MicOff => "Désactivé",
            VoiceText::MicMuted => "En sourdine · maintiens %@ pour parler",
            VoiceText::MicOpen => "Ouvert",
            VoiceText::MicListening => "À l’écoute",
            VoiceText::MicTalking => "Parle",
            VoiceText::Mode => "Mode",
            VoiceText::PushToTalk => "Appuyer pour parler",
            VoiceText::VoiceActivity => "Micro ouvert",
            VoiceText::PttNote => {
                "Maintiens la touche pour parler. Elle fonctionne aussi quand un jeu est au premier plan."
            }
            VoiceText::PttInGameNote => {
                "Maintiens la touche ou le bouton pour parler. Choisis-en un que le jeu n’utilise pas."
            }
            VoiceText::PttNoteOther => {
                "Maintiens la touche pour parler tant que le launcher est au premier plan. Un bouton pour parler sur la manette fonctionne aussi dans un jeu."
            }
            VoiceText::PttInGameNoteOther => {
                "Un jeu a le clavier, donc la touche n’arrive pas au launcher. Utilise un bouton pour parler sur la manette ou passe à Micro ouvert."
            }
            VoiceText::ActivityNote => {
                "Le micro s’ouvre dès que tu parles. Sans casque, les autres s’entendent en retour."
            }
            VoiceText::CallModeNote => {
                "Pendant un appel, le micro s’ouvre dès que tu parles. Appuyer pour parler revient à la fin de l’appel."
            }
            VoiceText::TalkKey => "Touche pour parler",
            VoiceText::TalkKeyNote => "Appuie sur une touche. Échap garde l’actuelle.",
            VoiceText::TalkButton => "Bouton pour parler",
            VoiceText::TalkButtonNone => "Aucun",
            VoiceText::NoiseReduction => "Réduire le bruit de fond",
            VoiceText::NoiseNote => {
                "Retire de ton micro les clics de clavier, les ventilateurs et autres bruits constants."
            }
            VoiceText::EchoCancellation => "Annulation d'écho",
            VoiceText::EchoNote => {
                "Pour les haut-parleurs. Désactive-la avec un casque si ta voix est coupée."
            }
            VoiceText::Speakers => "Haut-parleurs",
            VoiceText::SystemDefault => "Par défaut du système",
            VoiceText::InUse => "Utilisé",
            VoiceText::RefreshDevices => "Actualiser les appareils",
            VoiceText::DevicesNote => "Un changement redémarre la voix un instant.",
            VoiceText::NoMicrophone => {
                "Aucun micro trouvé. Branche-en un ou vérifie les paramètres de son du système, puis actualise."
            }
            VoiceText::NoSound => {
                "Le service réseau ne signale aucun appareil audio. Branche un micro et des haut-parleurs, ou vérifie les paramètres de son du système."
            }
            VoiceText::DevicesUnknown => {
                "Les appareils apparaissent quand le service réseau répond."
            }
            VoiceText::PickMicrophone => "Choisir le micro",
            VoiceText::PickSpeakers => "Choisir les haut-parleurs",
            VoiceText::DeviceGone => "Cet appareil n’est pas connecté pour le moment.",
            VoiceText::Volume => "Volume",
            VoiceText::VolumeValue => "%@\u{a0}%",
            VoiceText::Mute => "Mettre en sourdine",
            VoiceText::MuteDetail => "Toi seul n’entends plus %@",
            VoiceText::Speaking => "Parle",
            VoiceText::MutedByYou => "En sourdine",
            VoiceText::NotInVoice => "Pas en vocal",
            VoiceText::VoiceSettings => "Paramètres vocaux",
            VoiceText::ToastFailed => {
                "Impossible de démarrer la voix. Vérifie ton micro et tes haut-parleurs."
            }
            VoiceText::ToastDenied => "Le système n’a pas autorisé le micro.",
            VoiceText::ToastNoDevice => "Cet appareil n’est pas disponible. Actualise la liste.",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            VoiceText::Voice => "Voz",
            VoiceText::VoiceSubtitle => "Fale com sua party",
            VoiceText::PartyVoice => "Voz da party",
            VoiceText::StatusFailed => "Não funciona",
            VoiceText::StatusOff => "Desativada",
            VoiceText::StatusNoParty => "Começa quando você está em uma party",
            VoiceText::StatusConnecting => "Conectando…",
            VoiceText::StatusOn => "Ativada",
            VoiceText::FailedGeneric => {
                "Não foi possível iniciar a voz. Verifique o microfone e os alto-falantes e ative-a de novo."
            }
            VoiceText::FailedDenied => {
                "O sistema não permitiu o acesso ao microfone. Permita nas configurações de privacidade do sistema e ative a voz de novo."
            }
            VoiceText::FailedReason => "Motivo: %@",
            VoiceText::Microphone => "Microfone",
            VoiceText::MicOff => "Desativado",
            VoiceText::MicMuted => "Silenciado · segure %@ para falar",
            VoiceText::MicOpen => "Aberto",
            VoiceText::MicListening => "Ouvindo",
            VoiceText::MicTalking => "Falando",
            VoiceText::Mode => "Modo",
            VoiceText::PushToTalk => "Apertar para falar",
            VoiceText::VoiceActivity => "Microfone aberto",
            VoiceText::PttNote => {
                "Segure a tecla de falar. Ela também funciona com um jogo em primeiro plano."
            }
            VoiceText::PttInGameNote => {
                "Segure a tecla ou o botão de falar. Escolha um que o jogo não use."
            }
            VoiceText::PttNoteOther => {
                "Segure a tecla de falar enquanto o launcher está em primeiro plano. Um botão de falar no controle também funciona em um jogo."
            }
            VoiceText::PttInGameNoteOther => {
                "Um jogo está com o teclado, então a tecla não chega ao launcher. Use um botão de falar no controle ou mude para Microfone aberto."
            }
            VoiceText::ActivityNote => {
                "O microfone abre quando você fala. Sem fones de ouvido, os outros ouvem a si mesmos."
            }
            VoiceText::CallModeNote => {
                "Em uma chamada, o microfone abre quando você fala. Apertar para falar volta quando a chamada termina."
            }
            VoiceText::TalkKey => "Tecla de falar",
            VoiceText::TalkKeyNote => "Aperte uma tecla. Esc mantém a atual.",
            VoiceText::TalkButton => "Botão de falar",
            VoiceText::TalkButtonNone => "Nenhum",
            VoiceText::NoiseReduction => "Reduzir ruído de fundo",
            VoiceText::NoiseNote => {
                "Tira do seu microfone cliques de teclado, ventoinhas e outros ruídos constantes."
            }
            VoiceText::EchoCancellation => "Cancelamento de eco",
            VoiceText::EchoNote => {
                "Para alto-falantes. Desative com fone de ouvido se a sua voz soar cortada."
            }
            VoiceText::Speakers => "Alto-falantes",
            VoiceText::SystemDefault => "Padrão do sistema",
            VoiceText::InUse => "Em uso",
            VoiceText::RefreshDevices => "Atualizar dispositivos",
            VoiceText::DevicesNote => "Uma mudança reinicia a voz por um momento.",
            VoiceText::NoMicrophone => {
                "Nenhum microfone encontrado. Conecte um ou verifique as configurações de som do sistema e atualize."
            }
            VoiceText::NoSound => {
                "O serviço de rede não informa nenhum dispositivo de som. Conecte um microfone e alto-falantes ou verifique as configurações de som do sistema."
            }
            VoiceText::DevicesUnknown => {
                "Os dispositivos aparecem quando o serviço de rede responde."
            }
            VoiceText::PickMicrophone => "Escolher microfone",
            VoiceText::PickSpeakers => "Escolher alto-falantes",
            VoiceText::DeviceGone => "Este dispositivo não está conectado agora.",
            VoiceText::Volume => "Volume",
            VoiceText::VolumeValue => "%@%",
            VoiceText::Mute => "Silenciar",
            VoiceText::MuteDetail => "Só você deixa de ouvir %@",
            VoiceText::Speaking => "Falando",
            VoiceText::MutedByYou => "Silenciado",
            VoiceText::NotInVoice => "Fora da voz",
            VoiceText::VoiceSettings => "Configurações de voz",
            VoiceText::ToastFailed => {
                "Não foi possível iniciar a voz. Verifique o microfone e os alto-falantes."
            }
            VoiceText::ToastDenied => "O sistema não permitiu o microfone.",
            VoiceText::ToastNoDevice => "Esse dispositivo não está disponível. Atualize a lista.",
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
            for language in Language::ALL {
                assert_eq!(count(text.text(language)), 1, "{text:?} {language:?}");
            }
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
            VoiceText::EchoCancellation,
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
