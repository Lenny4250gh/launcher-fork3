//! Texts of the online area: signing in, friends, invitations and joining a friend's game.
//! English and Turkish like the rest. Row titles are in title case, notes and toasts in
//! sentence case, as everywhere in the launcher.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OnlineText {
    /// Panel title before sign-in, and the profile button's name.
    Online,
    Subtitle,
    Cloud,
    CloudNotSet,
    /// The note under Sign Out: which cloud the account is on. `%@` = the address.
    SignedInOn,
    /// The cloud AnyPlay runs, as the cloud row names it.
    CloudOfficial,
    CloudUseOfficial,
    CloudSubtitle,
    CloudAddress,
    CloudUse,
    CloudHint,
    /// Toast: the address is not an http or https URL.
    CloudInvalid,
    Sending,
    SignInNote,
    NeedsCloud,
    SignIn,
    Handle,
    CreateAccount,
    /// Subtitle while the browser sign-in waits.
    ContinueInBrowser,
    OpenAgain,
    BrowserNote,
    /// Toast: the browser sign-in ran out of time.
    SignInTimedOut,
    ManageAccount,
    ManageAccountDetail,
    StatusOnline,
    StatusConnecting,
    StatusOffline,
    Invitations,
    Requests,
    /// The row under the first few requests; leads to all of them.
    SeeAllRequests,
    /// `%@` = how many.
    RequestsWaiting,
    /// Under the friends title, after the handle: `%@` = how many friends are online.
    FriendsOnlineCount,
    Friends,
    Sent,
    Session,
    Blocked,
    /// `%@` = the game.
    Playing,
    /// In a session of the AnyPlay network. `%@` = the game.
    PlayingOnline,
    PlayingOnlineJoinable,
    FriendOnline,
    FriendOffline,
    NoFriends,
    AddFriend,
    AddFriendSubtitle,
    SendRequest,
    AddFriendHint,
    WantsFriend,
    Waiting,
    Accept,
    Decline,
    Withdraw,
    RemoveFriend,
    Block,
    Unblock,
    Report,
    /// `%@` = the friend.
    ReportTitle,
    Reason,
    ReasonCheating,
    ReasonHarassment,
    ReasonSpam,
    ReasonName,
    ReasonOther,
    SendReport,
    JoinGame,
    /// `%@` members of `%@`.
    JoinDetail,
    InviteToSession,
    /// `%@` = who invites.
    InvitedBy,
    LinkDirect,
    LinkRelay,
    LinkConnecting,
    You,
    Account,
    SignOut,
    /// `%@` = the game.
    PickerTitle,
    /// `%@` = how many at most.
    PickerSubtitle,
    SendInvitations,
    // Toasts
    /// `%@` = the handle.
    Welcome,
    SignedOut,
    /// `%@` = the handle.
    RequestSent,
    /// `%@` = the handle.
    NowFriends,
    /// Under the field of Add a Friend: what the cloud refused.
    AddSelf,
    AddBlocked,
    AddLimit,
    /// Under the field: the name cannot be one.
    AddEnterName,
    /// `%@` = the handle.
    RequestArrived,
    /// `%@` = who, `%@` = the game.
    InvitationArrived,
    /// `%@` = the handle.
    BlockedToast,
    Reported,
    /// `%@` = the friend, `%@` = the game.
    Joining,
    InvitationsSent,
    // Problems
    Offline,
    CodeWrong,
    CodeExpired,
    HandleTaken,
    HandleInvalid,
    NotFound,
    RateLimited,
    Banned,
    /// `%@` = the reason the service gave.
    Failed,
    /// `%@` = the game.
    NotOwned,
    /// `%@` = the game.
    NotUnlocked,
    /// `%@` friend, `%@` their version, `%@` their AnyPS5, `%@` yours, `%@` your AnyPS5.
    VersionMismatch,
    Full,
    ServiceMissing,
    ServiceEnded,
    // Settings
    RecordNetwork,
    RecordNetworkDetail,
    RelayOnly,
    RelayOnlyNote,
    /// Linux: the OpenH264 row and its states.
    Openh264,
    Openh264Off,
    Openh264Downloading,
    Openh264Ready,
    Openh264Missing,
    /// `%@`: why the download failed.
    Openh264Failed,
    /// The share worker found a file that is not Cisco's build.
    Openh264Refused,
    /// `%@`: why the share worker could not load it.
    Openh264NotLoaded,
    /// Cisco's licence asks for this text, unchanged, where the player turns the codec on and off.
    Openh264Notice,
    // Appearing offline
    AppearOffline,
    AppearOfflineDetail,
    PublicProfile,
    PublicProfileDetail,
    PublicProfileNote,
    /// The quiet words where the player sees their own status.
    AppearingOffline,
    // Trophies
    /// A friend hides their trophies from the player.
    TrophiesHidden,
    TrophiesNone,
    TrophyAudience,
    AudienceFriends,
    AudienceEveryone,
    AudienceOnlyYou,
    TrophyAudienceNote,
}

