//! Texts of Phone as Controller (docs/DESIGN.md 18). English and Turkish like the rest.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PhoneText {
    /// The settings category and the switch.
    Title,
    SwitchDetail,
    /// Status row: title, and its values (`%@` = the phone's name).
    Status,
    Waiting,
    ConnectedTo,
    ConnectingTo,
    StartFailed,
    /// Delay of the pad, `%d` milliseconds, said after the status.
    DelayValue,
    /// The address to type into the app, and why it is shown.
    Address,
    AddressNote,
    PortBusy,
    NotAnnounced,
    /// A phone asks to pair (`%@` = its name): the question, the code line, the two answers.
    PairAsk,
    Code,
    CodeNote,
    PairAllow,
    PairDeny,
    /// The paired phones. `Remove`: `%@` = the name; `PairedOn`: `%@` = the date.
    Paired,
    PairedNone,
    Remove,
    PairedOn,
    /// The virtual controller.
    Controller,
    ControllerReady,
    ControllerMissing,
    ControllerNoAccess,
    ControllerUnsupported,
    ViGemNote,
    ViGemOpen,
    UinputNote,
    UinputRule,
    UinputOpen,
    /// Toasts. `%@` = the phone's name.
    ToastPair,
    ToastConnected,
    ToastFull,
    ToastRemoved,
    /// Top bar: the accessible name of the status, `%@` = the phone's name.
    BarLabel,
}

