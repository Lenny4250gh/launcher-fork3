//! Texts of the Game Hub and the trophy overview (docs/DESIGN.md 5.21). English and Turkish like
//! the rest. Titles are in title case, notes and details in sentence case. Grades, "Back" and the
//! playing-time words come from the shared strings; this file holds only what the two screens add.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HubText {
    /// The small caption over the game's name.
    Overline,
    /// The accessible name of the screen: `%@` = the game.
    ScreenName,
    Trophies,
    /// `%@` = the earned count, `%@` = all of them.
    TrophyCount,
    /// Under the bar: `%@` = the trophy, `%@` = its grade.
    LatestTrophy,
    /// A game with a trophy list and nothing earned.
    NoneEarned,
    NoTrophyList,
    TrophyList,
    AllGames,
    PlayTime,
    NeverPlayed,
    /// `%@` = the time played this week.
    ThisWeek,
    /// The caption of the week's total in the profile.
    WeekLabel,
    /// `%@` = when it was last played.
    LastPlayed,
    Captures,
    /// `%@` = how many.
    CaptureCount,
    CaptureOne,
    NoCaptures,
    SeeAll,
    Friends,
    /// `%@` = the names of friends who play this now.
    FriendsPlaying,
    NoFriendsPlaying,
    FriendsSignedOut,
    PlayTogether,
    /// `%@` = the most players in one world.
    TogetherUpTo,
    TogetherNone,
    Compatibility,
    CompatNone,
    CompatOffline,
    GameMusic,
    MusicOn,
    MusicOff,
    MusicNone,
    // The overview of all games' trophies.
    OverviewOverline,
    OverviewTitle,
    /// `%@` = trophies earned, `%@` = games.
    OverviewSummary,
    OverviewLoading,
    OverviewEmptyTitle,
    OverviewEmptyNote,
    /// The accessible name of a row: `%@` game, `%@` earned, `%@` all, `%@` latest.
    OverviewRowName,
    /// `%@` = the date of the latest trophy of a game.
    OverviewLatest,
    /// Under the figure of a game without a trophy earned yet.
    OverviewNothingYet,
}

