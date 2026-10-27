//! Texts of the profile page and the feed in the online area. English and Turkish like the rest.
//! Row titles are in title case, notes and the sentences of the feed in sentence case.
//!
//! The sentences of the feed are told from the account they are about: "earned a gold trophy in
//! Nebula Drift" after "Lumen". For the player's own items the subject is "You", and Turkish
//! needs its own verb forms there (`*Own`). Arguments are filled in order; where Turkish wants
//! another order, the caller passes them in that order (see `feed::sentence`).

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FeedText {
    // Entry rows
    /// The row on Home and the page's title.
    Feed,
    FeedDetail,
    FeedSubtitle,
    /// The row on Home that opens the player's own profile.
    YourProfile,
    YourProfileDetail,
    /// The row on a friend's page.
    ViewProfile,
    // Feed states
    Loading,
    Empty,
    EmptyNote,
    Offline,
    OfflineNote,
    LoadFailedTitle,
    LoadFailed,
    TryAgain,
    ShowOlder,
    /// The "Show Older Activity" row after a refusal.
    OlderFailed,
    /// The profile page's title.
    Profile,
    EndOfFeed,
    // Sentences. The subject is the handle before them.
    /// `%@` = the grade (gold), `%@` = the title of the trophy, `%@` = the game.
    TrophyNamed,
    TrophyNamedOwn,
    /// `%@` = the grade, `%@` = the game.
    Trophy,
    TrophyOwn,
    /// `%@` = the grade.
    TrophyUnknownGame,
    TrophyUnknownGameOwn,
    /// `%@` = who.
    Friend,
    FriendOwn,
    /// The player is the one the account became friends with.
    FriendWithYou,
    /// `%@` = the game.
    FirstPlayed,
    FirstPlayedOwn,
    FirstPlayedUnknown,
    FirstPlayedUnknownOwn,
    /// `%@` = who.
    Party,
    PartyOwn,
    PartyAlone,
    PartyAloneOwn,
    ListTwo,
    ListThree,
    /// `%@` = first, `%@` = second, `%@` = how many more.
    ListMore,
    // Trophy facts
    GradePlatinum,
    GradeGold,
    GradeSilver,
    GradeBronze,
    RarityExtreme,
    RarityRare,
    RarityUncommon,
    RarityCommon,
    /// `%@` = the word (Rare), `%@` = the percent (2.3).
    RarityFact,
    // Time
    AgoNow,
    /// `%@` = how many.
    AgoMinutes,
    AgoHours,
    AgoDays,
    AgoYesterday,
    // Profile page
    /// Shown for the player's own profile.
    OwnSubtitle,
    ProfileLoading,
    ProfileFailed,
    Favourites,
    /// A favourite the player does not have: `%@` = the title ID.
    NotInLibrary,
    MostPlayed,
    LinkedAccounts,
    Discord,
    Steam,
    /// A connection whose name the provider did not give.
    Linked,
    TrophiesPrivate,
    EditProfile,
    EditProfileDetail,
    OwnEmpty,
    /// `%@` = hours, `%@` = minutes.
    HoursMinutes,
    /// `%@` = hours.
    Hours,
    /// `%@` = minutes.
    Minutes,
    /// The accessible name of a banner: `%@` = the handle.
    BannerName,
}

