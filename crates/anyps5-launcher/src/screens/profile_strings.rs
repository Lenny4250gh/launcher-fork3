//! Texts of the profile screen (docs/DESIGN.md 17, "Profile screen"). English and Turkish like the
//! rest. Titles of sections and buttons are in title case, notes and details in sentence case.
//! Grades and times come from `online_feed_strings`; this file holds only what
//! the screen adds.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ProfileText {
    // Relation chips
    Friend,
    RequestSent,
    RequestReceived,
    // Stat labels, singular and plural
    StatGame,
    StatGames,
    StatHour,
    StatHours,
    StatMinute,
    StatMinutes,
    StatTrophy,
    StatTrophies,
    StatLevel,
    /// `%@` = the points ("1,240 points").
    StatPoints,
    /// The accessible name of the level figure: `%@` = level, `%@` = points.
    StatLevelName,
    // Sections
    SectionGames,
    SectionTrophies,
    SectionActivity,
    // Rows
    /// `%@` = when ("2 h ago", "Oct 21").
    LastPlayed,
    NeverPlayed,
    Favourite,
    NotInLibraryDetail,
    /// The detail of a game both players have, whose row opens the trophy comparison.
    CompareHint,
    /// The accessible name of a progress: `%@` = earned, `%@` = all.
    TrophyProgress,
    /// `%@` = the grade (Gold).
    GradeTrophy,
    InAGame,
    /// The activity line of a profile: `%@` = the subject, `%@` = the sentence.
    ActivityLine,
    // Buttons
    FriendsOnline,
    /// `%@` = how many messages wait.
    FriendsOnlineUnread,
    FriendOptions,
    AnswerRequest,
    AddFriend,
    /// On the player's own profile while they appear offline: the way back.
    AppearOnline,
    // States
    LoadingNote,
    OfflineTitle,
    OfflineNote,
    FailedTitle,
    FailedNote,
    PrivateTitle,
    /// `%@` = the handle.
    PrivateNote,
    /// `%@` = the handle.
    EmptyNote,
    /// A profile that does not exist, or is not open to the player (deleted, blocked): asking again will not help.
    GoneTitle,
    GoneNote,
    // The player's own empty sections
    OwnGamesEmpty,
    OwnTrophiesEmpty,
    OwnActivityEmpty,
    /// A new player's page, instead of three empty sections.
    OwnNothingYet,
    /// Under the header of the player's own profile while the cloud is out of reach.
    OwnOfflineNote,
    /// On a friend's profile: `%@` = how many of their games the player has too.
    SharedGames,
    /// `%@` = the handle.
    ScreenName,
    /// `%@` = the figure, `%@` = the label ("12 hours").
    StatName,
}