impl HubText {
    pub const fn english(self) -> &'static str {
        match self {
            HubText::Overline => "Game Hub",
            HubText::ScreenName => "Game Hub of %@",
            HubText::Trophies => "Trophies",
            HubText::TrophyCount => "%@ of %@",
            HubText::LatestTrophy => "Latest: %@ (%@)",
            HubText::NoneEarned => "None earned yet.",
            HubText::NoTrophyList => "This game has no trophies.",
            HubText::TrophyList => "Trophy List",
            HubText::AllGames => "All Games",
            HubText::PlayTime => "Play Time",
            HubText::NeverPlayed => "Not played yet",
            HubText::ThisWeek => "%@ this week",
            HubText::WeekLabel => "This Week",
            HubText::LastPlayed => "Last played %@",
            HubText::Captures => "Captures",
            HubText::CaptureCount => "%@ captures",
            HubText::CaptureOne => "1 capture",
            HubText::NoCaptures => "None yet. Take one in the game with F12.",
            HubText::SeeAll => "See All",
            HubText::Friends => "Friends",
            HubText::FriendsPlaying => "%@ playing now",
            HubText::NoFriendsPlaying => "None of your friends is playing this right now.",
            HubText::FriendsSignedOut => "Sign in to see which friends play this.",
            HubText::PlayTogether => "Play Together",
            HubText::TogetherUpTo => "Up to %@ players",
            HubText::TogetherNone => "Not available for this game yet.",
            HubText::Compatibility => "Compatibility",
            HubText::CompatNone => "No reports from players yet.",
            HubText::CompatOffline => "Sign in to see what players found.",
            HubText::GameMusic => "Game Music",
            HubText::MusicOn => "On",
            HubText::MusicOff => "Off",
            HubText::MusicNone => "This game has none.",
            HubText::OverviewOverline => "All Games",
            HubText::OverviewTitle => "Trophies",
            HubText::OverviewSummary => "%@ trophies in %@ games",
            HubText::OverviewLoading => "Reading the trophy lists…",
            HubText::OverviewEmptyTitle => "No trophy lists yet",
            HubText::OverviewEmptyNote => "Games with trophies show their progress here.",
            HubText::OverviewRowName => "%@: %@ of %@ trophies. %@",
            HubText::OverviewLatest => "Latest on %@",
            HubText::OverviewNothingYet => "Nothing earned yet",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            HubText::Overline => "Oyun Merkezi",
            HubText::ScreenName => "%@ oyun merkezi",
            HubText::Trophies => "Kupalar",
            HubText::TrophyCount => "%@ / %@",
            HubText::LatestTrophy => "Son: %@ (%@)",
            HubText::NoneEarned => "Hen\u{fc}z kazan\u{131}lmad\u{131}.",
            HubText::NoTrophyList => "Bu oyunda kupa yok.",
            HubText::TrophyList => "Kupa Listesi",
            HubText::AllGames => "T\u{fc}m Oyunlar",
            HubText::PlayTime => "Oynama S\u{fc}resi",
            HubText::NeverPlayed => "Hen\u{fc}z oynanmad\u{131}",
            HubText::ThisWeek => "Bu hafta %@",
            HubText::WeekLabel => "Bu Hafta",
            HubText::LastPlayed => "Son oynama: %@",
            HubText::Captures => "Yakalamalar",
            HubText::CaptureCount => "%@ yakalama",
            HubText::CaptureOne => "1 yakalama",
            HubText::NoCaptures => "Hen\u{fc}z yok. Oyunda F12 ile bir tane al.",
            HubText::SeeAll => "Hepsini G\u{f6}r",
            HubText::Friends => "Arkada\u{15f}lar",
            HubText::FriendsPlaying => "%@ \u{15f}u an oynuyor",
            HubText::NoFriendsPlaying => {
                "Arkada\u{15f}lar\u{131}n\u{131}n hi\u{e7}biri \u{15f}u an bunu oynam\u{131}yor."
            }
            HubText::FriendsSignedOut => {
                "Hangi arkada\u{15f}lar\u{131}n\u{131}n oynad\u{131}\u{11f}\u{131}n\u{131} g\u{f6}rmek i\u{e7}in giri\u{15f} yap."
            }
            HubText::PlayTogether => "Birlikte Oyna",
            HubText::TogetherUpTo => "En fazla %@ oyuncu",
            HubText::TogetherNone => "Bu oyun i\u{e7}in hen\u{fc}z yok.",
            HubText::Compatibility => "Uyumluluk",
            HubText::CompatNone => "Oyunculardan hen\u{fc}z rapor yok.",
            HubText::CompatOffline => {
                "Oyuncular\u{131}n ne buldu\u{11f}unu g\u{f6}rmek i\u{e7}in giri\u{15f} yap."
            }
            HubText::GameMusic => "Oyun M\u{fc}zi\u{11f}i",
            HubText::MusicOn => "A\u{e7}\u{131}k",
            HubText::MusicOff => "Kapal\u{131}",
            HubText::MusicNone => "Bu oyunda yok.",
            HubText::OverviewOverline => "T\u{fc}m Oyunlar",
            HubText::OverviewTitle => "Kupalar",
            HubText::OverviewSummary => "%@ oyunda %@ kupa",
            HubText::OverviewLoading => "Kupa listeleri okunuyor\u{2026}",
            HubText::OverviewEmptyTitle => "Hen\u{fc}z kupa listesi yok",
            HubText::OverviewEmptyNote => {
                "Kupalar\u{131} olan oyunlar\u{131}n ilerlemesi burada g\u{f6}r\u{fc}n\u{fc}r."
            }
            HubText::OverviewRowName => "%@: %@ / %@ kupa. %@",
            HubText::OverviewLatest => "Son: %@",
            HubText::OverviewNothingYet => "Hen\u{fc}z kazan\u{131}lmad\u{131}",
        }
    }

    /// The German text; `None` falls back to English.
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            HubText::Overline => "Spiele-Hub",
            HubText::ScreenName => "Spiele-Hub von %@",
            HubText::Trophies => "Trophäen",
            HubText::TrophyCount => "%@ von %@",
            HubText::LatestTrophy => "Zuletzt: %@ (%@)",
            HubText::NoneEarned => "Noch keine verdient.",
            HubText::NoTrophyList => "Dieses Spiel hat keine Trophäen.",
            HubText::TrophyList => "Trophäenliste",
            HubText::AllGames => "Alle Spiele",
            HubText::PlayTime => "Spielzeit",
            HubText::NeverPlayed => "Noch nicht gespielt",
            HubText::ThisWeek => "%@ diese Woche",
            HubText::WeekLabel => "Diese Woche",
            HubText::LastPlayed => "Zuletzt gespielt: %@",
            HubText::Captures => "Aufnahmen",
            HubText::CaptureCount => "%@ Aufnahmen",
            HubText::CaptureOne => "1 Aufnahme",
            HubText::NoCaptures => "Noch keine. Mach im Spiel eine mit F12.",
            HubText::SeeAll => "Alle ansehen",
            HubText::Friends => "Freunde",
            HubText::FriendsPlaying => "%@ spielt gerade",
            HubText::NoFriendsPlaying => "Gerade spielt keiner deiner Freunde dieses Spiel.",
            HubText::FriendsSignedOut => "Melde dich an, um zu sehen, welche Freunde es spielen.",
            HubText::PlayTogether => "Zusammen spielen",
            HubText::TogetherUpTo => "Bis zu %@ Spieler",
            HubText::TogetherNone => "Für dieses Spiel noch nicht verfügbar.",
            HubText::Compatibility => "Kompatibilität",
            HubText::CompatNone => "Noch keine Berichte von Spielern.",
            HubText::CompatOffline => {
                "Melde dich an, um zu sehen, was Spieler herausgefunden haben."
            }
            HubText::GameMusic => "Spielemusik",
            HubText::MusicOn => "An",
            HubText::MusicOff => "Aus",
            HubText::MusicNone => "Dieses Spiel hat keine.",
            HubText::OverviewOverline => "Alle Spiele",
            HubText::OverviewTitle => "Trophäen",
            HubText::OverviewSummary => "%@ Trophäen in %@ Spielen",
            HubText::OverviewLoading => "Trophäenlisten werden gelesen …",
            HubText::OverviewEmptyTitle => "Noch keine Trophäenlisten",
            HubText::OverviewEmptyNote => "Spiele mit Trophäen zeigen hier ihren Fortschritt.",
            HubText::OverviewRowName => "%@: %@ von %@ Trophäen. %@",
            HubText::OverviewLatest => "Zuletzt am %@",
            HubText::OverviewNothingYet => "Noch nichts verdient",
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

    const ALL: &[HubText] = &[
        HubText::Overline,
        HubText::ScreenName,
        HubText::Trophies,
        HubText::TrophyCount,
        HubText::LatestTrophy,
        HubText::NoneEarned,
        HubText::NoTrophyList,
        HubText::TrophyList,
        HubText::AllGames,
        HubText::PlayTime,
        HubText::NeverPlayed,
        HubText::ThisWeek,
        HubText::WeekLabel,
        HubText::LastPlayed,
        HubText::Captures,
        HubText::CaptureCount,
        HubText::CaptureOne,
        HubText::NoCaptures,
        HubText::SeeAll,
        HubText::Friends,
        HubText::FriendsPlaying,
        HubText::NoFriendsPlaying,
        HubText::FriendsSignedOut,
        HubText::PlayTogether,
        HubText::TogetherUpTo,
        HubText::TogetherNone,
        HubText::Compatibility,
        HubText::CompatNone,
        HubText::CompatOffline,
        HubText::GameMusic,
        HubText::MusicOn,
        HubText::MusicOff,
        HubText::MusicNone,
        HubText::OverviewOverline,
        HubText::OverviewTitle,
        HubText::OverviewSummary,
        HubText::OverviewLoading,
        HubText::OverviewEmptyTitle,
        HubText::OverviewEmptyNote,
        HubText::OverviewRowName,
        HubText::OverviewLatest,
        HubText::OverviewNothingYet,
    ];

    #[test]
    fn placeholders_match_between_languages() {
        for &text in ALL {
            assert_eq!(
                text.english().matches("%@").count(),
                text.turkish().matches("%@").count(),
                "{text:?}"
            );
            assert!(!text.english().is_empty() && !text.turkish().is_empty());
            let german = text.german().expect("every text has German");
            assert_eq!(
                text.english().matches("%@").count(),
                german.matches("%@").count(),
                "{text:?}"
            );
        }
    }

    #[test]
    fn no_text_names_the_platform() {
        for &text in ALL {
            for words in [text.english(), text.turkish()] {
                assert!(!words.contains("PSN") && !words.contains("PlayStation"));
            }
        }
    }
}
