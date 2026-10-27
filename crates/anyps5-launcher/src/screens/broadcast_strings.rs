//! Texts of Broadcast: the settings rows, the Home button panel and the toasts. English,
//! Turkish and German like the rest; any other language reads English until it has its own.

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
    /// Toast: the service's certificate is not trusted, or is for another server.
    CertUntrusted,
    /// Toast: the certificate is not valid at this PC's date and time.
    CertExpired,
    /// Toast: the secure connection failed for another reason.
    SecureFailed,
    /// Toast: the connection broke while live.
    Lost,
    NoEncoder,
    NoWindow,
    /// Toast: anything else.
    Failed,
}

impl BroadcastText {
    pub const ALL: [BroadcastText; 34] = [
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
        BroadcastText::CertUntrusted,
        BroadcastText::CertExpired,
        BroadcastText::SecureFailed,
        BroadcastText::Lost,
        BroadcastText::NoEncoder,
        BroadcastText::NoWindow,
        BroadcastText::Failed,
    ];

    pub const fn english(self) -> &'static str {
        match self {
            BroadcastText::Title => "Broadcast",
            BroadcastText::Service => "Broadcast To",
            BroadcastText::ServiceDetail => "Shows the running game and its sound live",
            BroadcastText::Twitch => "Twitch",
            BroadcastText::YouTube => "YouTube",
            BroadcastText::Custom => "Custom RTMP",
            BroadcastText::Server => "Server Address",
            BroadcastText::ServerDetail => "Starts with rtmp:// or rtmps://",
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
            BroadcastText::CertUntrusted => {
                "This PC does not trust the service's security certificate. Check the server address."
            }
            BroadcastText::CertExpired => {
                "The service's security certificate is not valid right now. Check your PC's date and time."
            }
            BroadcastText::SecureFailed => {
                "A secure connection to the service could not be made. Check the address and your connection."
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
            BroadcastText::ServiceDetail => "Çalışan oyunu sesiyle birlikte canlı gösterir",
            BroadcastText::Twitch => "Twitch",
            BroadcastText::YouTube => "YouTube",
            BroadcastText::Custom => "Özel RTMP",
            BroadcastText::Server => "Sunucu Adresi",
            BroadcastText::ServerDetail => "rtmp:// veya rtmps:// ile başlar",
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
            BroadcastText::CertUntrusted => {
                "Bu bilgisayar hizmetin güvenlik sertifikasına güvenmiyor. Sunucu adresini kontrol edin."
            }
            BroadcastText::CertExpired => {
                "Hizmetin güvenlik sertifikası şu an geçerli değil. Bilgisayarınızın tarih ve saatini kontrol edin."
            }
            BroadcastText::SecureFailed => {
                "Hizmetle güvenli bağlantı kurulamadı. Adresi ve bağlantınızı kontrol edin."
            }
            BroadcastText::Lost => "Hizmetle bağlantı koptu. Yayın durdu.",
            BroadcastText::NoEncoder => {
                "Yayın için bir video kodlayıcı gerekir, bu bilgisayarda yok."
            }
            BroadcastText::NoWindow => "Yayın oyunun penceresini bulamadı.",
            BroadcastText::Failed => "Yayın durdu.",
        }
    }

    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            BroadcastText::Title => "Übertragung",
            BroadcastText::Service => "Übertragen an",
            BroadcastText::ServiceDetail => "Zeigt das laufende Spiel mit seinem Ton live",
            BroadcastText::Twitch => "Twitch",
            BroadcastText::YouTube => "YouTube",
            BroadcastText::Custom => "Eigenes RTMP",
            BroadcastText::Server => "Serveradresse",
            BroadcastText::ServerDetail => "Beginnt mit rtmp:// oder rtmps://",
            BroadcastText::Key => "Stream-Schlüssel",
            BroadcastText::KeyDetail => {
                "Füge den Schlüssel aus deinem Dashboard ein. Er bleibt auf diesem PC und wird nie angezeigt"
            }
            BroadcastText::Quality => "Übertragungsqualität",
            BroadcastText::Quality720p30 => "720p, 30 fps",
            BroadcastText::Quality720p60 => "720p, 60 fps",
            BroadcastText::Quality1080p60 => "1080p, 60 fps",
            BroadcastText::Start => "Übertragung starten",
            BroadcastText::Stop => "Übertragung beenden",
            BroadcastText::Connecting => "Verbindung zum Übertragungsserver wird aufgebaut",
            BroadcastText::Live => "Live",
            BroadcastText::Badge => "LIVE",
            BroadcastText::Started => "Du bist live",
            BroadcastText::Ended => "Übertragung beendet",
            BroadcastText::NeedKey => {
                "Trage zuerst deinen Stream-Schlüssel in den Einstellungen ein."
            }
            BroadcastText::NeedServer => "Trage zuerst die Serveradresse in den Einstellungen ein.",
            BroadcastText::KeyNotKept => {
                "Der Stream-Schlüssel konnte auf diesem PC nicht gespeichert werden."
            }
            BroadcastText::NoWorker => "Übertragen gehört nicht zu dieser Version.",
            BroadcastText::BadKey => {
                "Der Dienst hat deinen Stream-Schlüssel nicht angenommen. Prüfe ihn in den Einstellungen."
            }
            BroadcastText::NoServer => {
                "Der Übertragungsserver ist nicht erreichbar. Prüfe die Adresse und deine Verbindung."
            }
            BroadcastText::CertUntrusted => {
                "Dieser PC vertraut dem Sicherheitszertifikat des Dienstes nicht. Prüfe die Serveradresse."
            }
            BroadcastText::CertExpired => {
                "Das Sicherheitszertifikat des Dienstes ist gerade nicht gültig. Prüfe Datum und Uhrzeit deines PCs."
            }
            BroadcastText::SecureFailed => {
                "Mit dem Dienst konnte keine sichere Verbindung aufgebaut werden. Prüfe die Adresse und deine Verbindung."
            }
            BroadcastText::Lost => {
                "Die Verbindung zum Dienst ist abgebrochen. Die Übertragung wurde beendet."
            }
            BroadcastText::NoEncoder => {
                "Zum Übertragen braucht es einen Video-Encoder, und dieser PC hat keinen."
            }
            BroadcastText::NoWindow => "Das Fenster des Spiels wurde nicht gefunden.",
            BroadcastText::Failed => "Die Übertragung wurde beendet.",
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            BroadcastText::Title => "Transmisión",
            BroadcastText::Service => "Transmitir a",
            BroadcastText::ServiceDetail => {
                "Muestra en directo el juego en ejecución con su sonido"
            }
            BroadcastText::Twitch => "Twitch",
            BroadcastText::YouTube => "YouTube",
            BroadcastText::Custom => "RTMP personalizado",
            BroadcastText::Server => "Dirección del servidor",
            BroadcastText::ServerDetail => "Empieza por rtmp:// o rtmps://",
            BroadcastText::Key => "Clave de transmisión",
            BroadcastText::KeyDetail => {
                "Pega la clave de tu panel. Se queda en este PC y nunca se muestra"
            }
            BroadcastText::Quality => "Calidad de transmisión",
            BroadcastText::Quality720p30 => "720p, 30 fps",
            BroadcastText::Quality720p60 => "720p, 60 fps",
            BroadcastText::Quality1080p60 => "1080p, 60 fps",
            BroadcastText::Start => "Iniciar transmisión",
            BroadcastText::Stop => "Detener transmisión",
            BroadcastText::Connecting => "Conectando con el servidor de transmisión",
            BroadcastText::Live => "En directo",
            BroadcastText::Badge => "EN DIRECTO",
            BroadcastText::Started => "Estás en directo",
            BroadcastText::Ended => "Transmisión finalizada",
            BroadcastText::NeedKey => "Añade primero tu clave de transmisión en Ajustes.",
            BroadcastText::NeedServer => "Añade primero la dirección del servidor en Ajustes.",
            BroadcastText::KeyNotKept => "No se pudo guardar la clave de transmisión en este PC.",
            BroadcastText::NoWorker => "La transmisión no forma parte de esta versión.",
            BroadcastText::BadKey => {
                "El servicio no aceptó tu clave de transmisión. Revísala en Ajustes."
            }
            BroadcastText::NoServer => {
                "No se pudo conectar con el servidor de transmisión. Revisa la dirección y tu conexión."
            }
            BroadcastText::CertUntrusted => {
                "Este PC no confía en el certificado de seguridad del servicio. Revisa la dirección del servidor."
            }
            BroadcastText::CertExpired => {
                "El certificado de seguridad del servicio no es válido ahora mismo. Revisa la fecha y la hora de tu PC."
            }
            BroadcastText::SecureFailed => {
                "No se pudo establecer una conexión segura con el servicio. Revisa la dirección y tu conexión."
            }
            BroadcastText::Lost => {
                "Se perdió la conexión con el servicio. La transmisión se detuvo."
            }
            BroadcastText::NoEncoder => {
                "Para transmitir hace falta un codificador de video, y este PC no tiene ninguno."
            }
            BroadcastText::NoWindow => "La transmisión no encontró la ventana del juego.",
            BroadcastText::Failed => "La transmisión se detuvo.",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            BroadcastText::Title => "Diffusion",
            BroadcastText::Service => "Diffuser sur",
            BroadcastText::ServiceDetail => "Montre le jeu en cours en direct, avec son audio",
            BroadcastText::Twitch => "Twitch",
            BroadcastText::YouTube => "YouTube",
            BroadcastText::Custom => "RTMP personnalisé",
            BroadcastText::Server => "Adresse du serveur",
            BroadcastText::ServerDetail => "Commence par rtmp:// ou rtmps://",
            BroadcastText::Key => "Clé de diffusion",
            BroadcastText::KeyDetail => {
                "Colle la clé de ton tableau de bord. Elle reste sur ce PC et n’est jamais affichée"
            }
            BroadcastText::Quality => "Qualité de diffusion",
            BroadcastText::Quality720p30 => "720p, 30 fps",
            BroadcastText::Quality720p60 => "720p, 60 fps",
            BroadcastText::Quality1080p60 => "1080p, 60 fps",
            BroadcastText::Start => "Démarrer la diffusion",
            BroadcastText::Stop => "Arrêter la diffusion",
            BroadcastText::Connecting => "Connexion au serveur de diffusion",
            BroadcastText::Live => "En direct",
            BroadcastText::Badge => "EN DIRECT",
            BroadcastText::Started => "Tu es en direct",
            BroadcastText::Ended => "Diffusion terminée",
            BroadcastText::NeedKey => "Ajoute d’abord ta clé de diffusion dans Paramètres.",
            BroadcastText::NeedServer => "Ajoute d’abord l’adresse du serveur dans Paramètres.",
            BroadcastText::KeyNotKept => "Impossible de conserver la clé de diffusion sur ce PC.",
            BroadcastText::NoWorker => "La diffusion n’est pas incluse dans cette version.",
            BroadcastText::BadKey => {
                "Le service n’a pas accepté ta clé de diffusion. Vérifie-la dans Paramètres."
            }
            BroadcastText::NoServer => {
                "Le serveur de diffusion est injoignable. Vérifie l’adresse et ta connexion."
            }
            BroadcastText::CertUntrusted => {
                "Ce PC ne fait pas confiance au certificat de sécurité du service. Vérifie l’adresse du serveur."
            }
            BroadcastText::CertExpired => {
                "Le certificat de sécurité du service n’est pas valide pour le moment. Vérifie la date et l’heure de ton PC."
            }
            BroadcastText::SecureFailed => {
                "Impossible d’établir une connexion sécurisée avec le service. Vérifie l’adresse et ta connexion."
            }
            BroadcastText::Lost => {
                "La connexion au service a été perdue. La diffusion s’est arrêtée."
            }
            BroadcastText::NoEncoder => {
                "La diffusion nécessite un encodeur vidéo, et ce PC n’en a pas."
            }
            BroadcastText::NoWindow => "La diffusion n’a pas trouvé la fenêtre du jeu.",
            BroadcastText::Failed => "La diffusion s’est arrêtée.",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            BroadcastText::Title => "Transmissão",
            BroadcastText::Service => "Transmitir para",
            BroadcastText::ServiceDetail => "Mostra ao vivo o jogo em execução com o som",
            BroadcastText::Twitch => "Twitch",
            BroadcastText::YouTube => "YouTube",
            BroadcastText::Custom => "RTMP personalizado",
            BroadcastText::Server => "Endereço do servidor",
            BroadcastText::ServerDetail => "Começa com rtmp:// ou rtmps://",
            BroadcastText::Key => "Chave de transmissão",
            BroadcastText::KeyDetail => {
                "Cole a chave do seu painel. Ela fica neste PC e nunca é exibida"
            }
            BroadcastText::Quality => "Qualidade da transmissão",
            BroadcastText::Quality720p30 => "720p, 30 fps",
            BroadcastText::Quality720p60 => "720p, 60 fps",
            BroadcastText::Quality1080p60 => "1080p, 60 fps",
            BroadcastText::Start => "Iniciar transmissão",
            BroadcastText::Stop => "Parar transmissão",
            BroadcastText::Connecting => "Conectando ao servidor de transmissão",
            BroadcastText::Live => "Ao vivo",
            BroadcastText::Badge => "AO VIVO",
            BroadcastText::Started => "Você está ao vivo",
            BroadcastText::Ended => "Transmissão encerrada",
            BroadcastText::NeedKey => {
                "Adicione primeiro sua chave de transmissão em Configurações."
            }
            BroadcastText::NeedServer => {
                "Adicione primeiro o endereço do servidor em Configurações."
            }
            BroadcastText::KeyNotKept => {
                "Não foi possível guardar a chave de transmissão neste PC."
            }
            BroadcastText::NoWorker => "A transmissão não faz parte desta versão.",
            BroadcastText::BadKey => {
                "O serviço não aceitou sua chave de transmissão. Verifique-a em Configurações."
            }
            BroadcastText::NoServer => {
                "Não foi possível alcançar o servidor de transmissão. Verifique o endereço e sua conexão."
            }
            BroadcastText::CertUntrusted => {
                "Este PC não confia no certificado de segurança do serviço. Verifique o endereço do servidor."
            }
            BroadcastText::CertExpired => {
                "O certificado de segurança do serviço não é válido no momento. Verifique a data e a hora do seu PC."
            }
            BroadcastText::SecureFailed => {
                "Não foi possível criar uma conexão segura com o serviço. Verifique o endereço e sua conexão."
            }
            BroadcastText::Lost => "A conexão com o serviço foi perdida. A transmissão parou.",
            BroadcastText::NoEncoder => {
                "Para transmitir é preciso um codificador de vídeo, e este PC não tem nenhum."
            }
            BroadcastText::NoWindow => "A transmissão não encontrou a janela do jogo.",
            BroadcastText::Failed => "A transmissão parou.",
            _ => return None,
        })
    }

    pub const fn text(self, language: Language) -> &'static str {
        match language {
            Language::Turkish => self.turkish(),
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
