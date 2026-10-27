//! Texts of LudusCtrl on the Phone as Controller page (docs/DESIGN.md 18, "LudusCtrl"). English
//! and Turkish like the rest. The words the page shares with the phone server (status, the pairing
//! question, Remove, the toasts) are [`PhoneText`](crate::screens::phone_strings::PhoneText).

use crate::ui::i18n::{Arg, Language, substitute};

/// The project page of LudusCtrl, which explains the app and links where to get it. The Google
/// Play listing (`https://play.google.com/store/apps/details?id=com.parlov.ludusctrl`) opens only
/// for testers until the app is in production, so the page links this one.
pub const PROJECT_PAGE: &str = "https://github.com/parl0v/ludusctrl";
/// The address as the row's detail shows it.
pub const PROJECT_PAGE_SHOWN: &str = "github.com/parl0v/ludusctrl";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LudusText {
    /// The switch and its detail (the app and the browser client both).
    Title,
    SwitchDetail,
    /// The server could not start for the app.
    StartFailed,
    /// The title of the app's status row.
    AppStatus,
    /// Status: a phone holds the pad with no controller connected to it (`%@` = its name).
    NoController,
    /// The caption over the paired LudusCtrl phones, and the line while there is none.
    Paired,
    PairedNone,
    /// Where the app comes from: the note and the row that opens the project page.
    AppNote,
    AppOpen,
    /// Top bar: the accessible name of the status, `%@` = the phone's name.
    BarLabel,
    /// Windows: the firewall question on the first start, for the app and the page alike.
    Firewall,
}

impl LudusText {
    pub const ALL: [LudusText; 11] = [
        LudusText::Title,
        LudusText::SwitchDetail,
        LudusText::StartFailed,
        LudusText::AppStatus,
        LudusText::NoController,
        LudusText::Paired,
        LudusText::PairedNone,
        LudusText::AppNote,
        LudusText::AppOpen,
        LudusText::BarLabel,
        LudusText::Firewall,
    ];

