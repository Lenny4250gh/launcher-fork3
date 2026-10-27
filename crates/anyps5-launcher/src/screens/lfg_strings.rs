//! Texts of Find Players (looking for group) and of sending captures to a friend: the panels, the
//! media bubble and the toasts. English and Turkish like the rest.

use crate::ui::i18n::{Arg, Language, substitute};

macro_rules! texts {
    ($($(#[$doc:meta])* $name:ident => ($en:expr, $tr:expr)),* $(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum LfgText {
            $($(#[$doc])* $name,)*
        }

        impl LfgText {
            pub const fn english(self) -> &'static str {
                match self {
                    $(LfgText::$name => $en,)*
                }
            }

            pub const fn turkish(self) -> &'static str {
                match self {
                    $(LfgText::$name => $tr,)*
                }
            }

            pub const ALL: &'static [LfgText] = &[$(LfgText::$name,)*];
        }
    };
}

texts! {
    // The entry in a game's options.
    /// The row in the game's options and the subtitle of the panel.
    FindPlayers => ("Find Players", "Oyuncu Bul"),
    FindPlayersDetail => (
        "Players who own this game and look for a group",
        "Bu oyuna sahip olup grup arayan oyuncular"
    ),
    // The list page.
    PostListing => ("Post a Listing", "İlan Ver"),
    PostListingDetail => ("Open for 1 hour", "1 saat açık kalır"),
    YourListing => ("Your Listing", "İlanın"),
    /// `%@` = requests, `%@` = minutes left.
    YourListingDetail => ("%@ requests · %@ min left", "%@ istek · %@ dk kaldı"),
    OpenListings => ("Open Listings", "Açık İlanlar"),
    FilterMode => ("Mode", "Mod"),
    FilterLanguage => ("Language", "Dil"),
    ModeAll => ("All Modes", "Tüm Modlar"),
    ModeOnline => ("Online", "Çevrimiçi"),
    ModeLan => ("LAN", "LAN"),
    ModePlayTogether => ("Play Together", "Birlikte Oyna"),
    LanguageAny => ("Any Language", "Her Dil"),
    NobodyTitle => ("Nobody is looking right now", "Şu an kimse aramıyor"),
    NobodyBody => (
        "Post a listing and other owners of this game can ask to join.",
        "Bir ilan ver; bu oyuna sahip diğer oyuncular katılmayı isteyebilir."
    ),
    Loading => ("Looking for listings…", "İlanlar aranıyor…"),
    LoadFailed => ("The listings could not be loaded.", "İlanlar yüklenemedi."),
    TryAgain => ("Try Again", "Tekrar Dene"),
    SignedOut => (
        "Sign in to the AnyPlay network to find players.",
        "Oyuncu bulmak için AnyPlay ağında oturum aç."
    ),
    SignIn => ("Sign In", "Oturum Aç"),
    Offline => (
        "The network is not reachable. Check your connection.",
        "Ağa ulaşılamıyor. Bağlantını kontrol et."
    ),
    /// `%@` = places free, `%@` = places in all.
    PlacesFree => ("%@ of %@ places free", "%@ / %@ yer boş"),
    /// `%@` = minutes.
    MinutesLeft => ("%@ min left", "%@ dk kaldı"),
    // A listing.
    Mode => ("Mode", "Mod"),
    Language => ("Language", "Dil"),
    Places => ("Places", "Yerler"),
    ClosesIn => ("Closes in", "Kapanma"),
    NoNote => ("No note", "Not yok"),
    AskToJoin => ("Ask to Join", "Katılmayı İste"),
    AskToJoinDetail => (
        "The poster decides. You hear back when they accept.",
        "İlan sahibi karar verir. Kabul ederse haber alırsın."
    ),
    RequestSent => ("Request sent", "İstek gönderildi"),
    RequestSentNote => (
        "Waiting for the poster to answer.",
        "İlan sahibinin cevabı bekleniyor."
    ),
    Withdraw => ("Withdraw Request", "İsteği Geri Çek"),
    Accepted => ("Accepted", "Kabul edildi"),
    AcceptedNote => (
        "Their party invitation is waiting in Friends.",
        "Parti davetleri Arkadaşlar'da seni bekliyor."
    ),
    OpenFriends => ("Open Friends", "Arkadaşlar'ı Aç"),
    Full => ("Full", "Dolu"),
    ReportListing => ("Report Listing", "İlanı Bildir"),
    ReportTitle => ("Why do you report this listing?", "Bu ilanı neden bildiriyorsun?"),
    ReportSpam => ("Spam", "Spam"),
    ReportHarassment => ("Harassment", "Taciz"),
    ReportOther => ("Something else", "Başka bir şey"),
    /// `%@` = handle.
    BlockPoster => ("Block %@", "%@ kişisini engelle"),
    // Your listing.
    Requests => ("Requests", "İstekler"),
    NoRequests => ("Nobody asked yet.", "Henüz kimse istemedi."),
    Accept => ("Accept", "Kabul Et"),
    Decline => ("Decline", "Reddet"),
    CloseListing => ("Close Listing", "İlanı Kapat"),
    // Posting.
    PostTitle => ("Post a Listing", "İlan Ver"),
    Players => ("Players wanted", "Aranan oyuncu"),
    /// `%@` = how many.
    PlayersMany => ("%@ players", "%@ oyuncu"),
    PlayersOne => ("1 player", "1 oyuncu"),
    Note => ("Note", "Not"),
    /// `%@` = characters used, `%@` = the most.
    NoteCount => ("%@ of %@ characters", "%@ / %@ karakter"),
    PostNow => ("Post for 1 Hour", "1 Saatliğine Yayınla"),
    PostLimit => (
        "Close one of your listings first. You can have one per game and three in all.",
        "Önce ilanlarından birini kapat. Oyun başına bir, toplamda üç ilanın olabilir."
    ),
    PostRate => (
        "You posted a lot of listings. Try again in a few minutes.",
        "Çok fazla ilan verdin. Birkaç dakika sonra tekrar dene."
    ),
    PostRestricted => (
        "Your account cannot post listings right now.",
        "Hesabın şu an ilan veremiyor."
    ),
    PostBadNote => (
        "A note has at most 140 characters and no links.",
        "Not en fazla 140 karakter olabilir ve bağlantı içeremez."
    ),
    PostFailed => ("The listing could not be posted. Try again.", "İlan yayınlanamadı. Tekrar dene."),
    // Toasts.
    ToastPosted => ("Listing posted for 1 hour", "İlan 1 saatliğine yayınlandı"),
    ToastClosed => ("Listing closed", "İlan kapatıldı"),
    ToastAsked => ("Request sent", "İstek gönderildi"),
    ToastAccepted => (
        "Accepted. They get a party invitation from you.",
        "Kabul edildi. Senden bir parti daveti alacaklar."
    ),
    /// `%@` = handle.
    ToastRequest => ("%@ wants to join your listing", "%@ ilanına katılmak istiyor"),
    /// `%@` = handle.
    ToastAcceptedBy => (
        "%@ accepted your request. Their party invitation is in Friends.",
        "%@ isteğini kabul etti. Parti daveti Arkadaşlar'da."
    ),
    // Sending captures.
    SendTitle => ("Send to a Friend", "Bir Arkadaşa Gönder"),
    /// `%@` = the file's name.
    SendSubtitle => ("%@", "%@"),
    NoFriends => (
        "You have no friends to send to yet. Add a friend first.",
        "Henüz gönderebileceğin arkadaşın yok. Önce bir arkadaş ekle."
    ),
    SendSignedOut => (
        "Sign in to the AnyPlay network to send captures.",
        "Yakalama göndermek için AnyPlay ağında oturum aç."
    ),
    SendFriendsOnly => (
        "Captures go to friends only.",
        "Yakalamalar yalnızca arkadaşlara gönderilir."
    ),
    SendNote => (
        "Screenshots up to 10 MB, clips up to 100 MB and 60 seconds. The cloud keeps them for 30 days.",
        "Ekran görüntüleri 10 MB'a, klipler 100 MB'a ve 60 saniyeye kadar. Bulut 30 gün saklar."
    ),
    /// `%@` = handle.
    Sending => ("Sending to %@…", "%@ kişisine gönderiliyor…"),
    /// `%@` = handle.
    Sent => ("Sent to %@", "%@ kişisine gönderildi"),
    TooLargeImage => (
        "Screenshots can be up to 10 MB.",
        "Ekran görüntüleri en fazla 10 MB olabilir."
    ),
    TooLargeClip => (
        "Clips can be up to 100 MB.",
        "Klipler en fazla 100 MB olabilir."
    ),
    TooLongClip => (
        "Clips can be up to 60 seconds.",
        "Klipler en fazla 60 saniye olabilir."
    ),
    Unsupported => (
        "Only PNG and JPEG screenshots and MP4 clips can be sent.",
        "Yalnızca PNG ve JPEG ekran görüntüleri ile MP4 klipler gönderilebilir."
    ),
    EmptyFile => ("The file is empty.", "Dosya boş."),
    SendFailed => (
        "The capture could not be sent. Try again.",
        "Yakalama gönderilemedi. Tekrar dene."
    ),
    SendRestricted => (
        "Your account cannot send captures right now.",
        "Hesabın şu an yakalama gönderemiyor."
    ),
    SendRate => (
        "You sent a lot of captures. Try again in a few minutes.",
        "Çok fazla yakalama gönderdin. Birkaç dakika sonra tekrar dene."
    ),
    // The bubble.
    Screenshot => ("Screenshot", "Ekran görüntüsü"),
    Clip => ("Clip", "Klip"),
    Unavailable => ("No longer available", "Artık mevcut değil"),
    OpenMedia => ("Open", "Aç"),
    OpenMediaDetail => ("Downloads the file and opens it", "Dosyayı indirir ve açar"),
    OpenFailed => ("The file could not be opened.", "Dosya açılamadı."),
    Downloading => ("Downloading…", "İndiriliyor…"),
}

impl LfgText {
    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            LfgText::FindPlayers => "Mitspieler finden",
            LfgText::FindPlayersDetail => {
                "Spieler, die dieses Spiel besitzen und eine Gruppe suchen"
            }
            LfgText::PostListing => "Gesuch erstellen",
            LfgText::PostListingDetail => "1 Stunde lang offen",
            LfgText::YourListing => "Dein Gesuch",
            LfgText::YourListingDetail => "%@ Anfragen · noch %@ Min.",
            LfgText::OpenListings => "Offene Gesuche",
            LfgText::FilterMode => "Modus",
            LfgText::FilterLanguage => "Sprache",
            LfgText::ModeAll => "Alle Modi",
            LfgText::ModeOnline => "Online",
            LfgText::ModeLan => "LAN",
            LfgText::ModePlayTogether => "Gemeinsam spielen",
            LfgText::LanguageAny => "Jede Sprache",
            LfgText::NobodyTitle => "Gerade sucht niemand",
            LfgText::NobodyBody => {
                "Erstelle ein Gesuch, dann können andere Besitzer dieses Spiels anfragen."
            }
            LfgText::Loading => "Gesuche werden gesucht …",
            LfgText::LoadFailed => "Die Gesuche konnten nicht geladen werden.",
            LfgText::TryAgain => "Erneut versuchen",
            LfgText::SignedOut => "Melde dich im AnyPlay-Netzwerk an, um Mitspieler zu finden.",
            LfgText::SignIn => "Anmelden",
            LfgText::Offline => "Das Netzwerk ist nicht erreichbar. Prüfe deine Verbindung.",
            LfgText::PlacesFree => "%@ von %@ Plätzen frei",
            LfgText::MinutesLeft => "noch %@ Min.",
            LfgText::Mode => "Modus",
            LfgText::Language => "Sprache",
            LfgText::Places => "Plätze",
            LfgText::ClosesIn => "Endet in",
            LfgText::NoNote => "Keine Notiz",
            LfgText::AskToJoin => "Beitritt anfragen",
            LfgText::AskToJoinDetail => {
                "Der Ersteller entscheidet. Du hörst von ihm, wenn er annimmt."
            }
            LfgText::RequestSent => "Anfrage gesendet",
            LfgText::RequestSentNote => "Warte auf die Antwort des Erstellers.",
            LfgText::Withdraw => "Anfrage zurückziehen",
            LfgText::Accepted => "Angenommen",
            LfgText::AcceptedNote => "Die Gruppeneinladung wartet unter Freunde auf dich.",
            LfgText::OpenFriends => "Freunde öffnen",
            LfgText::Full => "Voll",
            LfgText::ReportListing => "Gesuch melden",
            LfgText::ReportTitle => "Warum meldest du dieses Gesuch?",
            LfgText::ReportSpam => "Spam",
            LfgText::ReportHarassment => "Belästigung",
            LfgText::ReportOther => "Etwas anderes",
            LfgText::BlockPoster => "%@ blockieren",
            LfgText::Requests => "Anfragen",
            LfgText::NoRequests => "Noch hat niemand angefragt.",
            LfgText::Accept => "Annehmen",
            LfgText::Decline => "Ablehnen",
            LfgText::CloseListing => "Gesuch schließen",
            LfgText::PostTitle => "Gesuch erstellen",
            LfgText::Players => "Gesuchte Spieler",
            LfgText::PlayersMany => "%@ Spieler",
            LfgText::PlayersOne => "1 Spieler",
            LfgText::Note => "Notiz",
            LfgText::NoteCount => "%@ von %@ Zeichen",
            LfgText::PostNow => "1 Stunde lang veröffentlichen",
            LfgText::PostLimit => {
                "Schließe zuerst eines deiner Gesuche. Du kannst eines pro Spiel und drei insgesamt haben."
            }
            LfgText::PostRate => {
                "Du hast viele Gesuche erstellt. Versuche es in ein paar Minuten erneut."
            }
            LfgText::PostRestricted => "Dein Konto kann gerade keine Gesuche erstellen.",
            LfgText::PostBadNote => "Eine Notiz hat höchstens 140 Zeichen und keine Links.",
            LfgText::PostFailed => {
                "Das Gesuch konnte nicht veröffentlicht werden. Versuche es erneut."
            }
            LfgText::ToastPosted => "Gesuch für 1 Stunde veröffentlicht",
            LfgText::ToastClosed => "Gesuch geschlossen",
            LfgText::ToastAsked => "Anfrage gesendet",
            LfgText::ToastAccepted => "Angenommen. Sie erhalten eine Gruppeneinladung von dir.",
            LfgText::ToastRequest => "%@ möchte deinem Gesuch beitreten",
            LfgText::ToastAcceptedBy => {
                "%@ hat deine Anfrage angenommen. Die Gruppeneinladung ist unter Freunde."
            }
            LfgText::SendTitle => "An einen Freund senden",
            LfgText::SendSubtitle => "%@",
            LfgText::NoFriends => {
                "Du hast noch keine Freunde, an die du senden kannst. Füge zuerst einen Freund hinzu."
            }
            LfgText::SendSignedOut => "Melde dich im AnyPlay-Netzwerk an, um Aufnahmen zu senden.",
            LfgText::SendFriendsOnly => "Aufnahmen gehen nur an Freunde.",
            LfgText::SendNote => {
                "Screenshots bis 10 MB, Clips bis 100 MB und 60 Sekunden. Die Cloud speichert sie 30 Tage."
            }
            LfgText::Sending => "Wird an %@ gesendet …",
            LfgText::Sent => "An %@ gesendet",
            LfgText::TooLargeImage => "Screenshots dürfen höchstens 10 MB groß sein.",
            LfgText::TooLargeClip => "Clips dürfen höchstens 100 MB groß sein.",
            LfgText::TooLongClip => "Clips dürfen höchstens 60 Sekunden lang sein.",
            LfgText::Unsupported => {
                "Nur PNG- und JPEG-Screenshots und MP4-Clips können gesendet werden."
            }
            LfgText::EmptyFile => "Die Datei ist leer.",
            LfgText::SendFailed => "Die Aufnahme konnte nicht gesendet werden. Versuche es erneut.",
            LfgText::SendRestricted => "Dein Konto kann gerade keine Aufnahmen senden.",
            LfgText::SendRate => {
                "Du hast viele Aufnahmen gesendet. Versuche es in ein paar Minuten erneut."
            }
            LfgText::Screenshot => "Screenshot",
            LfgText::Clip => "Clip",
            LfgText::Unavailable => "Nicht mehr verfügbar",
            LfgText::OpenMedia => "Öffnen",
            LfgText::OpenMediaDetail => "Lädt die Datei herunter und öffnet sie",
            LfgText::OpenFailed => "Die Datei konnte nicht geöffnet werden.",
            LfgText::Downloading => "Wird heruntergeladen …",
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            LfgText::FindPlayers => "Buscar jugadores",
            LfgText::FindPlayersDetail => "Jugadores que tienen este juego y buscan grupo",
            LfgText::PostListing => "Publicar anuncio",
            LfgText::PostListingDetail => "Abierto durante 1 hora",
            LfgText::YourListing => "Tu anuncio",
            LfgText::YourListingDetail => "%@ solicitudes · cierra en %@ min",
            LfgText::OpenListings => "Anuncios abiertos",
            LfgText::FilterMode => "Modo",
            LfgText::FilterLanguage => "Idioma",
            LfgText::ModeAll => "Todos los modos",
            LfgText::ModeOnline => "En línea",
            LfgText::ModeLan => "LAN",
            LfgText::ModePlayTogether => "Play Together",
            LfgText::LanguageAny => "Cualquier idioma",
            LfgText::NobodyTitle => "Ahora mismo no busca nadie",
            LfgText::NobodyBody => {
                "Publica un anuncio y otros dueños de este juego podrán pedir unirse."
            }
            LfgText::Loading => "Buscando anuncios…",
            LfgText::LoadFailed => "No se pudieron cargar los anuncios.",
            LfgText::TryAgain => "Reintentar",
            LfgText::SignedOut => "Inicia sesión en la red de AnyPlay para encontrar jugadores.",
            LfgText::SignIn => "Iniciar sesión",
            LfgText::Offline => "No se puede acceder a la red. Comprueba tu conexión.",
            LfgText::PlacesFree => "%@ de %@ plazas libres",
            LfgText::MinutesLeft => "Cierra en %@ min",
            LfgText::Mode => "Modo",
            LfgText::Language => "Idioma",
            LfgText::Places => "Plazas",
            LfgText::ClosesIn => "Cierra en",
            LfgText::NoNote => "Sin nota",
            LfgText::AskToJoin => "Pedir unirse",
            LfgText::AskToJoinDetail => {
                "Decide quien publicó el anuncio. Te avisarán si te aceptan."
            }
            LfgText::RequestSent => "Solicitud enviada",
            LfgText::RequestSentNote => "Esperando la respuesta de quien publicó el anuncio.",
            LfgText::Withdraw => "Retirar solicitud",
            LfgText::Accepted => "Aceptada",
            LfgText::AcceptedNote => "Su invitación a la Party te espera en Amigos.",
            LfgText::OpenFriends => "Abrir Amigos",
            LfgText::Full => "Completo",
            LfgText::ReportListing => "Denunciar anuncio",
            LfgText::ReportTitle => "¿Por qué denuncias este anuncio?",
            LfgText::ReportSpam => "Spam",
            LfgText::ReportHarassment => "Acoso",
            LfgText::ReportOther => "Otra cosa",
            LfgText::BlockPoster => "Bloquear a %@",
            LfgText::Requests => "Solicitudes",
            LfgText::NoRequests => "Nadie ha pedido unirse todavía.",
            LfgText::Accept => "Aceptar",
            LfgText::Decline => "Rechazar",
            LfgText::CloseListing => "Cerrar anuncio",
            LfgText::PostTitle => "Publicar anuncio",
            LfgText::Players => "Jugadores buscados",
            LfgText::PlayersMany => "%@ jugadores",
            LfgText::PlayersOne => "1 jugador",
            LfgText::Note => "Nota",
            LfgText::NoteCount => "%@ de %@ caracteres",
            LfgText::PostNow => "Publicar durante 1 hora",
            LfgText::PostLimit => {
                "Cierra primero uno de tus anuncios. Puedes tener uno por juego y tres en total."
            }
            LfgText::PostRate => {
                "Has publicado muchos anuncios. Inténtalo de nuevo en unos minutos."
            }
            LfgText::PostRestricted => "Tu cuenta no puede publicar anuncios ahora mismo.",
            LfgText::PostBadNote => {
                "Una nota tiene como máximo 140 caracteres y no puede incluir enlaces."
            }
            LfgText::PostFailed => "No se pudo publicar el anuncio. Inténtalo de nuevo.",
            LfgText::ToastPosted => "Anuncio publicado durante 1 hora",
            LfgText::ToastClosed => "Anuncio cerrado",
            LfgText::ToastAsked => "Solicitud enviada",
            LfgText::ToastAccepted => "Aceptada. Recibirán de ti una invitación a la Party.",
            LfgText::ToastRequest => "%@ quiere unirse a tu anuncio",
            LfgText::ToastAcceptedBy => {
                "%@ aceptó tu solicitud. Su invitación a la Party está en Amigos."
            }
            LfgText::SendTitle => "Enviar a un amigo",
            LfgText::SendSubtitle => "%@",
            LfgText::NoFriends => "Aún no tienes amigos a quienes enviar. Primero añade un amigo.",
            LfgText::SendSignedOut => "Inicia sesión en la red de AnyPlay para enviar capturas.",
            LfgText::SendFriendsOnly => "Las capturas solo se envían a amigos.",
            LfgText::SendNote => {
                "Capturas de pantalla de hasta 10 MB, clips de hasta 100 MB y 60 segundos. La nube los guarda 30 días."
            }
            LfgText::Sending => "Enviando a %@…",
            LfgText::Sent => "Enviado a %@",
            LfgText::TooLargeImage => "Las capturas de pantalla pueden pesar hasta 10 MB.",
            LfgText::TooLargeClip => "Los clips pueden pesar hasta 100 MB.",
            LfgText::TooLongClip => "Los clips pueden durar hasta 60 segundos.",
            LfgText::Unsupported => "Solo se pueden enviar capturas PNG y JPEG y clips MP4.",
            LfgText::EmptyFile => "El archivo está vacío.",
            LfgText::SendFailed => "No se pudo enviar la captura. Inténtalo de nuevo.",
            LfgText::SendRestricted => "Tu cuenta no puede enviar capturas ahora mismo.",
            LfgText::SendRate => "Has enviado muchas capturas. Inténtalo de nuevo en unos minutos.",
            LfgText::Screenshot => "Captura de pantalla",
            LfgText::Clip => "Clip",
            LfgText::Unavailable => "Ya no está disponible",
            LfgText::OpenMedia => "Abrir",
            LfgText::OpenMediaDetail => "Descarga el archivo y lo abre",
            LfgText::OpenFailed => "No se pudo abrir el archivo.",
            LfgText::Downloading => "Descargando…",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            LfgText::FindPlayers => "Trouver des joueurs",
            LfgText::FindPlayersDetail => "Joueurs qui possèdent ce jeu et cherchent un groupe",
            LfgText::PostListing => "Publier une annonce",
            LfgText::PostListingDetail => "Ouverte pendant 1 heure",
            LfgText::YourListing => "Ton annonce",
            LfgText::YourListingDetail => "%@ demandes · fermeture dans %@ min",
            LfgText::OpenListings => "Annonces ouvertes",
            LfgText::FilterMode => "Mode",
            LfgText::FilterLanguage => "Langue",
            LfgText::ModeAll => "Tous les modes",
            LfgText::ModeOnline => "En ligne",
            LfgText::ModeLan => "LAN",
            LfgText::ModePlayTogether => "Play Together",
            LfgText::LanguageAny => "Toutes les langues",
            LfgText::NobodyTitle => "Personne ne cherche pour l’instant",
            LfgText::NobodyBody => {
                "Publie une annonce et les autres joueurs qui possèdent ce jeu pourront demander à te rejoindre."
            }
            LfgText::Loading => "Recherche d’annonces…",
            LfgText::LoadFailed => "Impossible de charger les annonces.",
            LfgText::TryAgain => "Réessayer",
            LfgText::SignedOut => "Connecte-toi au réseau AnyPlay pour trouver des joueurs.",
            LfgText::SignIn => "Se connecter",
            LfgText::Offline => "Le réseau est injoignable. Vérifie ta connexion.",
            LfgText::PlacesFree => "%@ sur %@ places libres",
            LfgText::MinutesLeft => "Ferme dans %@ min",
            LfgText::Mode => "Mode",
            LfgText::Language => "Langue",
            LfgText::Places => "Places",
            LfgText::ClosesIn => "Fermeture dans",
            LfgText::NoNote => "Aucune note",
            LfgText::AskToJoin => "Demander à rejoindre",
            LfgText::AskToJoinDetail => {
                "C’est l’auteur de l’annonce qui décide. Tu es prévenu s’il accepte."
            }
            LfgText::RequestSent => "Demande envoyée",
            LfgText::RequestSentNote => "En attente de la réponse de l’auteur.",
            LfgText::Withdraw => "Retirer la demande",
            LfgText::Accepted => "Acceptée",
            LfgText::AcceptedNote => "Son invitation à la Party t’attend dans Amis.",
            LfgText::OpenFriends => "Ouvrir Amis",
            LfgText::Full => "Complet",
            LfgText::ReportListing => "Signaler l’annonce",
            LfgText::ReportTitle => "Pourquoi signales-tu cette annonce\u{a0}?",
            LfgText::ReportSpam => "Spam",
            LfgText::ReportHarassment => "Harcèlement",
            LfgText::ReportOther => "Autre chose",
            LfgText::BlockPoster => "Bloquer %@",
            LfgText::Requests => "Demandes",
            LfgText::NoRequests => "Personne n’a encore fait de demande.",
            LfgText::Accept => "Accepter",
            LfgText::Decline => "Refuser",
            LfgText::CloseListing => "Fermer l’annonce",
            LfgText::PostTitle => "Publier une annonce",
            LfgText::Players => "Joueurs recherchés",
            LfgText::PlayersMany => "%@ joueurs",
            LfgText::PlayersOne => "1 joueur",
            LfgText::Note => "Note",
            LfgText::NoteCount => "%@ sur %@ caractères",
            LfgText::PostNow => "Publier pour 1 heure",
            LfgText::PostLimit => {
                "Ferme d’abord l’une de tes annonces. Tu peux en avoir une par jeu et trois au total."
            }
            LfgText::PostRate => {
                "Tu as publié beaucoup d’annonces. Réessaie dans quelques minutes."
            }
            LfgText::PostRestricted => "Ton compte ne peut pas publier d’annonces pour le moment.",
            LfgText::PostBadNote => {
                "Une note fait 140 caractères au maximum et ne contient pas de liens."
            }
            LfgText::PostFailed => "L’annonce n’a pas pu être publiée. Réessaie.",
            LfgText::ToastPosted => "Annonce publiée pour 1 heure",
            LfgText::ToastClosed => "Annonce fermée",
            LfgText::ToastAsked => "Demande envoyée",
            LfgText::ToastAccepted => "Demande acceptée. Une invitation à ta Party est envoyée.",
            LfgText::ToastRequest => "%@ veut rejoindre ton annonce",
            LfgText::ToastAcceptedBy => {
                "%@ a accepté ta demande. L’invitation à la Party est dans Amis."
            }
            LfgText::SendTitle => "Envoyer à un ami",
            LfgText::SendSubtitle => "%@",
            LfgText::NoFriends => "Tu n’as pas encore d’ami à qui envoyer. Ajoute d’abord un ami.",
            LfgText::SendSignedOut => "Connecte-toi au réseau AnyPlay pour envoyer des captures.",
            LfgText::SendFriendsOnly => "Les captures ne vont qu’aux amis.",
            LfgText::SendNote => {
                "Captures d’écran jusqu’à 10\u{a0}Mo, clips jusqu’à 100\u{a0}Mo et 60 secondes. Le cloud les garde 30 jours."
            }
            LfgText::Sending => "Envoi à %@…",
            LfgText::Sent => "Envoyé à %@",
            LfgText::TooLargeImage => "Les captures d’écran font 10\u{a0}Mo au maximum.",
            LfgText::TooLargeClip => "Les clips font 100\u{a0}Mo au maximum.",
            LfgText::TooLongClip => "Les clips durent 60 secondes au maximum.",
            LfgText::Unsupported => {
                "Seuls les captures d’écran PNG et JPEG et les clips MP4 peuvent être envoyés."
            }
            LfgText::EmptyFile => "Le fichier est vide.",
            LfgText::SendFailed => "La capture n’a pas pu être envoyée. Réessaie.",
            LfgText::SendRestricted => "Ton compte ne peut pas envoyer de captures pour le moment.",
            LfgText::SendRate => {
                "Tu as envoyé beaucoup de captures. Réessaie dans quelques minutes."
            }
            LfgText::Screenshot => "Capture d’écran",
            LfgText::Clip => "Clip",
            LfgText::Unavailable => "Plus disponible",
            LfgText::OpenMedia => "Ouvrir",
            LfgText::OpenMediaDetail => "Télécharge le fichier et l’ouvre",
            LfgText::OpenFailed => "Le fichier n’a pas pu être ouvert.",
            LfgText::Downloading => "Téléchargement…",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            LfgText::FindPlayers => "Encontrar jogadores",
            LfgText::FindPlayersDetail => "Jogadores que têm este jogo e procuram um grupo",
            LfgText::PostListing => "Publicar anúncio",
            LfgText::PostListingDetail => "Aberto por 1 hora",
            LfgText::YourListing => "Seu anúncio",
            LfgText::YourListingDetail => "%@ pedidos · fecha em %@ min",
            LfgText::OpenListings => "Anúncios abertos",
            LfgText::FilterMode => "Modo",
            LfgText::FilterLanguage => "Idioma",
            LfgText::ModeAll => "Todos os modos",
            LfgText::ModeOnline => "Online",
            LfgText::ModeLan => "LAN",
            LfgText::ModePlayTogether => "Play Together",
            LfgText::LanguageAny => "Qualquer idioma",
            LfgText::NobodyTitle => "Ninguém está procurando agora",
            LfgText::NobodyBody => {
                "Publique um anúncio e outros donos deste jogo poderão pedir para entrar."
            }
            LfgText::Loading => "Procurando anúncios…",
            LfgText::LoadFailed => "Não foi possível carregar os anúncios.",
            LfgText::TryAgain => "Tentar de novo",
            LfgText::SignedOut => "Entre na rede AnyPlay para encontrar jogadores.",
            LfgText::SignIn => "Entrar",
            LfgText::Offline => "Não foi possível acessar a rede. Verifique sua conexão.",
            LfgText::PlacesFree => "%@ de %@ vagas livres",
            LfgText::MinutesLeft => "Fecha em %@ min",
            LfgText::Mode => "Modo",
            LfgText::Language => "Idioma",
            LfgText::Places => "Vagas",
            LfgText::ClosesIn => "Fecha em",
            LfgText::NoNote => "Sem nota",
            LfgText::AskToJoin => "Pedir para entrar",
            LfgText::AskToJoinDetail => "Quem publicou decide. Você é avisado se for aceito.",
            LfgText::RequestSent => "Pedido enviado",
            LfgText::RequestSentNote => "Aguardando a resposta de quem publicou.",
            LfgText::Withdraw => "Retirar pedido",
            LfgText::Accepted => "Aceito",
            LfgText::AcceptedNote => "O convite para a Party está esperando você em Amigos.",
            LfgText::OpenFriends => "Abrir Amigos",
            LfgText::Full => "Lotado",
            LfgText::ReportListing => "Denunciar anúncio",
            LfgText::ReportTitle => "Por que você está denunciando este anúncio?",
            LfgText::ReportSpam => "Spam",
            LfgText::ReportHarassment => "Assédio",
            LfgText::ReportOther => "Outra coisa",
            LfgText::BlockPoster => "Bloquear %@",
            LfgText::Requests => "Pedidos",
            LfgText::NoRequests => "Ninguém pediu ainda.",
            LfgText::Accept => "Aceitar",
            LfgText::Decline => "Recusar",
            LfgText::CloseListing => "Fechar anúncio",
            LfgText::PostTitle => "Publicar anúncio",
            LfgText::Players => "Jogadores procurados",
            LfgText::PlayersMany => "%@ jogadores",
            LfgText::PlayersOne => "1 jogador",
            LfgText::Note => "Nota",
            LfgText::NoteCount => "%@ de %@ caracteres",
            LfgText::PostNow => "Publicar por 1 hora",
            LfgText::PostLimit => {
                "Feche antes um dos seus anúncios. Você pode ter um por jogo e três no total."
            }
            LfgText::PostRate => "Você publicou muitos anúncios. Tente de novo em alguns minutos.",
            LfgText::PostRestricted => "Sua conta não pode publicar anúncios agora.",
            LfgText::PostBadNote => "Uma nota tem no máximo 140 caracteres e não pode ter links.",
            LfgText::PostFailed => "Não foi possível publicar o anúncio. Tente de novo.",
            LfgText::ToastPosted => "Anúncio publicado por 1 hora",
            LfgText::ToastClosed => "Anúncio fechado",
            LfgText::ToastAsked => "Pedido enviado",
            LfgText::ToastAccepted => "Aceito. Eles vão receber um convite seu para a Party.",
            LfgText::ToastRequest => "%@ quer entrar no seu anúncio",
            LfgText::ToastAcceptedBy => {
                "%@ aceitou seu pedido. O convite para a Party está em Amigos."
            }
            LfgText::SendTitle => "Enviar a um amigo",
            LfgText::SendSubtitle => "%@",
            LfgText::NoFriends => {
                "Você ainda não tem amigos para quem enviar. Adicione um amigo primeiro."
            }
            LfgText::SendSignedOut => "Entre na rede AnyPlay para enviar capturas.",
            LfgText::SendFriendsOnly => "As capturas só vão para amigos.",
            LfgText::SendNote => {
                "Capturas de tela de até 10 MB, clipes de até 100 MB e 60 segundos. A nuvem guarda por 30 dias."
            }
            LfgText::Sending => "Enviando para %@…",
            LfgText::Sent => "Enviado para %@",
            LfgText::TooLargeImage => "Capturas de tela podem ter até 10 MB.",
            LfgText::TooLargeClip => "Clipes podem ter até 100 MB.",
            LfgText::TooLongClip => "Clipes podem ter até 60 segundos.",
            LfgText::Unsupported => {
                "Só é possível enviar capturas de tela PNG e JPEG e clipes MP4."
            }
            LfgText::EmptyFile => "O arquivo está vazio.",
            LfgText::SendFailed => "Não foi possível enviar a captura. Tente de novo.",
            LfgText::SendRestricted => "Sua conta não pode enviar capturas agora.",
            LfgText::SendRate => "Você enviou muitas capturas. Tente de novo em alguns minutos.",
            LfgText::Screenshot => "Captura de tela",
            LfgText::Clip => "Clipe",
            LfgText::Unavailable => "Não está mais disponível",
            LfgText::OpenMedia => "Abrir",
            LfgText::OpenMediaDetail => "Baixa o arquivo e o abre",
            LfgText::OpenFailed => "Não foi possível abrir o arquivo.",
            LfgText::Downloading => "Baixando…",
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
    fn every_text_exists_in_both_languages_with_the_same_placeholders() {
        for text in LfgText::ALL {
            for language in Language::ALL {
                assert!(!text.text(language).is_empty(), "{text:?} {language:?}");
            }
            for language in Language::ALL {
                assert_eq!(
                    text.english().matches("%@").count(),
                    text.text(language).matches("%@").count(),
                    "{text:?} {language:?}"
                );
            }
        }
    }
}
