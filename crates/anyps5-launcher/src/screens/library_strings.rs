//! Texts of the library page (sort, filter, search, what players found), of the storage page and
//! of the home screen's shelves. English, Turkish and German like the rest.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LibraryText {
    // What players found, under a tile.
    RunsWell,
    RunsWithIssues,
    NoStart,
    Lan,
    /// The same after another tag ("Runs well · Together"), where the tile has less room.
    LanShort,
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
            LibraryText::Lan => "Play together",
            LibraryText::LanShort => "Together",
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
            LibraryText::Lan => "Birlikte oyna",
            LibraryText::LanShort => "Birlikte",
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

    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            LibraryText::RunsWell => "Läuft gut",
            LibraryText::RunsWithIssues => "Läuft mit Problemen",
            LibraryText::NoStart => "Startet nicht",
            LibraryText::Lan => "Zusammen spielen",
            LibraryText::LanShort => "Zusammen",
            LibraryText::SortBy => "Sortierung: %@",
            LibraryText::SortRecent => "Zuletzt gespielt",
            LibraryText::SortName => "Name",
            LibraryText::SortSize => "Größe",
            LibraryText::SortStatus => "Status",
            LibraryText::ShowFilter => "Anzeige: %@",
            LibraryText::FilterAll => "Alle Spiele",
            LibraryText::FilterReady => "Bereit",
            LibraryText::FilterNeedsUnlock => "Muss freigeschaltet werden",
            LibraryText::FilterTogether => "Zusammen spielen",
            LibraryText::Search => "Suchen",
            LibraryText::SearchFor => "Suche: %@",
            LibraryText::Storage => "Speicher",
            LibraryText::SearchTitle => "Bibliothek durchsuchen",
            LibraryText::SearchFound => "%@ gefunden",
            LibraryText::SearchNone => "Kein Spiel passt",
            LibraryText::SearchField => "Name",
            LibraryText::SearchShow => "Spiele anzeigen",
            LibraryText::SearchClear => "Suche löschen",
            LibraryText::KeyShift => "Umschalt",
            LibraryText::KeyLetters => "abc",
            LibraryText::KeySymbols => "123",
            LibraryText::KeySpace => "Leertaste",
            LibraryText::KeyDelete => "Löschen",
            LibraryText::NothingHere => "Hier sind keine Spiele",
            LibraryText::NothingHereText => "Kein Spiel passt zur Suche oder zum Filter.",
            LibraryText::ClearFilters => "Alle Spiele anzeigen",
            LibraryText::StorageTitle => "Speicher",
            LibraryText::StorageSummary => {
                "Freigeschaltete Kopien belegen %@. %@ frei auf diesem Laufwerk."
            }
            LibraryText::StorageSummaryNoDrive => "Freigeschaltete Kopien belegen %@.",
            LibraryText::StorageEmpty => "Keine freigeschalteten Kopien",
            LibraryText::StorageEmptyText => {
                "Ein Spiel belegt hier Platz, sobald es freigeschaltet ist. Deine Spieldateien zählen nicht mit."
            }
            LibraryText::StorageMeasuring => "Wird gemessen\u{2026}",
            LibraryText::StorageCopy => "Freigeschaltete Kopie: %@",
            LibraryText::StorageTotal => "Alle freigeschalteten Kopien",
            LibraryText::RemoveCopy => "Freigeschaltete Kopie entfernen",
            LibraryText::RemoveCopyTitle => "Freigeschaltete Kopie von %@ entfernen?",
            LibraryText::RemoveCopyBody => {
                "Das gibt %@ frei. Deine Spieldateien bleiben, wo sie sind, und die Spielstände werden vorher gesichert. Das Spiel erneut freizuschalten dauert eine Weile."
            }
            LibraryText::RemoveCopyRunning => "Beende zuerst das Spiel",
            LibraryText::Cancel => "Abbrechen",
            LibraryText::ShelfFriends => "Freunde spielen gerade",
            LibraryText::ShelfRecent => "Zuletzt gespielt",
            LibraryText::ShelfNew => "Neu",
            LibraryText::Join => "Beitreten",
            LibraryText::ShelfFriendPlays => "%@ spielt das",
            LibraryText::NewGame => "Neues Spiel: %@",
            LibraryText::NewGames => "%@ neue Spiele",
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            LibraryText::RunsWell => "Funciona bien",
            LibraryText::RunsWithIssues => "Funciona con problemas",
            LibraryText::NoStart => "No se inicia",
            LibraryText::Lan => "Jugar juntos",
            LibraryText::LanShort => "Juntos",
            LibraryText::SortBy => "Orden: %@",
            LibraryText::SortRecent => "Jugados recientemente",
            LibraryText::SortName => "Nombre",
            LibraryText::SortSize => "Tamaño",
            LibraryText::SortStatus => "Estado",
            LibraryText::ShowFilter => "Mostrar: %@",
            LibraryText::FilterAll => "Todos los juegos",
            LibraryText::FilterReady => "Listos",
            LibraryText::FilterNeedsUnlock => "Por desbloquear",
            LibraryText::FilterTogether => "Jugar juntos",
            LibraryText::Search => "Buscar",
            LibraryText::SearchFor => "Búsqueda: %@",
            LibraryText::Storage => "Almacenamiento",
            LibraryText::SearchTitle => "Buscar en la biblioteca",
            LibraryText::SearchFound => "Encontrados: %@",
            LibraryText::SearchNone => "Ningún juego coincide",
            LibraryText::SearchField => "Nombre",
            LibraryText::SearchShow => "Mostrar juegos",
            LibraryText::SearchClear => "Borrar búsqueda",
            LibraryText::KeyShift => "Mayús",
            LibraryText::KeyLetters => "abc",
            LibraryText::KeySymbols => "123",
            LibraryText::KeySpace => "Espacio",
            LibraryText::KeyDelete => "Borrar",
            LibraryText::NothingHere => "No hay juegos aquí",
            LibraryText::NothingHereText => "Ningún juego coincide con la búsqueda o el filtro.",
            LibraryText::ClearFilters => "Mostrar todos los juegos",
            LibraryText::StorageTitle => "Almacenamiento",
            LibraryText::StorageSummary => {
                "Las copias desbloqueadas ocupan %@. Quedan %@ libres en esta unidad."
            }
            LibraryText::StorageSummaryNoDrive => "Las copias desbloqueadas ocupan %@.",
            LibraryText::StorageEmpty => "Sin copias desbloqueadas",
            LibraryText::StorageEmptyText => {
                "Un juego ocupa espacio aquí cuando se desbloquea. Tus archivos del juego no cuentan."
            }
            LibraryText::StorageMeasuring => "Midiendo\u{2026}",
            LibraryText::StorageCopy => "Copia desbloqueada: %@",
            LibraryText::StorageTotal => "Todas las copias desbloqueadas",
            LibraryText::RemoveCopy => "Quitar copia desbloqueada",
            LibraryText::RemoveCopyTitle => "¿Quitar la copia desbloqueada de %@?",
            LibraryText::RemoveCopyBody => {
                "Esto libera %@. Tus archivos del juego se quedan donde están y las partidas guardadas se copian antes. Volver a desbloquear el juego tarda un rato."
            }
            LibraryText::RemoveCopyRunning => "Cierra primero el juego",
            LibraryText::Cancel => "Cancelar",
            LibraryText::ShelfFriends => "Amigos jugando ahora",
            LibraryText::ShelfRecent => "Jugados recientemente",
            LibraryText::ShelfNew => "Nuevo",
            LibraryText::Join => "Unirse",
            LibraryText::ShelfFriendPlays => "%@ juega a esto",
            LibraryText::NewGame => "Juego nuevo: %@",
            LibraryText::NewGames => "%@ juegos nuevos",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            LibraryText::RunsWell => "Fonctionne bien",
            LibraryText::RunsWithIssues => "Fonctionne avec des problèmes",
            LibraryText::NoStart => "Ne se lance pas",
            LibraryText::Lan => "Jouer ensemble",
            LibraryText::LanShort => "Ensemble",
            LibraryText::SortBy => "Tri\u{a0}: %@",
            LibraryText::SortRecent => "Joués récemment",
            LibraryText::SortName => "Nom",
            LibraryText::SortSize => "Taille",
            LibraryText::SortStatus => "État",
            LibraryText::ShowFilter => "Afficher\u{a0}: %@",
            LibraryText::FilterAll => "Tous les jeux",
            LibraryText::FilterReady => "Prêts",
            LibraryText::FilterNeedsUnlock => "À déverrouiller",
            LibraryText::FilterTogether => "Jouer ensemble",
            LibraryText::Search => "Rechercher",
            LibraryText::SearchFor => "Recherche\u{a0}: %@",
            LibraryText::Storage => "Stockage",
            LibraryText::SearchTitle => "Rechercher dans la bibliothèque",
            LibraryText::SearchFound => "Trouvés\u{a0}: %@",
            LibraryText::SearchNone => "Aucun jeu ne correspond",
            LibraryText::SearchField => "Nom",
            LibraryText::SearchShow => "Afficher les jeux",
            LibraryText::SearchClear => "Effacer la recherche",
            LibraryText::KeyShift => "Maj",
            LibraryText::KeyLetters => "abc",
            LibraryText::KeySymbols => "123",
            LibraryText::KeySpace => "Espace",
            LibraryText::KeyDelete => "Effacer",
            LibraryText::NothingHere => "Aucun jeu ici",
            LibraryText::NothingHereText => "Aucun jeu ne correspond à la recherche ou au filtre.",
            LibraryText::ClearFilters => "Afficher tous les jeux",
            LibraryText::StorageTitle => "Stockage",
            LibraryText::StorageSummary => {
                "Les copies déverrouillées occupent %@. %@ libres sur ce lecteur."
            }
            LibraryText::StorageSummaryNoDrive => "Les copies déverrouillées occupent %@.",
            LibraryText::StorageEmpty => "Aucune copie déverrouillée",
            LibraryText::StorageEmptyText => {
                "Un jeu prend de la place ici une fois déverrouillé. Tes fichiers de jeu ne comptent pas."
            }
            LibraryText::StorageMeasuring => "Calcul en cours\u{2026}",
            LibraryText::StorageCopy => "Copie déverrouillée\u{a0}: %@",
            LibraryText::StorageTotal => "Toutes les copies déverrouillées",
            LibraryText::RemoveCopy => "Supprimer la copie déverrouillée",
            LibraryText::RemoveCopyTitle => "Supprimer la copie déverrouillée de %@\u{a0}?",
            LibraryText::RemoveCopyBody => {
                "Cela libère %@. Tes fichiers de jeu restent où ils sont, et les sauvegardes sont copiées avant. Déverrouiller à nouveau le jeu prend un moment."
            }
            LibraryText::RemoveCopyRunning => "Ferme d’abord le jeu",
            LibraryText::Cancel => "Annuler",
            LibraryText::ShelfFriends => "Amis en train de jouer",
            LibraryText::ShelfRecent => "Joués récemment",
            LibraryText::ShelfNew => "Nouveau",
            LibraryText::Join => "Rejoindre",
            LibraryText::ShelfFriendPlays => "%@ joue à ce jeu",
            LibraryText::NewGame => "Nouveau jeu\u{a0}: %@",
            LibraryText::NewGames => "%@ nouveaux jeux",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            LibraryText::RunsWell => "Funciona bem",
            LibraryText::RunsWithIssues => "Funciona com problemas",
            LibraryText::NoStart => "Não inicia",
            LibraryText::Lan => "Jogar junto",
            LibraryText::LanShort => "Junto",
            LibraryText::SortBy => "Ordem: %@",
            LibraryText::SortRecent => "Jogados recentemente",
            LibraryText::SortName => "Nome",
            LibraryText::SortSize => "Tamanho",
            LibraryText::SortStatus => "Status",
            LibraryText::ShowFilter => "Mostrar: %@",
            LibraryText::FilterAll => "Todos os jogos",
            LibraryText::FilterReady => "Prontos",
            LibraryText::FilterNeedsUnlock => "Para desbloquear",
            LibraryText::FilterTogether => "Jogar junto",
            LibraryText::Search => "Pesquisar",
            LibraryText::SearchFor => "Pesquisa: %@",
            LibraryText::Storage => "Armazenamento",
            LibraryText::SearchTitle => "Pesquisar na biblioteca",
            LibraryText::SearchFound => "Encontrados: %@",
            LibraryText::SearchNone => "Nenhum jogo corresponde",
            LibraryText::SearchField => "Nome",
            LibraryText::SearchShow => "Mostrar jogos",
            LibraryText::SearchClear => "Limpar pesquisa",
            LibraryText::KeyShift => "Shift",
            LibraryText::KeyLetters => "abc",
            LibraryText::KeySymbols => "123",
            LibraryText::KeySpace => "Espaço",
            LibraryText::KeyDelete => "Apagar",
            LibraryText::NothingHere => "Nenhum jogo aqui",
            LibraryText::NothingHereText => "Nenhum jogo corresponde à pesquisa ou ao filtro.",
            LibraryText::ClearFilters => "Mostrar todos os jogos",
            LibraryText::StorageTitle => "Armazenamento",
            LibraryText::StorageSummary => {
                "As cópias desbloqueadas ocupam %@. %@ livres nesta unidade."
            }
            LibraryText::StorageSummaryNoDrive => "As cópias desbloqueadas ocupam %@.",
            LibraryText::StorageEmpty => "Nenhuma cópia desbloqueada",
            LibraryText::StorageEmptyText => {
                "Um jogo ocupa espaço aqui depois de desbloqueado. Seus arquivos do jogo não contam."
            }
            LibraryText::StorageMeasuring => "Medindo\u{2026}",
            LibraryText::StorageCopy => "Cópia desbloqueada: %@",
            LibraryText::StorageTotal => "Todas as cópias desbloqueadas",
            LibraryText::RemoveCopy => "Remover cópia desbloqueada",
            LibraryText::RemoveCopyTitle => "Remover a cópia desbloqueada de %@?",
            LibraryText::RemoveCopyBody => {
                "Isso libera %@. Seus arquivos do jogo ficam onde estão, e os jogos salvos passam por backup antes. Desbloquear o jogo de novo leva um tempo."
            }
            LibraryText::RemoveCopyRunning => "Feche o jogo primeiro",
            LibraryText::Cancel => "Cancelar",
            LibraryText::ShelfFriends => "Amigos jogando agora",
            LibraryText::ShelfRecent => "Jogados recentemente",
            LibraryText::ShelfNew => "Novo",
            LibraryText::Join => "Participar",
            LibraryText::ShelfFriendPlays => "%@ está jogando isto",
            LibraryText::NewGame => "Novo jogo: %@",
            LibraryText::NewGames => "%@ novos jogos",
            _ => return None,
        })
    }

    pub fn text(self, language: Language) -> &'static str {
        match language {
            Language::English => self.english(),
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
        LibraryText::LanShort,
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
