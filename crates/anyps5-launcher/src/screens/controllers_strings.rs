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
    fn every_text_has_a_german_one_with_the_same_arguments() {
        for text in Pads::ALL {
            let german = text
                .german()
                .unwrap_or_else(|| panic!("{text:?} has no German text"));
            assert_eq!(
                text.english().matches("%@").count(),
                german.matches("%@").count(),
                "{text:?}"
            );
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