impl ProfileText {
    pub const fn english(self) -> &'static str {
        match self {
            ProfileText::Friend => "Friend",
            ProfileText::RequestSent => "Request sent",
            ProfileText::RequestReceived => "Wants to be friends",
            ProfileText::StatGame => "Game",
            ProfileText::StatGames => "Games",
            ProfileText::StatHour => "Hour",
            ProfileText::StatHours => "Hours",
            ProfileText::StatMinute => "Minute",
            ProfileText::StatMinutes => "Minutes",
            ProfileText::StatTrophy => "Trophy",
            ProfileText::StatTrophies => "Trophies",
            ProfileText::StatLevel => "Level",
            ProfileText::StatPoints => "%@ points",
            ProfileText::StatLevelName => "Level %@, %@ points",
            ProfileText::SectionGames => "Games",
            ProfileText::SectionTrophies => "Recent Trophies",
            ProfileText::SectionActivity => "Recent Activity",
            ProfileText::LastPlayed => "Last played %@",
            ProfileText::NeverPlayed => "Not played yet",
            ProfileText::Favourite => "Favourite",
            ProfileText::NotInLibraryDetail => "Not in your library",
            ProfileText::CompareHint => "Compare trophies",
            ProfileText::TrophyProgress => "%@ of %@ trophies",
            ProfileText::GradeTrophy => "%@ trophy",
            ProfileText::InAGame => "In a game",
            ProfileText::ActivityLine => "%@ %@",
            ProfileText::FriendsOnline => "Friends",
            ProfileText::FriendsOnlineUnread => "Friends (%@)",
            ProfileText::FriendOptions => "Friend Options",
            ProfileText::AnswerRequest => "Answer Request",
            ProfileText::AddFriend => "Add Friend",
            ProfileText::AppearOnline => "Appear Online",
            ProfileText::LoadingNote => "Loading profile…",
            ProfileText::OfflineTitle => "You are offline",
            ProfileText::OfflineNote => "Profiles of other players load when you are back online.",
            ProfileText::FailedTitle => "This profile did not load",
            ProfileText::FailedNote => "Check your connection and try again.",
            ProfileText::PrivateTitle => "This profile is private",
            ProfileText::PrivateNote => "%@ shares their profile with friends only.",
            ProfileText::EmptyNote => "%@ has not played or shared anything yet.",
            ProfileText::GoneTitle => "No profile to show",
            ProfileText::GoneNote => "This profile does not exist or is not open to you.",
            ProfileText::OwnGamesEmpty => "Games in your library show up here with your time.",
            ProfileText::OwnTrophiesEmpty => "Earn a trophy in a game and it shows up here.",
            ProfileText::OwnActivityEmpty => "Trophies, new friends and first games show up here.",
            ProfileText::OwnNothingYet => {
                "Play a game and your hours, trophies and activity show up here."
            }
            ProfileText::OwnOfflineNote => {
                "You are offline. Your activity comes back when you are online again."
            }
            ProfileText::SharedGames => "You have %@ of their games",
            ProfileText::ScreenName => "Profile of %@",
            ProfileText::StatName => "%@ %@",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            ProfileText::Friend => "Arkadaş",
            ProfileText::RequestSent => "İstek gönderildi",
            ProfileText::RequestReceived => "Arkadaş olmak istiyor",
            ProfileText::StatGame => "Oyun",
            ProfileText::StatGames => "Oyun",
            ProfileText::StatHour => "Saat",
            ProfileText::StatHours => "Saat",
            ProfileText::StatMinute => "Dakika",
            ProfileText::StatMinutes => "Dakika",
            ProfileText::StatTrophy => "Kupa",
            ProfileText::StatTrophies => "Kupa",
            ProfileText::StatLevel => "Seviye",
            ProfileText::StatPoints => "%@ puan",
            ProfileText::StatLevelName => "Seviye %@, %@ puan",
            ProfileText::SectionGames => "Oyunlar",
            ProfileText::SectionTrophies => "Son Kupalar",
            ProfileText::SectionActivity => "Son Etkinlik",
            ProfileText::LastPlayed => "Son oynama: %@",
            ProfileText::NeverPlayed => "Henüz oynanmadı",
            ProfileText::Favourite => "Favori",
            ProfileText::NotInLibraryDetail => "Kitaplığında yok",
            ProfileText::CompareHint => "Kupaları karşılaştır",
            ProfileText::TrophyProgress => "%@ / %@ kupa",
            ProfileText::GradeTrophy => "%@ kupa",
            ProfileText::InAGame => "Bir oyunda",
            ProfileText::ActivityLine => "%@ %@",
            ProfileText::FriendsOnline => "Arkadaşlar",
            ProfileText::FriendsOnlineUnread => "Arkadaşlar (%@)",
            ProfileText::FriendOptions => "Arkadaş Seçenekleri",
            ProfileText::AnswerRequest => "İsteği Yanıtla",
            ProfileText::AddFriend => "Arkadaş Ekle",
            ProfileText::AppearOnline => "Çevrimiçi Görün",
            ProfileText::LoadingNote => "Profil yükleniyor…",
            ProfileText::OfflineTitle => "Çevrimdışısın",
            ProfileText::OfflineNote => {
                "Diğer oyuncuların profilleri tekrar çevrimiçi olunca yüklenir."
            }
            ProfileText::FailedTitle => "Bu profil yüklenemedi",
            ProfileText::FailedNote => "Bağlantını kontrol edip tekrar dene.",
            ProfileText::PrivateTitle => "Bu profil gizli",
            ProfileText::PrivateNote => "%@ profilini yalnızca arkadaşlarıyla paylaşıyor.",
            ProfileText::EmptyNote => "%@ henüz bir şey oynamadı veya paylaşmadı.",
            ProfileText::GoneTitle => "Gösterilecek profil yok",
            ProfileText::GoneNote => "Bu profil yok ya da sana açık değil.",
            ProfileText::OwnGamesEmpty => "Kitaplığındaki oyunlar süreleriyle burada görünür.",
            ProfileText::OwnTrophiesEmpty => "Bir oyunda kupa kazan, burada görünsün.",
            ProfileText::OwnNothingYet => {
                "Bir oyun oyna; süren, kupaların ve etkinliğin burada görünür."
            }
            ProfileText::OwnActivityEmpty => {
                "Kupalar, yeni arkadaşlar ve ilk oyunlar burada görünür."
            }
            ProfileText::OwnOfflineNote => {
                "Çevrimdışısın. Etkinliğin tekrar çevrimiçi olunca geri gelir."
            }
            ProfileText::SharedGames => "Onların oyunlarından %@ tanesi sende de var",
            ProfileText::ScreenName => "%@ Kullanıcısının Profili",
            ProfileText::StatName => "%@ %@",
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            ProfileText::Friend => "Freund",
            ProfileText::RequestSent => "Anfrage gesendet",
            ProfileText::RequestReceived => "Möchte befreundet sein",
            ProfileText::StatGame => "Spiel",
            ProfileText::StatGames => "Spiele",
            ProfileText::StatHour => "Stunde",
            ProfileText::StatHours => "Stunden",
            ProfileText::StatMinute => "Minute",
            ProfileText::StatMinutes => "Minuten",
            ProfileText::StatTrophy => "Trophäe",
            ProfileText::StatTrophies => "Trophäen",
            ProfileText::StatLevel => "Level",
            ProfileText::StatPoints => "%@ Punkte",
            ProfileText::StatLevelName => "Level %@, %@ Punkte",
            ProfileText::SectionGames => "Spiele",
            ProfileText::SectionTrophies => "Letzte Trophäen",
            ProfileText::SectionActivity => "Letzte Aktivität",
            ProfileText::LastPlayed => "Zuletzt gespielt: %@",
            ProfileText::NeverPlayed => "Noch nicht gespielt",
            ProfileText::Favourite => "Favorit",
            ProfileText::NotInLibraryDetail => "Nicht in deiner Bibliothek",
            ProfileText::CompareHint => "Trophäen vergleichen",
            ProfileText::TrophyProgress => "%@ von %@ Trophäen",
            ProfileText::GradeTrophy => "%@-Trophäe",
            ProfileText::InAGame => "In einem Spiel",
            ProfileText::ActivityLine => "%@ %@",
            ProfileText::FriendsOnline => "Freunde",
            ProfileText::FriendsOnlineUnread => "Freunde (%@)",
            ProfileText::FriendOptions => "Freund-Optionen",
            ProfileText::AnswerRequest => "Anfrage beantworten",
            ProfileText::AddFriend => "Freund hinzufügen",
            ProfileText::AppearOnline => "Online erscheinen",
            ProfileText::LoadingNote => "Profil wird geladen…",
            ProfileText::OfflineTitle => "Du bist offline",
            ProfileText::OfflineNote => {
                "Profile anderer Spieler laden, sobald du wieder online bist."
            }
            ProfileText::FailedTitle => "Dieses Profil wurde nicht geladen",
            ProfileText::FailedNote => "Prüfe deine Verbindung und versuche es erneut.",
            ProfileText::PrivateTitle => "Dieses Profil ist privat",
            ProfileText::PrivateNote => "%@ teilt das Profil nur mit Freunden.",
            ProfileText::EmptyNote => "%@ hat noch nichts gespielt oder geteilt.",
            ProfileText::GoneTitle => "Kein Profil zu zeigen",
            ProfileText::GoneNote => {
                "Dieses Profil gibt es nicht oder es ist für dich nicht offen."
            }
            ProfileText::OwnGamesEmpty => {
                "Spiele aus deiner Bibliothek erscheinen hier mit deiner Spielzeit."
            }
            ProfileText::OwnTrophiesEmpty => {
                "Verdiene eine Trophäe in einem Spiel, dann erscheint sie hier."
            }
            ProfileText::OwnActivityEmpty => {
                "Trophäen, neue Freunde und erste Spiele erscheinen hier."
            }
            ProfileText::OwnNothingYet => {
                "Spiele ein Spiel, dann erscheinen deine Stunden, Trophäen und Aktivitäten hier."
            }
            ProfileText::OwnOfflineNote => {
                "Du bist offline. Deine Aktivität kommt zurück, sobald du wieder online bist."
            }
            ProfileText::SharedGames => "Du besitzt %@ ihrer Spiele",
            ProfileText::ScreenName => "Profil von %@",
            ProfileText::StatName => "%@ %@",
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            ProfileText::Friend => "Amigo",
            ProfileText::RequestSent => "Solicitud enviada",
            ProfileText::RequestReceived => "Quiere ser tu amigo",
            ProfileText::StatGame => "Juego",
            ProfileText::StatGames => "Juegos",
            ProfileText::StatHour => "Hora",
            ProfileText::StatHours => "Horas",
            ProfileText::StatMinute => "Minuto",
            ProfileText::StatMinutes => "Minutos",
            ProfileText::StatTrophy => "Trofeo",
            ProfileText::StatTrophies => "Trofeos",
            ProfileText::StatLevel => "Nivel",
            ProfileText::StatPoints => "%@ puntos",
            ProfileText::StatLevelName => "Nivel %@, %@ puntos",
            ProfileText::SectionGames => "Juegos",
            ProfileText::SectionTrophies => "Trofeos recientes",
            ProfileText::SectionActivity => "Actividad reciente",
            ProfileText::LastPlayed => "Última partida: %@",
            ProfileText::NeverPlayed => "Aún sin jugar",
            ProfileText::Favourite => "Favorito",
            ProfileText::NotInLibraryDetail => "No está en tu biblioteca",
            ProfileText::CompareHint => "Comparar trofeos",
            ProfileText::TrophyProgress => "%@ de %@ trofeos",
            ProfileText::GradeTrophy => "Trofeo %@",
            ProfileText::InAGame => "En un juego",
            ProfileText::ActivityLine => "%@ %@",
            ProfileText::FriendsOnline => "Amigos",
            ProfileText::FriendsOnlineUnread => "Amigos (%@)",
            ProfileText::FriendOptions => "Opciones del amigo",
            ProfileText::AnswerRequest => "Responder a la solicitud",
            ProfileText::AddFriend => "Añadir amigo",
            ProfileText::AppearOnline => "Aparecer en línea",
            ProfileText::LoadingNote => "Cargando perfil…",
            ProfileText::OfflineTitle => "Estás sin conexión",
            ProfileText::OfflineNote => {
                "Los perfiles de otros jugadores se cargan cuando vuelves a estar en línea."
            }
            ProfileText::FailedTitle => "No se cargó este perfil",
            ProfileText::FailedNote => "Revisa tu conexión e inténtalo de nuevo.",
            ProfileText::PrivateTitle => "Este perfil es privado",
            ProfileText::PrivateNote => "%@ solo comparte su perfil con amigos.",
            ProfileText::EmptyNote => "%@ aún no ha jugado ni compartido nada.",
            ProfileText::GoneTitle => "No hay perfil que mostrar",
            ProfileText::GoneNote => "Este perfil no existe o no está abierto para ti.",
            ProfileText::OwnGamesEmpty => {
                "Los juegos de tu biblioteca aparecen aquí con tu tiempo de juego."
            }
            ProfileText::OwnTrophiesEmpty => "Consigue un trofeo en un juego y aparecerá aquí.",
            ProfileText::OwnActivityEmpty => {
                "Los trofeos, los nuevos amigos y los primeros juegos aparecen aquí."
            }
            ProfileText::OwnNothingYet => {
                "Juega a un juego y tus horas, trofeos y actividad aparecerán aquí."
            }
            ProfileText::OwnOfflineNote => {
                "Estás sin conexión. Tu actividad vuelve cuando te conectes de nuevo."
            }
            ProfileText::SharedGames => "Tienes %@ de sus juegos",
            ProfileText::ScreenName => "Perfil de %@",
            ProfileText::StatName => "%@ %@",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            ProfileText::Friend => "Ami",
            ProfileText::RequestSent => "Demande envoyée",
            ProfileText::RequestReceived => "Veut devenir ton ami",
            ProfileText::StatGame => "Jeu",
            ProfileText::StatGames => "Jeux",
            ProfileText::StatHour => "Heure",
            ProfileText::StatHours => "Heures",
            ProfileText::StatMinute => "Minute",
            ProfileText::StatMinutes => "Minutes",
            ProfileText::StatTrophy => "Trophée",
            ProfileText::StatTrophies => "Trophées",
            ProfileText::StatLevel => "Niveau",
            ProfileText::StatPoints => "%@ points",
            ProfileText::StatLevelName => "Niveau %@, %@ points",
            ProfileText::SectionGames => "Jeux",
            ProfileText::SectionTrophies => "Trophées récents",
            ProfileText::SectionActivity => "Activité récente",
            ProfileText::LastPlayed => "Dernière partie\u{a0}: %@",
            ProfileText::NeverPlayed => "Pas encore joué",
            ProfileText::Favourite => "Favori",
            ProfileText::NotInLibraryDetail => "Pas dans ta bibliothèque",
            ProfileText::CompareHint => "Comparer les trophées",
            ProfileText::TrophyProgress => "%@ trophées sur %@",
            ProfileText::GradeTrophy => "Trophée %@",
            ProfileText::InAGame => "Dans un jeu",
            ProfileText::ActivityLine => "%@ %@",
            ProfileText::FriendsOnline => "Amis",
            ProfileText::FriendsOnlineUnread => "Amis (%@)",
            ProfileText::FriendOptions => "Options de l’ami",
            ProfileText::AnswerRequest => "Répondre à la demande",
            ProfileText::AddFriend => "Ajouter un ami",
            ProfileText::AppearOnline => "Apparaître en ligne",
            ProfileText::LoadingNote => "Chargement du profil…",
            ProfileText::OfflineTitle => "Tu es hors ligne",
            ProfileText::OfflineNote => {
                "Les profils des autres joueurs se chargent quand tu es de nouveau en ligne."
            }
            ProfileText::FailedTitle => "Ce profil ne s’est pas chargé",
            ProfileText::FailedNote => "Vérifie ta connexion et réessaie.",
            ProfileText::PrivateTitle => "Ce profil est privé",
            ProfileText::PrivateNote => "%@ ne partage son profil qu’avec ses amis.",
            ProfileText::EmptyNote => "%@ n’a encore rien joué ni partagé.",
            ProfileText::GoneTitle => "Aucun profil à montrer",
            ProfileText::GoneNote => "Ce profil n’existe pas ou ne t’est pas ouvert.",
            ProfileText::OwnGamesEmpty => {
                "Les jeux de ta bibliothèque apparaissent ici avec ton temps de jeu."
            }
            ProfileText::OwnTrophiesEmpty => "Obtiens un trophée dans un jeu et il apparaîtra ici.",
            ProfileText::OwnActivityEmpty => {
                "Les trophées, les nouveaux amis et les premiers jeux apparaissent ici."
            }
            ProfileText::OwnNothingYet => {
                "Lance un jeu et tes heures, trophées et activités apparaîtront ici."
            }
            ProfileText::OwnOfflineNote => {
                "Tu es hors ligne. Ton activité revient dès que tu es de nouveau en ligne."
            }
            ProfileText::SharedGames => "Tu as %@ de ses jeux",
            ProfileText::ScreenName => "Profil de %@",
            ProfileText::StatName => "%@ %@",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            ProfileText::Friend => "Amigo",
            ProfileText::RequestSent => "Solicitação enviada",
            ProfileText::RequestReceived => "Quer ser seu amigo",
            ProfileText::StatGame => "Jogo",
            ProfileText::StatGames => "Jogos",
            ProfileText::StatHour => "Hora",
            ProfileText::StatHours => "Horas",
            ProfileText::StatMinute => "Minuto",
            ProfileText::StatMinutes => "Minutos",
            ProfileText::StatTrophy => "Troféu",
            ProfileText::StatTrophies => "Troféus",
            ProfileText::StatLevel => "Nível",
            ProfileText::StatPoints => "%@ pontos",
            ProfileText::StatLevelName => "Nível %@, %@ pontos",
            ProfileText::SectionGames => "Jogos",
            ProfileText::SectionTrophies => "Troféus recentes",
            ProfileText::SectionActivity => "Atividade recente",
            ProfileText::LastPlayed => "Última partida: %@",
            ProfileText::NeverPlayed => "Ainda não jogado",
            ProfileText::Favourite => "Favorito",
            ProfileText::NotInLibraryDetail => "Não está na sua biblioteca",
            ProfileText::CompareHint => "Comparar troféus",
            ProfileText::TrophyProgress => "%@ de %@ troféus",
            ProfileText::GradeTrophy => "Troféu %@",
            ProfileText::InAGame => "Em um jogo",
            ProfileText::ActivityLine => "%@ %@",
            ProfileText::FriendsOnline => "Amigos",
            ProfileText::FriendsOnlineUnread => "Amigos (%@)",
            ProfileText::FriendOptions => "Opções do amigo",
            ProfileText::AnswerRequest => "Responder à solicitação",
            ProfileText::AddFriend => "Adicionar amigo",
            ProfileText::AppearOnline => "Aparecer online",
            ProfileText::LoadingNote => "Carregando perfil…",
            ProfileText::OfflineTitle => "Você está offline",
            ProfileText::OfflineNote => {
                "Os perfis de outros jogadores carregam quando você voltar a ficar online."
            }
            ProfileText::FailedTitle => "Este perfil não carregou",
            ProfileText::FailedNote => "Verifique sua conexão e tente de novo.",
            ProfileText::PrivateTitle => "Este perfil é privado",
            ProfileText::PrivateNote => "%@ compartilha o perfil apenas com amigos.",
            ProfileText::EmptyNote => "%@ ainda não jogou nem compartilhou nada.",
            ProfileText::GoneTitle => "Nenhum perfil para mostrar",
            ProfileText::GoneNote => "Este perfil não existe ou não está aberto para você.",
            ProfileText::OwnGamesEmpty => {
                "Os jogos da sua biblioteca aparecem aqui com seu tempo de jogo."
            }
            ProfileText::OwnTrophiesEmpty => "Conquiste um troféu em um jogo e ele aparece aqui.",
            ProfileText::OwnActivityEmpty => {
                "Troféus, novos amigos e primeiros jogos aparecem aqui."
            }
            ProfileText::OwnNothingYet => {
                "Jogue um jogo e suas horas, troféus e atividades aparecem aqui."
            }
            ProfileText::OwnOfflineNote => {
                "Você está offline. Sua atividade volta quando você ficar online de novo."
            }
            ProfileText::SharedGames => "Você tem %@ dos jogos deles",
            ProfileText::ScreenName => "Perfil de %@",
            ProfileText::StatName => "%@ %@",
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

    fn placeholders(text: &str) -> usize {
        text.matches("%@").count()
    }

    #[test]
    fn placeholders_match_between_languages() {
        for text in [
            ProfileText::LastPlayed,
            ProfileText::TrophyProgress,
            ProfileText::StatPoints,
            ProfileText::StatLevelName,
            ProfileText::GradeTrophy,
            ProfileText::ActivityLine,
            ProfileText::FriendsOnlineUnread,
            ProfileText::PrivateNote,
            ProfileText::EmptyNote,
            ProfileText::SharedGames,
            ProfileText::ScreenName,
            ProfileText::StatName,
            ProfileText::OwnGamesEmpty,
        ] {
            assert_eq!(
                placeholders(text.english()),
                placeholders(text.turkish()),
                "{text:?}"
            );
        }
    }
}
