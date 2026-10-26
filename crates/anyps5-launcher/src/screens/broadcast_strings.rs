//! Texts of Broadcast: the settings rows, the Home button panel and the toasts. English and
//! Turkish like the rest; any other language reads English until it has its own.

use crate::ui::i18n::Language;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BroadcastText {
    /// The caption over the settings rows.
    Title,
    /// Settings row: where the stream goes, and what it does.
    Service,
    ServiceDetail,
    Twitch,
    YouTube,
    Custom,
    /// Settings row: the server address of a custom service.
    Server,
    ServerDetail,
    /// Settings row: the stream key, and what is done with it.
    Key,
    KeyDetail,
    /// Settings row: the picture size, rate and bit rate.
    Quality,
    Quality720p30,
    Quality720p60,
    Quality1080p60,
    /// Home button panel.
    Start,
    Stop,
    /// The line under the game's name while a broadcast runs.
    Connecting,
    Live,
    /// The mark in the corner while live.
    Badge,
    /// Toast: the server accepted the stream.
    Started,
    /// Toast: the player stopped the broadcast.
    Ended,
    /// Toast: no stream key is kept yet.
    NeedKey,
    /// Toast: a custom service has no usable address.
    NeedServer,
    /// Toast: the key could not be written to the credential store.
    KeyNotKept,
    /// Toast: this build has no worker.
    NoWorker,
    /// Toast: the service said no to the key.
    BadKey,
    /// Toast: the server could not be reached.
    NoServer,
    /// Toast: the connection broke while live.
    Lost,
    NoEncoder,
    NoWindow,
    /// Toast: anything else.
    Failed,
}

impl BroadcastText {
    pub const ALL: [BroadcastText; 31] = [
        BroadcastText::Title,
        BroadcastText::Service,
        BroadcastText::ServiceDetail,
        BroadcastText::Twitch,
        BroadcastText::YouTube,
        BroadcastText::Custom,
        BroadcastText::Server,
        BroadcastText::ServerDetail,
        BroadcastText::Key,
        BroadcastText::KeyDetail,
        BroadcastText::Quality,
        BroadcastText::Quality720p30,
        BroadcastText::Quality720p60,
        BroadcastText::Quality1080p60,
        BroadcastText::Start,
        BroadcastText::Stop,
        BroadcastText::Connecting,
        BroadcastText::Live,
        BroadcastText::Badge,
        BroadcastText::Started,
        BroadcastText::Ended,
        BroadcastText::NeedKey,
        BroadcastText::NeedServer,
        BroadcastText::KeyNotKept,
        BroadcastText::NoWorker,
        BroadcastText::BadKey,
        BroadcastText::NoServer,
        BroadcastText::Lost,
        BroadcastText::NoEncoder,
        BroadcastText::NoWindow,
        BroadcastText::Failed,
    ];

