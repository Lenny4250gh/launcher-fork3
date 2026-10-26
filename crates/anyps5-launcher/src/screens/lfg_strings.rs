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
    pub const fn text(self, language: Language) -> &'static str {
        match language {
            Language::Turkish => self.turkish(),
            Language::English => self.english(),
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
            assert_eq!(
                text.english().matches("%@").count(),
                text.turkish().matches("%@").count(),
                "{text:?}"
            );
        }
    }
}
