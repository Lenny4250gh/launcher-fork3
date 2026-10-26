//! Texts of the library page (sort, filter, search, what players found), of the storage page and
//! of the home screen's shelves. English and Turkish like the rest.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LibraryText {
    // What players found, under a tile.
    RunsWell,
    RunsWithIssues,
    NoStart,
    Lan,
    // The bar of the library page.
    /// `%@` = the order.
    SortBy,
    SortRecent,
    SortName,
    SortSize,
    SortStatus,
    /// `%@` = the filter.
    ShowFilter,
    FilterAll,
    FilterReady,
    FilterNeedsUnlock,
    FilterTogether,
    Search,
    /// `%@` = what was typed.
    SearchFor,
    Storage,
    // Search page.
    SearchTitle,
    /// `%@` = how many games.
    SearchFound,
    SearchNone,
    SearchField,
    SearchShow,
    SearchClear,
    KeyShift,
    KeyLetters,
    KeySymbols,
    KeySpace,
    KeyDelete,
    // An empty page under a filter or a search.
    NothingHere,
    NothingHereText,
    ClearFilters,
    // Storage page.
    StorageTitle,
    /// `%@` = size of the copies, `%@` = free space.
    StorageSummary,
    /// `%@` = size of the copies.
    StorageSummaryNoDrive,
    StorageEmpty,
    StorageEmptyText,
    StorageMeasuring,
    /// `%@` = size.
    StorageCopy,
    StorageTotal,
    RemoveCopy,
    /// `%@` = the game.
    RemoveCopyTitle,
    /// `%@` = size freed.
    RemoveCopyBody,
    RemoveCopyRunning,
    Cancel,
    // The shelves of the home screen.
    ShelfFriends,
    ShelfRecent,
    ShelfNew,
    Join,
    /// `%@` = a friend's name.
    ShelfFriendPlays,
    // A game was found.
    /// `%@` = the game.
    NewGame,
    /// `%@` = how many.
    NewGames,
}