impl FeedText {
    pub const fn english(self) -> &'static str {
        match self {
            FeedText::Feed => "Feed",
            FeedText::FeedDetail => "What your friends are up to",
            FeedText::FeedSubtitle => "Recent activity from you and your friends",
            FeedText::YourProfile => "Your Profile",
            FeedText::YourProfileDetail => "What other players see",
            FeedText::ViewProfile => "View Profile",
            FeedText::Loading => "Loading…",
            FeedText::Empty => "Nothing here yet",
            FeedText::EmptyNote => {
                "Trophies, new friends, first games and parties show up here, for you and your friends."
            }
            FeedText::Offline => "You are offline",
            FeedText::OfflineNote => "The feed comes back when you are online again.",
            FeedText::LoadFailedTitle => "The feed did not load",
            FeedText::LoadFailed => "Check your connection and try again.",
            FeedText::TryAgain => "Try Again",
            FeedText::ShowOlder => "Show Older Activity",
            FeedText::OlderFailed => "Could not load. Select to try again.",
            FeedText::Profile => "Profile",
            FeedText::EndOfFeed => "You are all caught up.",
            FeedText::TrophyNamed => "earned the %@ trophy “%@” in %@",
            FeedText::TrophyNamedOwn => "earned the %@ trophy “%@” in %@",
            FeedText::Trophy => "earned a %@ trophy in %@",
            FeedText::TrophyOwn => "earned a %@ trophy in %@",
            FeedText::TrophyUnknownGame => "earned a %@ trophy in a game",
            FeedText::TrophyUnknownGameOwn => "earned a %@ trophy in a game",
            FeedText::Friend => "became friends with %@",
            FeedText::FriendOwn => "became friends with %@",
            FeedText::FriendWithYou => "became friends with you",
            FeedText::FirstPlayed => "played %@ for the first time",
            FeedText::FirstPlayedOwn => "played %@ for the first time",
            FeedText::FirstPlayedUnknown => "played a game for the first time",
            FeedText::FirstPlayedUnknownOwn => "played a game for the first time",
            FeedText::Party => "started a party with %@",
            FeedText::PartyOwn => "started a party with %@",
            FeedText::PartyAlone => "started a party",
            FeedText::PartyAloneOwn => "started a party",
            FeedText::ListTwo => "%@ and %@",
            FeedText::ListThree => "%@, %@ and %@",
            FeedText::ListMore => "%@, %@ and %@ more",
            FeedText::GradePlatinum => "platinum",
            FeedText::GradeGold => "gold",
            FeedText::GradeSilver => "silver",
            FeedText::GradeBronze => "bronze",
            FeedText::RarityExtreme => "Extremely rare",
            FeedText::RarityRare => "Rare",
            FeedText::RarityUncommon => "Uncommon",
            FeedText::RarityCommon => "Common",
            FeedText::RarityFact => "%@, %@%%",
            FeedText::AgoNow => "Just now",
            FeedText::AgoMinutes => "%@ min ago",
            FeedText::AgoHours => "%@ h ago",
            FeedText::AgoDays => "%@ d ago",
            FeedText::AgoYesterday => "Yesterday",
            FeedText::OwnSubtitle => "This is what other players see",
            FeedText::ProfileLoading => "Loading…",
            FeedText::ProfileFailed => {
                "Unable to load this profile. Check your connection and try again."
            }
            FeedText::Favourites => "Favourite Games",
            FeedText::NotInLibrary => "%@, not in your library",
            FeedText::MostPlayed => "Most Played",
            FeedText::LinkedAccounts => "Linked Accounts",
            FeedText::Discord => "Discord",
            FeedText::Steam => "Steam",
            FeedText::Linked => "Linked",
            FeedText::TrophiesPrivate => "Trophies are private",
            FeedText::EditProfile => "Edit Profile",
            FeedText::EditProfileDetail => "Opens your account on the web",
            FeedText::OwnEmpty => {
                "Your profile is empty. Add a short text, favourite games and a picture on the web."
            }
            FeedText::HoursMinutes => "%@ h %@ min",
            FeedText::Hours => "%@ h",
            FeedText::Minutes => "%@ min",
            FeedText::BannerName => "Profile of %@",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            FeedText::Feed => "Akış",
            FeedText::FeedDetail => "Arkadaşların neler yapıyor",
            FeedText::FeedSubtitle => "Senin ve arkadaşlarının son hareketleri",
            FeedText::YourProfile => "Profilin",
            FeedText::YourProfileDetail => "Diğer oyuncuların gördüğü profil",
            FeedText::ViewProfile => "Profili Gör",
            FeedText::Loading => "Yükleniyor…",
            FeedText::Empty => "Henüz bir şey yok",
            FeedText::EmptyNote => {
                "Kupalar, yeni arkadaşlar, ilk oynanan oyunlar ve partiler burada görünür."
            }
            FeedText::Offline => "Çevrimdışısın",
            FeedText::OfflineNote => "Tekrar çevrimiçi olunca akış geri gelir.",
            FeedText::LoadFailedTitle => "Akış yüklenemedi",
            FeedText::LoadFailed => "Bağlantını kontrol edip tekrar dene.",
            FeedText::TryAgain => "Tekrar Dene",
            FeedText::ShowOlder => "Eski Hareketleri Göster",
            FeedText::OlderFailed => "Yüklenemedi. Tekrar denemek için seç.",
            FeedText::Profile => "Profil",
            FeedText::EndOfFeed => "Hepsini gördün.",
            FeedText::TrophyNamed => "%@ oyununda %@ kupa kazandı: “%@”",
            FeedText::TrophyNamedOwn => "%@ oyununda %@ kupa kazandın: “%@”",
            FeedText::Trophy => "%@ oyununda %@ kupa kazandı",
            FeedText::TrophyOwn => "%@ oyununda %@ kupa kazandın",
            FeedText::TrophyUnknownGame => "Bir oyunda %@ kupa kazandı",
            FeedText::TrophyUnknownGameOwn => "Bir oyunda %@ kupa kazandın",
            FeedText::Friend => "%@ ile arkadaş oldu",
            FeedText::FriendOwn => "%@ ile arkadaş oldun",
            FeedText::FriendWithYou => "seninle arkadaş oldu",
            FeedText::FirstPlayed => "%@ oyununu ilk kez oynadı",
            FeedText::FirstPlayedOwn => "%@ oyununu ilk kez oynadın",
            FeedText::FirstPlayedUnknown => "Bir oyunu ilk kez oynadı",
            FeedText::FirstPlayedUnknownOwn => "Bir oyunu ilk kez oynadın",
            FeedText::Party => "%@ ile parti kurdu",
            FeedText::PartyOwn => "%@ ile parti kurdun",
            FeedText::PartyAlone => "Parti kurdu",
            FeedText::PartyAloneOwn => "Parti kurdun",
            FeedText::ListTwo => "%@ ve %@",
            FeedText::ListThree => "%@, %@ ve %@",
            FeedText::ListMore => "%@, %@ ve %@ kişi daha",
            FeedText::GradePlatinum => "platin",
            FeedText::GradeGold => "altın",
            FeedText::GradeSilver => "gümüş",
            FeedText::GradeBronze => "bronz",
            FeedText::RarityExtreme => "Çok nadir",
            FeedText::RarityRare => "Nadir",
            FeedText::RarityUncommon => "Seyrek",
            FeedText::RarityCommon => "Yaygın",
            FeedText::RarityFact => "%@, %%%@",
            FeedText::AgoNow => "Az önce",
            FeedText::AgoMinutes => "%@ dk önce",
            FeedText::AgoHours => "%@ sa önce",
            FeedText::AgoDays => "%@ gün önce",
            FeedText::AgoYesterday => "Dün",
            FeedText::OwnSubtitle => "Diğer oyuncular profilini böyle görür",
            FeedText::ProfileLoading => "Yükleniyor…",
            FeedText::ProfileFailed => "Profil yüklenemedi. Bağlantını kontrol edip tekrar dene.",
            FeedText::Favourites => "Favori Oyunlar",
            FeedText::NotInLibrary => "%@, kitaplığında yok",
            FeedText::MostPlayed => "En Çok Oynanan",
            FeedText::LinkedAccounts => "Bağlı Hesaplar",
            FeedText::Discord => "Discord",
            FeedText::Steam => "Steam",
            FeedText::Linked => "Bağlı",
            FeedText::TrophiesPrivate => "Kupalar gizli",
            FeedText::EditProfile => "Profili Düzenle",
            FeedText::EditProfileDetail => "Hesabını webde açar",
            FeedText::OwnEmpty => {
                "Profilin boş. Kısa bir yazı, favori oyunlar ve bir resim eklemek için web sitesini aç."
            }
            FeedText::HoursMinutes => "%@ sa %@ dk",
            FeedText::Hours => "%@ sa",
            FeedText::Minutes => "%@ dk",
            FeedText::BannerName => "%@ Kullanıcısının Profili",
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            FeedText::Feed => "Feed",
            FeedText::FeedDetail => "Was deine Freunde gerade machen",
            FeedText::FeedSubtitle => "Letzte Aktivität von dir und deinen Freunden",
            FeedText::YourProfile => "Dein Profil",
            FeedText::YourProfileDetail => "Das sehen andere Spieler",
            FeedText::ViewProfile => "Profil ansehen",
            FeedText::Loading => "Wird geladen…",
            FeedText::Empty => "Noch nichts hier",
            FeedText::EmptyNote => {
                "Trophäen, neue Freunde, erste Spiele und Partys erscheinen hier, für dich und deine Freunde."
            }
            FeedText::Offline => "Du bist offline",
            FeedText::OfflineNote => "Der Feed kommt zurück, sobald du wieder online bist.",
            FeedText::LoadFailedTitle => "Der Feed wurde nicht geladen",
            FeedText::LoadFailed => "Prüfe deine Verbindung und versuche es erneut.",
            FeedText::TryAgain => "Erneut versuchen",
            FeedText::ShowOlder => "Ältere Aktivität anzeigen",
            FeedText::OlderFailed => {
                "Konnte nicht geladen werden. Wähle aus, um es erneut zu versuchen."
            }
            FeedText::Profile => "Profil",
            FeedText::EndOfFeed => "Du bist auf dem neuesten Stand.",
            FeedText::TrophyNamed => "hat die %@-Trophäe „%@“ in %@ erhalten",
            FeedText::TrophyNamedOwn => "hast die %@-Trophäe „%@“ in %@ erhalten",
            FeedText::Trophy => "hat eine %@-Trophäe in %@ erhalten",
            FeedText::TrophyOwn => "hast eine %@-Trophäe in %@ erhalten",
            FeedText::TrophyUnknownGame => "hat eine %@-Trophäe in einem Spiel erhalten",
            FeedText::TrophyUnknownGameOwn => "hast eine %@-Trophäe in einem Spiel erhalten",
            FeedText::Friend => "ist jetzt mit %@ befreundet",
            FeedText::FriendOwn => "bist jetzt mit %@ befreundet",
            FeedText::FriendWithYou => "ist jetzt mit dir befreundet",
            FeedText::FirstPlayed => "hat %@ zum ersten Mal gespielt",
            FeedText::FirstPlayedOwn => "hast %@ zum ersten Mal gespielt",
            FeedText::FirstPlayedUnknown => "hat ein Spiel zum ersten Mal gespielt",
            FeedText::FirstPlayedUnknownOwn => "hast ein Spiel zum ersten Mal gespielt",
            FeedText::Party => "hat eine Party mit %@ gestartet",
            FeedText::PartyOwn => "hast eine Party mit %@ gestartet",
            FeedText::PartyAlone => "hat eine Party gestartet",
            FeedText::PartyAloneOwn => "hast eine Party gestartet",
            FeedText::ListTwo => "%@ und %@",
            FeedText::ListThree => "%@, %@ und %@",
            FeedText::ListMore => "%@, %@ und %@ weitere",
            FeedText::GradePlatinum => "Platin",
            FeedText::GradeGold => "Gold",
            FeedText::GradeSilver => "Silber",
            FeedText::GradeBronze => "Bronze",
            FeedText::RarityExtreme => "Extrem selten",
            FeedText::RarityRare => "Selten",
            FeedText::RarityUncommon => "Ungewöhnlich",
            FeedText::RarityCommon => "Häufig",
            FeedText::RarityFact => "%@, %@%%",
            FeedText::AgoNow => "Gerade eben",
            FeedText::AgoMinutes => "vor %@ Min.",
            FeedText::AgoHours => "vor %@ Std.",
            FeedText::AgoDays => "vor %@ Tg.",
            FeedText::AgoYesterday => "Gestern",
            FeedText::OwnSubtitle => "Das sehen andere Spieler",
            FeedText::ProfileLoading => "Wird geladen…",
            FeedText::ProfileFailed => {
                "Dieses Profil konnte nicht geladen werden. Prüfe deine Verbindung und versuche es erneut."
            }
            FeedText::Favourites => "Lieblingsspiele",
            FeedText::NotInLibrary => "%@, nicht in deiner Bibliothek",
            FeedText::MostPlayed => "Am meisten gespielt",
            FeedText::LinkedAccounts => "Verknüpfte Konten",
            FeedText::Discord => "Discord",
            FeedText::Steam => "Steam",
            FeedText::Linked => "Verknüpft",
            FeedText::TrophiesPrivate => "Trophäen sind privat",
            FeedText::EditProfile => "Profil bearbeiten",
            FeedText::EditProfileDetail => "Öffnet dein Konto im Web",
            FeedText::OwnEmpty => {
                "Dein Profil ist leer. Füge im Web einen kurzen Text, Lieblingsspiele und ein Bild hinzu."
            }
            FeedText::HoursMinutes => "%@ Std. %@ Min.",
            FeedText::Hours => "%@ Std.",
            FeedText::Minutes => "%@ Min.",
            FeedText::BannerName => "Profil von %@",
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            FeedText::Feed => "Actividad",
            FeedText::FeedDetail => "Lo que hacen tus amigos",
            FeedText::FeedSubtitle => "Actividad reciente tuya y de tus amigos",
            FeedText::YourProfile => "Tu perfil",
            FeedText::YourProfileDetail => "Lo que ven otros jugadores",
            FeedText::ViewProfile => "Ver perfil",
            FeedText::Loading => "Cargando…",
            FeedText::Empty => "Aún no hay nada",
            FeedText::EmptyNote => {
                "Aquí aparecen trofeos, amigos nuevos, primeras partidas y parties, tuyos y de tus amigos."
            }
            FeedText::Offline => "Estás sin conexión",
            FeedText::OfflineNote => "La actividad vuelve cuando te conectes de nuevo.",
            FeedText::LoadFailedTitle => "No se cargó la actividad",
            FeedText::LoadFailed => "Comprueba tu conexión e inténtalo de nuevo.",
            FeedText::TryAgain => "Reintentar",
            FeedText::ShowOlder => "Mostrar actividad anterior",
            FeedText::OlderFailed => "No se pudo cargar. Selecciona para reintentar.",
            FeedText::Profile => "Perfil",
            FeedText::EndOfFeed => "Estás al día.",
            FeedText::TrophyNamed => "obtuvo el trofeo de %@ «%@» en %@",
            FeedText::TrophyNamedOwn => "obtuviste el trofeo de %@ «%@» en %@",
            FeedText::Trophy => "obtuvo un trofeo de %@ en %@",
            FeedText::TrophyOwn => "obtuviste un trofeo de %@ en %@",
            FeedText::TrophyUnknownGame => "obtuvo un trofeo de %@ en un juego",
            FeedText::TrophyUnknownGameOwn => "obtuviste un trofeo de %@ en un juego",
            FeedText::Friend => "ahora es amigo de %@",
            FeedText::FriendOwn => "ahora eres amigo de %@",
            FeedText::FriendWithYou => "ahora es amigo tuyo",
            FeedText::FirstPlayed => "jugó a %@ por primera vez",
            FeedText::FirstPlayedOwn => "jugaste a %@ por primera vez",
            FeedText::FirstPlayedUnknown => "jugó a un juego por primera vez",
            FeedText::FirstPlayedUnknownOwn => "jugaste a un juego por primera vez",
            FeedText::Party => "creó una party con %@",
            FeedText::PartyOwn => "creaste una party con %@",
            FeedText::PartyAlone => "creó una party",
            FeedText::PartyAloneOwn => "creaste una party",
            FeedText::ListTwo => "%@ y %@",
            FeedText::ListThree => "%@, %@ y %@",
            FeedText::ListMore => "%@, %@ y %@ más",
            FeedText::GradePlatinum => "platino",
            FeedText::GradeGold => "oro",
            FeedText::GradeSilver => "plata",
            FeedText::GradeBronze => "bronce",
            FeedText::RarityExtreme => "Extremadamente raro",
            FeedText::RarityRare => "Raro",
            FeedText::RarityUncommon => "Poco común",
            FeedText::RarityCommon => "Común",
            FeedText::RarityFact => "%@, %@\u{a0}%%",
            FeedText::AgoNow => "Ahora mismo",
            FeedText::AgoMinutes => "hace %@ min",
            FeedText::AgoHours => "hace %@ h",
            FeedText::AgoDays => "hace %@ d",
            FeedText::AgoYesterday => "Ayer",
            FeedText::OwnSubtitle => "Esto es lo que ven otros jugadores",
            FeedText::ProfileLoading => "Cargando…",
            FeedText::ProfileFailed => {
                "No se pudo cargar este perfil. Comprueba tu conexión e inténtalo de nuevo."
            }
            FeedText::Favourites => "Juegos favoritos",
            FeedText::NotInLibrary => "%@, no está en tu biblioteca",
            FeedText::MostPlayed => "Más jugados",
            FeedText::LinkedAccounts => "Cuentas vinculadas",
            FeedText::Discord => "Discord",
            FeedText::Steam => "Steam",
            FeedText::Linked => "Vinculada",
            FeedText::TrophiesPrivate => "Los trofeos son privados",
            FeedText::EditProfile => "Editar perfil",
            FeedText::EditProfileDetail => "Abre tu cuenta en la web",
            FeedText::OwnEmpty => {
                "Tu perfil está vacío. Añade en la web un texto breve, juegos favoritos y una imagen."
            }
            FeedText::HoursMinutes => "%@ h %@ min",
            FeedText::Hours => "%@ h",
            FeedText::Minutes => "%@ min",
            FeedText::BannerName => "Perfil de %@",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            FeedText::Feed => "Activité",
            FeedText::FeedDetail => "Ce que font tes amis",
            FeedText::FeedSubtitle => "Activité récente de toi et de tes amis",
            FeedText::YourProfile => "Ton profil",
            FeedText::YourProfileDetail => "Ce que voient les autres joueurs",
            FeedText::ViewProfile => "Voir le profil",
            FeedText::Loading => "Chargement…",
            FeedText::Empty => "Rien ici pour l’instant",
            FeedText::EmptyNote => {
                "Trophées, nouveaux amis, premiers jeux et nouvelles party apparaissent ici, pour toi et tes amis."
            }
            FeedText::Offline => "Tu es hors ligne",
            FeedText::OfflineNote => "L’activité revient dès que tu es de nouveau en ligne.",
            FeedText::LoadFailedTitle => "L’activité n’a pas été chargée",
            FeedText::LoadFailed => "Vérifie ta connexion et réessaie.",
            FeedText::TryAgain => "Réessayer",
            FeedText::ShowOlder => "Afficher les activités plus anciennes",
            FeedText::OlderFailed => "Chargement impossible. Sélectionne pour réessayer.",
            FeedText::Profile => "Profil",
            FeedText::EndOfFeed => "Tu es à jour.",
            FeedText::TrophyNamed => "a obtenu le trophée en %@ «\u{a0}%@\u{a0}» dans %@",
            FeedText::TrophyNamedOwn => "as obtenu le trophée en %@ «\u{a0}%@\u{a0}» dans %@",
            FeedText::Trophy => "a obtenu un trophée en %@ dans %@",
            FeedText::TrophyOwn => "as obtenu un trophée en %@ dans %@",
            FeedText::TrophyUnknownGame => "a obtenu un trophée en %@ dans un jeu",
            FeedText::TrophyUnknownGameOwn => "as obtenu un trophée en %@ dans un jeu",
            FeedText::Friend => "est maintenant ami avec %@",
            FeedText::FriendOwn => "es maintenant ami avec %@",
            FeedText::FriendWithYou => "est maintenant ami avec toi",
            FeedText::FirstPlayed => "a joué à %@ pour la première fois",
            FeedText::FirstPlayedOwn => "as joué à %@ pour la première fois",
            FeedText::FirstPlayedUnknown => "a joué à un jeu pour la première fois",
            FeedText::FirstPlayedUnknownOwn => "as joué à un jeu pour la première fois",
            FeedText::Party => "a lancé une party avec %@",
            FeedText::PartyOwn => "as lancé une party avec %@",
            FeedText::PartyAlone => "a lancé une party",
            FeedText::PartyAloneOwn => "as lancé une party",
            FeedText::ListTwo => "%@ et %@",
            FeedText::ListThree => "%@, %@ et %@",
            FeedText::ListMore => "%@, %@ et %@ de plus",
            FeedText::GradePlatinum => "platine",
            FeedText::GradeGold => "or",
            FeedText::GradeSilver => "argent",
            FeedText::GradeBronze => "bronze",
            FeedText::RarityExtreme => "Extrêmement rare",
            FeedText::RarityRare => "Rare",
            FeedText::RarityUncommon => "Peu courant",
            FeedText::RarityCommon => "Courant",
            FeedText::RarityFact => "%@, %@\u{a0}%%",
            FeedText::AgoNow => "À l’instant",
            FeedText::AgoMinutes => "il y a %@ min",
            FeedText::AgoHours => "il y a %@ h",
            FeedText::AgoDays => "il y a %@ j",
            FeedText::AgoYesterday => "Hier",
            FeedText::OwnSubtitle => "Voici ce que voient les autres joueurs",
            FeedText::ProfileLoading => "Chargement…",
            FeedText::ProfileFailed => {
                "Impossible de charger ce profil. Vérifie ta connexion et réessaie."
            }
            FeedText::Favourites => "Jeux favoris",
            FeedText::NotInLibrary => "%@, absent de ta bibliothèque",
            FeedText::MostPlayed => "Les plus joués",
            FeedText::LinkedAccounts => "Comptes liés",
            FeedText::Discord => "Discord",
            FeedText::Steam => "Steam",
            FeedText::Linked => "Lié",
            FeedText::TrophiesPrivate => "Les trophées sont privés",
            FeedText::EditProfile => "Modifier le profil",
            FeedText::EditProfileDetail => "Ouvre ton compte sur le web",
            FeedText::OwnEmpty => {
                "Ton profil est vide. Ajoute sur le web un court texte, des jeux favoris et une image."
            }
            FeedText::HoursMinutes => "%@ h %@ min",
            FeedText::Hours => "%@ h",
            FeedText::Minutes => "%@ min",
            FeedText::BannerName => "Profil de %@",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            FeedText::Feed => "Atividade",
            FeedText::FeedDetail => "O que seus amigos estão fazendo",
            FeedText::FeedSubtitle => "Atividade recente sua e dos seus amigos",
            FeedText::YourProfile => "Seu perfil",
            FeedText::YourProfileDetail => "O que outros jogadores veem",
            FeedText::ViewProfile => "Ver perfil",
            FeedText::Loading => "Carregando…",
            FeedText::Empty => "Nada aqui ainda",
            FeedText::EmptyNote => {
                "Troféus, novos amigos, primeiros jogos e parties aparecem aqui, para você e seus amigos."
            }
            FeedText::Offline => "Você está offline",
            FeedText::OfflineNote => "A atividade volta quando você estiver online de novo.",
            FeedText::LoadFailedTitle => "A atividade não carregou",
            FeedText::LoadFailed => "Verifique sua conexão e tente de novo.",
            FeedText::TryAgain => "Tentar de novo",
            FeedText::ShowOlder => "Mostrar atividade anterior",
            FeedText::OlderFailed => "Não foi possível carregar. Selecione para tentar de novo.",
            FeedText::Profile => "Perfil",
            FeedText::EndOfFeed => "Você está em dia.",
            FeedText::TrophyNamed => "ganhou o troféu de %@ “%@” em %@",
            FeedText::TrophyNamedOwn => "ganhou o troféu de %@ “%@” em %@",
            FeedText::Trophy => "ganhou um troféu de %@ em %@",
            FeedText::TrophyOwn => "ganhou um troféu de %@ em %@",
            FeedText::TrophyUnknownGame => "ganhou um troféu de %@ em um jogo",
            FeedText::TrophyUnknownGameOwn => "ganhou um troféu de %@ em um jogo",
            FeedText::Friend => "agora é amigo de %@",
            FeedText::FriendOwn => "agora é amigo de %@",
            FeedText::FriendWithYou => "agora é seu amigo",
            FeedText::FirstPlayed => "jogou %@ pela primeira vez",
            FeedText::FirstPlayedOwn => "jogou %@ pela primeira vez",
            FeedText::FirstPlayedUnknown => "jogou um jogo pela primeira vez",
            FeedText::FirstPlayedUnknownOwn => "jogou um jogo pela primeira vez",
            FeedText::Party => "criou uma party com %@",
            FeedText::PartyOwn => "criou uma party com %@",
            FeedText::PartyAlone => "criou uma party",
            FeedText::PartyAloneOwn => "criou uma party",
            FeedText::ListTwo => "%@ e %@",
            FeedText::ListThree => "%@, %@ e %@",
            FeedText::ListMore => "%@, %@ e mais %@",
            FeedText::GradePlatinum => "platina",
            FeedText::GradeGold => "ouro",
            FeedText::GradeSilver => "prata",
            FeedText::GradeBronze => "bronze",
            FeedText::RarityExtreme => "Extremamente raro",
            FeedText::RarityRare => "Raro",
            FeedText::RarityUncommon => "Incomum",
            FeedText::RarityCommon => "Comum",
            FeedText::RarityFact => "%@, %@%%",
            FeedText::AgoNow => "Agora mesmo",
            FeedText::AgoMinutes => "há %@ min",
            FeedText::AgoHours => "há %@ h",
            FeedText::AgoDays => "há %@ d",
            FeedText::AgoYesterday => "Ontem",
            FeedText::OwnSubtitle => "É isto que outros jogadores veem",
            FeedText::ProfileLoading => "Carregando…",
            FeedText::ProfileFailed => {
                "Não foi possível carregar este perfil. Verifique sua conexão e tente de novo."
            }
            FeedText::Favourites => "Jogos favoritos",
            FeedText::NotInLibrary => "%@, não está na sua biblioteca",
            FeedText::MostPlayed => "Mais jogados",
            FeedText::LinkedAccounts => "Contas vinculadas",
            FeedText::Discord => "Discord",
            FeedText::Steam => "Steam",
            FeedText::Linked => "Vinculada",
            FeedText::TrophiesPrivate => "Os troféus são privados",
            FeedText::EditProfile => "Editar perfil",
            FeedText::EditProfileDetail => "Abre sua conta na web",
            FeedText::OwnEmpty => {
                "Seu perfil está vazio. Adicione na web um texto curto, jogos favoritos e uma imagem."
            }
            FeedText::HoursMinutes => "%@ h %@ min",
            FeedText::Hours => "%@ h",
            FeedText::Minutes => "%@ min",
            FeedText::BannerName => "Perfil de %@",
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
    fn placeholders_match_between_languages() {
        for text in [
            FeedText::TrophyNamed,
            FeedText::TrophyNamedOwn,
            FeedText::Trophy,
            FeedText::TrophyOwn,
            FeedText::TrophyUnknownGame,
            FeedText::TrophyUnknownGameOwn,
            FeedText::Friend,
            FeedText::FriendOwn,
            FeedText::FirstPlayed,
            FeedText::FirstPlayedOwn,
            FeedText::Party,
            FeedText::PartyOwn,
            FeedText::ListTwo,
            FeedText::ListThree,
            FeedText::ListMore,
            FeedText::RarityFact,
            FeedText::AgoMinutes,
            FeedText::AgoHours,
            FeedText::AgoDays,
            FeedText::NotInLibrary,
            FeedText::HoursMinutes,
            FeedText::Hours,
            FeedText::Minutes,
            FeedText::BannerName,
        ] {
            let count = |s: &str| s.matches("%@").count();
            for language in Language::ALL {
                assert_eq!(
                    count(text.english()),
                    count(text.text(language)),
                    "{text:?} {language:?}"
                );
            }
        }
    }

    #[test]
    fn the_rarity_fact_reads_as_the_example() {
        assert_eq!(
            FeedText::RarityFact.format(Language::English, &[Arg::Str("Rare"), Arg::Str("2.3")]),
            "Rare, 2.3%"
        );
        assert_eq!(
            FeedText::RarityFact.format(Language::Turkish, &[Arg::Str("Nadir"), Arg::Str("2.3")]),
            "Nadir, %2.3"
        );
    }

    #[test]
    fn row_titles_are_in_title_case() {
        for title in [
            FeedText::Feed,
            FeedText::YourProfile,
            FeedText::ViewProfile,
            FeedText::TryAgain,
            FeedText::ShowOlder,
            FeedText::Profile,
            FeedText::Favourites,
            FeedText::MostPlayed,
            FeedText::LinkedAccounts,
            FeedText::EditProfile,
        ] {
            for word in title.english().split(' ') {
                let first = word.chars().next().unwrap();
                assert!(first.is_uppercase(), "{title:?}: {word}");
            }
        }
    }
}