    pub const fn english(self) -> &'static str {
        match self {
            LudusText::Title => "LudusCtrl",
            LudusText::SwitchDetail => {
                "Play with a controller connected to your phone: scan the code with the phone's camera, or use the LudusCtrl app on Android. A phone with the app has to be paired first."
            }
            LudusText::StartFailed => {
                "The connection for the LudusCtrl app could not start. Another program, such as the LudusCtrl receiver, may be using its network port."
            }
            LudusText::AppStatus => "App",
            LudusText::NoController => "Connected to %@, no controller",
            LudusText::Paired => "Paired LudusCtrl Phones",
            LudusText::PairedNone => {
                "No phone is paired yet. Open the LudusCtrl app on your phone and start the link."
            }
            LudusText::AppNote => {
                "LudusCtrl runs on Android phones and tablets. Pair your controller with the phone over Bluetooth, then open the app."
            }
            LudusText::AppOpen => "Open the LudusCtrl Page",
            LudusText::BarLabel => "LudusCtrl connected: %@",
            LudusText::Firewall => {
                "The first time LudusCtrl runs, Windows asks whether AnyPlay Launcher may use the network. Allow it on private networks, or neither the app nor the page can reach this PC."
            }
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            LudusText::Title => "LudusCtrl",
            LudusText::SwitchDetail => {
                "Telefonuna bağlı bir kumandayla oyna: kodu telefonun kamerasıyla tara ya da Android'de LudusCtrl uygulamasını kullan. Uygulamayı kullanan her telefon önce eşleştirilmelidir."
            }
            LudusText::StartFailed => {
                "LudusCtrl uygulamasının bağlantısı başlatılamadı. LudusCtrl alıcısı gibi başka bir program ağ bağlantı noktasını kullanıyor olabilir."
            }
            LudusText::AppStatus => "Uygulama",
            LudusText::NoController => "%@ bağlı, kumanda yok",
            LudusText::Paired => "Eşleştirilmiş LudusCtrl Telefonları",
            LudusText::PairedNone => {
                "Henüz eşleştirilmiş telefon yok. Telefonunda LudusCtrl uygulamasını açıp bağlantıyı başlat."
            }
            LudusText::AppNote => {
                "LudusCtrl, Android telefon ve tabletlerde çalışır. Kumandanı telefonla Bluetooth üzerinden eşleştir, sonra uygulamayı aç."
            }
            LudusText::AppOpen => "LudusCtrl Sayfasını Aç",
            LudusText::BarLabel => "LudusCtrl bağlı: %@",
            LudusText::Firewall => {
                "LudusCtrl ilk kez çalıştığında Windows, AnyPlay Launcher'ın ağı kullanmasına izin verip vermeyeceğini sorar. Özel ağlarda izin ver; yoksa ne uygulama ne de sayfa bu bilgisayara ulaşabilir."
            }
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            LudusText::Title => "LudusCtrl",
            LudusText::SwitchDetail => {
                "Spiele mit einem Controller, der an dein Telefon angeschlossen ist: Scanne den Code mit der Telefonkamera oder nutze die LudusCtrl-App auf Android. Ein Telefon mit der App muss zuerst gekoppelt werden."
            }
            LudusText::StartFailed => {
                "Die Verbindung für die LudusCtrl-App konnte nicht gestartet werden. Möglicherweise nutzt ein anderes Programm, etwa der LudusCtrl-Empfänger, den Netzwerkport."
            }
            LudusText::AppStatus => "App",
            LudusText::NoController => "Verbunden mit %@, kein Controller",
            LudusText::Paired => "Gekoppelte LudusCtrl-Telefone",
            LudusText::PairedNone => {
                "Noch kein Telefon gekoppelt. Öffne die LudusCtrl-App auf deinem Telefon und starte die Verbindung."
            }
            LudusText::AppNote => {
                "LudusCtrl läuft auf Android-Telefonen und -Tablets. Koppele deinen Controller per Bluetooth mit dem Telefon und öffne dann die App."
            }
            LudusText::AppOpen => "LudusCtrl-Seite öffnen",
            LudusText::BarLabel => "LudusCtrl verbunden: %@",
            LudusText::Firewall => {
                "Beim ersten Start von LudusCtrl fragt Windows, ob der AnyPlay Launcher das Netzwerk nutzen darf. Erlaube es in privaten Netzwerken, sonst erreichen weder App noch Seite diesen PC."
            }
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            LudusText::Title => "LudusCtrl",
            LudusText::SwitchDetail => {
                "Juega con un mando conectado a tu teléfono: escanea el código con la cámara del teléfono o usa la app LudusCtrl en Android. Un teléfono con la app debe vincularse primero."
            }
            LudusText::StartFailed => {
                "No se pudo iniciar la conexión para la app LudusCtrl. Puede que otro programa, como el receptor de LudusCtrl, esté usando su puerto de red."
            }
            LudusText::AppStatus => "App",
            LudusText::NoController => "Conectado a %@, sin mando",
            LudusText::Paired => "Teléfonos LudusCtrl vinculados",
            LudusText::PairedNone => {
                "Todavía no hay ningún teléfono vinculado. Abre la app LudusCtrl en tu teléfono e inicia el enlace."
            }
            LudusText::AppNote => {
                "LudusCtrl funciona en teléfonos y tablets Android. Vincula tu mando al teléfono por Bluetooth y abre la app."
            }
            LudusText::AppOpen => "Abrir la página de LudusCtrl",
            LudusText::BarLabel => "LudusCtrl conectado: %@",
            LudusText::Firewall => {
                "La primera vez que se ejecuta LudusCtrl, Windows pregunta si AnyPlay Launcher puede usar la red. Permítelo en redes privadas; si no, ni la app ni la página podrán acceder a este PC."
            }
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            LudusText::Title => "LudusCtrl",
            LudusText::SwitchDetail => {
                "Joue avec une manette connectée à ton téléphone\u{a0}: scanne le code avec l’appareil photo du téléphone ou utilise l’appli LudusCtrl sur Android. Un téléphone avec l’appli doit d’abord être associé."
            }
            LudusText::StartFailed => {
                "La connexion pour l’appli LudusCtrl n’a pas pu démarrer. Un autre programme, comme le récepteur LudusCtrl, utilise peut-être son port réseau."
            }
            LudusText::AppStatus => "Appli",
            LudusText::NoController => "Connecté à %@, sans manette",
            LudusText::Paired => "Téléphones LudusCtrl associés",
            LudusText::PairedNone => {
                "Aucun téléphone n’est encore associé. Ouvre l’appli LudusCtrl sur ton téléphone et lance la liaison."
            }
            LudusText::AppNote => {
                "LudusCtrl fonctionne sur les téléphones et tablettes Android. Associe ta manette au téléphone en Bluetooth, puis ouvre l’appli."
            }
            LudusText::AppOpen => "Ouvrir la page de LudusCtrl",
            LudusText::BarLabel => "LudusCtrl connecté\u{a0}: %@",
            LudusText::Firewall => {
                "Au premier lancement de LudusCtrl, Windows demande si AnyPlay Launcher peut utiliser le réseau. Autorise-le sur les réseaux privés, sinon ni l’appli ni la page ne pourront joindre ce PC."
            }
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            LudusText::Title => "LudusCtrl",
            LudusText::SwitchDetail => {
                "Jogue com um controle conectado ao seu celular: escaneie o código com a câmera do celular ou use o app LudusCtrl no Android. Um celular com o app precisa ser pareado antes."
            }
            LudusText::StartFailed => {
                "Não foi possível iniciar a conexão do app LudusCtrl. Outro programa, como o receptor do LudusCtrl, pode estar usando a porta de rede dele."
            }
            LudusText::AppStatus => "App",
            LudusText::NoController => "Conectado a %@, sem controle",
            LudusText::Paired => "Celulares LudusCtrl pareados",
            LudusText::PairedNone => {
                "Nenhum celular pareado ainda. Abra o app LudusCtrl no seu celular e inicie a conexão."
            }
            LudusText::AppNote => {
                "O LudusCtrl roda em celulares e tablets Android. Pareie seu controle com o celular por Bluetooth e abra o app."
            }
            LudusText::AppOpen => "Abrir a página do LudusCtrl",
            LudusText::BarLabel => "LudusCtrl conectado: %@",
            LudusText::Firewall => {
                "Na primeira vez que o LudusCtrl roda, o Windows pergunta se o AnyPlay Launcher pode usar a rede. Permita em redes privadas; senão, nem o app nem a página alcançam este PC."
            }
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
        for text in LudusText::ALL {
            for language in Language::ALL {
                assert!(
                    !text.text(language).trim().is_empty(),
                    "{text:?} {language:?}"
                );
            }
        }
    }

    #[test]
    fn the_name_reaches_the_bar_label_in_both_languages() {
        for language in Language::ALL {
            let text = LudusText::BarLabel.format(language, &[Arg::Str("Pixel 8")]);
            assert!(
                text.contains("Pixel 8") && text.contains("LudusCtrl"),
                "{text}"
            );
            assert!(!text.contains("%@"), "{text}");
        }
    }

    #[test]
    fn the_words_name_no_console_brand() {
        for text in LudusText::ALL {
            for language in Language::ALL {
                let words = text.text(language).to_lowercase();
                for brand in [
                    "playstation",
                    "dualsense",
                    "dualshock",
                    "xbox",
                    "ps5",
                    "sony",
                ] {
                    assert!(!words.contains(brand), "{text:?} {language:?}: {brand}");
                }
            }
        }
    }

    #[test]
    fn the_project_page_is_secure_and_matches_what_is_shown() {
        assert!(PROJECT_PAGE.starts_with("https://github.com/"));
        assert_eq!(
            PROJECT_PAGE.trim_start_matches("https://"),
            PROJECT_PAGE_SHOWN
        );
    }
}
