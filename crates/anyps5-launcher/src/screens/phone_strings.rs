//! Texts of Phone as Controller (docs/DESIGN.md 18). English and Turkish like the rest.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PhoneText {
    /// The settings category and the switch.
    Title,
    SwitchDetail,
    RemoveToTurnOff,
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
            PhoneText::RemoveToTurnOff => "Remove paired phones to turn this off.",
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
            PhoneText::RemoveToTurnOff => "Kapatmak için eşleştirilmiş telefonları kaldır.",
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
            PhoneText::RemoveToTurnOff => "Entferne gekoppelte Telefone, um dies auszuschalten.",
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

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            PhoneText::Title => "Teléfono como mando",
            PhoneText::SwitchDetail => {
                "Usa tu teléfono como mando. Solo pueden conectarse teléfonos de tu red doméstica y cada uno debe emparejarse antes."
            }
            PhoneText::RemoveToTurnOff => "Elimina los teléfonos emparejados para desactivarlo.",
            PhoneText::Status => "Estado",
            PhoneText::Waiting => "Esperando un teléfono",
            PhoneText::ConnectedTo => "Conectado a %@",
            PhoneText::ConnectingTo => "Conectando con %@",
            PhoneText::StartFailed => {
                "No se pudo iniciar. Puede que otro programa esté usando el puerto de red."
            }
            PhoneText::DelayValue => "%d ms",
            PhoneText::Address => "Dirección",
            PhoneText::AddressNote => {
                "Si la app no encuentra este PC por sí sola, escribe esta dirección en ella."
            }
            PhoneText::PortBusy => {
                "El puerto 47700 está en uso, así que la app no puede encontrar este PC por su dirección hasta que se libere."
            }
            PhoneText::NotAnnounced => {
                "Este PC no pudo anunciarse en la red. Escribe la dirección en la app."
            }
            PhoneText::PairAsk => "¿Emparejar %@?",
            PhoneText::Code => "Código",
            PhoneText::CodeNote => "Empareja solo si tu teléfono muestra el mismo código.",
            PhoneText::PairAllow => "Emparejar teléfono",
            PhoneText::PairDeny => "Rechazar",
            PhoneText::Paired => "Teléfonos emparejados",
            PhoneText::PairedNone => {
                "Aún no hay ningún teléfono emparejado. Abre la app del teléfono y elige este PC."
            }
            PhoneText::Remove => "Quitar %@",
            PhoneText::PairedOn => "Emparejado el %@",
            PhoneText::Controller => "Mando virtual",
            PhoneText::ControllerReady => "Listo",
            PhoneText::ControllerMissing => "No instalado",
            PhoneText::ControllerNoAccess => "Sin acceso",
            PhoneText::ControllerUnsupported => "No disponible",
            PhoneText::ViGemNote => {
                "Los juegos ven tu teléfono como un mando gracias a un pequeño controlador, ViGEmBus. El launcher nunca lo instala por ti: descárgalo desde su página oficial e instálalo tú mismo. Su autor retiró el proyecto, pero sigue funcionando en el Windows actual."
            }
            PhoneText::ViGemOpen => "Abrir la página de descarga de ViGEmBus",
            PhoneText::UinputNote => {
                "Los juegos ven tu teléfono como un mando a través de uinput. El launcher aún no puede abrirlo. Carga el módulo con modprobe uinput y da a tu usuario acceso de escritura a /dev/uinput."
            }
            PhoneText::UinputRule => {
                "Una regla udev lo resuelve para siempre: KERNEL==\"uinput\", GROUP=\"input\", MODE=\"0660\". Añádete al grupo input y vuelve a iniciar sesión."
            }
            PhoneText::UinputOpen => "Abrir la guía",
            PhoneText::ToastPair => "%@ quiere emparejarse. Abre Ajustes, Teléfono como mando.",
            PhoneText::ToastConnected => "%@ conectado",
            PhoneText::ToastFull => {
                "Hay ocho teléfonos emparejados. Quita uno antes de emparejar otro."
            }
            PhoneText::ToastRemoved => "%@ quitado",
            PhoneText::BarLabel => "Teléfono conectado: %@",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            PhoneText::Title => "Téléphone comme manette",
            PhoneText::SwitchDetail => {
                "Utilise ton téléphone comme manette. Seuls les téléphones de ton réseau domestique peuvent se connecter, et chacun doit d’abord être associé."
            }
            PhoneText::RemoveToTurnOff => "Retire les téléphones associés pour le désactiver.",
            PhoneText::Status => "État",
            PhoneText::Waiting => "En attente d’un téléphone",
            PhoneText::ConnectedTo => "Connecté à %@",
            PhoneText::ConnectingTo => "Connexion à %@",
            PhoneText::StartFailed => {
                "Démarrage impossible. Un autre programme utilise peut-être le port réseau."
            }
            PhoneText::DelayValue => "%d\u{a0}ms",
            PhoneText::Address => "Adresse",
            PhoneText::AddressNote => {
                "Si l’appli ne trouve pas ce PC toute seule, saisis cette adresse dedans."
            }
            PhoneText::PortBusy => {
                "Le port 47700 est utilisé, donc l’appli ne peut pas trouver ce PC par son adresse tant qu’il n’est pas libre."
            }
            PhoneText::NotAnnounced => {
                "Ce PC n’a pas pu s’annoncer sur le réseau. Saisis l’adresse dans l’appli."
            }
            PhoneText::PairAsk => "Associer %@\u{a0}?",
            PhoneText::Code => "Code",
            PhoneText::CodeNote => "Associe seulement si ton téléphone affiche le même code.",
            PhoneText::PairAllow => "Associer le téléphone",
            PhoneText::PairDeny => "Refuser",
            PhoneText::Paired => "Téléphones associés",
            PhoneText::PairedNone => {
                "Aucun téléphone associé pour l’instant. Ouvre l’appli du téléphone et choisis ce PC."
            }
            PhoneText::Remove => "Supprimer %@",
            PhoneText::PairedOn => "Associé le %@",
            PhoneText::Controller => "Manette virtuelle",
            PhoneText::ControllerReady => "Prêt",
            PhoneText::ControllerMissing => "Non installé",
            PhoneText::ControllerNoAccess => "Pas d’accès",
            PhoneText::ControllerUnsupported => "Indisponible",
            PhoneText::ViGemNote => {
                "Les jeux voient ton téléphone comme une manette grâce à un petit pilote, ViGEmBus. Le launcher ne l’installe jamais à ta place\u{a0}: télécharge-le depuis sa page officielle et installe-le toi-même. Son auteur a abandonné le projet, mais il fonctionne toujours sur le Windows actuel."
            }
            PhoneText::ViGemOpen => "Ouvrir la page de téléchargement de ViGEmBus",
            PhoneText::UinputNote => {
                "Les jeux voient ton téléphone comme une manette via uinput. Le launcher ne peut pas encore l’ouvrir. Charge le module avec modprobe uinput et donne à ton utilisateur l’accès en écriture à /dev/uinput."
            }
            PhoneText::UinputRule => {
                "Une règle udev règle cela définitivement\u{a0}: KERNEL==\"uinput\", GROUP=\"input\", MODE=\"0660\". Ajoute-toi au groupe input, puis reconnecte-toi."
            }
            PhoneText::UinputOpen => "Ouvrir le guide",
            PhoneText::ToastPair => {
                "%@ veut s’associer. Ouvre Paramètres, Téléphone comme manette."
            }
            PhoneText::ToastConnected => "%@ connecté",
            PhoneText::ToastFull => {
                "Huit téléphones sont associés. Supprime-en un avant d’en associer un autre."
            }
            PhoneText::ToastRemoved => "%@ supprimé",
            PhoneText::BarLabel => "Téléphone connecté\u{a0}: %@",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            PhoneText::Title => "Celular como controle",
            PhoneText::SwitchDetail => {
                "Use seu celular como controle. Só celulares da sua rede doméstica podem se conectar, e cada um precisa ser pareado antes."
            }
            PhoneText::RemoveToTurnOff => "Remova os celulares pareados para desativar.",
            PhoneText::Status => "Status",
            PhoneText::Waiting => "Aguardando um celular",
            PhoneText::ConnectedTo => "Conectado a %@",
            PhoneText::ConnectingTo => "Conectando a %@",
            PhoneText::StartFailed => {
                "Não foi possível iniciar. Outro programa pode estar usando a porta de rede."
            }
            PhoneText::DelayValue => "%d ms",
            PhoneText::Address => "Endereço",
            PhoneText::AddressNote => {
                "Se o app não encontrar este PC sozinho, digite este endereço nele."
            }
            PhoneText::PortBusy => {
                "A porta 47700 está em uso, então o app não consegue encontrar este PC pelo endereço até que ela seja liberada."
            }
            PhoneText::NotAnnounced => {
                "Este PC não conseguiu se anunciar na rede. Digite o endereço no app."
            }
            PhoneText::PairAsk => "Parear %@?",
            PhoneText::Code => "Código",
            PhoneText::CodeNote => "Pareie só se seu celular mostrar o mesmo código.",
            PhoneText::PairAllow => "Parear celular",
            PhoneText::PairDeny => "Recusar",
            PhoneText::Paired => "Celulares pareados",
            PhoneText::PairedNone => {
                "Nenhum celular pareado ainda. Abra o app do celular e escolha este PC."
            }
            PhoneText::Remove => "Remover %@",
            PhoneText::PairedOn => "Pareado em %@",
            PhoneText::Controller => "Controle virtual",
            PhoneText::ControllerReady => "Pronto",
            PhoneText::ControllerMissing => "Não instalado",
            PhoneText::ControllerNoAccess => "Sem acesso",
            PhoneText::ControllerUnsupported => "Indisponível",
            PhoneText::ViGemNote => {
                "Os jogos veem seu celular como um controle por meio de um pequeno driver, o ViGEmBus. O launcher nunca o instala por você: baixe-o da página oficial e instale você mesmo. O autor aposentou o projeto, mas ele ainda funciona no Windows atual."
            }
            PhoneText::ViGemOpen => "Abrir a página de download do ViGEmBus",
            PhoneText::UinputNote => {
                "Os jogos veem seu celular como um controle via uinput. O launcher ainda não consegue abri-lo. Carregue o módulo com modprobe uinput e dê ao seu usuário acesso de gravação a /dev/uinput."
            }
            PhoneText::UinputRule => {
                "Uma regra udev resolve isso de vez: KERNEL==\"uinput\", GROUP=\"input\", MODE=\"0660\". Adicione-se ao grupo input e entre de novo na sessão."
            }
            PhoneText::UinputOpen => "Abrir o guia",
            PhoneText::ToastPair => "%@ quer parear. Abra Configurações, Celular como controle.",
            PhoneText::ToastConnected => "%@ conectado",
            PhoneText::ToastFull => {
                "Oito celulares estão pareados. Remova um antes de parear outro."
            }
            PhoneText::ToastRemoved => "%@ removido",
            PhoneText::BarLabel => "Celular conectado: %@",
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