impl PhoneText {
    pub const fn english(self) -> &'static str {
        match self {
            PhoneText::Title => "Phone as Controller",
            PhoneText::SwitchDetail => {
                "Use your phone as a controller. Only phones on your home network can connect, and each one has to be paired first."
            }
            PhoneText::Status => "Status",
            PhoneText::Waiting => "Waiting for a phone",
            PhoneText::ConnectedTo => "Connected to %@",
            PhoneText::ConnectingTo => "Connecting to %@",
            PhoneText::StartFailed => {
                "Could not start. Another program may be using the network port."
            }
            PhoneText::DelayValue => "%d ms",
            PhoneText::Address => "Address",
            PhoneText::AddressNote => {
                "If the app does not find this PC by itself, type this address into it."
            }
            PhoneText::PortBusy => {
                "Port 47700 is in use, so the app cannot find this PC by its address until it is free."
            }
            PhoneText::NotAnnounced => {
                "This PC could not announce itself on the network. Type the address into the app."
            }
            PhoneText::PairAsk => "Pair %@?",
            PhoneText::Code => "Code",
            PhoneText::CodeNote => "Pair only if your phone shows the same code.",
            PhoneText::PairAllow => "Pair Phone",
            PhoneText::PairDeny => "Deny",
            PhoneText::Paired => "Paired Phones",
            PhoneText::PairedNone => {
                "No phone is paired yet. Open the phone app and choose this PC."
            }
            PhoneText::Remove => "Remove %@",
            PhoneText::PairedOn => "Paired on %@",
            PhoneText::Controller => "Virtual Controller",
            PhoneText::ControllerReady => "Ready",
            PhoneText::ControllerMissing => "Not installed",
            PhoneText::ControllerNoAccess => "No access",
            PhoneText::ControllerUnsupported => "Not available",
            PhoneText::ViGemNote => {
                "Games see your phone as a controller through a small driver, ViGEmBus. The launcher never installs it for you: download it from its official page and install it yourself. Its author has retired the project, and it still works on current Windows."
            }
            PhoneText::ViGemOpen => "Open the ViGEmBus Download Page",
            PhoneText::UinputNote => {
                "Games see your phone as a controller through uinput. The launcher cannot open it yet. Load the module with modprobe uinput, and give your user write access to /dev/uinput."
            }
            PhoneText::UinputRule => {
                "A udev rule does it for good: KERNEL==\"uinput\", GROUP=\"input\", MODE=\"0660\". Add yourself to the input group, then sign in again."
            }
            PhoneText::UinputOpen => "Open the Guide",
            PhoneText::ToastPair => "%@ wants to pair. Open Settings, Phone as Controller.",
            PhoneText::ToastConnected => "%@ connected",
            PhoneText::ToastFull => "Eight phones are paired. Remove one before pairing another.",
            PhoneText::ToastRemoved => "%@ removed",
            PhoneText::BarLabel => "Phone connected: %@",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            PhoneText::Title => "Telefonu Kumanda Yap",
            PhoneText::SwitchDetail => {
                "Telefonunu kumanda olarak kullan. Yalnızca ev ağındaki telefonlar bağlanabilir ve her biri önce eşleştirilmelidir."
            }
            PhoneText::Status => "Durum",
            PhoneText::Waiting => "Bir telefon bekleniyor",
            PhoneText::ConnectedTo => "%@ bağlı",
            PhoneText::ConnectingTo => "%@ bağlanıyor",
            PhoneText::StartFailed => {
                "Başlatılamadı. Ağ bağlantı noktasını başka bir program kullanıyor olabilir."
            }
            PhoneText::DelayValue => "%d ms",
            PhoneText::Address => "Adres",
            PhoneText::AddressNote => {
                "Uygulama bu bilgisayarı kendiliğinden bulamazsa bu adresi uygulamaya yaz."
            }
            PhoneText::PortBusy => {
                "47700 numaralı bağlantı noktası kullanımda; boşalana kadar uygulama bu bilgisayarı adresle bulamaz."
            }
            PhoneText::NotAnnounced => {
                "Bu bilgisayar kendini ağda duyuramadı. Adresi uygulamaya yaz."
            }
            PhoneText::PairAsk => "%@ eşleştirilsin mi?",
            PhoneText::Code => "Kod",
            PhoneText::CodeNote => "Yalnızca telefonun aynı kodu gösteriyorsa eşleştir.",
            PhoneText::PairAllow => "Telefonu Eşleştir",
            PhoneText::PairDeny => "Reddet",
            PhoneText::Paired => "Eşleştirilmiş Telefonlar",
            PhoneText::PairedNone => {
                "Henüz eşleştirilmiş telefon yok. Telefon uygulamasını açıp bu bilgisayarı seç."
            }
            PhoneText::Remove => "%@ Kaldır",
            PhoneText::PairedOn => "%@ tarihinde eşleştirildi",
            PhoneText::Controller => "Sanal Kumanda",
            PhoneText::ControllerReady => "Hazır",
            PhoneText::ControllerMissing => "Kurulu değil",
            PhoneText::ControllerNoAccess => "Erişim yok",
            PhoneText::ControllerUnsupported => "Kullanılamıyor",
            PhoneText::ViGemNote => {
                "Oyunlar telefonunu küçük bir sürücü olan ViGEmBus üzerinden kumanda olarak görür. Başlatıcı bunu senin yerine asla kurmaz: resmi sayfasından indirip kendin kur. Yazarı projeyi bırakmış, güncel Windows'ta yine de çalışıyor."
            }
            PhoneText::ViGemOpen => "ViGEmBus İndirme Sayfasını Aç",
            PhoneText::UinputNote => {
                "Oyunlar telefonunu uinput üzerinden kumanda olarak görür. Başlatıcı henüz onu açamıyor. Modülü modprobe uinput ile yükle ve kullanıcına /dev/uinput için yazma izni ver."
            }
            PhoneText::UinputRule => {
                "Kalıcı çözüm bir udev kuralı: KERNEL==\"uinput\", GROUP=\"input\", MODE=\"0660\". Kendini input grubuna ekle, sonra oturumu yeniden aç."
            }
            PhoneText::UinputOpen => "Kılavuzu Aç",
            PhoneText::ToastPair => "%@ eşleşmek istiyor. Ayarlar'da Telefonu Kumanda Yap'ı aç.",
            PhoneText::ToastConnected => "%@ bağlandı",
            PhoneText::ToastFull => {
                "Sekiz telefon eşleşmiş. Başka birini eşleştirmeden önce birini kaldır."
            }
            PhoneText::ToastRemoved => "%@ kaldırıldı",
            PhoneText::BarLabel => "Telefon bağlı: %@",
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            PhoneText::Title => "Telefon als Controller",
            PhoneText::SwitchDetail => {
                "Nutze dein Telefon als Controller. Nur Telefone in deinem Heimnetz können sich verbinden, und jedes muss zuerst gekoppelt werden."
            }
            PhoneText::Status => "Status",
            PhoneText::Waiting => "Warten auf ein Telefon",
            PhoneText::ConnectedTo => "Verbunden mit %@",
            PhoneText::ConnectingTo => "Verbindung mit %@ wird aufgebaut",
            PhoneText::StartFailed => {
                "Start nicht möglich. Möglicherweise nutzt ein anderes Programm den Netzwerkport."
            }
            PhoneText::DelayValue => "%d ms",
            PhoneText::Address => "Adresse",
            PhoneText::AddressNote => {
                "Findet die App diesen PC nicht von selbst, gib diese Adresse dort ein."
            }
            PhoneText::PortBusy => {
                "Port 47700 ist belegt, daher findet die App diesen PC erst über seine Adresse, wenn er frei ist."
            }
            PhoneText::NotAnnounced => {
                "Dieser PC konnte sich im Netzwerk nicht bekannt machen. Gib die Adresse in der App ein."
            }
            PhoneText::PairAsk => "%@ koppeln?",
            PhoneText::Code => "Code",
            PhoneText::CodeNote => "Koppeln nur, wenn dein Telefon denselben Code zeigt.",
            PhoneText::PairAllow => "Telefon koppeln",
            PhoneText::PairDeny => "Ablehnen",
            PhoneText::Paired => "Gekoppelte Telefone",
            PhoneText::PairedNone => {
                "Noch kein Telefon gekoppelt. Öffne die Telefon-App und wähle diesen PC."
            }
            PhoneText::Remove => "%@ entfernen",
            PhoneText::PairedOn => "Gekoppelt am %@",
            PhoneText::Controller => "Virtueller Controller",
            PhoneText::ControllerReady => "Bereit",
            PhoneText::ControllerMissing => "Nicht installiert",
            PhoneText::ControllerNoAccess => "Kein Zugriff",
            PhoneText::ControllerUnsupported => "Nicht verfügbar",
            PhoneText::ViGemNote => {
                "Spiele erkennen dein Telefon über einen kleinen Treiber, ViGEmBus, als Controller. Der Launcher installiert ihn nie für dich: Lade ihn von der offiziellen Seite herunter und installiere ihn selbst. Der Autor hat das Projekt eingestellt, es funktioniert aber weiterhin unter aktuellem Windows."
            }
            PhoneText::ViGemOpen => "ViGEmBus-Downloadseite öffnen",
            PhoneText::UinputNote => {
                "Spiele erkennen dein Telefon über uinput als Controller. Der Launcher kann es noch nicht öffnen. Lade das Modul mit modprobe uinput und gib deinem Benutzer Schreibzugriff auf /dev/uinput."
            }
            PhoneText::UinputRule => {
                "Eine udev-Regel erledigt das dauerhaft: KERNEL==\"uinput\", GROUP=\"input\", MODE=\"0660\". Füge dich der Gruppe input hinzu und melde dich dann neu an."
            }
            PhoneText::UinputOpen => "Anleitung öffnen",
            PhoneText::ToastPair => {
                "%@ möchte sich koppeln. Öffne Einstellungen, Telefon als Controller."
            }
            PhoneText::ToastConnected => "%@ verbunden",
            PhoneText::ToastFull => {
                "Acht Telefone sind gekoppelt. Entferne eines, bevor du ein weiteres koppelst."
            }
            PhoneText::ToastRemoved => "%@ entfernt",
            PhoneText::BarLabel => "Telefon verbunden: %@",
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