impl OnlineText {
    pub const fn english(self) -> &'static str {
        match self {
            OnlineText::Online => "Online",
            OnlineText::Subtitle => "Play with your friends over the internet",
            OnlineText::SignedInOn => "Signed in on %@.",
            OnlineText::Cloud => "Cloud",
            OnlineText::CloudNotSet => "Not set up yet",
            OnlineText::CloudOfficial => "AnyPlay",
            OnlineText::CloudUseOfficial => "Use the AnyPlay Cloud",
            OnlineText::CloudSubtitle => "The address of the cloud you sign in to",
            OnlineText::CloudAddress => "Address",
            OnlineText::CloudUse => "Use This Cloud",
            OnlineText::CloudHint => {
                "A cloud of your own, or of friends who run one. Ask whoever runs it for its address, such as https://cloud.example.org."
            }
            OnlineText::CloudInvalid => "Enter an address that starts with https://",
            OnlineText::Sending => "Sending…",
            OnlineText::SignInNote => {
                "You sign in or create your account on the cloud's page in your browser. Nothing goes online before that."
            }
            OnlineText::NeedsCloud => "Set up the cloud first.",
            OnlineText::SignIn => "Sign In",
            OnlineText::Handle => "Name",
            OnlineText::CreateAccount => "Create Account",
            OnlineText::ContinueInBrowser => "Continue in your browser",
            OnlineText::OpenAgain => "Open the Page Again",
            OnlineText::BrowserNote => "When you are done there, the launcher signs in by itself.",
            OnlineText::SignInTimedOut => "The sign-in timed out. Try again.",
            OnlineText::ManageAccount => "Manage Account",
            OnlineText::ManageAccountDetail => "E-mail, passkeys, devices and deletion",
            OnlineText::StatusOnline => "Online",
            OnlineText::StatusConnecting => "Connecting…",
            OnlineText::StatusOffline => "Offline, trying again",
            OnlineText::Invitations => "Invitations",
            OnlineText::Requests => "Friend Requests",
            OnlineText::SeeAllRequests => "See All Requests",
            OnlineText::RequestsWaiting => "%@ waiting",
            OnlineText::FriendsOnlineCount => "%@ online",
            OnlineText::Friends => "Friends",
            OnlineText::Sent => "Sent Requests",
            OnlineText::Session => "Playing Together",
            OnlineText::Blocked => "Blocked",
            OnlineText::Playing => "Playing %@",
            OnlineText::PlayingOnline => "Playing %@ · Online",
            OnlineText::PlayingOnlineJoinable => "Playing %@ · Online · You can join",
            OnlineText::FriendOnline => "Online",
            OnlineText::FriendOffline => "Offline",
            OnlineText::NoFriends => "No friends yet. Add one by the name they chose.",
            OnlineText::AddFriend => "Add a Friend",
            OnlineText::AddFriendSubtitle => "Enter the name of their account",
            OnlineText::SendRequest => "Send Request",
            OnlineText::AddFriendHint => {
                "Capital letters do not matter. They see the request once they are online."
            }
            OnlineText::WantsFriend => "Wants to be your friend",
            OnlineText::Waiting => "Waiting for an answer",
            OnlineText::Accept => "Accept",
            OnlineText::Decline => "Decline",
            OnlineText::Withdraw => "Withdraw Request",
            OnlineText::RemoveFriend => "Remove Friend",
            OnlineText::Block => "Block",
            OnlineText::Unblock => "Unblock",
            OnlineText::Report => "Report",
            OnlineText::ReportTitle => "Report %@",
            OnlineText::Reason => "Reason",
            OnlineText::ReasonCheating => "Cheating",
            OnlineText::ReasonHarassment => "Harassment",
            OnlineText::ReasonSpam => "Spam",
            OnlineText::ReasonName => "Offensive Name",
            OnlineText::ReasonOther => "Something Else",
            OnlineText::SendReport => "Send Report",
            OnlineText::JoinGame => "Join Game",
            OnlineText::JoinDetail => "%@ of %@ players",
            OnlineText::InviteToSession => "Invite to Your Game",
            OnlineText::InvitedBy => "%@ invites you",
            OnlineText::LinkDirect => "Direct",
            OnlineText::LinkRelay => "Through a relay",
            OnlineText::LinkConnecting => "Connecting…",
            OnlineText::You => "You",
            OnlineText::Account => "Account",
            OnlineText::SignOut => "Sign Out",
            OnlineText::PickerTitle => "Invite to %@",
            OnlineText::PickerSubtitle => "Pick up to %@ friends",
            OnlineText::SendInvitations => "Send Invitations",
            OnlineText::Welcome => "Signed in as %@",
            OnlineText::SignedOut => "Signed out",
            OnlineText::RequestSent => "Request sent to %@",
            OnlineText::NowFriends => "You and %@ are friends",
            OnlineText::AddSelf => "You cannot add yourself.",
            OnlineText::AddBlocked => "You blocked this account. Unblock it first.",
            OnlineText::AddLimit => "A limit on friends or requests is reached. Try again later.",
            OnlineText::AddEnterName => "Enter their name first",
            OnlineText::RequestArrived => "%@ wants to be your friend",
            OnlineText::InvitationArrived => "%@ invites you to %@",
            OnlineText::BlockedToast => "%@ is blocked",
            OnlineText::Reported => "Report sent. Thank you.",
            OnlineText::Joining => "Joining %@ in %@",
            OnlineText::InvitationsSent => "Invitations sent",
            OnlineText::Offline => {
                "The cloud cannot be reached. Check your connection and try again."
            }
            OnlineText::CodeWrong => "That code does not match. Check the latest e-mail.",
            OnlineText::CodeExpired => "The code expired. Send a new one.",
            OnlineText::HandleTaken => "That name is taken. Try another one.",
            OnlineText::HandleInvalid => {
                "Use 3 to 16 letters, digits, - or _, starting with a letter."
            }
            OnlineText::NotFound => "No account has this name.",
            OnlineText::RateLimited => "Too many tries. Wait a moment and try again.",
            OnlineText::Banned => "This account is banned.",
            OnlineText::Failed => "That did not work: %@",
            OnlineText::NotOwned => "%@ is not in your library.",
            OnlineText::NotUnlocked => "Unlock %@ first, then join.",
            OnlineText::VersionMismatch => {
                "%@ plays version %@ with AnyPS5 %@. You have %@ with AnyPS5 %@."
            }
            OnlineText::Full => "The game is full.",
            OnlineText::ServiceMissing => {
                "This installation has no network service. Install the launcher again."
            }
            OnlineText::ServiceEnded => {
                "The network service stopped. Open Online to start it again."
            }
            OnlineText::RecordNetwork => "Record Network Calls",
            OnlineText::RecordNetworkDetail => {
                "Games write their network requests to the Logs folder, to help build online support. Nothing is uploaded."
            }
            OnlineText::AppearOffline => "Appear Offline",
            OnlineText::AppearOfflineDetail => "Friends see you as offline",
            OnlineText::PublicProfile => "Public Profile",
            OnlineText::PublicProfileDetail => "Anyone can open your page on the web",
            OnlineText::PublicProfileNote => {
                "Off until you turn it on. Even then, people who are not your friends see only the parts you share with Everyone."
            }
            OnlineText::AppearingOffline => "Appearing offline",
            OnlineText::RelayOnly => "Hide Your IP Address",
            OnlineText::RelayOnlyNote => {
                "Other players reach you only through a relay and never see your IP address. Games can respond a little slower."
            }
            OnlineText::Openh264 => "Video for Share Screen",
            OnlineText::Openh264Off => {
                "Turn on to download Cisco's video codec. Share Screen cannot send or show video without it"
            }
            OnlineText::Openh264Downloading => "Downloading from Cisco",
            OnlineText::Openh264Ready => "Downloaded from Cisco",
            OnlineText::Openh264Missing => "Not downloaded yet",
            OnlineText::Openh264Failed => "The download failed: %@",
            OnlineText::Openh264Refused => {
                "The file is not Cisco's build. Turn it off and on to download it again."
            }
            OnlineText::Openh264NotLoaded => "It did not load: %@",
            OnlineText::Openh264Notice => "OpenH264 Video Codec provided by Cisco Systems, Inc.",
            OnlineText::TrophiesHidden => "Hidden",
            OnlineText::TrophiesNone => "None yet",
            OnlineText::TrophyAudience => "Who Sees Your Trophies",
            OnlineText::AudienceFriends => "Friends",
            OnlineText::AudienceEveryone => "Everyone",
            OnlineText::AudienceOnlyYou => "Only You",
            OnlineText::TrophyAudienceNote => {
                "The players you choose see how many trophies you have of each grade. Only each trophy's number, grade and date leave this PC."
            }
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            OnlineText::Online => "Çevrimiçi",
            OnlineText::Subtitle => "Arkadaşlarınla internet üzerinden oyna",
            OnlineText::SignedInOn => "%@ üzerinde oturum açık.",
            OnlineText::Cloud => "Bulut",
            OnlineText::CloudNotSet => "Henüz ayarlanmadı",
            OnlineText::CloudOfficial => "AnyPlay",
            OnlineText::CloudUseOfficial => "AnyPlay Bulutunu Kullan",
            OnlineText::CloudSubtitle => "Giriş yaptığın bulutun adresi",
            OnlineText::CloudAddress => "Adres",
            OnlineText::CloudUse => "Bu Bulutu Kullan",
            OnlineText::CloudHint => {
                "Kendi bulutun ya da bir tane işleten arkadaşlarının bulutu. Adresi işleten kişiden iste, örneğin https://cloud.example.org."
            }
            OnlineText::CloudInvalid => "https:// ile başlayan bir adres gir",
            OnlineText::Sending => "Gönderiliyor…",
            OnlineText::SignInNote => {
                "Giriş yapma ve hesap oluşturma tarayıcında, bulutun kendi sayfasında olur. Bundan önce hiçbir şey çevrimiçi olmaz."
            }
            OnlineText::NeedsCloud => "Önce bulutu ayarla.",
            OnlineText::SignIn => "Giriş Yap",
            OnlineText::Handle => "Ad",
            OnlineText::CreateAccount => "Hesap Oluştur",
            OnlineText::ContinueInBrowser => "Tarayıcında devam et",
            OnlineText::OpenAgain => "Sayfayı Yeniden Aç",
            OnlineText::BrowserNote => "Orada işin bitince başlatıcı kendiliğinden giriş yapar.",
            OnlineText::SignInTimedOut => "Girişin süresi doldu. Yeniden dene.",
            OnlineText::ManageAccount => "Hesabı Yönet",
            OnlineText::ManageAccountDetail => "E-posta, geçiş anahtarları, cihazlar ve silme",
            OnlineText::StatusOnline => "Çevrimiçi",
            OnlineText::StatusConnecting => "Bağlanıyor…",
            OnlineText::StatusOffline => "Çevrimdışı, yeniden deneniyor",
            OnlineText::Invitations => "Davetler",
            OnlineText::Requests => "Arkadaşlık İstekleri",
            OnlineText::SeeAllRequests => "Tüm İstekleri Gör",
            OnlineText::RequestsWaiting => "%@ bekliyor",
            OnlineText::FriendsOnlineCount => "%@ çevrimiçi",
            OnlineText::Friends => "Arkadaşlar",
            OnlineText::Sent => "Gönderilen İstekler",
            OnlineText::Session => "Birlikte Oynananlar",
            OnlineText::Blocked => "Engellenenler",
            OnlineText::Playing => "%@ oynuyor",
            OnlineText::PlayingOnline => "%@ oynuyor · Çevrimiçi",
            OnlineText::PlayingOnlineJoinable => "%@ oynuyor · Çevrimiçi · Katılabilirsin",
            OnlineText::FriendOnline => "Çevrimiçi",
            OnlineText::FriendOffline => "Çevrimdışı",
            OnlineText::NoFriends => "Henüz arkadaşın yok. Seçtikleri adla birini ekle.",
            OnlineText::AddFriend => "Arkadaş Ekle",
            OnlineText::AddFriendSubtitle => "Hesabının adını gir",
            OnlineText::SendRequest => "İstek Gönder",
            OnlineText::AddFriendHint => {
                "Büyük harfler fark etmez. İsteği çevrimiçi olduklarında görürler."
            }
            OnlineText::WantsFriend => "Arkadaşın olmak istiyor",
            OnlineText::Waiting => "Yanıt bekleniyor",
            OnlineText::Accept => "Kabul Et",
            OnlineText::Decline => "Reddet",
            OnlineText::Withdraw => "İsteği Geri Çek",
            OnlineText::RemoveFriend => "Arkadaşlıktan Çıkar",
            OnlineText::Block => "Engelle",
            OnlineText::Unblock => "Engeli Kaldır",
            OnlineText::Report => "Şikâyet Et",
            OnlineText::ReportTitle => "%@ Kişisini Şikâyet Et",
            OnlineText::Reason => "Neden",
            OnlineText::ReasonCheating => "Hile",
            OnlineText::ReasonHarassment => "Taciz",
            OnlineText::ReasonSpam => "Spam",
            OnlineText::ReasonName => "Uygunsuz Ad",
            OnlineText::ReasonOther => "Başka Bir Şey",
            OnlineText::SendReport => "Şikâyeti Gönder",
            OnlineText::JoinGame => "Oyuna Katıl",
            OnlineText::JoinDetail => "%@ / %@ oyuncu",
            OnlineText::InviteToSession => "Oyununa Davet Et",
            OnlineText::InvitedBy => "%@ seni davet ediyor",
            OnlineText::LinkDirect => "Doğrudan",
            OnlineText::LinkRelay => "Aktarıcı üzerinden",
            OnlineText::LinkConnecting => "Bağlanıyor…",
            OnlineText::You => "Sen",
            OnlineText::Account => "Hesap",
            OnlineText::SignOut => "Çıkış Yap",
            OnlineText::PickerTitle => "%@ Oyununa Davet Et",
            OnlineText::PickerSubtitle => "En fazla %@ arkadaş seç",
            OnlineText::SendInvitations => "Davetleri Gönder",
            OnlineText::Welcome => "%@ olarak giriş yapıldı",
            OnlineText::SignedOut => "Çıkış yapıldı",
            OnlineText::RequestSent => "%@ kişisine istek gönderildi",
            OnlineText::NowFriends => "%@ ile artık arkadaşsınız",
            OnlineText::AddSelf => "Kendini ekleyemezsin.",
            OnlineText::AddBlocked => "Bu hesabı engelledin. Önce engeli kaldır.",
            OnlineText::AddLimit => "Arkadaş veya istek sınırına ulaşıldı. Sonra yeniden dene.",
            OnlineText::AddEnterName => "Önce adını gir",
            OnlineText::RequestArrived => "%@ arkadaşın olmak istiyor",
            OnlineText::InvitationArrived => "%@ seni %@ oyununa davet ediyor",
            OnlineText::BlockedToast => "%@ engellendi",
            OnlineText::Reported => "Şikâyet gönderildi. Teşekkürler.",
            OnlineText::Joining => "%@ kişisinin %@ oyununa katılınıyor",
            OnlineText::InvitationsSent => "Davetler gönderildi",
            OnlineText::Offline => "Buluta ulaşılamıyor. Bağlantını kontrol edip yeniden dene.",
            OnlineText::CodeWrong => "Kod eşleşmiyor. En son e-postaya bak.",
            OnlineText::CodeExpired => "Kodun süresi doldu. Yeni bir kod gönder.",
            OnlineText::HandleTaken => "Bu ad alınmış. Başka bir ad dene.",
            OnlineText::HandleInvalid => {
                "Harfle başlayan, 3 ile 16 arası harf, rakam, - veya _ kullan."
            }
            OnlineText::NotFound => "Bu adda bir hesap yok.",
            OnlineText::RateLimited => "Çok fazla deneme. Biraz bekleyip yeniden dene.",
            OnlineText::Banned => "Bu hesap engellendi.",
            OnlineText::Failed => "Olmadı: %@",
            OnlineText::NotOwned => "%@ kütüphanende yok.",
            OnlineText::NotUnlocked => "Önce %@ oyununun kilidini aç, sonra katıl.",
            OnlineText::VersionMismatch => {
                "%@, %@ sürümünü AnyPS5 %@ ile oynuyor. Sende %@ sürümü AnyPS5 %@ ile var."
            }
            OnlineText::Full => "Oyun dolu.",
            OnlineText::ServiceMissing => "Bu kurulumda ağ hizmeti yok. Başlatıcıyı yeniden kur.",
            OnlineText::ServiceEnded => "Ağ hizmeti durdu. Yeniden başlatmak için Çevrimiçi'yi aç.",
            OnlineText::RecordNetwork => "Ağ Çağrılarını Kaydet",
            OnlineText::RecordNetworkDetail => {
                "Oyunlar ağdan ne istediklerini Logs klasörüne yazar; çevrimiçi desteğin geliştirilmesine yardım eder. Hiçbir şey yüklenmez."
            }
            OnlineText::AppearOffline => "Çevrimdışı Görün",
            OnlineText::AppearOfflineDetail => "Arkadaşların seni çevrimdışı görür",
            OnlineText::PublicProfile => "Herkese Açık Profil",
            OnlineText::PublicProfileDetail => "Herkes web sayfanı açabilir",
            OnlineText::PublicProfileNote => {
                "Sen açana kadar kapalıdır. Açıkken bile arkadaşın olmayanlar yalnızca Herkes için paylaştığın bölümleri görür."
            }
            OnlineText::AppearingOffline => "Çevrimdışı görünüyorsun",
            OnlineText::RelayOnly => "IP Adresini Gizle",
            OnlineText::RelayOnlyNote => {
                "Diğer oyuncular sana yalnızca bir aktarıcı üzerinden ulaşır ve IP adresini hiç görmez. Oyunlar biraz daha geç tepki verebilir."
            }
            OnlineText::Openh264 => "Ekran Paylaşımında Video",
            OnlineText::Openh264Off => {
                "Cisco'nun video kodeğini indirmek için aç. Kodek olmadan ekran paylaşımı video gönderemez ve gösteremez"
            }
            OnlineText::Openh264Downloading => "Cisco'dan indiriliyor",
            OnlineText::Openh264Ready => "Cisco'dan indirildi",
            OnlineText::Openh264Missing => "Henüz indirilmedi",
            OnlineText::Openh264Failed => "İndirme başarısız oldu: %@",
            OnlineText::Openh264Refused => {
                "Dosya Cisco'nun derlemesi değil. Yeniden indirmek için kapatıp aç."
            }
            OnlineText::Openh264NotLoaded => "Yüklenemedi: %@",
            // Cisco's text stays in English in every language.
            OnlineText::Openh264Notice => "OpenH264 Video Codec provided by Cisco Systems, Inc.",
            OnlineText::TrophiesHidden => "Gizli",
            OnlineText::TrophiesNone => "Henüz yok",
            OnlineText::TrophyAudience => "Kupalarını Kimler Görür",
            OnlineText::AudienceFriends => "Arkadaşlar",
            OnlineText::AudienceEveryone => "Herkes",
            OnlineText::AudienceOnlyYou => "Yalnızca Sen",
            OnlineText::TrophyAudienceNote => {
                "Seçtiğin oyuncular her dereceden kaç kupan olduğunu görür. Bilgisayarından yalnızca her kupanın numarası, derecesi ve tarihi çıkar."
            }
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            OnlineText::Online => "Online",
            OnlineText::Subtitle => "Spiele mit deinen Freunden über das Internet",
            OnlineText::SignedInOn => "Angemeldet auf %@.",
            OnlineText::Cloud => "Cloud",
            OnlineText::CloudNotSet => "Noch nicht eingerichtet",
            OnlineText::CloudOfficial => "AnyPlay",
            OnlineText::CloudUseOfficial => "AnyPlay-Cloud verwenden",
            OnlineText::CloudSubtitle => "Die Adresse der Cloud, bei der du dich anmeldest",
            OnlineText::CloudAddress => "Adresse",
            OnlineText::CloudUse => "Diese Cloud verwenden",
            OnlineText::CloudHint => {
                "Eine eigene Cloud oder die von Freunden, die eine betreiben. Frage den Betreiber nach der Adresse, zum Beispiel https://cloud.example.org."
            }
            OnlineText::CloudInvalid => "Gib eine Adresse ein, die mit https:// beginnt",
            OnlineText::Sending => "Sende …",
            OnlineText::SignInNote => {
                "Du meldest dich an oder erstellst dein Konto auf der Seite der Cloud in deinem Browser. Vorher geht nichts online."
            }
            OnlineText::NeedsCloud => "Richte zuerst die Cloud ein.",
            OnlineText::SignIn => "Anmelden",
            OnlineText::Handle => "Name",
            OnlineText::CreateAccount => "Konto erstellen",
            OnlineText::ContinueInBrowser => "Weiter in deinem Browser",
            OnlineText::OpenAgain => "Seite erneut öffnen",
            OnlineText::BrowserNote => {
                "Sobald du dort fertig bist, meldet sich der Launcher selbst an."
            }
            OnlineText::SignInTimedOut => "Die Anmeldung ist abgelaufen. Versuche es erneut.",
            OnlineText::ManageAccount => "Konto verwalten",
            OnlineText::ManageAccountDetail => "E-Mail, Passkeys, Geräte und Löschung",
            OnlineText::StatusOnline => "Online",
            OnlineText::StatusConnecting => "Verbinde …",
            OnlineText::StatusOffline => "Offline, neuer Versuch",
            OnlineText::Invitations => "Einladungen",
            OnlineText::Requests => "Freundschaftsanfragen",
            OnlineText::SeeAllRequests => "Alle Anfragen ansehen",
            OnlineText::RequestsWaiting => "%@ warten",
            OnlineText::FriendsOnlineCount => "%@ online",
            OnlineText::Friends => "Freunde",
            OnlineText::Sent => "Gesendete Anfragen",
            OnlineText::Session => "Gemeinsam spielen",
            OnlineText::Blocked => "Blockiert",
            OnlineText::Playing => "Spielt %@",
            OnlineText::PlayingOnline => "Spielt %@ · Online",
            OnlineText::PlayingOnlineJoinable => "Spielt %@ · Online · Du kannst beitreten",
            OnlineText::FriendOnline => "Online",
            OnlineText::FriendOffline => "Offline",
            OnlineText::NoFriends => {
                "Noch keine Freunde. Füge jemanden mit dem gewählten Namen hinzu."
            }
            OnlineText::AddFriend => "Freund hinzufügen",
            OnlineText::AddFriendSubtitle => "Gib den Namen des Kontos ein",
            OnlineText::SendRequest => "Anfrage senden",
            OnlineText::AddFriendHint => {
                "Groß- und Kleinschreibung spielt keine Rolle. Die Anfrage sehen sie, sobald sie online sind."
            }
            OnlineText::WantsFriend => "Möchte dein Freund sein",
            OnlineText::Waiting => "Warte auf Antwort",
            OnlineText::Accept => "Annehmen",
            OnlineText::Decline => "Ablehnen",
            OnlineText::Withdraw => "Anfrage zurückziehen",
            OnlineText::RemoveFriend => "Freund entfernen",
            OnlineText::Block => "Blockieren",
            OnlineText::Unblock => "Blockierung aufheben",
            OnlineText::Report => "Melden",
            OnlineText::ReportTitle => "%@ melden",
            OnlineText::Reason => "Grund",
            OnlineText::ReasonCheating => "Cheaten",
            OnlineText::ReasonHarassment => "Belästigung",
            OnlineText::ReasonSpam => "Spam",
            OnlineText::ReasonName => "Anstößiger Name",
            OnlineText::ReasonOther => "Etwas anderes",
            OnlineText::SendReport => "Meldung senden",
            OnlineText::JoinGame => "Spiel beitreten",
            OnlineText::JoinDetail => "%@ von %@ Spielern",
            OnlineText::InviteToSession => "In dein Spiel einladen",
            OnlineText::InvitedBy => "%@ lädt dich ein",
            OnlineText::LinkDirect => "Direkt",
            OnlineText::LinkRelay => "Über ein Relay",
            OnlineText::LinkConnecting => "Verbinde …",
            OnlineText::You => "Du",
            OnlineText::Account => "Konto",
            OnlineText::SignOut => "Abmelden",
            OnlineText::PickerTitle => "Zu %@ einladen",
            OnlineText::PickerSubtitle => "Wähle bis zu %@ Freunde",
            OnlineText::SendInvitations => "Einladungen senden",
            OnlineText::Welcome => "Angemeldet als %@",
            OnlineText::SignedOut => "Abgemeldet",
            OnlineText::RequestSent => "Anfrage an %@ gesendet",
            OnlineText::NowFriends => "Du und %@ seid jetzt Freunde",
            OnlineText::AddSelf => "Du kannst dich nicht selbst hinzufügen.",
            OnlineText::AddBlocked => {
                "Du hast dieses Konto blockiert. Hebe die Blockierung zuerst auf."
            }
            OnlineText::AddLimit => {
                "Das Limit für Freunde oder Anfragen ist erreicht. Versuche es später erneut."
            }
            OnlineText::AddEnterName => "Gib zuerst den Namen ein",
            OnlineText::RequestArrived => "%@ möchte dein Freund sein",
            OnlineText::InvitationArrived => "%@ lädt dich zu %@ ein",
            OnlineText::BlockedToast => "%@ ist blockiert",
            OnlineText::Reported => "Meldung gesendet. Danke.",
            OnlineText::Joining => "Trete dem Spiel von %@ bei: %@",
            OnlineText::InvitationsSent => "Einladungen gesendet",
            OnlineText::Offline => {
                "Die Cloud ist nicht erreichbar. Prüfe deine Verbindung und versuche es erneut."
            }
            OnlineText::CodeWrong => "Der Code stimmt nicht. Sieh in der neuesten E-Mail nach.",
            OnlineText::CodeExpired => "Der Code ist abgelaufen. Sende einen neuen.",
            OnlineText::HandleTaken => "Dieser Name ist vergeben. Versuche einen anderen.",
            OnlineText::HandleInvalid => {
                "Nutze 3 bis 16 Buchstaben, Ziffern, - oder _, beginnend mit einem Buchstaben."
            }
            OnlineText::NotFound => "Kein Konto hat diesen Namen.",
            OnlineText::RateLimited => {
                "Zu viele Versuche. Warte einen Moment und versuche es erneut."
            }
            OnlineText::Banned => "Dieses Konto ist gesperrt.",
            OnlineText::Failed => "Das hat nicht geklappt: %@",
            OnlineText::NotOwned => "%@ ist nicht in deiner Bibliothek.",
            OnlineText::NotUnlocked => "Schalte zuerst %@ frei, dann tritt bei.",
            OnlineText::VersionMismatch => {
                "%@ spielt Version %@ mit AnyPS5 %@. Du hast %@ mit AnyPS5 %@."
            }
            OnlineText::Full => "Das Spiel ist voll.",
            OnlineText::ServiceMissing => {
                "Diese Installation hat keinen Netzwerkdienst. Installiere den Launcher erneut."
            }
            OnlineText::ServiceEnded => {
                "Der Netzwerkdienst wurde beendet. Öffne Online, um ihn neu zu starten."
            }
            OnlineText::RecordNetwork => "Netzwerkaufrufe aufzeichnen",
            OnlineText::RecordNetworkDetail => {
                "Spiele schreiben ihre Netzwerkanfragen in den Ordner Logs, um den Online-Support zu verbessern. Es wird nichts hochgeladen."
            }
            OnlineText::AppearOffline => "Offline erscheinen",
            OnlineText::AppearOfflineDetail => "Freunde sehen dich als offline",
            OnlineText::PublicProfile => "Öffentliches Profil",
            OnlineText::PublicProfileDetail => "Jeder kann deine Seite im Web öffnen",
            OnlineText::PublicProfileNote => {
                "Aus, bis du es einschaltest. Auch dann sehen Personen, die nicht deine Freunde sind, nur die Teile, die du für Alle freigegeben hast."
            }
            OnlineText::AppearingOffline => "Du erscheinst offline",
            OnlineText::RelayOnly => "IP-Adresse verbergen",
            OnlineText::RelayOnlyNote => {
                "Andere Spieler erreichen dich nur über ein Relay und sehen nie deine IP-Adresse. Spiele können etwas langsamer reagieren."
            }
            OnlineText::Openh264 => "Video für Bildschirmfreigabe",
            OnlineText::Openh264Off => {
                "Einschalten, um den Video-Codec von Cisco herunterzuladen. Ohne ihn kann die Bildschirmfreigabe kein Video senden oder zeigen"
            }
            OnlineText::Openh264Downloading => "Wird von Cisco heruntergeladen",
            OnlineText::Openh264Ready => "Von Cisco heruntergeladen",
            OnlineText::Openh264Missing => "Noch nicht heruntergeladen",
            OnlineText::Openh264Failed => "Der Download ist fehlgeschlagen: %@",
            OnlineText::Openh264Refused => {
                "Die Datei ist nicht der Build von Cisco. Schalte aus und wieder ein, um sie erneut zu laden."
            }
            OnlineText::Openh264NotLoaded => "Es wurde nicht geladen: %@",
            // Cisco's text stays in English in every language.
            OnlineText::Openh264Notice => "OpenH264 Video Codec provided by Cisco Systems, Inc.",
            OnlineText::TrophiesHidden => "Versteckt",
            OnlineText::TrophiesNone => "Noch keine",
            OnlineText::TrophyAudience => "Wer deine Trophäen sieht",
            OnlineText::AudienceFriends => "Freunde",
            OnlineText::AudienceEveryone => "Alle",
            OnlineText::AudienceOnlyYou => "Nur du",
            OnlineText::TrophyAudienceNote => {
                "Die Spieler, die du wählst, sehen, wie viele Trophäen du pro Stufe hast. Nur Nummer, Stufe und Datum jeder Trophäe verlassen diesen PC."
            }
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            OnlineText::Online => "En línea",
            OnlineText::Subtitle => "Juega con tus amigos por internet",
            OnlineText::SignedInOn => "Sesión iniciada en %@.",
            OnlineText::Cloud => "Nube",
            OnlineText::CloudNotSet => "Aún sin configurar",
            OnlineText::CloudOfficial => "AnyPlay",
            OnlineText::CloudUseOfficial => "Usar la nube de AnyPlay",
            OnlineText::CloudSubtitle => "La dirección de la nube en la que inicias sesión",
            OnlineText::CloudAddress => "Dirección",
            OnlineText::CloudUse => "Usar esta nube",
            OnlineText::CloudHint => {
                "Una nube propia o de amigos que tengan una. Pide la dirección a quien la gestione, por ejemplo https://cloud.example.org."
            }
            OnlineText::CloudInvalid => "Escribe una dirección que empiece por https://",
            OnlineText::Sending => "Enviando…",
            OnlineText::SignInNote => {
                "Inicias sesión o creas tu cuenta en la página de la nube, en tu navegador. Antes de eso, nada se conecta."
            }
            OnlineText::NeedsCloud => "Configura primero la nube.",
            OnlineText::SignIn => "Iniciar sesión",
            OnlineText::Handle => "Nombre",
            OnlineText::CreateAccount => "Crear cuenta",
            OnlineText::ContinueInBrowser => "Continúa en tu navegador",
            OnlineText::OpenAgain => "Abrir la página de nuevo",
            OnlineText::BrowserNote => "Cuando termines allí, el launcher inicia sesión solo.",
            OnlineText::SignInTimedOut => {
                "Se agotó el tiempo del inicio de sesión. Inténtalo de nuevo."
            }
            OnlineText::ManageAccount => "Gestionar cuenta",
            OnlineText::ManageAccountDetail => {
                "Correo electrónico, llaves de acceso, dispositivos y eliminación"
            }
            OnlineText::StatusOnline => "En línea",
            OnlineText::StatusConnecting => "Conectando…",
            OnlineText::StatusOffline => "Sin conexión, reintentando",
            OnlineText::Invitations => "Invitaciones",
            OnlineText::Requests => "Solicitudes de amistad",
            OnlineText::SeeAllRequests => "Ver todas las solicitudes",
            OnlineText::RequestsWaiting => "%@ en espera",
            OnlineText::FriendsOnlineCount => "%@ en línea",
            OnlineText::Friends => "Amigos",
            OnlineText::Sent => "Solicitudes enviadas",
            OnlineText::Session => "Play Together",
            OnlineText::Blocked => "Bloqueados",
            OnlineText::Playing => "Jugando a %@",
            OnlineText::PlayingOnline => "Jugando a %@ · En línea",
            OnlineText::PlayingOnlineJoinable => "Jugando a %@ · En línea · Puedes unirte",
            OnlineText::FriendOnline => "En línea",
            OnlineText::FriendOffline => "Sin conexión",
            OnlineText::NoFriends => {
                "Aún no tienes amigos. Añade a alguien con el nombre que eligió."
            }
            OnlineText::AddFriend => "Añadir amigo",
            OnlineText::AddFriendSubtitle => "Escribe el nombre de su cuenta",
            OnlineText::SendRequest => "Enviar solicitud",
            OnlineText::AddFriendHint => {
                "Las mayúsculas no importan. Verán la solicitud cuando estén en línea."
            }
            OnlineText::WantsFriend => "Quiere ser tu amigo",
            OnlineText::Waiting => "Esperando respuesta",
            OnlineText::Accept => "Aceptar",
            OnlineText::Decline => "Rechazar",
            OnlineText::Withdraw => "Retirar solicitud",
            OnlineText::RemoveFriend => "Quitar amigo",
            OnlineText::Block => "Bloquear",
            OnlineText::Unblock => "Desbloquear",
            OnlineText::Report => "Denunciar",
            OnlineText::ReportTitle => "Denunciar a %@",
            OnlineText::Reason => "Motivo",
            OnlineText::ReasonCheating => "Trampas",
            OnlineText::ReasonHarassment => "Acoso",
            OnlineText::ReasonSpam => "Spam",
            OnlineText::ReasonName => "Nombre ofensivo",
            OnlineText::ReasonOther => "Otro motivo",
            OnlineText::SendReport => "Enviar denuncia",
            OnlineText::JoinGame => "Unirse a la partida",
            OnlineText::JoinDetail => "%@ de %@ jugadores",
            OnlineText::InviteToSession => "Invitar a tu partida",
            OnlineText::InvitedBy => "%@ te invita",
            OnlineText::LinkDirect => "Directa",
            OnlineText::LinkRelay => "A través de un relay",
            OnlineText::LinkConnecting => "Conectando…",
            OnlineText::You => "Tú",
            OnlineText::Account => "Cuenta",
            OnlineText::SignOut => "Cerrar sesión",
            OnlineText::PickerTitle => "Invitar a %@",
            OnlineText::PickerSubtitle => "Elige hasta %@ amigos",
            OnlineText::SendInvitations => "Enviar invitaciones",
            OnlineText::Welcome => "Sesión iniciada como %@",
            OnlineText::SignedOut => "Sesión cerrada",
            OnlineText::RequestSent => "Solicitud enviada a %@",
            OnlineText::NowFriends => "Ya eres amigo de %@",
            OnlineText::AddSelf => "No puedes añadirte a ti mismo.",
            OnlineText::AddBlocked => "Has bloqueado esta cuenta. Desbloquéala primero.",
            OnlineText::AddLimit => {
                "Has alcanzado el límite de amigos o solicitudes. Inténtalo más tarde."
            }
            OnlineText::AddEnterName => "Escribe primero su nombre",
            OnlineText::RequestArrived => "%@ quiere ser tu amigo",
            OnlineText::InvitationArrived => "%@ te invita a %@",
            OnlineText::BlockedToast => "%@ está bloqueado",
            OnlineText::Reported => "Denuncia enviada. Gracias.",
            OnlineText::Joining => "Entrando en la partida de %@: %@",
            OnlineText::InvitationsSent => "Invitaciones enviadas",
            OnlineText::Offline => {
                "No se puede acceder a la nube. Revisa tu conexión e inténtalo de nuevo."
            }
            OnlineText::CodeWrong => "El código no coincide. Mira el último correo.",
            OnlineText::CodeExpired => "El código caducó. Envía uno nuevo.",
            OnlineText::HandleTaken => "Ese nombre ya está en uso. Prueba con otro.",
            OnlineText::HandleInvalid => {
                "Usa de 3 a 16 letras, dígitos, - o _, empezando por una letra."
            }
            OnlineText::NotFound => "Ninguna cuenta tiene este nombre.",
            OnlineText::RateLimited => {
                "Demasiados intentos. Espera un momento e inténtalo de nuevo."
            }
            OnlineText::Banned => "Esta cuenta está suspendida.",
            OnlineText::Failed => "No funcionó: %@",
            OnlineText::NotOwned => "%@ no está en tu biblioteca.",
            OnlineText::NotUnlocked => "Desbloquea %@ primero y luego únete.",
            OnlineText::VersionMismatch => {
                "%@ juega la versión %@ con AnyPS5 %@. Tú tienes la %@ con AnyPS5 %@."
            }
            OnlineText::Full => "La partida está llena.",
            OnlineText::ServiceMissing => {
                "Esta instalación no tiene servicio de red. Vuelve a instalar el launcher."
            }
            OnlineText::ServiceEnded => {
                "El servicio de red se detuvo. Abre En línea para iniciarlo de nuevo."
            }
            OnlineText::RecordNetwork => "Registrar llamadas de red",
            OnlineText::RecordNetworkDetail => {
                "Los juegos escriben sus solicitudes de red en la carpeta Logs, para ayudar a crear el soporte en línea. No se sube nada."
            }
            OnlineText::AppearOffline => "Aparecer sin conexión",
            OnlineText::AppearOfflineDetail => "Tus amigos te ven sin conexión",
            OnlineText::PublicProfile => "Perfil público",
            OnlineText::PublicProfileDetail => "Cualquiera puede abrir tu página en la web",
            OnlineText::PublicProfileNote => {
                "Desactivado hasta que lo actives. Aun así, quien no es tu amigo solo ve lo que compartes con Todos."
            }
            OnlineText::AppearingOffline => "Apareces sin conexión",
            OnlineText::RelayOnly => "Ocultar tu dirección IP",
            OnlineText::RelayOnlyNote => {
                "Los demás jugadores te alcanzan solo a través de un relay y nunca ven tu dirección IP. Los juegos pueden responder algo más lento."
            }
            OnlineText::Openh264 => "Video para compartir pantalla",
            OnlineText::Openh264Off => {
                "Actívalo para descargar el códec de video de Cisco. Sin él, compartir pantalla no puede enviar ni mostrar video"
            }
            OnlineText::Openh264Downloading => "Descargando de Cisco",
            OnlineText::Openh264Ready => "Descargado de Cisco",
            OnlineText::Openh264Missing => "Aún sin descargar",
            OnlineText::Openh264Failed => "Falló la descarga: %@",
            OnlineText::Openh264Refused => {
                "El archivo no es la compilación de Cisco. Desactiva y activa de nuevo para descargarlo otra vez."
            }
            OnlineText::Openh264NotLoaded => "No se cargó: %@",
            // Cisco's text stays in English in every language.
            OnlineText::Openh264Notice => "OpenH264 Video Codec provided by Cisco Systems, Inc.",
            OnlineText::TrophiesHidden => "Ocultos",
            OnlineText::TrophiesNone => "Aún ninguno",
            OnlineText::TrophyAudience => "Quién ve tus trofeos",
            OnlineText::AudienceFriends => "Amigos",
            OnlineText::AudienceEveryone => "Todos",
            OnlineText::AudienceOnlyYou => "Solo tú",
            OnlineText::TrophyAudienceNote => {
                "Los jugadores que elijas ven cuántos trofeos tienes de cada grado. Solo el número, el grado y la fecha de cada trofeo salen de este PC."
            }
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            OnlineText::Online => "En ligne",
            OnlineText::Subtitle => "Joue avec tes amis par internet",
            OnlineText::SignedInOn => "Connecté sur %@.",
            OnlineText::Cloud => "Cloud",
            OnlineText::CloudNotSet => "Pas encore configuré",
            OnlineText::CloudOfficial => "AnyPlay",
            OnlineText::CloudUseOfficial => "Utiliser le cloud AnyPlay",
            OnlineText::CloudSubtitle => "L’adresse du cloud auquel tu te connectes",
            OnlineText::CloudAddress => "Adresse",
            OnlineText::CloudUse => "Utiliser ce cloud",
            OnlineText::CloudHint => {
                "Un cloud à toi, ou celui d’amis qui en font tourner un. Demande l’adresse à la personne qui le gère, par exemple https://cloud.example.org."
            }
            OnlineText::CloudInvalid => "Saisis une adresse qui commence par https://",
            OnlineText::Sending => "Envoi…",
            OnlineText::SignInNote => {
                "Tu te connectes ou tu crées ton compte sur la page du cloud, dans ton navigateur. Rien n’est en ligne avant cela."
            }
            OnlineText::NeedsCloud => "Configure d’abord le cloud.",
            OnlineText::SignIn => "Se connecter",
            OnlineText::Handle => "Nom",
            OnlineText::CreateAccount => "Créer un compte",
            OnlineText::ContinueInBrowser => "Continue dans ton navigateur",
            OnlineText::OpenAgain => "Rouvrir la page",
            OnlineText::BrowserNote => {
                "Quand tu as terminé là-bas, le launcher se connecte tout seul."
            }
            OnlineText::SignInTimedOut => "La connexion a expiré. Réessaie.",
            OnlineText::ManageAccount => "Gérer le compte",
            OnlineText::ManageAccountDetail => "E-mail, clés d’accès, appareils et suppression",
            OnlineText::StatusOnline => "En ligne",
            OnlineText::StatusConnecting => "Connexion…",
            OnlineText::StatusOffline => "Hors ligne, nouvelle tentative",
            OnlineText::Invitations => "Invitations",
            OnlineText::Requests => "Demandes d’ami",
            OnlineText::SeeAllRequests => "Voir toutes les demandes",
            OnlineText::RequestsWaiting => "%@ en attente",
            OnlineText::FriendsOnlineCount => "%@ en ligne",
            OnlineText::Friends => "Amis",
            OnlineText::Sent => "Demandes envoyées",
            OnlineText::Session => "Play Together",
            OnlineText::Blocked => "Bloqués",
            OnlineText::Playing => "Joue à %@",
            OnlineText::PlayingOnline => "Joue à %@ · En ligne",
            OnlineText::PlayingOnlineJoinable => "Joue à %@ · En ligne · Tu peux rejoindre",
            OnlineText::FriendOnline => "En ligne",
            OnlineText::FriendOffline => "Hors ligne",
            OnlineText::NoFriends => {
                "Pas encore d’amis. Ajoute quelqu’un avec le nom qu’il a choisi."
            }
            OnlineText::AddFriend => "Ajouter un ami",
            OnlineText::AddFriendSubtitle => "Saisis le nom de son compte",
            OnlineText::SendRequest => "Envoyer la demande",
            OnlineText::AddFriendHint => {
                "Les majuscules n’ont pas d’importance. La personne verra la demande quand elle sera en ligne."
            }
            OnlineText::WantsFriend => "Veut devenir ton ami",
            OnlineText::Waiting => "En attente d’une réponse",
            OnlineText::Accept => "Accepter",
            OnlineText::Decline => "Refuser",
            OnlineText::Withdraw => "Retirer la demande",
            OnlineText::RemoveFriend => "Retirer l’ami",
            OnlineText::Block => "Bloquer",
            OnlineText::Unblock => "Débloquer",
            OnlineText::Report => "Signaler",
            OnlineText::ReportTitle => "Signaler %@",
            OnlineText::Reason => "Motif",
            OnlineText::ReasonCheating => "Triche",
            OnlineText::ReasonHarassment => "Harcèlement",
            OnlineText::ReasonSpam => "Spam",
            OnlineText::ReasonName => "Nom offensant",
            OnlineText::ReasonOther => "Autre chose",
            OnlineText::SendReport => "Envoyer le signalement",
            OnlineText::JoinGame => "Rejoindre la partie",
            OnlineText::JoinDetail => "%@ sur %@ joueurs",
            OnlineText::InviteToSession => "Inviter dans ta partie",
            OnlineText::InvitedBy => "%@ t’invite",
            OnlineText::LinkDirect => "Direct",
            OnlineText::LinkRelay => "Via un relais",
            OnlineText::LinkConnecting => "Connexion…",
            OnlineText::You => "Toi",
            OnlineText::Account => "Compte",
            OnlineText::SignOut => "Se déconnecter",
            OnlineText::PickerTitle => "Inviter à %@",
            OnlineText::PickerSubtitle => "Choisis jusqu’à %@ amis",
            OnlineText::SendInvitations => "Envoyer les invitations",
            OnlineText::Welcome => "Connecté en tant que %@",
            OnlineText::SignedOut => "Déconnecté",
            OnlineText::RequestSent => "Demande envoyée à %@",
            OnlineText::NowFriends => "Toi et %@ êtes maintenant amis",
            OnlineText::AddSelf => "Tu ne peux pas t’ajouter toi-même.",
            OnlineText::AddBlocked => "Tu as bloqué ce compte. Débloque-le d’abord.",
            OnlineText::AddLimit => {
                "La limite d’amis ou de demandes est atteinte. Réessaie plus tard."
            }
            OnlineText::AddEnterName => "Saisis d’abord son nom",
            OnlineText::RequestArrived => "%@ veut devenir ton ami",
            OnlineText::InvitationArrived => "%@ t’invite à %@",
            OnlineText::BlockedToast => "%@ est bloqué",
            OnlineText::Reported => "Signalement envoyé. Merci.",
            OnlineText::Joining => "Connexion à la partie de %@\u{a0}: %@",
            OnlineText::InvitationsSent => "Invitations envoyées",
            OnlineText::Offline => "Le cloud est inaccessible. Vérifie ta connexion et réessaie.",
            OnlineText::CodeWrong => "Le code ne correspond pas. Regarde le dernier e-mail.",
            OnlineText::CodeExpired => "Le code a expiré. Envoies-en un nouveau.",
            OnlineText::HandleTaken => "Ce nom est déjà pris. Essaie-en un autre.",
            OnlineText::HandleInvalid => {
                "Utilise de 3 à 16 lettres, chiffres, - ou _, en commençant par une lettre."
            }
            OnlineText::NotFound => "Aucun compte ne porte ce nom.",
            OnlineText::RateLimited => "Trop d’essais. Attends un instant et réessaie.",
            OnlineText::Banned => "Ce compte est banni.",
            OnlineText::Failed => "Cela n’a pas fonctionné\u{a0}: %@",
            OnlineText::NotOwned => "%@ n’est pas dans ta bibliothèque.",
            OnlineText::NotUnlocked => "Déverrouille d’abord %@, puis rejoins.",
            OnlineText::VersionMismatch => {
                "%@ joue à la version %@ avec AnyPS5 %@. Tu as la %@ avec AnyPS5 %@."
            }
            OnlineText::Full => "La partie est complète.",
            OnlineText::ServiceMissing => {
                "Cette installation n’a pas de service réseau. Réinstalle le launcher."
            }
            OnlineText::ServiceEnded => {
                "Le service réseau s’est arrêté. Ouvre En ligne pour le relancer."
            }
            OnlineText::RecordNetwork => "Enregistrer les appels réseau",
            OnlineText::RecordNetworkDetail => {
                "Les jeux écrivent leurs requêtes réseau dans le dossier Logs pour aider à créer la prise en charge en ligne. Rien n’est envoyé."
            }
            OnlineText::AppearOffline => "Apparaître hors ligne",
            OnlineText::AppearOfflineDetail => "Tes amis te voient hors ligne",
            OnlineText::PublicProfile => "Profil public",
            OnlineText::PublicProfileDetail => "N’importe qui peut ouvrir ta page sur le web",
            OnlineText::PublicProfileNote => {
                "Désactivé tant que tu ne l’actives pas. Même alors, les personnes qui ne sont pas tes amis ne voient que ce que tu partages avec Tout le monde."
            }
            OnlineText::AppearingOffline => "Tu apparais hors ligne",
            OnlineText::RelayOnly => "Masquer ton adresse IP",
            OnlineText::RelayOnlyNote => {
                "Les autres joueurs te joignent uniquement via un relais et ne voient jamais ton adresse IP. Les jeux peuvent répondre un peu plus lentement."
            }
            OnlineText::Openh264 => "Vidéo pour le partage d’écran",
            OnlineText::Openh264Off => {
                "Active pour télécharger le codec vidéo de Cisco. Sans lui, le partage d’écran ne peut ni envoyer ni afficher de vidéo"
            }
            OnlineText::Openh264Downloading => "Téléchargement depuis Cisco",
            OnlineText::Openh264Ready => "Téléchargé depuis Cisco",
            OnlineText::Openh264Missing => "Pas encore téléchargé",
            OnlineText::Openh264Failed => "Échec du téléchargement\u{a0}: %@",
            OnlineText::Openh264Refused => {
                "Le fichier n’est pas le build de Cisco. Désactive puis réactive pour le télécharger de nouveau."
            }
            OnlineText::Openh264NotLoaded => "Chargement impossible\u{a0}: %@",
            // Cisco's text stays in English in every language.
            OnlineText::Openh264Notice => "OpenH264 Video Codec provided by Cisco Systems, Inc.",
            OnlineText::TrophiesHidden => "Masqués",
            OnlineText::TrophiesNone => "Aucun pour l’instant",
            OnlineText::TrophyAudience => "Qui voit tes trophées",
            OnlineText::AudienceFriends => "Amis",
            OnlineText::AudienceEveryone => "Tout le monde",
            OnlineText::AudienceOnlyYou => "Toi seul",
            OnlineText::TrophyAudienceNote => {
                "Les joueurs que tu choisis voient combien de trophées tu as de chaque rang. Seuls le numéro, le rang et la date de chaque trophée quittent ce PC."
            }
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            OnlineText::Online => "Online",
            OnlineText::Subtitle => "Jogue com seus amigos pela internet",
            OnlineText::SignedInOn => "Conectado em %@.",
            OnlineText::Cloud => "Nuvem",
            OnlineText::CloudNotSet => "Ainda não configurada",
            OnlineText::CloudOfficial => "AnyPlay",
            OnlineText::CloudUseOfficial => "Usar a nuvem da AnyPlay",
            OnlineText::CloudSubtitle => "O endereço da nuvem em que você entra",
            OnlineText::CloudAddress => "Endereço",
            OnlineText::CloudUse => "Usar esta nuvem",
            OnlineText::CloudHint => {
                "Uma nuvem sua ou de amigos que mantêm uma. Peça o endereço a quem a administra, como https://cloud.example.org."
            }
            OnlineText::CloudInvalid => "Digite um endereço que comece com https://",
            OnlineText::Sending => "Enviando…",
            OnlineText::SignInNote => {
                "Você entra ou cria sua conta na página da nuvem, no navegador. Antes disso, nada fica online."
            }
            OnlineText::NeedsCloud => "Configure a nuvem primeiro.",
            OnlineText::SignIn => "Entrar",
            OnlineText::Handle => "Nome",
            OnlineText::CreateAccount => "Criar conta",
            OnlineText::ContinueInBrowser => "Continue no navegador",
            OnlineText::OpenAgain => "Abrir a página de novo",
            OnlineText::BrowserNote => "Quando terminar lá, o launcher entra sozinho.",
            OnlineText::SignInTimedOut => "O tempo da entrada esgotou. Tente de novo.",
            OnlineText::ManageAccount => "Gerenciar conta",
            OnlineText::ManageAccountDetail => "E-mail, chaves de acesso, dispositivos e exclusão",
            OnlineText::StatusOnline => "Online",
            OnlineText::StatusConnecting => "Conectando…",
            OnlineText::StatusOffline => "Offline, tentando de novo",
            OnlineText::Invitations => "Convites",
            OnlineText::Requests => "Pedidos de amizade",
            OnlineText::SeeAllRequests => "Ver todos os pedidos",
            OnlineText::RequestsWaiting => "%@ aguardando",
            OnlineText::FriendsOnlineCount => "%@ online",
            OnlineText::Friends => "Amigos",
            OnlineText::Sent => "Pedidos enviados",
            OnlineText::Session => "Play Together",
            OnlineText::Blocked => "Bloqueados",
            OnlineText::Playing => "Jogando %@",
            OnlineText::PlayingOnline => "Jogando %@ · Online",
            OnlineText::PlayingOnlineJoinable => "Jogando %@ · Online · Você pode entrar",
            OnlineText::FriendOnline => "Online",
            OnlineText::FriendOffline => "Offline",
            OnlineText::NoFriends => {
                "Você ainda não tem amigos. Adicione alguém pelo nome que a pessoa escolheu."
            }
            OnlineText::AddFriend => "Adicionar amigo",
            OnlineText::AddFriendSubtitle => "Digite o nome da conta da pessoa",
            OnlineText::SendRequest => "Enviar pedido",
            OnlineText::AddFriendHint => {
                "Maiúsculas e minúsculas não importam. A pessoa vê o pedido quando ficar online."
            }
            OnlineText::WantsFriend => "Quer ser seu amigo",
            OnlineText::Waiting => "Aguardando resposta",
            OnlineText::Accept => "Aceitar",
            OnlineText::Decline => "Recusar",
            OnlineText::Withdraw => "Cancelar pedido",
            OnlineText::RemoveFriend => "Remover amigo",
            OnlineText::Block => "Bloquear",
            OnlineText::Unblock => "Desbloquear",
            OnlineText::Report => "Denunciar",
            OnlineText::ReportTitle => "Denunciar %@",
            OnlineText::Reason => "Motivo",
            OnlineText::ReasonCheating => "Trapaça",
            OnlineText::ReasonHarassment => "Assédio",
            OnlineText::ReasonSpam => "Spam",
            OnlineText::ReasonName => "Nome ofensivo",
            OnlineText::ReasonOther => "Outro motivo",
            OnlineText::SendReport => "Enviar denúncia",
            OnlineText::JoinGame => "Entrar no jogo",
            OnlineText::JoinDetail => "%@ de %@ jogadores",
            OnlineText::InviteToSession => "Convidar para seu jogo",
            OnlineText::InvitedBy => "%@ convidou você",
            OnlineText::LinkDirect => "Direta",
            OnlineText::LinkRelay => "Por um relay",
            OnlineText::LinkConnecting => "Conectando…",
            OnlineText::You => "Você",
            OnlineText::Account => "Conta",
            OnlineText::SignOut => "Sair",
            OnlineText::PickerTitle => "Convidar para %@",
            OnlineText::PickerSubtitle => "Escolha até %@ amigos",
            OnlineText::SendInvitations => "Enviar convites",
            OnlineText::Welcome => "Conectado como %@",
            OnlineText::SignedOut => "Sessão encerrada",
            OnlineText::RequestSent => "Pedido enviado para %@",
            OnlineText::NowFriends => "Você e %@ agora são amigos",
            OnlineText::AddSelf => "Você não pode adicionar a si mesmo.",
            OnlineText::AddBlocked => "Você bloqueou esta conta. Desbloqueie primeiro.",
            OnlineText::AddLimit => {
                "O limite de amigos ou pedidos foi atingido. Tente de novo mais tarde."
            }
            OnlineText::AddEnterName => "Digite o nome primeiro",
            OnlineText::RequestArrived => "%@ quer ser seu amigo",
            OnlineText::InvitationArrived => "%@ convidou você para %@",
            OnlineText::BlockedToast => "%@ está bloqueado",
            OnlineText::Reported => "Denúncia enviada. Obrigado.",
            OnlineText::Joining => "Entrando no jogo de %@: %@",
            OnlineText::InvitationsSent => "Convites enviados",
            OnlineText::Offline => {
                "Não foi possível acessar a nuvem. Verifique sua conexão e tente de novo."
            }
            OnlineText::CodeWrong => "O código não confere. Veja o e-mail mais recente.",
            OnlineText::CodeExpired => "O código expirou. Envie um novo.",
            OnlineText::HandleTaken => "Esse nome já está em uso. Tente outro.",
            OnlineText::HandleInvalid => {
                "Use de 3 a 16 letras, dígitos, - ou _, começando por uma letra."
            }
            OnlineText::NotFound => "Nenhuma conta tem este nome.",
            OnlineText::RateLimited => "Muitas tentativas. Aguarde um momento e tente de novo.",
            OnlineText::Banned => "Esta conta está banida.",
            OnlineText::Failed => "Não funcionou: %@",
            OnlineText::NotOwned => "%@ não está na sua biblioteca.",
            OnlineText::NotUnlocked => "Desbloqueie %@ primeiro e depois entre.",
            OnlineText::VersionMismatch => {
                "%@ joga a versão %@ com o AnyPS5 %@. Você tem a %@ com o AnyPS5 %@."
            }
            OnlineText::Full => "O jogo está cheio.",
            OnlineText::ServiceMissing => {
                "Esta instalação não tem serviço de rede. Instale o launcher de novo."
            }
            OnlineText::ServiceEnded => {
                "O serviço de rede parou. Abra Online para iniciá-lo de novo."
            }
            OnlineText::RecordNetwork => "Registrar chamadas de rede",
            OnlineText::RecordNetworkDetail => {
                "Os jogos gravam suas requisições de rede na pasta Logs, para ajudar a criar o suporte online. Nada é enviado."
            }
            OnlineText::AppearOffline => "Aparecer offline",
            OnlineText::AppearOfflineDetail => "Seus amigos veem você offline",
            OnlineText::PublicProfile => "Perfil público",
            OnlineText::PublicProfileDetail => "Qualquer pessoa pode abrir sua página na web",
            OnlineText::PublicProfileNote => {
                "Desativado até você ativar. Mesmo assim, quem não é seu amigo vê só o que você compartilha com Todos."
            }
            OnlineText::AppearingOffline => "Você aparece offline",
            OnlineText::RelayOnly => "Ocultar seu endereço IP",
            OnlineText::RelayOnlyNote => {
                "Outros jogadores alcançam você só por um relay e nunca veem seu endereço IP. Os jogos podem responder um pouco mais devagar."
            }
            OnlineText::Openh264 => "Vídeo para compartilhar tela",
            OnlineText::Openh264Off => {
                "Ative para baixar o codec de vídeo da Cisco. Sem ele, compartilhar tela não consegue enviar nem mostrar vídeo"
            }
            OnlineText::Openh264Downloading => "Baixando da Cisco",
            OnlineText::Openh264Ready => "Baixado da Cisco",
            OnlineText::Openh264Missing => "Ainda não baixado",
            OnlineText::Openh264Failed => "O download falhou: %@",
            OnlineText::Openh264Refused => {
                "O arquivo não é o build da Cisco. Desative e ative de novo para baixá-lo outra vez."
            }
            OnlineText::Openh264NotLoaded => "Não foi possível carregar: %@",
            // Cisco's text stays in English in every language.
            OnlineText::Openh264Notice => "OpenH264 Video Codec provided by Cisco Systems, Inc.",
            OnlineText::TrophiesHidden => "Ocultos",
            OnlineText::TrophiesNone => "Nenhum ainda",
            OnlineText::TrophyAudience => "Quem vê seus troféus",
            OnlineText::AudienceFriends => "Amigos",
            OnlineText::AudienceEveryone => "Todos",
            OnlineText::AudienceOnlyYou => "Só você",
            OnlineText::TrophyAudienceNote => {
                "Os jogadores que você escolher veem quantos troféus você tem de cada grau. Só o número, o grau e a data de cada troféu saem deste PC."
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

    const ALL: &[OnlineText] = &[
        OnlineText::Playing,
        OnlineText::PlayingOnline,
        OnlineText::PlayingOnlineJoinable,
        OnlineText::ReportTitle,
        OnlineText::JoinDetail,
        OnlineText::InvitedBy,
        OnlineText::PickerTitle,
        OnlineText::PickerSubtitle,
        OnlineText::Welcome,
        OnlineText::RequestSent,
        OnlineText::NowFriends,
        OnlineText::RequestsWaiting,
        OnlineText::FriendsOnlineCount,
        OnlineText::RequestArrived,
        OnlineText::InvitationArrived,
        OnlineText::BlockedToast,
        OnlineText::Joining,
        OnlineText::Failed,
        OnlineText::NotOwned,
        OnlineText::NotUnlocked,
        OnlineText::VersionMismatch,
        OnlineText::Openh264Failed,
        OnlineText::Openh264NotLoaded,
    ];

    #[test]
    fn placeholders_match_between_languages() {
        for text in ALL {
            let count = |s: &str| s.matches("%@").count();
            assert_eq!(count(text.english()), count(text.turkish()), "{text:?}");
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
    fn the_version_message_names_both_sides() {
        let text = OnlineText::VersionMismatch.format(
            Language::English,
            &[
                Arg::Str("Lumen"),
                Arg::Str("01.100"),
                Arg::Str("0.1.4"),
                Arg::Str("01.000"),
                Arg::Str("0.1.3"),
            ],
        );
        assert_eq!(
            text,
            "Lumen plays version 01.100 with AnyPS5 0.1.4. You have 01.000 with AnyPS5 0.1.3."
        );
    }
}