    pub const fn english(self) -> &'static str {
        match self {
            BroadcastText::Title => "Broadcast",
            BroadcastText::Service => "Broadcast To",
            BroadcastText::ServiceDetail => {
                "Shows the running game live. For now without the game's sound"
            }
            BroadcastText::Twitch => "Twitch",
            BroadcastText::YouTube => "YouTube",
            BroadcastText::Custom => "Custom RTMP",
            BroadcastText::Server => "Server Address",
            BroadcastText::ServerDetail => "Starts with rtmp://",
            BroadcastText::Key => "Stream Key",
            BroadcastText::KeyDetail => {
                "Paste the key from your dashboard. It stays on this PC and is never shown"
            }
            BroadcastText::Quality => "Broadcast Quality",
            BroadcastText::Quality720p30 => "720p, 30 fps",
            BroadcastText::Quality720p60 => "720p, 60 fps",
            BroadcastText::Quality1080p60 => "1080p, 60 fps",
            BroadcastText::Start => "Start Broadcast",
            BroadcastText::Stop => "Stop Broadcast",
            BroadcastText::Connecting => "Connecting to the broadcast server",
            BroadcastText::Live => "Live",
            BroadcastText::Badge => "LIVE",
            BroadcastText::Started => "You are live",
            BroadcastText::Ended => "Broadcast ended",
            BroadcastText::NeedKey => "Add your stream key in Settings first.",
            BroadcastText::NeedServer => "Add the server address in Settings first.",
            BroadcastText::KeyNotKept => "The stream key could not be kept on this PC.",
            BroadcastText::NoWorker => "Broadcasting is not part of this build.",
            BroadcastText::BadKey => {
                "The service did not accept your stream key. Check it in Settings."
            }
            BroadcastText::NoServer => {
                "The broadcast server could not be reached. Check the address and your connection."
            }
            BroadcastText::Lost => "The connection to the service was lost. The broadcast stopped.",
            BroadcastText::NoEncoder => "Broadcasting needs a video encoder, and this PC has none.",
            BroadcastText::NoWindow => "The broadcast could not find the game's window.",
            BroadcastText::Failed => "The broadcast stopped.",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            BroadcastText::Title => "Yayın",
            BroadcastText::Service => "Yayın Yeri",
            BroadcastText::ServiceDetail => {
                "Çalışan oyunu canlı gösterir. Şimdilik oyunun sesi olmadan"
            }
            BroadcastText::Twitch => "Twitch",
            BroadcastText::YouTube => "YouTube",
            BroadcastText::Custom => "Özel RTMP",
            BroadcastText::Server => "Sunucu Adresi",
            BroadcastText::ServerDetail => "rtmp:// ile başlar",
            BroadcastText::Key => "Yayın Anahtarı",
            BroadcastText::KeyDetail => {
                "Panelinizdeki anahtarı yapıştırın. Bu bilgisayarda kalır ve hiç gösterilmez"
            }
            BroadcastText::Quality => "Yayın Kalitesi",
            BroadcastText::Quality720p30 => "720p, 30 kare/sn",
            BroadcastText::Quality720p60 => "720p, 60 kare/sn",
            BroadcastText::Quality1080p60 => "1080p, 60 kare/sn",
            BroadcastText::Start => "Yayını Başlat",
            BroadcastText::Stop => "Yayını Durdur",
            BroadcastText::Connecting => "Yayın sunucusuna bağlanılıyor",
            BroadcastText::Live => "Canlı",
            BroadcastText::Badge => "CANLI",
            BroadcastText::Started => "Canlıdasınız",
            BroadcastText::Ended => "Yayın bitti",
            BroadcastText::NeedKey => "Önce Ayarlar'a yayın anahtarınızı ekleyin.",
            BroadcastText::NeedServer => "Önce Ayarlar'a sunucu adresini ekleyin.",
            BroadcastText::KeyNotKept => "Yayın anahtarı bu bilgisayarda saklanamadı.",
            BroadcastText::NoWorker => "Yayın bu sürümün parçası değil.",
            BroadcastText::BadKey => {
                "Hizmet yayın anahtarınızı kabul etmedi. Ayarlar'dan kontrol edin."
            }
            BroadcastText::NoServer => {
                "Yayın sunucusuna ulaşılamadı. Adresi ve bağlantınızı kontrol edin."
            }
            BroadcastText::Lost => "Hizmetle bağlantı koptu. Yayın durdu.",
            BroadcastText::NoEncoder => {
                "Yayın için bir video kodlayıcı gerekir, bu bilgisayarda yok."
            }
            BroadcastText::NoWindow => "Yayın oyunun penceresini bulamadı.",
            BroadcastText::Failed => "Yayın durdu.",
        }
    }

    pub const fn text(self, language: Language) -> &'static str {
        match language {
            Language::Turkish => self.turkish(),
            _ => self.english(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_text_exists_in_every_language() {
        for text in BroadcastText::ALL {
            for language in Language::ALL {
                assert!(!text.text(language).is_empty(), "{text:?} {language:?}");
            }
        }
    }

    #[test]
    fn the_names_are_ours() {
        for text in BroadcastText::ALL {
            for language in Language::ALL {
                let shown = text.text(language);
                assert!(
                    !shown.contains("PlayStation") && !shown.contains("PSN"),
                    "{shown}"
                );
            }
        }
    }
}
