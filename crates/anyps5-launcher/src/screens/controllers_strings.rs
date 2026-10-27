//! Texts of the Controllers page (docs/DESIGN.md 6.10): the pads, the tests, the launcher's own
//! controls and the troubleshooter. English and Turkish like the rest; captions in title case,
//! answers in plain sentences.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Pads {
    /// The row in Settings, Behaviour.
    Entry,
    EntryDetail,
    /// The page title and its subtitle.
    Title,
    Subtitle,
    CaptionConnected,
    CaptionTest,
    CaptionControls,
    CaptionHelp,
    NoPad,
    NoPadDetail,
    /// `%@` = the link, `%@` = battery, `%@` = backend.
    PadDetail,
    InUse,
    Usb,
    Bluetooth,
    LinkUnknown,
    /// `%@` = percent.
    Battery,
    BatteryCharging,
    BatteryNone,
    TestButtons,
    TestButtonsDetail,
    TestVibration,
    TestVibrationDetail,
    NotWhileGame,
    NoRumble,
    VibrationSent,
    DeadZone,
    DeadZoneDetail,
    LightBar,
    LightBarDetail,
    LightBarSet,
    LightBarFailed,
    /// Colour names, in the order of `ui::lightbar::COLOURS`.
    ColourBlue,
    ColourRed,
    ColourGreen,
    ColourPurple,
    ColourOrange,
    ColourWhite,
    ColourOff,
    /// The entry after the presets, and the two sliders it opens.
    ColourCustom,
    LightBarHue,
    LightBarBrightness,
    /// `%@` = the button that confirms.
    ConfirmWith,
    ConfirmWithDetail,
    ControlsNote,
    Select,
    Back,
    Options,
    Previous,
    Next,
    ReleaseToCapture,
    Reset,
    ResetDetail,
    /// Toast: `%@` button, `%@` job it had, `%@` job it gave, `%@` button that went there.
    Swapped,
    /// Toast: `%@` button, `%@` job.
    SwappedNoReturn,
    /// Toast: the button has a fixed job.
    FixedButton,
    /// The test page.
    TestTitle,
    /// `%@` = the button to hold.
    TestLeave,
    TestWaiting,
    TestNoPad,
    /// The troubleshooter.
    StatusOk,
    StatusCheck,
    HelpMissing,
    HelpMissingFound,
    HelpMissingNone,
    HelpDouble,
    HelpSteam,
    HelpHide,
    HelpHome,
    HelpHomeLinux,
    HelpLinux,
    HelpBattery,
    /// `%@` = percent.
    LowBattery,
    HelpMissingTitle,
    HelpDoubleTitle,
    HelpSteamTitle,
    HelpHideTitle,
    HelpHomeTitle,
    HelpLinuxTitle,
    HelpBatteryTitle,
}