impl LibraryText {
    pub const fn english(self) -> &'static str {
        match self {
            LibraryText::RunsWell => "Runs well",
            LibraryText::RunsWithIssues => "Runs with issues",
            LibraryText::NoStart => "Doesn't start",
            LibraryText::Lan => "LAN",
            LibraryText::SortBy => "Sort: %@",
            LibraryText::SortRecent => "Recently played",
            LibraryText::SortName => "Name",
            LibraryText::SortSize => "Size",
            LibraryText::SortStatus => "Status",
            LibraryText::ShowFilter => "Show: %@",
            LibraryText::FilterAll => "All games",
            LibraryText::FilterReady => "Ready",
            LibraryText::FilterNeedsUnlock => "Needs unlock",
            LibraryText::FilterTogether => "Play together",
            LibraryText::Search => "Search",
            LibraryText::SearchFor => "Search: %@",
            LibraryText::Storage => "Storage",
            LibraryText::SearchTitle => "Search Library",
            LibraryText::SearchFound => "%@ found",
            LibraryText::SearchNone => "No game matches",
            LibraryText::SearchField => "Name",
            LibraryText::SearchShow => "Show Games",
            LibraryText::SearchClear => "Clear Search",
            LibraryText::KeyShift => "Shift",
            LibraryText::KeyLetters => "abc",
            LibraryText::KeySymbols => "123",
            LibraryText::KeySpace => "Space",
            LibraryText::KeyDelete => "Delete",
            LibraryText::NothingHere => "No games here",
            LibraryText::NothingHereText => "No game matches the search or the filter.",
            LibraryText::ClearFilters => "Show All Games",
            LibraryText::StorageTitle => "Storage",
            LibraryText::StorageSummary => "Unlocked copies use %@. %@ free on this drive.",
            LibraryText::StorageSummaryNoDrive => "Unlocked copies use %@.",
            LibraryText::StorageEmpty => "No unlocked copies",
            LibraryText::StorageEmptyText => {
                "A game takes room here once it is unlocked. Your game files are not counted."
            }
            LibraryText::StorageMeasuring => "Measuring\u{2026}",
            LibraryText::StorageCopy => "Unlocked copy: %@",
            LibraryText::StorageTotal => "All unlocked copies",
            LibraryText::RemoveCopy => "Remove Unlocked Copy",
            LibraryText::RemoveCopyTitle => "Remove the unlocked copy of %@?",
            LibraryText::RemoveCopyBody => {
                "This frees %@. Your game files stay where they are, and the saves are backed up first. Unlocking the game again takes a while."
            }
            LibraryText::RemoveCopyRunning => "Close the game first",
            LibraryText::Cancel => "Cancel",
            LibraryText::ShelfFriends => "Friends Playing Now",
            LibraryText::ShelfRecent => "Recently Played",
            LibraryText::ShelfNew => "New",
            LibraryText::Join => "Join",
            LibraryText::ShelfFriendPlays => "%@ plays this",
            LibraryText::NewGame => "New game: %@",
            LibraryText::NewGames => "%@ new games",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            LibraryText::RunsWell => "Sorunsuz çalışıyor",
            LibraryText::RunsWithIssues => "Sorunlarla çalışıyor",
            LibraryText::NoStart => "Başlamıyor",
            LibraryText::Lan => "LAN",
            LibraryText::SortBy => "Sırala: %@",
            LibraryText::SortRecent => "Son oynanan",
            LibraryText::SortName => "Ad",
            LibraryText::SortSize => "Boyut",
            LibraryText::SortStatus => "Durum",
            LibraryText::ShowFilter => "Göster: %@",
            LibraryText::FilterAll => "Tüm oyunlar",
            LibraryText::FilterReady => "Hazır",
            LibraryText::FilterNeedsUnlock => "Kilit açılmalı",
            LibraryText::FilterTogether => "Birlikte oyna",
            LibraryText::Search => "Ara",
            LibraryText::SearchFor => "Ara: %@",
            LibraryText::Storage => "Depolama",
            LibraryText::SearchTitle => "Kütüphanede Ara",
            LibraryText::SearchFound => "%@ bulundu",
            LibraryText::SearchNone => "Eşleşen oyun yok",
            LibraryText::SearchField => "Ad",
            LibraryText::SearchShow => "Oyunları Göster",
            LibraryText::SearchClear => "Aramayı Temizle",
            LibraryText::KeyShift => "Shift",
            LibraryText::KeyLetters => "abc",
            LibraryText::KeySymbols => "123",
            LibraryText::KeySpace => "Boşluk",
            LibraryText::KeyDelete => "Sil",
            LibraryText::NothingHere => "Burada oyun yok",
            LibraryText::NothingHereText => "Arama veya süzgeçle eşleşen oyun yok.",
            LibraryText::ClearFilters => "Tüm Oyunları Göster",
            LibraryText::StorageTitle => "Depolama",
            LibraryText::StorageSummary => {
                "Kilidi açılmış kopyalar %@ kullanıyor. Bu sürücüde %@ boş."
            }
            LibraryText::StorageSummaryNoDrive => "Kilidi açılmış kopyalar %@ kullanıyor.",
            LibraryText::StorageEmpty => "Kilidi açılmış kopya yok",
            LibraryText::StorageEmptyText => {
                "Bir oyunun kilidi açılınca burada yer kaplar. Oyun dosyalarınız sayılmaz."
            }
            LibraryText::StorageMeasuring => "Ölçülüyor\u{2026}",
            LibraryText::StorageCopy => "Kilidi açılmış kopya: %@",
            LibraryText::StorageTotal => "Tüm kilidi açılmış kopyalar",
            LibraryText::RemoveCopy => "Kilidi Açılmış Kopyayı Kaldır",
            LibraryText::RemoveCopyTitle => "%@ için kilidi açılmış kopya kaldırılsın mı?",
            LibraryText::RemoveCopyBody => {
                "%@ yer açılır. Oyun dosyalarınız olduğu yerde kalır, kayıtlar önce yedeklenir. Oyunun kilidini yeniden açmak biraz sürer."
            }
            LibraryText::RemoveCopyRunning => "Önce oyunu kapatın",
            LibraryText::Cancel => "Vazgeç",
            LibraryText::ShelfFriends => "Şimdi Oynayan Arkadaşlar",
            LibraryText::ShelfRecent => "Son Oynananlar",
            LibraryText::ShelfNew => "Yeni",
            LibraryText::Join => "Katıl",
            LibraryText::ShelfFriendPlays => "%@ bunu oynuyor",
            LibraryText::NewGame => "Yeni oyun: %@",
            LibraryText::NewGames => "%@ yeni oyun",
        }
    }

    pub fn text(self, language: Language) -> &'static str {
        match language {
            Language::English => self.english(),
            Language::Turkish => self.turkish(),
            // German follows in its own pass; until then it reads English.
            Language::German => self.english(),
        }
    }

    pub fn format(self, language: Language, args: &[Arg<'_>]) -> String {
        substitute(self.text(language), args)
    }

    pub const ALL: &[LibraryText] = &[
        LibraryText::RunsWell,
        LibraryText::RunsWithIssues,
        LibraryText::NoStart,
        LibraryText::Lan,
        LibraryText::SortBy,
        LibraryText::SortRecent,
        LibraryText::SortName,
        LibraryText::SortSize,
        LibraryText::SortStatus,
        LibraryText::ShowFilter,
        LibraryText::FilterAll,
        LibraryText::FilterReady,
        LibraryText::FilterNeedsUnlock,
        LibraryText::FilterTogether,
        LibraryText::Search,
        LibraryText::SearchFor,
        LibraryText::Storage,
        LibraryText::SearchTitle,
        LibraryText::SearchFound,
        LibraryText::SearchNone,
        LibraryText::SearchField,
        LibraryText::SearchShow,
        LibraryText::SearchClear,
        LibraryText::KeyShift,
        LibraryText::KeyLetters,
        LibraryText::KeySymbols,
        LibraryText::KeySpace,
        LibraryText::KeyDelete,
        LibraryText::NothingHere,
        LibraryText::NothingHereText,
        LibraryText::ClearFilters,
        LibraryText::StorageTitle,
        LibraryText::StorageSummary,
        LibraryText::StorageSummaryNoDrive,
        LibraryText::StorageEmpty,
        LibraryText::StorageEmptyText,
        LibraryText::StorageMeasuring,
        LibraryText::StorageCopy,
        LibraryText::StorageTotal,
        LibraryText::RemoveCopy,
        LibraryText::RemoveCopyTitle,
        LibraryText::RemoveCopyBody,
        LibraryText::RemoveCopyRunning,
        LibraryText::Cancel,
        LibraryText::ShelfFriends,
        LibraryText::ShelfRecent,
        LibraryText::ShelfNew,
        LibraryText::Join,
        LibraryText::ShelfFriendPlays,
        LibraryText::NewGame,
        LibraryText::NewGames,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_text_exists_in_both_languages_with_the_same_placeholders() {
        for &text in LibraryText::ALL {
            for language in Language::ALL {
                assert!(!text.text(language).is_empty(), "{text:?} {language:?}");
            }
            assert_eq!(
                text.english().matches("%@").count(),
                text.turkish().matches("%@").count(),
                "{text:?}"
            );
        }
    }
}