impl Pads {
    pub const ALL: &'static [Pads] = &[
        Pads::Entry,
        Pads::EntryDetail,
        Pads::Title,
        Pads::Subtitle,
        Pads::CaptionConnected,
        Pads::CaptionTest,
        Pads::CaptionControls,
        Pads::CaptionHelp,
        Pads::NoPad,
        Pads::NoPadDetail,
        Pads::PadDetail,
        Pads::InUse,
        Pads::Usb,
        Pads::Bluetooth,
        Pads::LinkUnknown,
        Pads::Battery,
        Pads::BatteryCharging,
        Pads::BatteryNone,
        Pads::TestButtons,
        Pads::TestButtonsDetail,
        Pads::TestVibration,
        Pads::TestVibrationDetail,
        Pads::NotWhileGame,
        Pads::NoRumble,
        Pads::VibrationSent,
        Pads::DeadZone,
        Pads::DeadZoneDetail,
        Pads::LightBar,
        Pads::LightBarDetail,
        Pads::LightBarSet,
        Pads::LightBarFailed,
        Pads::ColourBlue,
        Pads::ColourRed,
        Pads::ColourGreen,
        Pads::ColourPurple,
        Pads::ColourOrange,
        Pads::ColourWhite,
        Pads::ColourOff,
        Pads::ColourCustom,
        Pads::LightBarHue,
        Pads::LightBarBrightness,
        Pads::ConfirmWith,
        Pads::ConfirmWithDetail,
        Pads::ControlsNote,
        Pads::Select,
        Pads::Back,
        Pads::Options,
        Pads::Previous,
        Pads::Next,
        Pads::ReleaseToCapture,
        Pads::Reset,
        Pads::ResetDetail,
        Pads::Swapped,
        Pads::SwappedNoReturn,
        Pads::FixedButton,
        Pads::TestTitle,
        Pads::TestLeave,
        Pads::TestWaiting,
        Pads::TestNoPad,
        Pads::StatusOk,
        Pads::StatusCheck,
        Pads::HelpMissing,
        Pads::HelpMissingFound,
        Pads::HelpMissingNone,
        Pads::HelpDouble,
        Pads::HelpSteam,
        Pads::HelpHide,
        Pads::HelpHome,
        Pads::HelpHomeLinux,
        Pads::HelpLinux,
        Pads::HelpBattery,
        Pads::LowBattery,
        Pads::HelpMissingTitle,
        Pads::HelpDoubleTitle,
        Pads::HelpSteamTitle,
        Pads::HelpHideTitle,
        Pads::HelpHomeTitle,
        Pads::HelpLinuxTitle,
        Pads::HelpBatteryTitle,
    ];

    pub const fn english(self) -> &'static str {
        match self {
            Pads::Entry => "Controllers",
            Pads::EntryDetail => {
                "Test buttons, vibration and the light bar, change the launcher's buttons, fix problems"
            }
            Pads::Title => "Controllers",
            Pads::Subtitle => {
                "Everything about the pads on this PC. Games keep their own controls."
            }
            Pads::CaptionConnected => "Connected",
            Pads::CaptionTest => "Test and Tune",
            Pads::CaptionControls => "Launcher Controls",
            Pads::CaptionHelp => "Troubleshooter",
            Pads::NoPad => "No controller found",
            Pads::NoPadDetail => "Connect a pad and press a button on it.",
            Pads::PadDetail => "%@ · %@ · %@",
            Pads::InUse => "In use",
            Pads::Usb => "USB",
            Pads::Bluetooth => "Bluetooth",
            Pads::LinkUnknown => "Connection unknown",
            Pads::Battery => "Battery %@%",
            Pads::BatteryCharging => "Charging %@%",
            Pads::BatteryNone => "No battery reading",
            Pads::TestButtons => "Test Buttons",
            Pads::TestButtonsDetail => "Press buttons and move the sticks; they light up",
            Pads::TestVibration => "Test Vibration",
            Pads::TestVibrationDetail => "A short buzz, light motor first",
            Pads::NotWhileGame => "Not while a game is running.",
            Pads::NoRumble => "No connected pad can vibrate.",
            Pads::VibrationSent => "Vibrating",
            Pads::DeadZone => "Stick Dead Zone",
            Pads::DeadZoneDetail => {
                "How far the left stick must lean before the launcher moves. Games have their own."
            }
            Pads::LightBar => "Light Bar Colour",
            Pads::LightBarDetail => {
                "DualSense only. A game takes the light bar back while it runs."
            }
            Pads::LightBarSet => "Light bar changed",
            Pads::LightBarFailed => "Could not reach the pad's light bar.",
            Pads::ColourBlue => "Blue",
            Pads::ColourRed => "Red",
            Pads::ColourGreen => "Green",
            Pads::ColourPurple => "Purple",
            Pads::ColourOrange => "Orange",
            Pads::ColourWhite => "White",
            Pads::ColourOff => "Off",
            Pads::ColourCustom => "Custom",
            Pads::LightBarHue => "Hue",
            Pads::LightBarBrightness => "Brightness",
            Pads::ConfirmWith => "Confirm with %@",
            Pads::ConfirmWithDetail => {
                "Swaps Select and Back on the pad, as some players are used to"
            }
            Pads::ControlsNote => {
                "Choose a job, then press the button or key. A button that is already used swaps places."
            }
            Pads::Select => "Select",
            Pads::Back => "Back",
            Pads::Options => "Options",
            Pads::Previous => "Previous Section",
            Pads::Next => "Next Section",
            Pads::ReleaseToCapture => "Press a button or a key",
            Pads::Reset => "Reset to Default",
            Pads::ResetDetail => "Puts every launcher button and key back",
            Pads::Swapped => "%@ was %@. %@ now uses %@.",
            Pads::SwappedNoReturn => "%@ moved from %@",
            Pads::FixedButton => "That button always does the same thing and cannot be changed.",
            Pads::TestTitle => "Test Buttons",
            Pads::TestLeave => "Hold %@ to leave, or press Esc",
            Pads::TestWaiting => "Press any button",
            Pads::TestNoPad => "No controller found. Connect one and press a button.",
            Pads::StatusOk => "OK",
            Pads::StatusCheck => "Check",
            Pads::HelpMissingTitle => "Controller Not Found",
            Pads::HelpMissing => {
                "Plug the pad in or pair it, then press a button. Try another cable or port: some cables only charge. A pad connected after a game started is seen in the launcher but not by that game."
            }
            Pads::HelpMissingFound => {
                "Seen. If a game does not see it, start the game after the pad is connected."
            }
            Pads::HelpMissingNone => {
                "No pad is connected to the launcher. Plug it in or pair it, then press a button. Try another cable or port: some cables only charge."
            }
            Pads::HelpDoubleTitle => "Every Press Counts Twice",
            Pads::HelpDouble => {
                "Steam Input or DS4Windows adds a second, virtual pad next to the real one. The launcher ignores the echo, a game may not. Close the tool, or turn its controller emulation off for the launcher and the game."
            }
            Pads::HelpSteamTitle => "Steam and PlayStation Pads",
            Pads::HelpSteam => {
                "With PlayStation Configuration Support on, Steam takes DualSense and DualShock pads over and passes on a virtual Xbox pad. Turn it off in Steam, Settings, Controller, or set Steam Input to off for the launcher and the game."
            }
            Pads::HelpHideTitle => "DS4Windows and HidHide",
            Pads::HelpHide => {
                "DS4Windows and HidHide hide the real pad from other programs on purpose. If the launcher or a game sees no pad, add it to the allowed applications in HidHide, or turn off Hide DS4 Controller in DS4Windows."
            }
            Pads::HelpHomeTitle => "PS Button Does Nothing",
            Pads::HelpHome => {
                "Check Settings, Behaviour, Home Button Opens the Launcher. It works with a DualSense or DualShock 4 on USB or Bluetooth. Another tool that holds the pad exclusively can block it."
            }
            Pads::HelpHomeLinux => {
                "The PS button is read from the pad's hidraw node, which many desktops let only root read. Save the udev rule from docs/INSTALL.md as /etc/udev/rules.d/70-anyplay-pad.rules, run sudo udevadm control --reload && sudo udevadm trigger, and plug the pad in again."
            }
            Pads::HelpLinuxTitle => "Linux Permissions",
            Pads::HelpLinux => {
                "Pads are read from /dev/input and /dev/hidraw. If none shows up, add yourself to the input group (sudo usermod -aG input $USER) and sign in again, or install the udev rule from docs/INSTALL.md."
            }
            Pads::HelpBatteryTitle => "Battery Low",
            Pads::HelpBattery => {
                "Charge the pad with a USB cable. A pad on a cable shows no battery reading."
            }
            Pads::LowBattery => "Low, %@%",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            Pads::Entry => "Kollar",
            Pads::EntryDetail => {
                "Tuşları, titreşimi ve ışık çubuğunu dene, başlatıcının tuşlarını değiştir, sorunları gider"
            }
            Pads::Title => "Kollar",
            Pads::Subtitle => {
                "Bu bilgisayardaki kollarla ilgili her şey. Oyunlar kendi kontrollerini korur."
            }
            Pads::CaptionConnected => "Bağlı",
            Pads::CaptionTest => "Dene ve Ayarla",
            Pads::CaptionControls => "Başlatıcı Kontrolleri",
            Pads::CaptionHelp => "Sorun Giderici",
            Pads::NoPad => "Kol bulunamadı",
            Pads::NoPadDetail => "Bir kol bağla ve üzerindeki bir tuşa bas.",
            Pads::PadDetail => "%@ · %@ · %@",
            Pads::InUse => "Kullanımda",
            Pads::Usb => "USB",
            Pads::Bluetooth => "Bluetooth",
            Pads::LinkUnknown => "Bağlantı bilinmiyor",
            Pads::Battery => "Pil %@%",
            Pads::BatteryCharging => "Şarj oluyor %@%",
            Pads::BatteryNone => "Pil bilgisi yok",
            Pads::TestButtons => "Tuşları Dene",
            Pads::TestButtonsDetail => "Tuşlara bas, çubukları oynat; yanarlar",
            Pads::TestVibration => "Titreşimi Dene",
            Pads::TestVibrationDetail => "Kısa bir titreşim, önce hafif motor",
            Pads::NotWhileGame => "Bir oyun çalışırken olmaz.",
            Pads::NoRumble => "Bağlı hiçbir kol titreşemiyor.",
            Pads::VibrationSent => "Titreşiyor",
            Pads::DeadZone => "Çubuk Ölü Bölgesi",
            Pads::DeadZoneDetail => {
                "Başlatıcının hareket etmesi için sol çubuğun ne kadar yatması gerekir. Oyunların kendi ayarı var."
            }
            Pads::LightBar => "Işık Çubuğu Rengi",
            Pads::LightBarDetail => "Yalnızca DualSense. Oyun çalışırken ışık çubuğunu geri alır.",
            Pads::LightBarSet => "Işık çubuğu değişti",
            Pads::LightBarFailed => "Kolun ışık çubuğuna ulaşılamadı.",
            Pads::ColourBlue => "Mavi",
            Pads::ColourRed => "Kırmızı",
            Pads::ColourGreen => "Yeşil",
            Pads::ColourPurple => "Mor",
            Pads::ColourOrange => "Turuncu",
            Pads::ColourWhite => "Beyaz",
            Pads::ColourOff => "Kapalı",
            Pads::ColourCustom => "Özel",
            Pads::LightBarHue => "Ton",
            Pads::LightBarBrightness => "Parlaklık",
            Pads::ConfirmWith => "%@ ile Onayla",
            Pads::ConfirmWithDetail => "Koldaki Seç ve Geri tuşlarının yerini değiştirir",
            Pads::ControlsNote => {
                "Bir iş seç, sonra tuşa veya klavyeye bas. Kullanımdaki bir tuş yer değiştirir."
            }
            Pads::Select => "Seç",
            Pads::Back => "Geri",
            Pads::Options => "Seçenekler",
            Pads::Previous => "Önceki Bölüm",
            Pads::Next => "Sonraki Bölüm",
            Pads::ReleaseToCapture => "Bir tuşa bas",
            Pads::Reset => "Varsayılana Döndür",
            Pads::ResetDetail => "Başlatıcının tüm tuşlarını eski haline getirir",
            Pads::Swapped => "%@ önceden %@ idi. %@ artık %@ kullanıyor.",
            Pads::SwappedNoReturn => "%@ şuradan taşındı: %@",
            Pads::FixedButton => "Bu tuş hep aynı işi yapar ve değiştirilemez.",
            Pads::TestTitle => "Tuşları Dene",
            Pads::TestLeave => "Çıkmak için %@ tuşunu basılı tut veya Esc'ye bas",
            Pads::TestWaiting => "Herhangi bir tuşa bas",
            Pads::TestNoPad => "Kol bulunamadı. Bir kol bağla ve bir tuşa bas.",
            Pads::StatusOk => "Tamam",
            Pads::StatusCheck => "Bak",
            Pads::HelpMissingTitle => "Kol Bulunamadı",
            Pads::HelpMissing => {
                "Kolu tak veya eşleştir, sonra bir tuşa bas. Başka bir kablo veya bağlantı noktası dene: bazı kablolar yalnızca şarj eder."
            }
            Pads::HelpMissingFound => {
                "Görüldü. Bir oyun görmüyorsa oyunu kol bağlandıktan sonra başlat."
            }
            Pads::HelpMissingNone => {
                "Başlatıcıya bağlı kol yok. Kolu tak veya eşleştir, sonra bir tuşa bas. Başka bir kablo veya bağlantı noktası dene: bazı kablolar yalnızca şarj eder."
            }
            Pads::HelpDoubleTitle => "Her Basış İki Kez Sayılıyor",
            Pads::HelpDouble => {
                "Steam Input veya DS4Windows, gerçek kolun yanına ikinci, sanal bir kol ekler. Başlatıcı yankıyı yok sayar, bir oyun saymayabilir. Aracı kapat veya başlatıcı ile oyun için kol öykünmesini kapat."
            }
            Pads::HelpSteamTitle => "Steam ve PlayStation Kolları",
            Pads::HelpSteam => {
                "PlayStation Yapılandırma Desteği açıkken Steam, DualSense ve DualShock kollarını devralır ve sanal bir Xbox kolu iletir. Steam'de Ayarlar, Kontrolcü bölümünden kapat veya başlatıcı ile oyun için Steam Input'u kapat."
            }
            Pads::HelpHideTitle => "DS4Windows ve HidHide",
            Pads::HelpHide => {
                "DS4Windows ve HidHide gerçek kolu diğer programlardan bilerek gizler. Başlatıcı veya oyun kol görmüyorsa HidHide'da izin verilen uygulamalara ekle veya DS4Windows'ta Hide DS4 Controller seçeneğini kapat."
            }
            Pads::HelpHomeTitle => "PS Tuşu Çalışmıyor",
            Pads::HelpHome => {
                "Ayarlar, Davranış, Ana Tuş Başlatıcıyı Açar seçeneğine bak. USB veya Bluetooth üzerinden DualSense ve DualShock 4 ile çalışır. Kolu özel olarak tutan başka bir araç engelleyebilir."
            }
            Pads::HelpHomeLinux => {
                "PS tuşu kolun hidraw düğümünden okunur; birçok masaüstü bunu yalnızca root'a açar. docs/INSTALL.md içindeki udev kuralını /etc/udev/rules.d/70-anyplay-pad.rules olarak kaydet, sudo udevadm control --reload && sudo udevadm trigger çalıştır ve kolu yeniden tak."
            }
            Pads::HelpLinuxTitle => "Linux İzinleri",
            Pads::HelpLinux => {
                "Kollar /dev/input ve /dev/hidraw üzerinden okunur. Hiçbiri görünmüyorsa kendini input grubuna ekle (sudo usermod -aG input $USER) ve oturumu yeniden aç veya docs/INSTALL.md içindeki udev kuralını kur."
            }
            Pads::HelpBatteryTitle => "Pil Zayıf",
            Pads::HelpBattery => {
                "Kolu USB kablosuyla şarj et. Kabloya bağlı kol pil bilgisi göstermez."
            }
            Pads::LowBattery => "Zayıf, %@%",
        }
    }

    /// The German text; `None` falls back to English.
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            Pads::Entry | Pads::Title => "Controller",
            Pads::EntryDetail => {
                "Tasten, Vibration und Lightbar testen, die Tasten des Launchers ändern, Probleme beheben"
            }
            Pads::Subtitle => {
                "Alles zu den Controllern an diesem PC. Spiele behalten ihre eigene Steuerung."
            }
            Pads::CaptionConnected => "Verbunden",
            Pads::CaptionTest => "Testen und Einstellen",
            Pads::CaptionControls => "Steuerung des Launchers",
            Pads::CaptionHelp => "Problemlösung",
            Pads::NoPad => "Kein Controller gefunden",
            Pads::NoPadDetail => "Schließe einen Controller an und drücke eine Taste.",
            Pads::PadDetail => "%@ · %@ · %@",
            Pads::InUse => "In Benutzung",
            Pads::Usb => "USB",
            Pads::Bluetooth => "Bluetooth",
            Pads::LinkUnknown => "Verbindung unbekannt",
            Pads::Battery => "Akku %@%",
            Pads::BatteryCharging => "Lädt %@%",
            Pads::BatteryNone => "Keine Akkuanzeige",
            Pads::TestButtons => "Tasten testen",
            Pads::TestButtonsDetail => "Drücke Tasten und bewege die Sticks; sie leuchten auf",
            Pads::TestVibration => "Vibration testen",
            Pads::TestVibrationDetail => "Ein kurzes Brummen, zuerst der leichte Motor",
            Pads::NotWhileGame => "Nicht, solange ein Spiel läuft.",
            Pads::NoRumble => "Kein verbundener Controller kann vibrieren.",
            Pads::VibrationSent => "Vibriert",
            Pads::DeadZone => "Stick-Totzone",
            Pads::DeadZoneDetail => {
                "Wie weit der linke Stick geneigt sein muss, bevor sich der Launcher bewegt. Spiele haben eine eigene."
            }
            Pads::LightBar => "Lightbar-Farbe",
            Pads::LightBarDetail => {
                "Nur DualSense. Ein Spiel übernimmt die Lightbar, solange es läuft."
            }
            Pads::LightBarSet => "Lightbar geändert",
            Pads::LightBarFailed => "Die Lightbar des Controllers war nicht erreichbar.",
            Pads::ColourBlue => "Blau",
            Pads::ColourRed => "Rot",
            Pads::ColourGreen => "Grün",
            Pads::ColourPurple => "Violett",
            Pads::ColourOrange => "Orange",
            Pads::ColourWhite => "Weiß",
            Pads::ColourOff => "Aus",
            Pads::ColourCustom => "Eigene",
            Pads::LightBarHue => "Farbton",
            Pads::LightBarBrightness => "Helligkeit",
            Pads::ConfirmWith => "Mit %@ bestätigen",
            Pads::ConfirmWithDetail => {
                "Tauscht Auswählen und Zurück auf dem Controller, wie manche es gewohnt sind"
            }
            Pads::ControlsNote => {
                "Wähle eine Aufgabe und drücke dann die Taste. Eine bereits belegte Taste tauscht den Platz."
            }
            Pads::Select => "Auswählen",
            Pads::Back => "Zurück",
            Pads::Options => "Optionen",
            Pads::Previous => "Vorheriger Bereich",
            Pads::Next => "Nächster Bereich",
            Pads::ReleaseToCapture => "Drücke eine Taste",
            Pads::Reset => "Auf Standard zurücksetzen",
            Pads::ResetDetail => "Setzt alle Tasten des Launchers zurück",
            Pads::Swapped => "%@ war %@. %@ nutzt jetzt %@.",
            Pads::SwappedNoReturn => "%@ wurde von %@ übernommen",
            Pads::FixedButton => {
                "Diese Taste hat immer dieselbe Aufgabe und lässt sich nicht ändern."
            }
            Pads::TestTitle => "Tasten testen",
            Pads::TestLeave => "Zum Verlassen %@ gedrückt halten oder Esc drücken",
            Pads::TestWaiting => "Drücke eine beliebige Taste",
            Pads::TestNoPad => "Kein Controller gefunden. Schließe einen an und drücke eine Taste.",
            Pads::StatusOk => "OK",
            Pads::StatusCheck => "Prüfen",
            Pads::HelpMissingTitle => "Controller nicht gefunden",
            Pads::HelpMissing | Pads::HelpMissingNone => {
                "Der Launcher sieht keinen Controller. Stecke ihn an oder koppele ihn und drücke eine Taste. Probiere ein anderes Kabel oder einen anderen Anschluss: Manche Kabel laden nur."
            }
            Pads::HelpMissingFound => {
                "Gesehen. Sieht ein Spiel ihn nicht, starte das Spiel, nachdem der Controller verbunden ist."
            }
            Pads::HelpDoubleTitle => "Jeder Tastendruck zählt doppelt",
            Pads::HelpDouble => {
                "Steam Input oder DS4Windows fügt neben dem echten einen zweiten, virtuellen Controller hinzu. Der Launcher ignoriert das Echo, ein Spiel eventuell nicht. Beende das Programm oder schalte seine Controller-Emulation für Launcher und Spiel aus."
            }
            Pads::HelpSteamTitle => "Steam und PlayStation-Controller",
            Pads::HelpSteam => {
                "Mit eingeschalteter PlayStation-Konfigurationsunterstützung übernimmt Steam DualSense und DualShock und reicht einen virtuellen Xbox-Controller weiter. Schalte sie in Steam unter Einstellungen, Controller aus, oder stelle Steam Input für Launcher und Spiel auf Aus."
            }
            Pads::HelpHideTitle => "DS4Windows und HidHide",
            Pads::HelpHide => {
                "DS4Windows und HidHide verstecken den echten Controller absichtlich vor anderen Programmen. Sieht der Launcher oder ein Spiel keinen Controller, füge es in HidHide zu den erlaubten Anwendungen hinzu oder schalte in DS4Windows „Hide DS4 Controller“ aus."
            }
            Pads::HelpHomeTitle => "PS-Taste tut nichts",
            Pads::HelpHome => {
                "Prüfe unter Einstellungen, Verhalten, Home-Taste öffnet den Launcher. Sie funktioniert mit DualSense und DualShock 4 über USB oder Bluetooth. Ein anderes Programm, das den Controller exklusiv hält, kann sie blockieren."
            }
            Pads::HelpHomeLinux => {
                "Die PS-Taste wird vom hidraw-Knoten des Controllers gelesen, den viele Desktops nur root erlauben. Speichere die udev-Regel aus docs/INSTALL.md als /etc/udev/rules.d/70-anyplay-pad.rules, führe sudo udevadm control --reload && sudo udevadm trigger aus und stecke den Controller neu an."
            }
            Pads::HelpLinuxTitle => "Linux-Berechtigungen",
            Pads::HelpLinux => {
                "Controller werden aus /dev/input und /dev/hidraw gelesen. Erscheint keiner, füge dich der Gruppe input hinzu (sudo usermod -aG input $USER) und melde dich neu an, oder installiere die udev-Regel aus docs/INSTALL.md."
            }
            Pads::HelpBatteryTitle => "Akku schwach",
            Pads::HelpBattery => {
                "Lade den Controller per USB-Kabel. Ein Controller am Kabel zeigt keinen Akkustand."
            }
            Pads::LowBattery => "Schwach, %@%",
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            Pads::Entry | Pads::Title => "Mandos",
            Pads::EntryDetail => {
                "Prueba los botones, la vibración y la barra de luz, cambia los botones del launcher y resuelve problemas"
            }
            Pads::Subtitle => {
                "Todo sobre los mandos de este PC. Los juegos conservan sus propios controles."
            }
            Pads::CaptionConnected => "Conectados",
            Pads::CaptionTest => "Probar y ajustar",
            Pads::CaptionControls => "Controles del launcher",
            Pads::CaptionHelp => "Solución de problemas",
            Pads::NoPad => "No se encontró ningún mando",
            Pads::NoPadDetail => "Conecta un mando y pulsa un botón.",
            Pads::PadDetail => "%@ · %@ · %@",
            Pads::InUse => "En uso",
            Pads::Usb => "USB",
            Pads::Bluetooth => "Bluetooth",
            Pads::LinkUnknown => "Conexión desconocida",
            Pads::Battery => "Batería %@\u{a0}%",
            Pads::BatteryCharging => "Cargando %@\u{a0}%",
            Pads::BatteryNone => "Sin lectura de batería",
            Pads::TestButtons => "Probar botones",
            Pads::TestButtonsDetail => "Pulsa botones y mueve los sticks; se iluminan",
            Pads::TestVibration => "Probar vibración",
            Pads::TestVibrationDetail => "Una vibración corta, primero el motor suave",
            Pads::NotWhileGame => "No mientras se ejecuta un juego.",
            Pads::NoRumble => "Ningún mando conectado puede vibrar.",
            Pads::VibrationSent => "Vibrando",
            Pads::DeadZone => "Zona muerta del stick",
            Pads::DeadZoneDetail => {
                "Cuánto debe inclinarse el stick izquierdo para que se mueva el launcher. Los juegos tienen la suya."
            }
            Pads::LightBar => "Color de la barra de luz",
            Pads::LightBarDetail => {
                "Solo DualSense. Un juego recupera la barra de luz mientras se ejecuta."
            }
            Pads::LightBarSet => "Barra de luz cambiada",
            Pads::LightBarFailed => "No se pudo acceder a la barra de luz del mando.",
            Pads::ColourBlue => "Azul",
            Pads::ColourRed => "Rojo",
            Pads::ColourGreen => "Verde",
            Pads::ColourPurple => "Morado",
            Pads::ColourOrange => "Naranja",
            Pads::ColourWhite => "Blanco",
            Pads::ColourOff => "Apagada",
            Pads::ColourCustom => "Personalizado",
            Pads::LightBarHue => "Tono",
            Pads::LightBarBrightness => "Brillo",
            Pads::ConfirmWith => "Confirmar con %@",
            Pads::ConfirmWithDetail => {
                "Intercambia Seleccionar y Atrás en el mando, como están acostumbrados algunos jugadores"
            }
            Pads::ControlsNote => {
                "Elige una función y pulsa el botón o la tecla. Un botón que ya se usa intercambia su lugar."
            }
            Pads::Select => "Seleccionar",
            Pads::Back => "Atrás",
            Pads::Options => "Opciones",
            Pads::Previous => "Sección anterior",
            Pads::Next => "Sección siguiente",
            Pads::ReleaseToCapture => "Pulsa un botón o una tecla",
            Pads::Reset => "Restablecer valores",
            Pads::ResetDetail => "Restablece todos los botones y teclas del launcher",
            Pads::Swapped => "%@ era %@. %@ ahora usa %@.",
            Pads::SwappedNoReturn => "%@ pasó de %@",
            Pads::FixedButton => "Ese botón siempre hace lo mismo y no se puede cambiar.",
            Pads::TestTitle => "Probar botones",
            Pads::TestLeave => "Mantén pulsado %@ para salir, o pulsa Esc",
            Pads::TestWaiting => "Pulsa cualquier botón",
            Pads::TestNoPad => "No se encontró ningún mando. Conecta uno y pulsa un botón.",
            Pads::StatusOk => "OK",
            Pads::StatusCheck => "Revisar",
            Pads::HelpMissingTitle => "Mando no encontrado",
            Pads::HelpMissing => {
                "Conecta o empareja el mando y pulsa un botón. Prueba con otro cable u otro puerto: algunos cables solo cargan. Un mando conectado después de iniciar un juego lo ve el launcher, pero no ese juego."
            }
            Pads::HelpMissingFound => {
                "Detectado. Si un juego no lo ve, inicia el juego después de conectar el mando."
            }
            Pads::HelpMissingNone => {
                "No hay ningún mando conectado al launcher. Conéctalo o empárejalo y pulsa un botón. Prueba con otro cable u otro puerto: algunos cables solo cargan."
            }
            Pads::HelpDoubleTitle => "Cada pulsación cuenta doble",
            Pads::HelpDouble => {
                "Steam Input o DS4Windows añade un segundo mando virtual junto al real. El launcher ignora el eco, pero un juego puede no hacerlo. Cierra la herramienta o desactiva su emulación de mando para el launcher y el juego."
            }
            Pads::HelpSteamTitle => "Steam y mandos DualSense",
            Pads::HelpSteam => {
                "Con la compatibilidad de configuración para mandos DualSense y DualShock activada, Steam se encarga de ellos y pasa un mando Xbox virtual. Desactívala en Steam, Ajustes, Mando, o pon Steam Input en desactivado para el launcher y el juego."
            }
            Pads::HelpHideTitle => "DS4Windows y HidHide",
            Pads::HelpHide => {
                "DS4Windows y HidHide ocultan a propósito el mando real a otros programas. Si el launcher o un juego no ve ningún mando, añádelo a las aplicaciones permitidas en HidHide o desactiva «Hide DS4 Controller» en DS4Windows."
            }
            Pads::HelpHomeTitle => "El botón PS no hace nada",
            Pads::HelpHome => {
                "Revisa Ajustes, Comportamiento, El botón Home abre el launcher. Funciona con un DualSense o un DualShock 4 por USB o Bluetooth. Otra herramienta que retenga el mando en exclusiva puede bloquearlo."
            }
            Pads::HelpHomeLinux => {
                "El botón PS se lee del nodo hidraw del mando, que muchos escritorios solo dejan leer a root. Guarda la regla udev de docs/INSTALL.md como /etc/udev/rules.d/70-anyplay-pad.rules, ejecuta sudo udevadm control --reload && sudo udevadm trigger y vuelve a conectar el mando."
            }
            Pads::HelpLinuxTitle => "Permisos en Linux",
            Pads::HelpLinux => {
                "Los mandos se leen de /dev/input y /dev/hidraw. Si no aparece ninguno, añádete al grupo input (sudo usermod -aG input $USER) y vuelve a iniciar sesión, o instala la regla udev de docs/INSTALL.md."
            }
            Pads::HelpBatteryTitle => "Batería baja",
            Pads::HelpBattery => {
                "Carga el mando con un cable USB. Un mando con cable no muestra el nivel de batería."
            }
            Pads::LowBattery => "Baja, %@\u{a0}%",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            Pads::Entry | Pads::Title => "Manettes",
            Pads::EntryDetail => {
                "Teste les boutons, la vibration et la barre lumineuse, change les boutons du launcher et résous les problèmes"
            }
            Pads::Subtitle => {
                "Tout sur les manettes de ce PC. Les jeux gardent leurs propres commandes."
            }
            Pads::CaptionConnected => "Connectées",
            Pads::CaptionTest => "Tester et régler",
            Pads::CaptionControls => "Commandes du launcher",
            Pads::CaptionHelp => "Dépannage",
            Pads::NoPad => "Aucune manette trouvée",
            Pads::NoPadDetail => "Branche une manette et appuie sur un bouton.",
            Pads::PadDetail => "%@ · %@ · %@",
            Pads::InUse => "Utilisée",
            Pads::Usb => "USB",
            Pads::Bluetooth => "Bluetooth",
            Pads::LinkUnknown => "Connexion inconnue",
            Pads::Battery => "Batterie %@\u{a0}%",
            Pads::BatteryCharging => "En charge %@\u{a0}%",
            Pads::BatteryNone => "Niveau de batterie indisponible",
            Pads::TestButtons => "Tester les boutons",
            Pads::TestButtonsDetail => {
                "Appuie sur les boutons et bouge les sticks\u{a0}; ils s’allument"
            }
            Pads::TestVibration => "Tester la vibration",
            Pads::TestVibrationDetail => "Une courte vibration, moteur léger d’abord",
            Pads::NotWhileGame => "Impossible pendant qu’un jeu tourne.",
            Pads::NoRumble => "Aucune manette connectée ne peut vibrer.",
            Pads::VibrationSent => "Vibration en cours",
            Pads::DeadZone => "Zone morte du stick",
            Pads::DeadZoneDetail => {
                "De combien le stick gauche doit être incliné pour que le launcher réagisse. Les jeux ont la leur."
            }
            Pads::LightBar => "Couleur de la barre lumineuse",
            Pads::LightBarDetail => {
                "DualSense uniquement. Un jeu reprend la barre lumineuse pendant qu’il tourne."
            }
            Pads::LightBarSet => "Barre lumineuse modifiée",
            Pads::LightBarFailed => "Barre lumineuse de la manette inaccessible.",
            Pads::ColourBlue => "Bleu",
            Pads::ColourRed => "Rouge",
            Pads::ColourGreen => "Vert",
            Pads::ColourPurple => "Violet",
            Pads::ColourOrange => "Orange",
            Pads::ColourWhite => "Blanc",
            Pads::ColourOff => "Éteinte",
            Pads::ColourCustom => "Personnalisée",
            Pads::LightBarHue => "Teinte",
            Pads::LightBarBrightness => "Luminosité",
            Pads::ConfirmWith => "Confirmer avec %@",
            Pads::ConfirmWithDetail => {
                "Échange Sélectionner et Retour sur la manette, comme certains joueurs en ont l’habitude"
            }
            Pads::ControlsNote => {
                "Choisis une fonction, puis appuie sur le bouton ou la touche. Un bouton déjà utilisé échange sa place."
            }
            Pads::Select => "Sélectionner",
            Pads::Back => "Retour",
            Pads::Options => "Options",
            Pads::Previous => "Section précédente",
            Pads::Next => "Section suivante",
            Pads::ReleaseToCapture => "Appuie sur un bouton ou une touche",
            Pads::Reset => "Rétablir par défaut",
            Pads::ResetDetail => "Rétablit tous les boutons et touches du launcher",
            Pads::Swapped => "%@ était %@. %@ utilise maintenant %@.",
            Pads::SwappedNoReturn => "%@ vient de %@",
            Pads::FixedButton => {
                "Ce bouton a toujours la même fonction et ne peut pas être changé."
            }
            Pads::TestTitle => "Tester les boutons",
            Pads::TestLeave => "Maintiens %@ pour quitter, ou appuie sur Échap",
            Pads::TestWaiting => "Appuie sur un bouton",
            Pads::TestNoPad => "Aucune manette trouvée. Branches-en une et appuie sur un bouton.",
            Pads::StatusOk => "OK",
            Pads::StatusCheck => "Vérifier",
            Pads::HelpMissingTitle => "Manette introuvable",
            Pads::HelpMissing => {
                "Branche ou associe la manette, puis appuie sur un bouton. Essaie un autre câble ou un autre port\u{a0}: certains câbles ne font que charger. Une manette connectée après le lancement d’un jeu est vue par le launcher, mais pas par ce jeu."
            }
            Pads::HelpMissingFound => {
                "Détectée. Si un jeu ne la voit pas, lance-le une fois la manette connectée."
            }
            Pads::HelpMissingNone => {
                "Aucune manette n’est connectée au launcher. Branche-la ou associe-la, puis appuie sur un bouton. Essaie un autre câble ou un autre port\u{a0}: certains câbles ne font que charger."
            }
            Pads::HelpDoubleTitle => "Chaque appui compte double",
            Pads::HelpDouble => {
                "Steam Input ou DS4Windows ajoute une seconde manette virtuelle à côté de la vraie. Le launcher ignore l’écho, pas forcément un jeu. Ferme l’outil, ou désactive son émulation de manette pour le launcher et le jeu."
            }
            Pads::HelpSteamTitle => "Steam et manettes DualSense",
            Pads::HelpSteam => {
                "Quand la prise en charge de la configuration des manettes DualSense et DualShock est activée, Steam les prend en charge et transmet une manette Xbox virtuelle. Désactive-la dans Steam, Paramètres, Manette, ou mets Steam Input sur désactivé pour le launcher et le jeu."
            }
            Pads::HelpHideTitle => "DS4Windows et HidHide",
            Pads::HelpHide => {
                "DS4Windows et HidHide cachent volontairement la vraie manette aux autres programmes. Si le launcher ou un jeu ne voit aucune manette, ajoute-le aux applications autorisées dans HidHide, ou désactive «\u{a0}Hide DS4 Controller\u{a0}» dans DS4Windows."
            }
            Pads::HelpHomeTitle => "Le bouton PS ne fait rien",
            Pads::HelpHome => {
                "Vérifie Paramètres, Comportement, Le bouton Home ouvre le launcher. Il fonctionne avec une DualSense ou une DualShock 4 en USB ou Bluetooth. Un autre outil qui retient la manette en exclusivité peut le bloquer."
            }
            Pads::HelpHomeLinux => {
                "Le bouton PS est lu depuis le nœud hidraw de la manette, que beaucoup de bureaux ne laissent lire qu’à root. Enregistre la règle udev de docs/INSTALL.md sous /etc/udev/rules.d/70-anyplay-pad.rules, lance sudo udevadm control --reload && sudo udevadm trigger, puis rebranche la manette."
            }
            Pads::HelpLinuxTitle => "Autorisations Linux",
            Pads::HelpLinux => {
                "Les manettes sont lues depuis /dev/input et /dev/hidraw. Si aucune n’apparaît, ajoute-toi au groupe input (sudo usermod -aG input $USER) et reconnecte-toi, ou installe la règle udev de docs/INSTALL.md."
            }
            Pads::HelpBatteryTitle => "Batterie faible",
            Pads::HelpBattery => {
                "Recharge la manette avec un câble USB. Une manette sur câble n’affiche pas le niveau de batterie."
            }
            Pads::LowBattery => "Faible, %@\u{a0}%",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            Pads::Entry | Pads::Title => "Controles",
            Pads::EntryDetail => {
                "Teste botões, vibração e a barra de luz, mude os botões do launcher e resolva problemas"
            }
            Pads::Subtitle => {
                "Tudo sobre os controles deste PC. Os jogos mantêm os seus próprios comandos."
            }
            Pads::CaptionConnected => "Conectados",
            Pads::CaptionTest => "Testar e ajustar",
            Pads::CaptionControls => "Botões do launcher",
            Pads::CaptionHelp => "Solução de problemas",
            Pads::NoPad => "Nenhum controle encontrado",
            Pads::NoPadDetail => "Conecte um controle e pressione um botão.",
            Pads::PadDetail => "%@ · %@ · %@",
            Pads::InUse => "Em uso",
            Pads::Usb => "USB",
            Pads::Bluetooth => "Bluetooth",
            Pads::LinkUnknown => "Conexão desconhecida",
            Pads::Battery => "Bateria %@%",
            Pads::BatteryCharging => "Carregando %@%",
            Pads::BatteryNone => "Sem leitura da bateria",
            Pads::TestButtons => "Testar botões",
            Pads::TestButtonsDetail => "Pressione botões e mova os analógicos; eles acendem",
            Pads::TestVibration => "Testar vibração",
            Pads::TestVibrationDetail => "Uma vibração curta, motor leve primeiro",
            Pads::NotWhileGame => "Não enquanto um jogo está em execução.",
            Pads::NoRumble => "Nenhum controle conectado pode vibrar.",
            Pads::VibrationSent => "Vibrando",
            Pads::DeadZone => "Zona morta do analógico",
            Pads::DeadZoneDetail => {
                "O quanto o analógico esquerdo precisa se inclinar para o launcher se mover. Os jogos têm a sua própria."
            }
            Pads::LightBar => "Cor da barra de luz",
            Pads::LightBarDetail => {
                "Somente DualSense. Um jogo assume a barra de luz enquanto está em execução."
            }
            Pads::LightBarSet => "Barra de luz alterada",
            Pads::LightBarFailed => "Não foi possível acessar a barra de luz do controle.",
            Pads::ColourBlue => "Azul",
            Pads::ColourRed => "Vermelho",
            Pads::ColourGreen => "Verde",
            Pads::ColourPurple => "Roxo",
            Pads::ColourOrange => "Laranja",
            Pads::ColourWhite => "Branco",
            Pads::ColourOff => "Desligada",
            Pads::ColourCustom => "Personalizada",
            Pads::LightBarHue => "Matiz",
            Pads::LightBarBrightness => "Brilho",
            Pads::ConfirmWith => "Confirmar com %@",
            Pads::ConfirmWithDetail => {
                "Troca Selecionar e Voltar no controle, como alguns jogadores estão acostumados"
            }
            Pads::ControlsNote => {
                "Escolha uma função e pressione o botão ou a tecla. Um botão que já está em uso troca de lugar."
            }
            Pads::Select => "Selecionar",
            Pads::Back => "Voltar",
            Pads::Options => "Opções",
            Pads::Previous => "Seção anterior",
            Pads::Next => "Próxima seção",
            Pads::ReleaseToCapture => "Pressione um botão ou uma tecla",
            Pads::Reset => "Restaurar padrão",
            Pads::ResetDetail => "Restaura todos os botões e teclas do launcher",
            Pads::Swapped => "%@ era %@. %@ agora usa %@.",
            Pads::SwappedNoReturn => "%@ veio de %@",
            Pads::FixedButton => "Esse botão sempre faz a mesma coisa e não pode ser alterado.",
            Pads::TestTitle => "Testar botões",
            Pads::TestLeave => "Segure %@ para sair ou pressione Esc",
            Pads::TestWaiting => "Pressione qualquer botão",
            Pads::TestNoPad => "Nenhum controle encontrado. Conecte um e pressione um botão.",
            Pads::StatusOk => "OK",
            Pads::StatusCheck => "Verificar",
            Pads::HelpMissingTitle => "Controle não encontrado",
            Pads::HelpMissing => {
                "Conecte o controle ou faça o pareamento e pressione um botão. Tente outro cabo ou outra porta: alguns cabos só carregam. Um controle conectado depois que um jogo começou aparece no launcher, mas não nesse jogo."
            }
            Pads::HelpMissingFound => {
                "Encontrado. Se um jogo não o vê, inicie o jogo depois de conectar o controle."
            }
            Pads::HelpMissingNone => {
                "Nenhum controle está conectado ao launcher. Conecte-o ou faça o pareamento e pressione um botão. Tente outro cabo ou outra porta: alguns cabos só carregam."
            }
            Pads::HelpDoubleTitle => "Cada pressionamento conta em dobro",
            Pads::HelpDouble => {
                "O Steam Input ou o DS4Windows adiciona um segundo controle, virtual, ao lado do real. O launcher ignora o eco, mas um jogo pode não ignorar. Feche a ferramenta ou desative a emulação de controle dela para o launcher e o jogo."
            }
            Pads::HelpSteamTitle => "Steam e controles DualSense",
            Pads::HelpSteam => {
                "Com o suporte à configuração de controles DualSense e DualShock ativado, o Steam assume esses controles e repassa um controle Xbox virtual. Desative em Steam, Configurações, Controle, ou coloque o Steam Input como desativado para o launcher e o jogo."
            }
            Pads::HelpHideTitle => "DS4Windows e HidHide",
            Pads::HelpHide => {
                "O DS4Windows e o HidHide escondem de propósito o controle real de outros programas. Se o launcher ou um jogo não vê nenhum controle, adicione-o aos aplicativos permitidos no HidHide ou desative “Hide DS4 Controller” no DS4Windows."
            }
            Pads::HelpHomeTitle => "O botão PS não faz nada",
            Pads::HelpHome => {
                "Verifique Configurações, Comportamento, Botão Home abre o launcher. Funciona com DualSense ou DualShock 4 por USB ou Bluetooth. Outra ferramenta que prende o controle com exclusividade pode bloqueá-lo."
            }
            Pads::HelpHomeLinux => {
                "O botão PS é lido do nó hidraw do controle, que muitos desktops só deixam o root ler. Salve a regra udev de docs/INSTALL.md como /etc/udev/rules.d/70-anyplay-pad.rules, execute sudo udevadm control --reload && sudo udevadm trigger e conecte o controle de novo."
            }
            Pads::HelpLinuxTitle => "Permissões no Linux",
            Pads::HelpLinux => {
                "Os controles são lidos de /dev/input e /dev/hidraw. Se nenhum aparecer, adicione-se ao grupo input (sudo usermod -aG input $USER) e entre de novo, ou instale a regra udev de docs/INSTALL.md."
            }
            Pads::HelpBatteryTitle => "Bateria fraca",
            Pads::HelpBattery => {
                "Carregue o controle com um cabo USB. Um controle no cabo não mostra a leitura da bateria."
            }
            Pads::LowBattery => "Fraca, %@%",
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
        for text in Pads::ALL {
            for language in Language::ALL {
                assert!(
                    !text.text(language).trim().is_empty(),
                    "{text:?} {language:?}"
                );
            }
        }
    }

    #[test]
    fn every_text_has_a_translation_with_the_same_arguments() {
        for text in Pads::ALL {
            for (name, translated) in [
                ("German", text.german()),
                ("Spanish", text.spanish()),
                ("French", text.french()),
                ("Portuguese", text.portuguese()),
            ] {
                let translated =
                    translated.unwrap_or_else(|| panic!("{text:?} has no {name} text"));
                assert_eq!(
                    text.english().matches("%@").count(),
                    translated.matches("%@").count(),
                    "{text:?} {name}"
                );
            }
        }
    }

    #[test]
    fn both_languages_take_the_same_number_of_arguments() {
        for text in Pads::ALL {
            assert_eq!(
                text.english().matches("%@").count(),
                text.turkish().matches("%@").count(),
                "{text:?}"
            );
        }
    }
}
