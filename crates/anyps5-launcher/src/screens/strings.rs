//! Texts the Windows port adds to the 212 keys of the original (`ui::i18n`): the toolkit rows of the
//! settings, the CPU conversion choice and the host-CPU diagnosis. English and Turkish like the rest.

use crate::ui::i18n::{Arg, Language, substitute};

/// A Windows-only text. The generated table of the original stays untouched.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Extra {
    /// Settings category.
    Toolkit,
    UpdateCheck,
    UpdateCheckDetail,
    CheckNow,
    /// Value while the check runs.
    Checking,
    UpToDate,
    /// `%@` = the new version.
    UpdateAvailable,
    CheckFailed,
    LastChecked,
    ToolkitMissing,
    ToolkitInstall,
    ToolkitInstallUpdate,
    ToolkitChooseFolder,
    CpuConversion,
    CpuConversionDetail,
    CpuAuto,
    CpuNone,
    CpuIntel,
    /// `%@` = the missing CPU features.
    DiagnosisHostCpu,
    /// Linux: the user cannot open `/dev/udmabuf`.
    DiagnosisNoUdmabuf,
    SaveReport,
    OpenOnlineLog,
    /// The toast after a game ended in trouble.
    SendReport,
    /// Toast while the toolkit downloads.
    ToolkitDownloading,
    /// Fonts row: the set a game gets when no folder is picked.
    FontsBuiltIn,
    /// Fonts row, after the built-in set: Japanese, Chinese and Korean are installed.
    FontsCjkReady,
    FontsCjkMissing,
    FontsCjkDownloading,
    /// Command row that starts the download.
    FontsDownloadCjk,
    /// Toast: `%@` = the installed version.
    ToolkitReady,
    ToolkitFailed,
    /// Toast: the picked folder has no relinker or libraries.
    ToolkitFolderInvalid,
    /// Settings row: the home screen music toggle.
    HomeMusic,
    HomeMusicDetail,
    /// Settings row: the selected game's own music on the home screen.
    GameMusic,
    GameMusicDetail,
    /// Settings row: how strongly the controller vibrates to answer an action.
    Vibration,
    VibrationDetail,
    VibrationLight,
    VibrationMedium,
    VibrationStrong,
    /// Settings note under the vibration row: what to try when the controller is not found.
    ControllerHelp,
    /// Top bar, read out by screen readers: a controller is connected (and how full it is).
    PadStatus,
    PadStatusBattery,
    PadStatusCharging,
    /// Settings row: the running game on Discord.
    DiscordPresence,
    DiscordPresenceDetail,
    /// Settings row: hidden trophies in the trophy list, and the action on one hidden trophy.
    ShowHiddenTrophies,
    ShowHiddenTrophiesDetail,
    TrophyShow,
    /// The trophy list of a game that has no trophies.
    TrophiesNone,
    /// The log box before the game wrote anything.
    EmptyLog,
    /// The button of the removal confirmation (its title already says what is removed).
    /// Controls row: the keyboard layout and its choices.
    Layout,
    LayoutLauncher,
    LayoutMouse,
    /// The PC keys the game's own files name.
    LayoutGame,
    /// The layout after the user moved keys.
    LayoutCustom,
    /// `%@` = how many buttons the game's files give a PC key.
    LayoutGameDetail,
    /// About row value and toast: `%@` = the new launcher version.
    LauncherReady,
    /// Toast when a launcher update is downloaded and checked.
    LauncherToast,
    RestartNow,
    /// About row value: `%@` = the new version, but the folder cannot be written to.
    LauncherReadOnly,
    /// Toast when "Restart Now" or the swap on exit fails.
    LauncherUpdateFailed,
    /// Settings row: the launcher answers anyps5:// links and folders.
    Links,
    LinksDetail,
    LinksDetailOther,
    LinksFailed,
    /// Toast: a link or folder the launcher cannot use.
    LinkInvalid,
    /// Toast: `%@` = the title ID.
    LinkNoGame,
    /// Toast: the folder of a link or of the folder menu holds no game.
    LinkNotADump,
    /// Toast: `%@` = the game name.
    LinkOutside,
    /// Toast: `%@` = the game name.
    LinkUnlock,
    /// Toast: `%@` = the game name.
    LinkUnlocked,
    /// Settings row: which toolkit to follow.
    ChannelTitle,
    ChannelStable,
    ChannelPinned,
    ChannelLocal,
    ChannelStableDetail,
    ChannelPinnedDetail,
    ChannelLocalDetail,
    ChannelEngine,
    ChannelEngineDetail,
    EngineBuilt,
    EngineAvailable,
    EngineNoBuild,
    EngineUnverified,
    /// Settings row: the release a pinned channel stays on.
    PinnedVersion,
    /// Settings row and panel title.
    WhatsNew,
    /// Detail of Check Now: `%@` = the time.
    LastCheckedOn,
    /// Detail of the toolkit row.
    FitNewer,
    /// Detail of the toolkit row.
    FitOlder,
    /// Toast: `%@` = the release.
    ToolkitAvailable,
    /// Value of a turned-off feature: `%@` = the first release that has it.
    NeedsToolkit,
    /// Value of a turned-off feature: `%@` = the last release without it.
    NeedsNewerThan,
    /// Header of the notes: release, date.
    NotesHeader,
    /// `%@` = how many.
    ChangesCount,
    ChangeOne,
    NotesHighlights,
    NotesOther,
    NotesEverything,
    NotesLoading,
    NotesUnavailable,
    /// Button: the release page on GitHub.
    NotesOpenRelease,
    /// Game sheet: `%@` = the version it was unlocked with, then the installed one.
    UnlockedWith,
    ConversionChanged,
    UnlockOptional,
    UnlockEarlierBuild,
    /// Game sheet row and panel subtitle.
    RecTitle,
    /// Detail of the row: the game does not start without it.
    RecNeeded,
    /// Detail of the row when there is only a warning.
    RecWorthALook,
    /// The one button of the panel.
    RecApply,
    /// Value before a setting is changed.
    RecDefault,
    RecOn,
    RecOff,
    /// Setting name.
    RecPlayGo,
    RecPlayGoInstalled,
    RecPlayGoAbsent,
    /// Setting name.
    RecImageBase,
    /// Setting name.
    RecModuleFolder,
    /// Setting name.
    RecExecutable,
    /// Setting name.
    RecTextureCache,
    /// Setting name.
    RecShaderCache,
    /// Hint under the changes.
    RecUnlockHint,
    /// Toast.
    RecApplied,
    /// Toast.
    RecAppliedUnlock,
    /// Facts line of the home screen: the label over the time played.
    Played,
    /// Time played, under a minute.
    PlayedUnderMinute,
    /// `%@` = minutes.
    PlayedMinutes,
    /// `%@` = hours.
    PlayedHours,
    /// `%@` = hours, then minutes.
    PlayedHoursMinutes,
    /// Game sheet row and panel subtitle.
    SavesTitle,
    /// Detail of the row: no backup yet.
    SavesNone,
    /// Detail of the row.
    SavesOne,
    /// Detail of the row: `%@` = how many.
    SavesCount,
    /// Command.
    SavesBackUp,
    /// Detail of the command and its toast when there is nothing to back up.
    SavesNothing,
    /// Note of the panel without backups.
    SavesEmpty,
    /// Section caption over the list.
    SavesCaption,
    /// Captions over the three groups of the game settings.
    GroupScreen,
    GroupPlay,
    GroupMaintenance,
    /// Note under the list: `%@` = how many are kept.
    SavesKept,
    /// Why a backup was made.
    SavesReasonManual,
    /// Why a backup was made.
    SavesReasonUnlock,
    /// Why a backup was made.
    SavesReasonRestore,
    /// Why a backup was made.
    SavesReasonUninstall,
    /// Why a backup was made.
    SavesReasonPlayed,
    /// Toast.
    SavesBackedUp,
    /// Command and title of the question.
    SavesRestore,
    /// The question: `%@` = date of the backup.
    SavesRestoreAsk,
    /// Toast.
    SavesRestored,
    /// Note while the game runs, and the toast of a refusal.
    SavesRunning,
    /// Toast.
    SavesFailed,
    /// Toast.
    SavesRestoreFailed,
    /// Toast.
    SavesCorrupt,
    /// The removal question's title.
    UninstallTitle,
    /// The restore question's title.
    SavesRestoreTitle,
    /// Added to the removal question.
    UninstallBacksUp,
    /// Toast: the removal did not start.
    UninstallNeedsBackup,
    /// The state line and the greyed-out button while a game's copy is being removed.
    Uninstalling,
    /// The state line, the greyed-out button and the bar's label while an unlock that was cancelled
    /// is still being stopped.
    Cancelling,
    /// The button of an unlock under way: it stops where it is, Resume goes on.
    Pause,
    /// The state line, the greyed-out button and the bar's label while a paused unlock is still
    /// being stopped.
    Pausing,
    /// The button of a game whose unlock was paused.
    Resume,
    /// The note beside the bar of such a game.
    PausedNote,
    /// The note beside the bar of an unlock the player paused: it goes on from where it stood.
    HeldNote,
    /// Toast: the restore stopped part way.
    SavesRestoreIncomplete,
    /// Toast: the restore stopped part way, nothing had to be kept.
    SavesRestoreIncompleteNone,
    /// Toast: a backup or restore is still under way.
    SavesBusy,
}

impl Extra {
    pub const fn english(self) -> &'static str {
        match self {
            Extra::Toolkit => "Toolkit",
            Extra::UpdateCheck => "Check for Updates",
            Extra::UpdateCheckDetail => {
                "Looks for a newer toolkit when the launcher starts. Only the version number is requested."
            }
            Extra::CheckNow => "Check Now",
            Extra::Checking => "Checking…",
            Extra::UpToDate => "Up to date",
            Extra::UpdateAvailable => "Update available: %@",
            Extra::CheckFailed => "Could not check",
            Extra::LastChecked => "Last Checked",
            Extra::ToolkitMissing => "Not installed",
            Extra::ToolkitInstall => "Download the Toolkit",
            Extra::ToolkitInstallUpdate => "Install the Update",
            Extra::ToolkitChooseFolder => "Use a Toolkit Folder",
            Extra::CpuConversion => "CPU Conversion",
            Extra::CpuConversionDetail => {
                "How games are prepared for your processor. Automatic suits most PCs."
            }
            Extra::CpuAuto => "Automatic",
            Extra::CpuNone => "None",
            Extra::CpuIntel => "Intel",
            Extra::DiagnosisHostCpu => "This CPU lacks %@, which the game needs.",
            Extra::DiagnosisNoUdmabuf => {
                "The game cannot open /dev/udmabuf. Add your user to the kvm group and log in again."
            }
            Extra::SaveReport => "Save Report",
            Extra::OpenOnlineLog => "Open Online Log",
            Extra::SendReport => "Send Report",
            Extra::ToolkitDownloading => "Downloading the toolkit…",
            Extra::FontsBuiltIn => "Built-in Noto (Latin, Thai, Arabic)",
            Extra::FontsCjkReady => "Japanese, Chinese and Korean included",
            Extra::FontsCjkMissing => "Japanese, Chinese and Korean not downloaded",
            Extra::FontsCjkDownloading => "Downloading Japanese, Chinese and Korean",
            Extra::FontsDownloadCjk => "Download Japanese, Chinese and Korean Fonts",
            Extra::ToolkitReady => "Toolkit %@ is ready",
            Extra::ToolkitFailed => {
                "Could not install the toolkit. Check your connection and try again."
            }
            Extra::HomeMusic => "Home Screen Music",
            Extra::HomeMusicDetail => {
                "Plays quiet ambient music on the home screen. It fades out while a game runs."
            }
            Extra::GameMusic => "Game Music",
            Extra::GameMusicDetail => {
                "Plays the selected game's own music in place of the home screen music."
            }
            Extra::Vibration => "Controller Vibration",
            Extra::VibrationDetail => {
                "A short, soft pulse answers a press on the controller. Never while a game is in front."
            }
            Extra::ControllerHelp => {
                "Steam Input and DS4Windows can block the controller. Turn off PlayStation support in Steam, or quit both apps, then reconnect it."
            }
            Extra::PadStatus => "Controller connected",
            Extra::PadStatusBattery => "Controller connected, battery %@ percent",
            Extra::PadStatusCharging => "Controller connected, battery %@ percent, charging",
            Extra::VibrationLight => "Light",
            Extra::VibrationMedium => "Medium",
            Extra::VibrationStrong => "Strong",
            Extra::DiscordPresence => "Show Game on Discord",
            Extra::DiscordPresenceDetail => {
                "With Discord open, your profile shows the game and for how long."
            }
            Extra::ShowHiddenTrophies => "Always Show Hidden Trophies",
            Extra::ShowHiddenTrophiesDetail => {
                "Hidden trophies show their name and description before you earn them. Only you see this."
            }
            Extra::TrophyShow => "Show",
            Extra::TrophiesNone => "This game has no trophies.",
            Extra::EmptyLog => "No log yet. Play the game once and it appears here.",
            Extra::ToolkitFolderInvalid => {
                "This folder is not a toolkit. Pick the folder that holds the relinker and libs."
            }
            Extra::Layout => "Layout",
            Extra::LayoutLauncher => "Launcher",
            Extra::LayoutMouse => "Mouse and Keyboard",
            Extra::LayoutGame => "From the Game",
            Extra::LayoutCustom => "Custom",
            Extra::LayoutGameDetail => "Found the game's own PC keys for %@ buttons.",
            Extra::LauncherReady => "%@ ready",
            Extra::LauncherToast => {
                "Launcher %@ is ready. It installs when you close the launcher."
            }
            Extra::RestartNow => "Restart Now",
            Extra::LauncherReadOnly => "%@ available, but this folder is read-only",
            Extra::LauncherUpdateFailed => {
                "Could not install the launcher update. Download it from GitHub instead."
            }
            Extra::Links => "Open AnyPlay Links with This Launcher",
            Extra::LinksDetail => {
                "Links on websites and in messages open here, and folders get an \"Open with AnyPlay Launcher\" entry."
            }
            Extra::LinksDetailOther => {
                "Another copy of the launcher opens them now. Turn this on to use this one."
            }
            Extra::LinksFailed => "Could not change the link settings. Try again.",
            Extra::LinkInvalid => "This is not a valid AnyPlay link.",
            Extra::LinkNoGame => "%@ is not in your library. Check the games folder in Settings.",
            Extra::LinkNotADump => {
                "This folder is not a game dump. Open the folder that holds eboot.bin."
            }
            Extra::LinkOutside => "%@ is outside your games folder. Move it there to add it.",
            Extra::LinkUnlock => "Select Unlock to prepare %@.",
            Extra::LinkUnlocked => "%@ is already unlocked.",
            Extra::ChannelTitle => "Channel",
            Extra::ChannelStable => "Stable",
            Extra::ChannelPinned => "Pinned",
            Extra::ChannelLocal => "Local Build",
            Extra::ChannelStableDetail => "Follows the newest release.",
            Extra::ChannelPinnedDetail => "Stays on one release. No update is offered.",
            Extra::ChannelLocalDetail => "Uses a folder you built or picked. No update is offered.",
            Extra::ChannelEngine => "AnyPlay Engine",
            Extra::ChannelEngineDetail => {
                "The nightly build with online play. Changes often and may break games."
            }
            Extra::EngineBuilt => "Built on %@",
            Extra::EngineAvailable => "A new engine build is available",
            Extra::EngineNoBuild => {
                "There is no engine build for this system yet. Nothing was installed. Choose another channel."
            }
            Extra::EngineUnverified => {
                "The engine download could not be checked. Nothing was installed. Try again later."
            }
            Extra::PinnedVersion => "Version",
            Extra::WhatsNew => "What's New",
            Extra::LastCheckedOn => "Last checked %@",
            Extra::FitNewer => "Newer than this launcher was tested with. It should still work.",
            Extra::FitOlder => "An older toolkit: some features are turned off.",
            Extra::ToolkitAvailable => "Toolkit %@ is available",
            Extra::NeedsToolkit => "Needs toolkit %@",
            Extra::NeedsNewerThan => "Needs a build newer than %@",
            Extra::NotesHeader => "%@, released %@",
            Extra::ChangesCount => "%@ changes",
            Extra::ChangeOne => "1 change",
            Extra::NotesHighlights => "Highlights",
            Extra::NotesOther => "Other Areas",
            Extra::NotesEverything => "Everything",
            Extra::NotesLoading => "Loading the notes…",
            Extra::NotesUnavailable => "The notes could not be loaded.",
            Extra::NotesOpenRelease => "Open on GitHub",
            Extra::UnlockedWith => "Unlocked with %@. %@ is installed.",
            Extra::ConversionChanged => "This update changes how games are converted.",
            Extra::UnlockOptional => "Unlocking again is optional.",
            Extra::UnlockEarlierBuild => "Unlocked with an earlier build of this toolkit.",
            Extra::RecTitle => "Recommended for This Game",
            Extra::RecNeeded => "Needed to start",
            Extra::RecWorthALook => "Worth a look",
            Extra::RecApply => "Apply",
            Extra::RecDefault => "Default",
            Extra::RecOn => "On",
            Extra::RecOff => "Off",
            Extra::RecPlayGo => "Missing PlayGo Data",
            Extra::RecPlayGoInstalled => "Report as Installed",
            Extra::RecPlayGoAbsent => "Report as Absent",
            Extra::RecImageBase => "Image Base",
            Extra::RecModuleFolder => "Module Folder",
            Extra::RecExecutable => "Executable",
            Extra::RecTextureCache => "Texture Cache",
            Extra::RecShaderCache => "Shader Cache",
            Extra::RecUnlockHint => "After you apply this, unlock the game again.",
            Extra::RecApplied => "Applied",
            Extra::RecAppliedUnlock => "Saved. Unlock the game again to use it.",
            Extra::Played => "Played",
            Extra::PlayedUnderMinute => "Under 1 min",
            Extra::PlayedMinutes => "%@ min",
            Extra::PlayedHours => "%@ h",
            Extra::PlayedHoursMinutes => "%@ h %@ min",
            Extra::SavesTitle => "Save Data",
            Extra::SavesNone => "No backups yet",
            Extra::SavesOne => "1 backup",
            Extra::SavesCount => "%@ backups",
            Extra::SavesBackUp => "Back Up Now",
            Extra::SavesNothing => "No save data to back up yet",
            Extra::SavesEmpty => {
                "No backups yet. Back Up Now keeps a copy of your saves and trophies."
            }
            Extra::SavesCaption => "Backups",
            Extra::GroupScreen => "Screen",
            Extra::GroupPlay => "Play",
            Extra::GroupMaintenance => "Maintenance",
            Extra::SavesKept => "Up to %@ backups are kept. Backups made after playing go first.",
            Extra::SavesReasonManual => "Manual",
            Extra::SavesReasonUnlock => "Before unlocking again",
            Extra::SavesReasonRestore => "Before a restore",
            Extra::SavesReasonUninstall => "Before uninstall",
            Extra::SavesReasonPlayed => "After playing",
            Extra::SavesBackedUp => "Saves backed up",
            Extra::SavesRestore => "Restore",
            Extra::SavesRestoreAsk => {
                "This replaces the saves and trophies the game has now with the backup from %@. The current ones are backed up first, so you can undo it."
            }
            Extra::SavesRestored => "Saves restored",
            Extra::SavesRunning => "Close the game to restore a backup.",
            Extra::SavesFailed => {
                "Could not back up the saves. Check the free disk space and try again."
            }
            Extra::SavesRestoreFailed => {
                "Could not restore the backup. Your saves were not changed."
            }
            Extra::SavesCorrupt => {
                "This backup is damaged. Your saves were not changed. Pick another backup."
            }
            Extra::UninstallTitle => "Remove this installation?",
            Extra::SavesRestoreTitle => "Restore this backup?",
            Extra::UninstallBacksUp => "Your saves and trophies are backed up first.",
            Extra::UninstallNeedsBackup => {
                "Could not back up the saves, so the game was not removed. Check the free disk space and try again."
            }
            Extra::Uninstalling => "Removing",
            Extra::Cancelling => "Cancelling…",
            Extra::Pause => "Pause",
            Extra::Pausing => "Pausing…",
            Extra::Resume => "Resume",
            Extra::PausedNote => "Incomplete: progress was not kept. Resume starts again from the beginning.",
            Extra::HeldNote => "Paused. Resume goes on from here.",
            Extra::SavesRestoreIncomplete => {
                "The restore did not finish. The saves from before are in the newest backup, Before a restore."
            }
            Extra::SavesRestoreIncompleteNone => "The restore did not finish. Try it again.",
            Extra::SavesBusy => "Still working on the saves. Try again in a moment.",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            Extra::Toolkit => "Araç Seti",
            Extra::UpdateCheck => "Güncellemeleri Denetle",
            Extra::UpdateCheckDetail => {
                "Başlatıcı açılırken daha yeni bir araç seti arar. Yalnızca sürüm numarası istenir."
            }
            Extra::CheckNow => "Şimdi Denetle",
            Extra::Checking => "Denetleniyor…",
            Extra::UpToDate => "Güncel",
            Extra::UpdateAvailable => "Güncelleme var: %@",
            Extra::CheckFailed => "Denetlenemedi",
            Extra::LastChecked => "Son Denetleme",
            Extra::ToolkitMissing => "Kurulu değil",
            Extra::ToolkitInstall => "Araç Setini İndir",
            Extra::ToolkitInstallUpdate => "Güncellemeyi Kur",
            Extra::ToolkitChooseFolder => "Araç Seti Klasörü Kullan",
            Extra::CpuConversion => "CPU Dönüştürme",
            Extra::CpuConversionDetail => {
                "Oyunların işlemcine göre nasıl hazırlanacağı. Otomatik çoğu bilgisayara uyar."
            }
            Extra::CpuAuto => "Otomatik",
            Extra::CpuNone => "Yok",
            Extra::CpuIntel => "Intel",
            Extra::DiagnosisHostCpu => "Bu işlemcide oyunun ihtiyaç duyduğu %@ yok.",
            Extra::DiagnosisNoUdmabuf => {
                "Oyun /dev/udmabuf dosyasını açamıyor. Kullanıcını kvm grubuna ekleyip yeniden oturum aç."
            }
            Extra::SaveReport => "Raporu Kaydet",
            Extra::OpenOnlineLog => "Çevrimiçi Günlüğü Aç",
            Extra::SendReport => "Rapor Gönder",
            Extra::ToolkitDownloading => "Araç seti indiriliyor…",
            Extra::FontsBuiltIn => "Yerleşik Noto (Latin, Tayca, Arapça)",
            Extra::FontsCjkReady => "Japonca, Çince ve Korece dahil",
            Extra::FontsCjkMissing => "Japonca, Çince ve Korece indirilmedi",
            Extra::FontsCjkDownloading => "Japonca, Çince ve Korece indiriliyor",
            Extra::FontsDownloadCjk => "Japonca, Çince ve Korece Fontlarını İndir",
            Extra::ToolkitReady => "Araç seti %@ hazır",
            Extra::ToolkitFailed => "Araç seti kurulamadı. Bağlantını denetleyip yeniden dene.",
            Extra::HomeMusic => "Ana Ekran Müziği",
            Extra::HomeMusicDetail => {
                "Ana ekranda sakin bir ortam müziği çalar. Oyun çalışırken susar."
            }
            Extra::GameMusic => "Oyun Müziği",
            Extra::GameMusicDetail => "Ana ekran müziği yerine seçili oyunun kendi müziğini çalar.",
            Extra::Vibration => "Kol Titreşimi",
            Extra::VibrationDetail => {
                "Kolda bir tuşa basınca kısa, yumuşak bir titreşim karşılık verir. Oyun öndeyken asla."
            }
            Extra::ControllerHelp => {
                "Steam Input ve DS4Windows kolu devralabilir. Steam'de PlayStation desteğini kapat ya da iki uygulamadan da çık, sonra kolu yeniden bağla."
            }
            Extra::PadStatus => "Kol bağlı",
            Extra::PadStatusBattery => "Kol bağlı, pil yüzde %@",
            Extra::PadStatusCharging => "Kol bağlı, pil yüzde %@, şarj oluyor",
            Extra::VibrationLight => "Hafif",
            Extra::VibrationMedium => "Orta",
            Extra::VibrationStrong => "Güçlü",
            Extra::DiscordPresence => "Oyunu Discord'da Göster",
            Extra::DiscordPresenceDetail => {
                "Discord açıksa profilinde oyunun adı ve süresi görünür."
            }
            Extra::ShowHiddenTrophies => "Gizli Kupaları Her Zaman Göster",
            Extra::ShowHiddenTrophiesDetail => {
                "Gizli kupaların adı ve açıklaması kazanmadan önce de görünür. Bunu yalnızca sen görürsün."
            }
            Extra::TrophyShow => "Göster",
            Extra::TrophiesNone => "Bu oyunun kupası yok.",
            Extra::EmptyLog => "Henüz günlük yok. Oyunu bir kez oynadığında burada görünür.",
            Extra::ToolkitFolderInvalid => {
                "Bu klasör bir araç seti değil. relinker ve libs içeren klasörü seç."
            }
            Extra::Layout => "Düzen",
            Extra::LayoutLauncher => "Başlatıcı",
            Extra::LayoutMouse => "Fare ve Klavye",
            Extra::LayoutGame => "Oyundan",
            Extra::LayoutCustom => "Özel",
            Extra::LayoutGameDetail => "Oyunun kendi PC tuşları %@ düğme için bulundu.",
            Extra::LauncherReady => "%@ hazır",
            Extra::LauncherToast => "Başlatıcı %@ hazır. Başlatıcıyı kapattığında kurulur.",
            Extra::RestartNow => "Şimdi Yeniden Başlat",
            Extra::LauncherReadOnly => "%@ var, ama bu klasör salt okunur",
            Extra::LauncherUpdateFailed => {
                "Başlatıcı güncellemesi kurulamadı. Bunun yerine GitHub'dan indir."
            }
            Extra::Links => "AnyPlay Bağlantılarını Bu Başlatıcıyla Aç",
            Extra::LinksDetail => {
                "Web sitelerindeki ve mesajlardaki bağlantılar burada açılır, klasörlere \"AnyPlay Launcher ile aç\" girişi eklenir."
            }
            Extra::LinksDetailOther => {
                "Şu an bunları başka bir başlatıcı kopyası açıyor. Bunu kullanmak için aç."
            }
            Extra::LinksFailed => "Bağlantı ayarları değiştirilemedi. Yeniden dene.",
            Extra::LinkInvalid => "Bu geçerli bir AnyPlay bağlantısı değil.",
            Extra::LinkNoGame => "%@ kitaplığında yok. Ayarlar'daki oyun klasörünü denetle.",
            Extra::LinkNotADump => "Bu klasör bir oyun dökümü değil. eboot.bin içeren klasörü aç.",
            Extra::LinkOutside => "%@ oyun klasörünün dışında. Eklemek için oraya taşı.",
            Extra::LinkUnlock => "%@ oyununu hazırlamak için Kilidi Aç'ı seç.",
            Extra::LinkUnlocked => "%@ zaten açık.",
            Extra::ChannelTitle => "Kanal",
            Extra::ChannelStable => "Kararlı",
            Extra::ChannelPinned => "Sabit",
            Extra::ChannelLocal => "Yerel Yapı",
            Extra::ChannelStableDetail => "En yeni sürümü izler.",
            Extra::ChannelPinnedDetail => "Tek bir sürümde kalır. Güncelleme önerilmez.",
            Extra::ChannelLocalDetail => {
                "Kendi yaptığın ya da seçtiğin klasörü kullanır. Güncelleme önerilmez."
            }
            Extra::ChannelEngine => "AnyPlay Motoru",
            Extra::ChannelEngineDetail => {
                "Çevrimiçi oyun için gecelik derleme. Sık değişir, oyunları bozabilir."
            }
            Extra::EngineBuilt => "Derleme tarihi: %@",
            Extra::EngineAvailable => "Yeni bir motor derlemesi indirilebilir",
            Extra::EngineNoBuild => {
                "Bu sistem için henüz motor derlemesi yok. Hiçbir şey kurulmadı. Başka bir kanal seç."
            }
            Extra::EngineUnverified => {
                "Motor indirmesi doğrulanamadı. Hiçbir şey kurulmadı. Daha sonra yeniden dene."
            }
            Extra::PinnedVersion => "Sürüm",
            Extra::WhatsNew => "Yenilikler",
            Extra::LastCheckedOn => "Son denetleme: %@",
            Extra::FitNewer => {
                "Bu başlatıcının denendiğinden daha yeni. Yine de çalışması beklenir."
            }
            Extra::FitOlder => "Eski bir araç seti: bazı özellikler kapalı.",
            Extra::ToolkitAvailable => "Araç seti %@ indirilebilir",
            Extra::NeedsToolkit => "Araç seti %@ gerekir",
            Extra::NeedsNewerThan => "%@ sürümünden yeni bir yapı gerekir",
            Extra::NotesHeader => "%@, yayın tarihi %@",
            Extra::ChangesCount => "%@ değişiklik",
            Extra::ChangeOne => "1 değişiklik",
            Extra::NotesHighlights => "Öne Çıkanlar",
            Extra::NotesOther => "Diğer Alanlar",
            Extra::NotesEverything => "Tümü",
            Extra::NotesLoading => "Notlar yükleniyor…",
            Extra::NotesUnavailable => "Notlar yüklenemedi.",
            Extra::NotesOpenRelease => "GitHub'da Aç",
            Extra::UnlockedWith => "%@ ile açıldı. %@ kurulu.",
            Extra::ConversionChanged => {
                "Bu güncelleme oyunların dönüştürülme şeklini değiştiriyor."
            }
            Extra::UnlockOptional => "Yeniden açmak isteğe bağlı.",
            Extra::UnlockEarlierBuild => "Bu araç setinin önceki bir yapısıyla açıldı.",
            Extra::RecTitle => "Bu Oyun için Önerilenler",
            Extra::RecNeeded => "Başlamak için gerekli",
            Extra::RecWorthALook => "Göz atmaya değer",
            Extra::RecApply => "Uygula",
            Extra::RecDefault => "Varsayılan",
            Extra::RecOn => "Açık",
            Extra::RecOff => "Kapalı",
            Extra::RecPlayGo => "Eksik PlayGo Verisi",
            Extra::RecPlayGoInstalled => "Kurulu Say",
            Extra::RecPlayGoAbsent => "Yok Say",
            Extra::RecImageBase => "Yükleme Adresi",
            Extra::RecModuleFolder => "Modül Klasörü",
            Extra::RecExecutable => "Çalıştırılabilir Dosya",
            Extra::RecTextureCache => "Doku Önbelleği",
            Extra::RecShaderCache => "Gölgelendirici Önbelleği",
            Extra::RecUnlockHint => "Bunun geçerli olması için oyunun kilidini yeniden aç.",
            Extra::RecApplied => "Uygulandı",
            Extra::RecAppliedUnlock => "Kaydedildi. Kullanmak için oyunun kilidini yeniden aç.",
            Extra::Played => "Oynama Süresi",
            Extra::PlayedUnderMinute => "1 dk altı",
            Extra::PlayedMinutes => "%@ dk",
            Extra::PlayedHours => "%@ sa",
            Extra::PlayedHoursMinutes => "%@ sa %@ dk",
            Extra::SavesTitle => "Kayıt Verileri",
            Extra::SavesNone => "Henüz yedek yok",
            Extra::SavesOne => "1 yedek",
            Extra::SavesCount => "%@ yedek",
            Extra::SavesBackUp => "Şimdi Yedekle",
            Extra::SavesNothing => "Yedeklenecek kayıt verisi henüz yok",
            Extra::SavesEmpty => {
                "Henüz yedek yok. Şimdi Yedekle, kayıtlarının ve kupalarının bir kopyasını saklar."
            }
            Extra::SavesCaption => "Yedekler",
            Extra::GroupScreen => "Ekran",
            Extra::GroupPlay => "Oyun",
            Extra::GroupMaintenance => "Bakım",
            Extra::SavesKept => {
                "En fazla %@ yedek saklanır. Oynadıktan sonra alınanlar önce silinir."
            }
            Extra::SavesReasonManual => "Elle",
            Extra::SavesReasonUnlock => "Kilidi yeniden açmadan önce",
            Extra::SavesReasonRestore => "Geri yüklemeden önce",
            Extra::SavesReasonUninstall => "Kaldırmadan önce",
            Extra::SavesReasonPlayed => "Oynadıktan sonra",
            Extra::SavesBackedUp => "Kayıtlar yedeklendi",
            Extra::SavesRestore => "Geri Yükle",
            Extra::SavesRestoreAsk => {
                "Oyunun şu anki kayıtları ve kupaları, %@ tarihli yedekle değiştirilir. Şu anki hâli önce yedeklenir, böylece geri alabilirsin."
            }
            Extra::SavesRestored => "Kayıtlar geri yüklendi",
            Extra::SavesRunning => "Yedeği geri yüklemek için oyunu kapat.",
            Extra::SavesFailed => {
                "Kayıtlar yedeklenemedi. Boş disk alanını kontrol edip yeniden dene."
            }
            Extra::SavesRestoreFailed => "Yedek geri yüklenemedi. Kayıtların değişmedi.",
            Extra::SavesCorrupt => "Bu yedek hasarlı. Kayıtların değişmedi. Başka bir yedek seç.",
            Extra::UninstallTitle => "Bu kurulum kaldırılsın mı?",
            Extra::SavesRestoreTitle => "Bu yedek geri yüklensin mi?",
            Extra::UninstallBacksUp => "Kayıtların ve kupaların önce yedeklenir.",
            Extra::UninstallNeedsBackup => {
                "Kayıtlar yedeklenemedi, bu yüzden oyun kaldırılmadı. Boş disk alanını kontrol edip yeniden dene."
            }
            Extra::Uninstalling => "Kaldırılıyor",
            Extra::Cancelling => "İptal ediliyor…",
            Extra::Pause => "Duraklat",
            Extra::Pausing => "Duraklatılıyor…",
            Extra::Resume => "Devam Et",
            Extra::PausedNote => "Yarım kaldı: ilerleme korunmadı. Devam Et baştan başlar.",
            Extra::HeldNote => "Duraklatıldı. Devam Et buradan sürdürür.",
            Extra::SavesRestoreIncomplete => {
                "Geri yükleme tamamlanmadı. Önceki kayıtlar en yeni yedekte, \"Geri yüklemeden önce\"."
            }
            Extra::SavesRestoreIncompleteNone => "Geri yükleme tamamlanmadı. Yeniden dene.",
            Extra::SavesBusy => "Kayıtlar üzerinde hâlâ çalışılıyor. Birazdan yeniden dene.",
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            Extra::Toolkit => "Toolkit",
            Extra::UpdateCheck => "Nach Updates suchen",
            Extra::UpdateCheckDetail => {
                "Sucht beim Start des Launchers nach einem neueren Toolkit. Es wird nur die Versionsnummer abgefragt."
            }
            Extra::CheckNow => "Jetzt prüfen",
            Extra::Checking => "Prüfe …",
            Extra::UpToDate => "Aktuell",
            Extra::UpdateAvailable => "Update verfügbar: %@",
            Extra::CheckFailed => "Prüfung fehlgeschlagen",
            Extra::LastChecked => "Zuletzt geprüft",
            Extra::ToolkitMissing => "Nicht installiert",
            Extra::ToolkitInstall => "Toolkit herunterladen",
            Extra::ToolkitInstallUpdate => "Update installieren",
            Extra::ToolkitChooseFolder => "Toolkit-Ordner verwenden",
            Extra::CpuConversion => "CPU-Umwandlung",
            Extra::CpuConversionDetail => {
                "Wie Spiele für deinen Prozessor vorbereitet werden. Automatisch passt für die meisten PCs."
            }
            Extra::CpuAuto => "Automatisch",
            Extra::CpuNone => "Keine",
            Extra::CpuIntel => "Intel",
            Extra::DiagnosisHostCpu => "Dieser CPU fehlt %@, das das Spiel braucht.",
            Extra::DiagnosisNoUdmabuf => {
                "Das Spiel kann /dev/udmabuf nicht öffnen. Füge deinen Benutzer der Gruppe kvm hinzu und melde dich neu an."
            }
            Extra::SaveReport => "Bericht speichern",
            Extra::OpenOnlineLog => "Online-Protokoll öffnen",
            Extra::SendReport => "Bericht senden",
            Extra::ToolkitDownloading => "Toolkit wird heruntergeladen …",
            Extra::FontsBuiltIn => "Integrierte Noto (Lateinisch, Thai, Arabisch)",
            Extra::FontsCjkReady => "Japanisch, Chinesisch und Koreanisch enthalten",
            Extra::FontsCjkMissing => "Japanisch, Chinesisch und Koreanisch nicht heruntergeladen",
            Extra::FontsCjkDownloading => "Japanisch, Chinesisch und Koreanisch werden geladen",
            Extra::FontsDownloadCjk => "Schriften für Japanisch, Chinesisch und Koreanisch laden",
            Extra::ToolkitReady => "Toolkit %@ ist bereit",
            Extra::ToolkitFailed => {
                "Das Toolkit konnte nicht installiert werden. Prüfe deine Verbindung und versuche es erneut."
            }
            Extra::HomeMusic => "Musik auf dem Startbildschirm",
            Extra::HomeMusicDetail => {
                "Spielt ruhige Hintergrundmusik auf dem Startbildschirm. Sie blendet aus, solange ein Spiel läuft."
            }
            Extra::GameMusic => "Spielmusik",
            Extra::GameMusicDetail => {
                "Spielt statt der Startbildschirm-Musik die eigene Musik des gewählten Spiels."
            }
            Extra::Vibration => "Controller-Vibration",
            Extra::VibrationDetail => {
                "Ein kurzer, sanfter Impuls antwortet auf einen Tastendruck am Controller. Nie, solange ein Spiel im Vordergrund ist."
            }
            Extra::ControllerHelp => {
                "Steam Input und DS4Windows können den Controller blockieren. Schalte die PlayStation-Unterstützung in Steam aus oder beende beide Apps, dann verbinde ihn neu."
            }
            Extra::PadStatus => "Controller verbunden",
            Extra::PadStatusBattery => "Controller verbunden, Akku %@ Prozent",
            Extra::PadStatusCharging => "Controller verbunden, Akku %@ Prozent, wird geladen",
            Extra::VibrationLight => "Leicht",
            Extra::VibrationMedium => "Mittel",
            Extra::VibrationStrong => "Stark",
            Extra::DiscordPresence => "Spiel auf Discord zeigen",
            Extra::DiscordPresenceDetail => {
                "Ist Discord geöffnet, zeigt dein Profil das Spiel und wie lange du spielst."
            }
            Extra::ShowHiddenTrophies => "Versteckte Trophäen immer zeigen",
            Extra::ShowHiddenTrophiesDetail => {
                "Versteckte Trophäen zeigen Name und Beschreibung schon vor dem Freischalten. Das siehst nur du."
            }
            Extra::TrophyShow => "Zeigen",
            Extra::TrophiesNone => "Dieses Spiel hat keine Trophäen.",
            Extra::EmptyLog => {
                "Noch kein Protokoll. Spiele das Spiel einmal, dann erscheint es hier."
            }
            Extra::ToolkitFolderInvalid => {
                "Dieser Ordner ist kein Toolkit. Wähle den Ordner mit Relinker und libs."
            }
            Extra::Layout => "Belegung",
            Extra::LayoutLauncher => "Launcher",
            Extra::LayoutMouse => "Maus und Tastatur",
            Extra::LayoutGame => "Aus dem Spiel",
            Extra::LayoutCustom => "Eigene",
            Extra::LayoutGameDetail => "Eigene PC-Tasten des Spiels für %@ Tasten gefunden.",
            Extra::LauncherReady => "%@ bereit",
            Extra::LauncherToast => {
                "Launcher %@ ist bereit. Er wird installiert, wenn du den Launcher schließt."
            }
            Extra::RestartNow => "Jetzt neu starten",
            Extra::LauncherReadOnly => "%@ verfügbar, aber dieser Ordner ist schreibgeschützt",
            Extra::LauncherUpdateFailed => {
                "Das Launcher-Update konnte nicht installiert werden. Lade es stattdessen von GitHub herunter."
            }
            Extra::Links => "AnyPlay-Links mit diesem Launcher öffnen",
            Extra::LinksDetail => {
                "Links auf Websites und in Nachrichten öffnen sich hier, und Ordner erhalten den Eintrag \"Mit AnyPlay Launcher öffnen\"."
            }
            Extra::LinksDetailOther => {
                "Eine andere Kopie des Launchers öffnet sie gerade. Schalte dies ein, um diese zu verwenden."
            }
            Extra::LinksFailed => {
                "Die Link-Einstellungen konnten nicht geändert werden. Versuche es erneut."
            }
            Extra::LinkInvalid => "Das ist kein gültiger AnyPlay-Link.",
            Extra::LinkNoGame => {
                "%@ ist nicht in deiner Bibliothek. Prüfe den Spieleordner in den Einstellungen."
            }
            Extra::LinkNotADump => {
                "Dieser Ordner ist kein Spiel-Dump. Öffne den Ordner mit eboot.bin."
            }
            Extra::LinkOutside => {
                "%@ liegt außerhalb deines Spieleordners. Verschiebe es dorthin, um es hinzuzufügen."
            }
            Extra::LinkUnlock => "Wähle Freischalten, um %@ vorzubereiten.",
            Extra::LinkUnlocked => "%@ ist schon freigeschaltet.",
            Extra::ChannelTitle => "Kanal",
            Extra::ChannelStable => "Stabil",
            Extra::ChannelPinned => "Fest",
            Extra::ChannelLocal => "Lokaler Build",
            Extra::ChannelStableDetail => "Folgt dem neuesten Release.",
            Extra::ChannelPinnedDetail => {
                "Bleibt auf einem Release. Es wird kein Update angeboten."
            }
            Extra::ChannelLocalDetail => {
                "Nutzt einen Ordner, den du gebaut oder gewählt hast. Es wird kein Update angeboten."
            }
            Extra::ChannelEngine => "AnyPlay-Engine",
            Extra::ChannelEngineDetail => {
                "Der Nightly-Build mit Online-Spiel. Ändert sich oft und kann Spiele beschädigen."
            }
            Extra::EngineBuilt => "Gebaut am %@",
            Extra::EngineAvailable => "Ein neuer Engine-Build ist verfügbar",
            Extra::EngineNoBuild => {
                "Für dieses System gibt es noch keinen Engine-Build. Es wurde nichts installiert. Wähle einen anderen Kanal."
            }
            Extra::EngineUnverified => {
                "Der Engine-Download konnte nicht geprüft werden. Es wurde nichts installiert. Versuche es später erneut."
            }
            Extra::PinnedVersion => "Version",
            Extra::WhatsNew => "Neuigkeiten",
            Extra::LastCheckedOn => "Zuletzt geprüft: %@",
            Extra::FitNewer => {
                "Neuer, als dieser Launcher getestet wurde. Es sollte trotzdem funktionieren."
            }
            Extra::FitOlder => "Ein älteres Toolkit: Einige Funktionen sind ausgeschaltet.",
            Extra::ToolkitAvailable => "Toolkit %@ ist verfügbar",
            Extra::NeedsToolkit => "Braucht Toolkit %@",
            Extra::NeedsNewerThan => "Braucht einen Build neuer als %@",
            Extra::NotesHeader => "%@, veröffentlicht am %@",
            Extra::ChangesCount => "%@ Änderungen",
            Extra::ChangeOne => "1 Änderung",
            Extra::NotesHighlights => "Highlights",
            Extra::NotesOther => "Weitere Bereiche",
            Extra::NotesEverything => "Alles",
            Extra::NotesLoading => "Notizen werden geladen …",
            Extra::NotesUnavailable => "Die Notizen konnten nicht geladen werden.",
            Extra::NotesOpenRelease => "Auf GitHub öffnen",
            Extra::UnlockedWith => "Freigeschaltet mit %@. Installiert ist %@.",
            Extra::ConversionChanged => "Dieses Update ändert, wie Spiele umgewandelt werden.",
            Extra::UnlockOptional => "Erneutes Freischalten ist optional.",
            Extra::UnlockEarlierBuild => "Mit einem früheren Build dieses Toolkits freigeschaltet.",
            Extra::RecTitle => "Empfohlen für dieses Spiel",
            Extra::RecNeeded => "Nötig zum Start",
            Extra::RecWorthALook => "Einen Blick wert",
            Extra::RecApply => "Anwenden",
            Extra::RecDefault => "Standard",
            Extra::RecOn => "An",
            Extra::RecOff => "Aus",
            Extra::RecPlayGo => "Fehlende PlayGo-Daten",
            Extra::RecPlayGoInstalled => "Als installiert melden",
            Extra::RecPlayGoAbsent => "Als fehlend melden",
            Extra::RecImageBase => "Image-Basis",
            Extra::RecModuleFolder => "Modulordner",
            Extra::RecExecutable => "Programmdatei",
            Extra::RecTextureCache => "Textur-Cache",
            Extra::RecShaderCache => "Shader-Cache",
            Extra::RecUnlockHint => "Schalte das Spiel nach dem Anwenden erneut frei.",
            Extra::RecApplied => "Angewendet",
            Extra::RecAppliedUnlock => {
                "Gespeichert. Schalte das Spiel erneut frei, um es zu nutzen."
            }
            Extra::Played => "Gespielt",
            Extra::PlayedUnderMinute => "Unter 1 Min.",
            Extra::PlayedMinutes => "%@ Min.",
            Extra::PlayedHours => "%@ Std.",
            Extra::PlayedHoursMinutes => "%@ Std. %@ Min.",
            Extra::SavesTitle => "Spielstände",
            Extra::SavesNone => "Noch keine Sicherungen",
            Extra::SavesOne => "1 Sicherung",
            Extra::SavesCount => "%@ Sicherungen",
            Extra::SavesBackUp => "Jetzt sichern",
            Extra::SavesNothing => "Noch keine Spielstände zum Sichern",
            Extra::SavesEmpty => {
                "Noch keine Sicherungen. Jetzt sichern bewahrt eine Kopie deiner Spielstände und Trophäen auf."
            }
            Extra::SavesCaption => "Sicherungen",
            Extra::GroupScreen => "Bildschirm",
            Extra::GroupPlay => "Spielen",
            Extra::GroupMaintenance => "Wartung",
            Extra::SavesKept => {
                "Bis zu %@ Sicherungen werden behalten. Sicherungen nach dem Spielen werden zuerst entfernt."
            }
            Extra::SavesReasonManual => "Manuell",
            Extra::SavesReasonUnlock => "Vor erneutem Freischalten",
            Extra::SavesReasonRestore => "Vor einer Wiederherstellung",
            Extra::SavesReasonUninstall => "Vor dem Deinstallieren",
            Extra::SavesReasonPlayed => "Nach dem Spielen",
            Extra::SavesBackedUp => "Spielstände gesichert",
            Extra::SavesRestore => "Wiederherstellen",
            Extra::SavesRestoreAsk => {
                "Die aktuellen Spielstände und Trophäen des Spiels werden durch die Sicherung vom %@ ersetzt. Die aktuellen werden vorher gesichert, du kannst es also rückgängig machen."
            }
            Extra::SavesRestored => "Spielstände wiederhergestellt",
            Extra::SavesRunning => "Beende das Spiel, um eine Sicherung wiederherzustellen.",
            Extra::SavesFailed => {
                "Die Spielstände konnten nicht gesichert werden. Prüfe den freien Speicherplatz und versuche es erneut."
            }
            Extra::SavesRestoreFailed => {
                "Die Sicherung konnte nicht wiederhergestellt werden. Deine Spielstände wurden nicht geändert."
            }
            Extra::SavesCorrupt => {
                "Diese Sicherung ist beschädigt. Deine Spielstände wurden nicht geändert. Wähle eine andere Sicherung."
            }
            Extra::UninstallTitle => "Diese Installation entfernen?",
            Extra::SavesRestoreTitle => "Diese Sicherung wiederherstellen?",
            Extra::UninstallBacksUp => "Deine Spielstände und Trophäen werden vorher gesichert.",
            Extra::UninstallNeedsBackup => {
                "Die Spielstände konnten nicht gesichert werden, deshalb wurde das Spiel nicht entfernt. Prüfe den freien Speicherplatz und versuche es erneut."
            }
            Extra::Uninstalling => "Wird entfernt",
            Extra::Cancelling => "Wird abgebrochen…",
            Extra::Pause => "Pausieren",
            Extra::Pausing => "Wird pausiert…",
            Extra::Resume => "Fortsetzen",
            Extra::PausedNote => "Unvollständig: Fortschritt nicht erhalten. Fortsetzen beginnt von vorn.",
            Extra::HeldNote => "Pausiert. Fortsetzen macht hier weiter.",
            Extra::SavesRestoreIncomplete => {
                "Die Wiederherstellung wurde nicht abgeschlossen. Die früheren Spielstände sind in der neuesten Sicherung, Vor einer Wiederherstellung."
            }
            Extra::SavesRestoreIncompleteNone => {
                "Die Wiederherstellung wurde nicht abgeschlossen. Versuche es erneut."
            }
            Extra::SavesBusy => {
                "Die Spielstände werden noch bearbeitet. Versuche es gleich erneut."
            }
            _ => return None,
        })
    }

    /// The Spanish text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            Extra::Toolkit => "Toolkit",
            Extra::UpdateCheck => "Buscar actualizaciones",
            Extra::UpdateCheckDetail => {
                "Busca un Toolkit más reciente al iniciar el launcher. Solo se solicita el número de versión."
            }
            Extra::CheckNow => "Comprobar ahora",
            Extra::Checking => "Comprobando…",
            Extra::UpToDate => "Al día",
            Extra::UpdateAvailable => "Actualización disponible: %@",
            Extra::CheckFailed => "No se pudo comprobar",
            Extra::LastChecked => "Última comprobación",
            Extra::ToolkitMissing => "No instalado",
            Extra::ToolkitInstall => "Descargar el Toolkit",
            Extra::ToolkitInstallUpdate => "Instalar la actualización",
            Extra::ToolkitChooseFolder => "Usar una carpeta de Toolkit",
            Extra::CpuConversion => "Conversión de CPU",
            Extra::CpuConversionDetail => {
                "Cómo se preparan los juegos para tu procesador. Automático sirve para la mayoría de los PC."
            }
            Extra::CpuAuto => "Automático",
            Extra::CpuNone => "Ninguna",
            Extra::CpuIntel => "Intel",
            Extra::DiagnosisHostCpu => "A esta CPU le falta %@, que el juego necesita.",
            Extra::DiagnosisNoUdmabuf => {
                "El juego no puede abrir /dev/udmabuf. Añade tu usuario al grupo kvm y vuelve a iniciar sesión."
            }
            Extra::SaveReport => "Guardar informe",
            Extra::OpenOnlineLog => "Abrir registro en línea",
            Extra::SendReport => "Enviar informe",
            Extra::ToolkitDownloading => "Descargando el Toolkit…",
            Extra::FontsBuiltIn => "Noto integrada (latino, tailandés, árabe)",
            Extra::FontsCjkReady => "Japonés, chino y coreano incluidos",
            Extra::FontsCjkMissing => "Japonés, chino y coreano sin descargar",
            Extra::FontsCjkDownloading => "Descargando japonés, chino y coreano",
            Extra::FontsDownloadCjk => "Descargar fuentes de japonés, chino y coreano",
            Extra::ToolkitReady => "Toolkit %@ listo",
            Extra::ToolkitFailed => {
                "No se pudo instalar el Toolkit. Comprueba tu conexión e inténtalo de nuevo."
            }
            Extra::HomeMusic => "Música de la pantalla de inicio",
            Extra::HomeMusicDetail => {
                "Reproduce música ambiental tranquila en la pantalla de inicio. Se desvanece mientras se ejecuta un juego."
            }
            Extra::GameMusic => "Música del juego",
            Extra::GameMusicDetail => {
                "Reproduce la música propia del juego seleccionado en lugar de la de la pantalla de inicio."
            }
            Extra::Vibration => "Vibración del mando",
            Extra::VibrationDetail => {
                "Un pulso breve y suave responde a cada pulsación en el mando. Nunca mientras un juego está en primer plano."
            }
            Extra::ControllerHelp => {
                "Steam Input y DS4Windows pueden bloquear el mando. Desactiva la compatibilidad con PlayStation en Steam o cierra ambas apps y vuelve a conectarlo."
            }
            Extra::PadStatus => "Mando conectado",
            Extra::PadStatusBattery => "Mando conectado, batería al %@ por ciento",
            Extra::PadStatusCharging => "Mando conectado, batería al %@ por ciento, cargando",
            Extra::VibrationLight => "Suave",
            Extra::VibrationMedium => "Media",
            Extra::VibrationStrong => "Fuerte",
            Extra::DiscordPresence => "Mostrar el juego en Discord",
            Extra::DiscordPresenceDetail => {
                "Con Discord abierto, tu perfil muestra el juego y cuánto tiempo llevas jugando."
            }
            Extra::ShowHiddenTrophies => "Mostrar siempre los trofeos ocultos",
            Extra::ShowHiddenTrophiesDetail => {
                "Los trofeos ocultos muestran su nombre y descripción antes de conseguirlos. Solo lo ves tú."
            }
            Extra::TrophyShow => "Mostrar",
            Extra::TrophiesNone => "Este juego no tiene trofeos.",
            Extra::EmptyLog => "Aún no hay registro. Juega una vez y aparecerá aquí.",
            Extra::ToolkitFolderInvalid => {
                "Esta carpeta no es un Toolkit. Elige la carpeta que contiene el relinker y las libs."
            }
            Extra::Layout => "Disposición",
            Extra::LayoutLauncher => "Launcher",
            Extra::LayoutMouse => "Ratón y teclado",
            Extra::LayoutGame => "Del juego",
            Extra::LayoutCustom => "Personalizada",
            Extra::LayoutGameDetail => {
                "Se encontraron las teclas de PC propias del juego para %@ botones."
            }
            Extra::LauncherReady => "%@ listo",
            Extra::LauncherToast => "El launcher %@ está listo. Se instala al cerrar el launcher.",
            Extra::RestartNow => "Reiniciar ahora",
            Extra::LauncherReadOnly => "%@ disponible, pero esta carpeta es de solo lectura",
            Extra::LauncherUpdateFailed => {
                "No se pudo instalar la actualización del launcher. Descárgala desde GitHub."
            }
            Extra::Links => "Abrir enlaces de AnyPlay con este launcher",
            Extra::LinksDetail => {
                "Los enlaces de sitios web y mensajes se abren aquí, y las carpetas reciben la entrada «Abrir con AnyPlay Launcher»."
            }
            Extra::LinksDetailOther => {
                "Ahora los abre otra copia del launcher. Activa esto para usar esta."
            }
            Extra::LinksFailed => "No se pudo cambiar el ajuste de enlaces. Inténtalo de nuevo.",
            Extra::LinkInvalid => "Este no es un enlace de AnyPlay válido.",
            Extra::LinkNoGame => {
                "%@ no está en tu biblioteca. Revisa la carpeta de juegos en Ajustes."
            }
            Extra::LinkNotADump => {
                "Esta carpeta no es un dump de juego. Abre la carpeta que contiene eboot.bin."
            }
            Extra::LinkOutside => {
                "%@ está fuera de tu carpeta de juegos. Muévelo allí para añadirlo."
            }
            Extra::LinkUnlock => "Selecciona Desbloquear para preparar %@.",
            Extra::LinkUnlocked => "%@ ya está desbloqueado.",
            Extra::ChannelTitle => "Canal",
            Extra::ChannelStable => "Estable",
            Extra::ChannelPinned => "Fijado",
            Extra::ChannelLocal => "Compilación local",
            Extra::ChannelStableDetail => "Sigue la versión más reciente.",
            Extra::ChannelPinnedDetail => {
                "Se queda en una versión. No se ofrece ninguna actualización."
            }
            Extra::ChannelLocalDetail => {
                "Usa una carpeta que compilaste o elegiste. No se ofrece ninguna actualización."
            }
            Extra::ChannelEngine => "Motor de AnyPlay",
            Extra::ChannelEngineDetail => {
                "La compilación nocturna con juego en línea. Cambia a menudo y puede romper juegos."
            }
            Extra::EngineBuilt => "Compilado el %@",
            Extra::EngineAvailable => "Hay una nueva compilación del motor disponible",
            Extra::EngineNoBuild => {
                "Todavía no hay compilación del motor para este sistema. No se instaló nada. Elige otro canal."
            }
            Extra::EngineUnverified => {
                "No se pudo comprobar la descarga del motor. No se instaló nada. Inténtalo más tarde."
            }
            Extra::PinnedVersion => "Versión",
            Extra::WhatsNew => "Novedades",
            Extra::LastCheckedOn => "Última comprobación: %@",
            Extra::FitNewer => {
                "Más reciente de lo probado con este launcher. Debería funcionar igualmente."
            }
            Extra::FitOlder => "Un Toolkit anterior: algunas funciones están desactivadas.",
            Extra::ToolkitAvailable => "Toolkit %@ disponible",
            Extra::NeedsToolkit => "Requiere el Toolkit %@",
            Extra::NeedsNewerThan => "Requiere una compilación posterior a %@",
            Extra::NotesHeader => "%@, publicada el %@",
            Extra::ChangesCount => "%@ cambios",
            Extra::ChangeOne => "1 cambio",
            Extra::NotesHighlights => "Destacados",
            Extra::NotesOther => "Otras áreas",
            Extra::NotesEverything => "Todo",
            Extra::NotesLoading => "Cargando las notas…",
            Extra::NotesUnavailable => "No se pudieron cargar las notas.",
            Extra::NotesOpenRelease => "Abrir en GitHub",
            Extra::UnlockedWith => "Desbloqueado con %@. La versión instalada es %@.",
            Extra::ConversionChanged => "Esta actualización cambia cómo se convierten los juegos.",
            Extra::UnlockOptional => "Volver a desbloquear es opcional.",
            Extra::UnlockEarlierBuild => {
                "Desbloqueado con una compilación anterior de este Toolkit."
            }
            Extra::RecTitle => "Recomendado para este juego",
            Extra::RecNeeded => "Necesario para iniciar",
            Extra::RecWorthALook => "Conviene revisarlo",
            Extra::RecApply => "Aplicar",
            Extra::RecDefault => "Predeterminado",
            Extra::RecOn => "Activado",
            Extra::RecOff => "Desactivado",
            Extra::RecPlayGo => "Faltan datos de PlayGo",
            Extra::RecPlayGoInstalled => "Informar como instalado",
            Extra::RecPlayGoAbsent => "Informar como ausente",
            Extra::RecImageBase => "Base de imagen",
            Extra::RecModuleFolder => "Carpeta de módulos",
            Extra::RecExecutable => "Ejecutable",
            Extra::RecTextureCache => "Caché de texturas",
            Extra::RecShaderCache => "Caché de shaders",
            Extra::RecUnlockHint => "Después de aplicar esto, vuelve a desbloquear el juego.",
            Extra::RecApplied => "Aplicado",
            Extra::RecAppliedUnlock => "Guardado. Vuelve a desbloquear el juego para usarlo.",
            Extra::Played => "Jugado",
            Extra::PlayedUnderMinute => "Menos de 1 min",
            Extra::PlayedMinutes => "%@ min",
            Extra::PlayedHours => "%@ h",
            Extra::PlayedHoursMinutes => "%@ h %@ min",
            Extra::SavesTitle => "Partidas guardadas",
            Extra::SavesNone => "Aún no hay copias",
            Extra::SavesOne => "1 copia",
            Extra::SavesCount => "%@ copias",
            Extra::SavesBackUp => "Hacer copia ahora",
            Extra::SavesNothing => "Aún no hay partidas guardadas para copiar",
            Extra::SavesEmpty => {
                "Aún no hay copias. Hacer copia ahora guarda una copia de tus partidas guardadas y trofeos."
            }
            Extra::SavesCaption => "Copias de seguridad",
            Extra::GroupScreen => "Pantalla",
            Extra::GroupPlay => "Jugar",
            Extra::GroupMaintenance => "Mantenimiento",
            Extra::SavesKept => {
                "Se conservan hasta %@ copias. Las hechas después de jugar se eliminan primero."
            }
            Extra::SavesReasonManual => "Manual",
            Extra::SavesReasonUnlock => "Antes de volver a desbloquear",
            Extra::SavesReasonRestore => "Antes de restaurar",
            Extra::SavesReasonUninstall => "Antes de quitar la instalación",
            Extra::SavesReasonPlayed => "Después de jugar",
            Extra::SavesBackedUp => "Partidas guardadas copiadas",
            Extra::SavesRestore => "Restaurar",
            Extra::SavesRestoreAsk => {
                "Esto reemplaza las partidas guardadas y los trofeos actuales del juego por la copia del %@. Primero se hace una copia de los actuales, así que puedes deshacerlo."
            }
            Extra::SavesRestored => "Partidas guardadas restauradas",
            Extra::SavesRunning => "Cierra el juego para restaurar una copia.",
            Extra::SavesFailed => {
                "No se pudieron copiar las partidas guardadas. Revisa el espacio libre en disco e inténtalo de nuevo."
            }
            Extra::SavesRestoreFailed => {
                "No se pudo restaurar la copia. Tus partidas guardadas no han cambiado."
            }
            Extra::SavesCorrupt => {
                "Esta copia está dañada. Tus partidas guardadas no han cambiado. Elige otra copia."
            }
            Extra::UninstallTitle => "¿Quitar esta instalación?",
            Extra::SavesRestoreTitle => "¿Restaurar esta copia?",
            Extra::UninstallBacksUp => {
                "Primero se hace una copia de tus partidas guardadas y trofeos."
            }
            Extra::UninstallNeedsBackup => {
                "No se pudieron copiar las partidas guardadas, así que no se quitó el juego. Revisa el espacio libre en disco e inténtalo de nuevo."
            }
            Extra::Uninstalling => "Quitando",
            Extra::SavesRestoreIncomplete => {
                "La restauración no terminó. Las partidas anteriores están en la copia más reciente, Antes de restaurar."
            }
            Extra::SavesRestoreIncompleteNone => "La restauración no terminó. Inténtalo de nuevo.",
            Extra::SavesBusy => {
                "Aún se están procesando las partidas guardadas. Inténtalo de nuevo en un momento."
            }
            Extra::Cancelling => "Cancelando…",
            Extra::Pause => "Pausar",
            Extra::Pausing => "Pausando…",
            Extra::Resume => "Reanudar",
            Extra::PausedNote => "Incompleto: el progreso no se conservó. Reanudar empieza desde el principio.",
            Extra::HeldNote => "En pausa. Reanudar continúa desde aquí.",
            _ => return None,
        })
    }

    /// The French text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn french(self) -> Option<&'static str> {
        Some(match self {
            Extra::Toolkit => "Toolkit",
            Extra::UpdateCheck => "Rechercher des mises à jour",
            Extra::UpdateCheckDetail => {
                "Cherche un Toolkit plus récent au démarrage du launcher. Seul le numéro de version est demandé."
            }
            Extra::CheckNow => "Vérifier maintenant",
            Extra::Checking => "Vérification…",
            Extra::UpToDate => "À jour",
            Extra::UpdateAvailable => "Mise à jour disponible\u{a0}: %@",
            Extra::CheckFailed => "Vérification impossible",
            Extra::LastChecked => "Dernière vérification",
            Extra::ToolkitMissing => "Non installé",
            Extra::ToolkitInstall => "Télécharger le Toolkit",
            Extra::ToolkitInstallUpdate => "Installer la mise à jour",
            Extra::ToolkitChooseFolder => "Utiliser un dossier Toolkit",
            Extra::CpuConversion => "Conversion du CPU",
            Extra::CpuConversionDetail => {
                "Comment les jeux sont préparés pour ton processeur. Automatique convient à la plupart des PC."
            }
            Extra::CpuAuto => "Automatique",
            Extra::CpuNone => "Aucune",
            Extra::CpuIntel => "Intel",
            Extra::DiagnosisHostCpu => "Ce CPU n’a pas %@, dont le jeu a besoin.",
            Extra::DiagnosisNoUdmabuf => {
                "Le jeu ne peut pas ouvrir /dev/udmabuf. Ajoute ton utilisateur au groupe kvm, puis reconnecte-toi."
            }
            Extra::SaveReport => "Enregistrer le rapport",
            Extra::OpenOnlineLog => "Ouvrir le journal en ligne",
            Extra::SendReport => "Envoyer le rapport",
            Extra::ToolkitDownloading => "Téléchargement du Toolkit…",
            Extra::FontsBuiltIn => "Noto intégrée (latin, thaï, arabe)",
            Extra::FontsCjkReady => "Japonais, chinois et coréen inclus",
            Extra::FontsCjkMissing => "Japonais, chinois et coréen non téléchargés",
            Extra::FontsCjkDownloading => "Téléchargement du japonais, du chinois et du coréen",
            Extra::FontsDownloadCjk => "Télécharger les polices japonaise, chinoise et coréenne",
            Extra::ToolkitReady => "Toolkit %@ prêt",
            Extra::ToolkitFailed => {
                "Impossible d’installer le Toolkit. Vérifie ta connexion et réessaie."
            }
            Extra::HomeMusic => "Musique de l’écran d’accueil",
            Extra::HomeMusicDetail => {
                "Joue une musique d’ambiance calme sur l’écran d’accueil. Elle s’efface pendant qu’un jeu tourne."
            }
            Extra::GameMusic => "Musique du jeu",
            Extra::GameMusicDetail => {
                "Joue la musique propre au jeu sélectionné à la place de celle de l’écran d’accueil."
            }
            Extra::Vibration => "Vibration de la manette",
            Extra::VibrationDetail => {
                "Une impulsion brève et douce répond à chaque appui sur la manette. Jamais pendant qu’un jeu est au premier plan."
            }
            Extra::ControllerHelp => {
                "Steam Input et DS4Windows peuvent bloquer la manette. Désactive la prise en charge de PlayStation dans Steam, ou quitte les deux applications, puis reconnecte-la."
            }
            Extra::PadStatus => "Manette connectée",
            Extra::PadStatusBattery => "Manette connectée, batterie à %@ pour cent",
            Extra::PadStatusCharging => "Manette connectée, batterie à %@ pour cent, en charge",
            Extra::VibrationLight => "Légère",
            Extra::VibrationMedium => "Moyenne",
            Extra::VibrationStrong => "Forte",
            Extra::DiscordPresence => "Afficher le jeu sur Discord",
            Extra::DiscordPresenceDetail => {
                "Quand Discord est ouvert, ton profil affiche le jeu et depuis combien de temps tu joues."
            }
            Extra::ShowHiddenTrophies => "Toujours afficher les trophées cachés",
            Extra::ShowHiddenTrophiesDetail => {
                "Les trophées cachés montrent leur nom et leur description avant que tu les obtiennes. Toi seul le vois."
            }
            Extra::TrophyShow => "Afficher",
            Extra::TrophiesNone => "Ce jeu n’a pas de trophées.",
            Extra::EmptyLog => "Pas encore de journal. Lance le jeu une fois et il apparaîtra ici.",
            Extra::ToolkitFolderInvalid => {
                "Ce dossier n’est pas un Toolkit. Choisis le dossier qui contient le relinker et les libs."
            }
            Extra::Layout => "Disposition",
            Extra::LayoutLauncher => "Launcher",
            Extra::LayoutMouse => "Souris et clavier",
            Extra::LayoutGame => "Celle du jeu",
            Extra::LayoutCustom => "Personnalisée",
            Extra::LayoutGameDetail => "Touches PC propres au jeu trouvées pour %@ boutons.",
            Extra::LauncherReady => "%@ prêt",
            Extra::LauncherToast => {
                "Le launcher %@ est prêt. Il s’installe quand tu fermes le launcher."
            }
            Extra::RestartNow => "Redémarrer maintenant",
            Extra::LauncherReadOnly => "%@ disponible, mais ce dossier est en lecture seule",
            Extra::LauncherUpdateFailed => {
                "Impossible d’installer la mise à jour du launcher. Télécharge-la depuis GitHub."
            }
            Extra::Links => "Ouvrir les liens AnyPlay avec ce launcher",
            Extra::LinksDetail => {
                "Les liens des sites web et des messages s’ouvrent ici, et les dossiers reçoivent l’entrée «\u{a0}Ouvrir avec AnyPlay Launcher\u{a0}»."
            }
            Extra::LinksDetailOther => {
                "Une autre copie du launcher les ouvre actuellement. Active cette option pour utiliser celle-ci."
            }
            Extra::LinksFailed => "Impossible de modifier le réglage des liens. Réessaie.",
            Extra::LinkInvalid => "Ce n’est pas un lien AnyPlay valide.",
            Extra::LinkNoGame => {
                "%@ n’est pas dans ta bibliothèque. Vérifie le dossier des jeux dans les paramètres."
            }
            Extra::LinkNotADump => {
                "Ce dossier n’est pas un dump de jeu. Ouvre le dossier qui contient eboot.bin."
            }
            Extra::LinkOutside => {
                "%@ est en dehors de ton dossier des jeux. Déplace-le dedans pour l’ajouter."
            }
            Extra::LinkUnlock => "Sélectionne Déverrouiller pour préparer %@.",
            Extra::LinkUnlocked => "%@ est déjà déverrouillé.",
            Extra::ChannelTitle => "Canal",
            Extra::ChannelStable => "Stable",
            Extra::ChannelPinned => "Fixé",
            Extra::ChannelLocal => "Build local",
            Extra::ChannelStableDetail => "Suit la version la plus récente.",
            Extra::ChannelPinnedDetail => {
                "Reste sur une seule version. Aucune mise à jour n’est proposée."
            }
            Extra::ChannelLocalDetail => {
                "Utilise un dossier que tu as compilé ou choisi. Aucune mise à jour n’est proposée."
            }
            Extra::ChannelEngine => "Moteur AnyPlay",
            Extra::ChannelEngineDetail => {
                "Le build nocturne avec le jeu en ligne. Change souvent et peut casser des jeux."
            }
            Extra::EngineBuilt => "Compilé le %@",
            Extra::EngineAvailable => "Un nouveau build du moteur est disponible",
            Extra::EngineNoBuild => {
                "Il n’y a pas encore de build du moteur pour ce système. Rien n’a été installé. Choisis un autre canal."
            }
            Extra::EngineUnverified => {
                "Le téléchargement du moteur n’a pas pu être vérifié. Rien n’a été installé. Réessaie plus tard."
            }
            Extra::PinnedVersion => "Version",
            Extra::WhatsNew => "Nouveautés",
            Extra::LastCheckedOn => "Dernière vérification\u{a0}: %@",
            Extra::FitNewer => {
                "Plus récent que ce avec quoi ce launcher a été testé. Cela devrait quand même fonctionner."
            }
            Extra::FitOlder => {
                "Un Toolkit plus ancien\u{a0}: certaines fonctions sont désactivées."
            }
            Extra::ToolkitAvailable => "Toolkit %@ disponible",
            Extra::NeedsToolkit => "Nécessite le Toolkit %@",
            Extra::NeedsNewerThan => "Nécessite un build plus récent que %@",
            Extra::NotesHeader => "%@, publiée le %@",
            Extra::ChangesCount => "%@ modifications",
            Extra::ChangeOne => "1 modification",
            Extra::NotesHighlights => "À retenir",
            Extra::NotesOther => "Autres domaines",
            Extra::NotesEverything => "Tout",
            Extra::NotesLoading => "Chargement des notes…",
            Extra::NotesUnavailable => "Impossible de charger les notes.",
            Extra::NotesOpenRelease => "Ouvrir sur GitHub",
            Extra::UnlockedWith => "Déverrouillé avec %@. La version installée est %@.",
            Extra::ConversionChanged => {
                "Cette mise à jour change la façon dont les jeux sont convertis."
            }
            Extra::UnlockOptional => "Déverrouiller à nouveau est facultatif.",
            Extra::UnlockEarlierBuild => "Déverrouillé avec un build plus ancien de ce Toolkit.",
            Extra::RecTitle => "Recommandé pour ce jeu",
            Extra::RecNeeded => "Nécessaire pour démarrer",
            Extra::RecWorthALook => "À examiner",
            Extra::RecApply => "Appliquer",
            Extra::RecDefault => "Par défaut",
            Extra::RecOn => "Activé",
            Extra::RecOff => "Désactivé",
            Extra::RecPlayGo => "Données PlayGo manquantes",
            Extra::RecPlayGoInstalled => "Signaler comme installé",
            Extra::RecPlayGoAbsent => "Signaler comme absent",
            Extra::RecImageBase => "Base de l’image",
            Extra::RecModuleFolder => "Dossier des modules",
            Extra::RecExecutable => "Exécutable",
            Extra::RecTextureCache => "Cache des textures",
            Extra::RecShaderCache => "Cache des shaders",
            Extra::RecUnlockHint => "Après l’avoir appliqué, déverrouille à nouveau le jeu.",
            Extra::RecApplied => "Appliqué",
            Extra::RecAppliedUnlock => "Enregistré. Déverrouille à nouveau le jeu pour l’utiliser.",
            Extra::Played => "Temps de jeu",
            Extra::PlayedUnderMinute => "Moins de 1 min",
            Extra::PlayedMinutes => "%@ min",
            Extra::PlayedHours => "%@ h",
            Extra::PlayedHoursMinutes => "%@ h %@ min",
            Extra::SavesTitle => "Sauvegardes",
            Extra::SavesNone => "Aucune copie pour l’instant",
            Extra::SavesOne => "1 copie",
            Extra::SavesCount => "%@ copies",
            Extra::SavesBackUp => "Sauvegarder maintenant",
            Extra::SavesNothing => "Aucune sauvegarde de jeu à copier pour l’instant",
            Extra::SavesEmpty => {
                "Aucune copie pour l’instant. Sauvegarder maintenant garde une copie de tes sauvegardes et de tes trophées."
            }
            Extra::SavesCaption => "Copies de sauvegarde",
            Extra::GroupScreen => "Écran",
            Extra::GroupPlay => "Jeu",
            Extra::GroupMaintenance => "Maintenance",
            Extra::SavesKept => {
                "Jusqu’à %@ copies sont conservées. Celles faites après une partie sont supprimées en premier."
            }
            Extra::SavesReasonManual => "Manuelle",
            Extra::SavesReasonUnlock => "Avant un nouveau déverrouillage",
            Extra::SavesReasonRestore => "Avant une restauration",
            Extra::SavesReasonUninstall => "Avant la désinstallation",
            Extra::SavesReasonPlayed => "Après une partie",
            Extra::SavesBackedUp => "Sauvegardes copiées",
            Extra::SavesRestore => "Restaurer",
            Extra::SavesRestoreAsk => {
                "Cela remplace les sauvegardes et les trophées actuels du jeu par la copie du %@. Les actuels sont d’abord copiés, tu peux donc annuler."
            }
            Extra::SavesRestored => "Sauvegardes restaurées",
            Extra::SavesRunning => "Ferme le jeu pour restaurer une copie.",
            Extra::SavesFailed => {
                "Impossible de copier les sauvegardes. Vérifie l’espace disque libre et réessaie."
            }
            Extra::SavesRestoreFailed => {
                "Impossible de restaurer la copie. Tes sauvegardes n’ont pas changé."
            }
            Extra::SavesCorrupt => {
                "Cette copie est endommagée. Tes sauvegardes n’ont pas changé. Choisis une autre copie."
            }
            Extra::UninstallTitle => "Supprimer cette installation\u{a0}?",
            Extra::SavesRestoreTitle => "Restaurer cette copie\u{a0}?",
            Extra::UninstallBacksUp => "Tes sauvegardes et tes trophées sont d’abord copiés.",
            Extra::UninstallNeedsBackup => {
                "Impossible de copier les sauvegardes, le jeu n’a donc pas été supprimé. Vérifie l’espace disque libre et réessaie."
            }
            Extra::Uninstalling => "Suppression",
            Extra::SavesRestoreIncomplete => {
                "La restauration n’est pas terminée. Les sauvegardes d’avant sont dans la copie la plus récente, Avant une restauration."
            }
            Extra::SavesRestoreIncompleteNone => "La restauration n’est pas terminée. Réessaie.",
            Extra::SavesBusy => {
                "Les sauvegardes sont encore en cours de traitement. Réessaie dans un instant."
            }
            Extra::Cancelling => "Annulation…",
            Extra::Pause => "Pause",
            Extra::Pausing => "Mise en pause…",
            Extra::Resume => "Reprendre",
            Extra::PausedNote => "Incomplet : la progression n’a pas été conservée. Reprendre repart du début.",
            Extra::HeldNote => "En pause. Reprendre continue à partir d’ici.",
            _ => return None,
        })
    }

    /// The Portuguese text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            Extra::Toolkit => "Toolkit",
            Extra::UpdateCheck => "Buscar atualizações",
            Extra::UpdateCheckDetail => {
                "Procura um Toolkit mais novo ao abrir o launcher. Só o número da versão é consultado."
            }
            Extra::CheckNow => "Verificar agora",
            Extra::Checking => "Verificando…",
            Extra::UpToDate => "Atualizado",
            Extra::UpdateAvailable => "Atualização disponível: %@",
            Extra::CheckFailed => "Não foi possível verificar",
            Extra::LastChecked => "Última verificação",
            Extra::ToolkitMissing => "Não instalado",
            Extra::ToolkitInstall => "Baixar o Toolkit",
            Extra::ToolkitInstallUpdate => "Instalar a atualização",
            Extra::ToolkitChooseFolder => "Usar uma pasta de Toolkit",
            Extra::CpuConversion => "Conversão de CPU",
            Extra::CpuConversionDetail => {
                "Como os jogos são preparados para o seu processador. Automático serve para a maioria dos PCs."
            }
            Extra::CpuAuto => "Automático",
            Extra::CpuNone => "Nenhuma",
            Extra::CpuIntel => "Intel",
            Extra::DiagnosisHostCpu => "Esta CPU não tem %@, que o jogo precisa.",
            Extra::DiagnosisNoUdmabuf => {
                "O jogo não consegue abrir /dev/udmabuf. Adicione seu usuário ao grupo kvm e entre de novo."
            }
            Extra::SaveReport => "Salvar relatório",
            Extra::OpenOnlineLog => "Abrir registro online",
            Extra::SendReport => "Enviar relatório",
            Extra::ToolkitDownloading => "Baixando o Toolkit…",
            Extra::FontsBuiltIn => "Noto integrada (latim, tailandês, árabe)",
            Extra::FontsCjkReady => "Japonês, chinês e coreano incluídos",
            Extra::FontsCjkMissing => "Japonês, chinês e coreano não baixados",
            Extra::FontsCjkDownloading => "Baixando japonês, chinês e coreano",
            Extra::FontsDownloadCjk => "Baixar fontes de japonês, chinês e coreano",
            Extra::ToolkitReady => "Toolkit %@ pronto",
            Extra::ToolkitFailed => {
                "Não foi possível instalar o Toolkit. Verifique sua conexão e tente de novo."
            }
            Extra::HomeMusic => "Música da tela inicial",
            Extra::HomeMusicDetail => {
                "Toca uma música ambiente tranquila na tela inicial. Ela some aos poucos enquanto um jogo roda."
            }
            Extra::GameMusic => "Música do jogo",
            Extra::GameMusicDetail => {
                "Toca a música própria do jogo selecionado no lugar da música da tela inicial."
            }
            Extra::Vibration => "Vibração do controle",
            Extra::VibrationDetail => {
                "Um pulso curto e suave responde a cada toque no controle. Nunca enquanto um jogo está em primeiro plano."
            }
            Extra::ControllerHelp => {
                "O Steam Input e o DS4Windows podem bloquear o controle. Desative o suporte a PlayStation no Steam ou feche os dois apps e reconecte o controle."
            }
            Extra::PadStatus => "Controle conectado",
            Extra::PadStatusBattery => "Controle conectado, bateria em %@ por cento",
            Extra::PadStatusCharging => "Controle conectado, bateria em %@ por cento, carregando",
            Extra::VibrationLight => "Leve",
            Extra::VibrationMedium => "Média",
            Extra::VibrationStrong => "Forte",
            Extra::DiscordPresence => "Mostrar o jogo no Discord",
            Extra::DiscordPresenceDetail => {
                "Com o Discord aberto, seu perfil mostra o jogo e há quanto tempo você joga."
            }
            Extra::ShowHiddenTrophies => "Sempre mostrar troféus ocultos",
            Extra::ShowHiddenTrophiesDetail => {
                "Troféus ocultos mostram nome e descrição antes de você conquistá-los. Só você vê isso."
            }
            Extra::TrophyShow => "Mostrar",
            Extra::TrophiesNone => "Este jogo não tem troféus.",
            Extra::EmptyLog => "Ainda não há registro. Jogue uma vez e ele aparece aqui.",
            Extra::ToolkitFolderInvalid => {
                "Esta pasta não é um Toolkit. Escolha a pasta que contém o relinker e as libs."
            }
            Extra::Layout => "Layout",
            Extra::LayoutLauncher => "Launcher",
            Extra::LayoutMouse => "Mouse e teclado",
            Extra::LayoutGame => "Do jogo",
            Extra::LayoutCustom => "Personalizado",
            Extra::LayoutGameDetail => "Teclas de PC do próprio jogo encontradas para %@ botões.",
            Extra::LauncherReady => "%@ pronto",
            Extra::LauncherToast => {
                "O launcher %@ está pronto. Ele é instalado quando você fecha o launcher."
            }
            Extra::RestartNow => "Reiniciar agora",
            Extra::LauncherReadOnly => "%@ disponível, mas esta pasta é somente leitura",
            Extra::LauncherUpdateFailed => {
                "Não foi possível instalar a atualização do launcher. Baixe-a pelo GitHub."
            }
            Extra::Links => "Abrir links do AnyPlay com este launcher",
            Extra::LinksDetail => {
                "Links de sites e de mensagens abrem aqui, e as pastas ganham a opção \"Abrir com o AnyPlay Launcher\"."
            }
            Extra::LinksDetailOther => {
                "Outra cópia do launcher os abre agora. Ative isto para usar esta."
            }
            Extra::LinksFailed => {
                "Não foi possível alterar a configuração de links. Tente de novo."
            }
            Extra::LinkInvalid => "Este não é um link válido do AnyPlay.",
            Extra::LinkNoGame => {
                "%@ não está na sua biblioteca. Verifique a pasta de jogos em Configurações."
            }
            Extra::LinkNotADump => {
                "Esta pasta não é um dump de jogo. Abra a pasta que contém o eboot.bin."
            }
            Extra::LinkOutside => {
                "%@ está fora da sua pasta de jogos. Mova-o para lá para adicioná-lo."
            }
            Extra::LinkUnlock => "Selecione Desbloquear para preparar %@.",
            Extra::LinkUnlocked => "%@ já está desbloqueado.",
            Extra::ChannelTitle => "Canal",
            Extra::ChannelStable => "Estável",
            Extra::ChannelPinned => "Fixo",
            Extra::ChannelLocal => "Build local",
            Extra::ChannelStableDetail => "Acompanha a versão mais recente.",
            Extra::ChannelPinnedDetail => "Fica em uma só versão. Nenhuma atualização é oferecida.",
            Extra::ChannelLocalDetail => {
                "Usa uma pasta que você compilou ou escolheu. Nenhuma atualização é oferecida."
            }
            Extra::ChannelEngine => "Engine do AnyPlay",
            Extra::ChannelEngineDetail => {
                "O build noturno com jogo online. Muda com frequência e pode quebrar jogos."
            }
            Extra::EngineBuilt => "Compilado em %@",
            Extra::EngineAvailable => "Há um novo build da engine disponível",
            Extra::EngineNoBuild => {
                "Ainda não há build da engine para este sistema. Nada foi instalado. Escolha outro canal."
            }
            Extra::EngineUnverified => {
                "Não foi possível verificar o download da engine. Nada foi instalado. Tente mais tarde."
            }
            Extra::PinnedVersion => "Versão",
            Extra::WhatsNew => "Novidades",
            Extra::LastCheckedOn => "Última verificação: %@",
            Extra::FitNewer => {
                "Mais novo do que o testado com este launcher. Deve funcionar mesmo assim."
            }
            Extra::FitOlder => "Um Toolkit mais antigo: alguns recursos estão desativados.",
            Extra::ToolkitAvailable => "Toolkit %@ disponível",
            Extra::NeedsToolkit => "Requer o Toolkit %@",
            Extra::NeedsNewerThan => "Requer um build mais novo que %@",
            Extra::NotesHeader => "%@, lançada em %@",
            Extra::ChangesCount => "%@ mudanças",
            Extra::ChangeOne => "1 mudança",
            Extra::NotesHighlights => "Destaques",
            Extra::NotesOther => "Outras áreas",
            Extra::NotesEverything => "Tudo",
            Extra::NotesLoading => "Carregando as notas…",
            Extra::NotesUnavailable => "Não foi possível carregar as notas.",
            Extra::NotesOpenRelease => "Abrir no GitHub",
            Extra::UnlockedWith => "Desbloqueado com %@. A versão instalada é %@.",
            Extra::ConversionChanged => "Esta atualização muda como os jogos são convertidos.",
            Extra::UnlockOptional => "Desbloquear de novo é opcional.",
            Extra::UnlockEarlierBuild => "Desbloqueado com um build anterior deste Toolkit.",
            Extra::RecTitle => "Recomendado para este jogo",
            Extra::RecNeeded => "Necessário para iniciar",
            Extra::RecWorthALook => "Vale conferir",
            Extra::RecApply => "Aplicar",
            Extra::RecDefault => "Padrão",
            Extra::RecOn => "Ativado",
            Extra::RecOff => "Desativado",
            Extra::RecPlayGo => "Dados do PlayGo ausentes",
            Extra::RecPlayGoInstalled => "Informar como instalado",
            Extra::RecPlayGoAbsent => "Informar como ausente",
            Extra::RecImageBase => "Base da imagem",
            Extra::RecModuleFolder => "Pasta de módulos",
            Extra::RecExecutable => "Executável",
            Extra::RecTextureCache => "Cache de texturas",
            Extra::RecShaderCache => "Cache de shaders",
            Extra::RecUnlockHint => "Depois de aplicar, desbloqueie o jogo de novo.",
            Extra::RecApplied => "Aplicado",
            Extra::RecAppliedUnlock => "Salvo. Desbloqueie o jogo de novo para usar.",
            Extra::Played => "Tempo de jogo",
            Extra::PlayedUnderMinute => "Menos de 1 min",
            Extra::PlayedMinutes => "%@ min",
            Extra::PlayedHours => "%@ h",
            Extra::PlayedHoursMinutes => "%@ h %@ min",
            Extra::SavesTitle => "Jogos salvos",
            Extra::SavesNone => "Nenhum backup ainda",
            Extra::SavesOne => "1 backup",
            Extra::SavesCount => "%@ backups",
            Extra::SavesBackUp => "Fazer backup agora",
            Extra::SavesNothing => "Ainda não há jogos salvos para fazer backup",
            Extra::SavesEmpty => {
                "Nenhum backup ainda. Fazer backup agora guarda uma cópia dos seus jogos salvos e troféus."
            }
            Extra::SavesCaption => "Backups",
            Extra::GroupScreen => "Tela",
            Extra::GroupPlay => "Jogar",
            Extra::GroupMaintenance => "Manutenção",
            Extra::SavesKept => {
                "Até %@ backups são mantidos. Os feitos depois de jogar são removidos primeiro."
            }
            Extra::SavesReasonManual => "Manual",
            Extra::SavesReasonUnlock => "Antes de desbloquear de novo",
            Extra::SavesReasonRestore => "Antes de uma restauração",
            Extra::SavesReasonUninstall => "Antes de remover a instalação",
            Extra::SavesReasonPlayed => "Depois de jogar",
            Extra::SavesBackedUp => "Backup dos jogos salvos feito",
            Extra::SavesRestore => "Restaurar",
            Extra::SavesRestoreAsk => {
                "Isso substitui os jogos salvos e os troféus atuais do jogo pelo backup de %@. Os atuais passam por backup antes, então você pode desfazer."
            }
            Extra::SavesRestored => "Jogos salvos restaurados",
            Extra::SavesRunning => "Feche o jogo para restaurar um backup.",
            Extra::SavesFailed => {
                "Não foi possível fazer backup dos jogos salvos. Verifique o espaço livre em disco e tente de novo."
            }
            Extra::SavesRestoreFailed => {
                "Não foi possível restaurar o backup. Seus jogos salvos não foram alterados."
            }
            Extra::SavesCorrupt => {
                "Este backup está danificado. Seus jogos salvos não foram alterados. Escolha outro backup."
            }
            Extra::UninstallTitle => "Remover esta instalação?",
            Extra::SavesRestoreTitle => "Restaurar este backup?",
            Extra::UninstallBacksUp => "Seus jogos salvos e troféus passam por backup antes.",
            Extra::UninstallNeedsBackup => {
                "Não foi possível fazer backup dos jogos salvos, então o jogo não foi removido. Verifique o espaço livre em disco e tente de novo."
            }
            Extra::Uninstalling => "Removendo",
            Extra::SavesRestoreIncomplete => {
                "A restauração não terminou. Os jogos salvos de antes estão no backup mais recente, Antes de uma restauração."
            }
            Extra::SavesRestoreIncompleteNone => "A restauração não terminou. Tente de novo.",
            Extra::SavesBusy => {
                "Os jogos salvos ainda estão sendo processados. Tente de novo em instantes."
            }
            Extra::Cancelling => "Cancelando…",
            Extra::Pause => "Pausar",
            Extra::Pausing => "Pausando…",
            Extra::Resume => "Retomar",
            Extra::PausedNote => "Incompleto: o progresso não foi mantido. Retomar recomeça do início.",
            Extra::HeldNote => "Em pausa. Retomar continua daqui.",
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

    pub const ALL: [Extra; 167] = [
        Extra::Toolkit,
        Extra::UpdateCheck,
        Extra::UpdateCheckDetail,
        Extra::CheckNow,
        Extra::Checking,
        Extra::UpToDate,
        Extra::UpdateAvailable,
        Extra::CheckFailed,
        Extra::LastChecked,
        Extra::ToolkitMissing,
        Extra::ToolkitInstall,
        Extra::ToolkitInstallUpdate,
        Extra::ToolkitChooseFolder,
        Extra::CpuConversion,
        Extra::CpuConversionDetail,
        Extra::CpuAuto,
        Extra::CpuNone,
        Extra::CpuIntel,
        Extra::DiagnosisHostCpu,
        Extra::DiagnosisNoUdmabuf,
        Extra::SaveReport,
        Extra::OpenOnlineLog,
        Extra::SendReport,
        Extra::ToolkitDownloading,
        Extra::FontsBuiltIn,
        Extra::FontsCjkReady,
        Extra::FontsCjkMissing,
        Extra::FontsCjkDownloading,
        Extra::FontsDownloadCjk,
        Extra::ToolkitReady,
        Extra::ToolkitFailed,
        Extra::ToolkitFolderInvalid,
        Extra::HomeMusic,
        Extra::HomeMusicDetail,
        Extra::Vibration,
        Extra::VibrationDetail,
        Extra::VibrationLight,
        Extra::VibrationMedium,
        Extra::VibrationStrong,
        Extra::ControllerHelp,
        Extra::PadStatus,
        Extra::PadStatusBattery,
        Extra::PadStatusCharging,
        Extra::DiscordPresence,
        Extra::DiscordPresenceDetail,
        Extra::ShowHiddenTrophies,
        Extra::ShowHiddenTrophiesDetail,
        Extra::TrophyShow,
        Extra::TrophiesNone,
        Extra::EmptyLog,
        Extra::Layout,
        Extra::LayoutLauncher,
        Extra::LayoutMouse,
        Extra::LayoutGame,
        Extra::LayoutCustom,
        Extra::LayoutGameDetail,
        Extra::LauncherReady,
        Extra::LauncherToast,
        Extra::RestartNow,
        Extra::LauncherReadOnly,
        Extra::LauncherUpdateFailed,
        Extra::Links,
        Extra::LinksDetail,
        Extra::LinksDetailOther,
        Extra::LinksFailed,
        Extra::LinkInvalid,
        Extra::LinkNoGame,
        Extra::LinkNotADump,
        Extra::LinkOutside,
        Extra::LinkUnlock,
        Extra::LinkUnlocked,
        Extra::ChannelTitle,
        Extra::ChannelStable,
        Extra::ChannelPinned,
        Extra::ChannelLocal,
        Extra::ChannelStableDetail,
        Extra::ChannelPinnedDetail,
        Extra::ChannelLocalDetail,
        Extra::ChannelEngine,
        Extra::ChannelEngineDetail,
        Extra::EngineBuilt,
        Extra::EngineAvailable,
        Extra::EngineNoBuild,
        Extra::EngineUnverified,
        Extra::PinnedVersion,
        Extra::WhatsNew,
        Extra::LastCheckedOn,
        Extra::FitNewer,
        Extra::FitOlder,
        Extra::ToolkitAvailable,
        Extra::NeedsToolkit,
        Extra::NeedsNewerThan,
        Extra::NotesHeader,
        Extra::ChangesCount,
        Extra::ChangeOne,
        Extra::NotesHighlights,
        Extra::NotesOther,
        Extra::NotesEverything,
        Extra::NotesLoading,
        Extra::NotesUnavailable,
        Extra::NotesOpenRelease,
        Extra::UnlockedWith,
        Extra::ConversionChanged,
        Extra::UnlockOptional,
        Extra::UnlockEarlierBuild,
        Extra::RecTitle,
        Extra::RecNeeded,
        Extra::RecWorthALook,
        Extra::RecApply,
        Extra::RecDefault,
        Extra::RecOn,
        Extra::RecOff,
        Extra::RecPlayGo,
        Extra::RecPlayGoInstalled,
        Extra::RecPlayGoAbsent,
        Extra::RecImageBase,
        Extra::RecModuleFolder,
        Extra::RecExecutable,
        Extra::RecTextureCache,
        Extra::RecShaderCache,
        Extra::RecUnlockHint,
        Extra::RecApplied,
        Extra::RecAppliedUnlock,
        Extra::Played,
        Extra::PlayedUnderMinute,
        Extra::PlayedMinutes,
        Extra::PlayedHours,
        Extra::PlayedHoursMinutes,
        Extra::SavesTitle,
        Extra::SavesNone,
        Extra::SavesOne,
        Extra::SavesCount,
        Extra::SavesBackUp,
        Extra::SavesNothing,
        Extra::SavesEmpty,
        Extra::SavesCaption,
        Extra::GroupScreen,
        Extra::GroupPlay,
        Extra::GroupMaintenance,
        Extra::SavesKept,
        Extra::SavesReasonManual,
        Extra::SavesReasonUnlock,
        Extra::SavesReasonRestore,
        Extra::SavesReasonUninstall,
        Extra::SavesReasonPlayed,
        Extra::SavesBackedUp,
        Extra::SavesRestore,
        Extra::SavesRestoreAsk,
        Extra::SavesRestored,
        Extra::SavesRunning,
        Extra::SavesFailed,
        Extra::SavesRestoreFailed,
        Extra::SavesCorrupt,
        Extra::UninstallTitle,
        Extra::SavesRestoreTitle,
        Extra::UninstallBacksUp,
        Extra::UninstallNeedsBackup,
        Extra::Uninstalling,
        Extra::Cancelling,
        Extra::Pause,
        Extra::Pausing,
        Extra::Resume,
        Extra::PausedNote,
        Extra::HeldNote,
        Extra::SavesRestoreIncomplete,
        Extra::SavesRestoreIncompleteNone,
        Extra::SavesBusy,
    ];
}

/// The time played as the home screen shows it: under a minute, whole minutes below an hour, hours
/// and minutes below ten hours, then whole hours (rounded down, so it never claims more than was
/// played).
pub fn playtime_text(seconds: u64, language: Language) -> String {
    let minutes = seconds / 60;
    if minutes == 0 {
        return Extra::PlayedUnderMinute.text(language).to_owned();
    }
    let (hours, rest) = (minutes / 60, minutes % 60);
    match (hours, rest) {
        (0, minutes) => Extra::PlayedMinutes.format(language, &[Arg::Int(minutes as i64)]),
        (hours, 0) => Extra::PlayedHours.format(language, &[Arg::Int(hours as i64)]),
        (hours, rest) if hours < 10 => Extra::PlayedHoursMinutes
            .format(language, &[Arg::Int(hours as i64), Arg::Int(rest as i64)]),
        (hours, _) => Extra::PlayedHours.format(language, &[Arg::Int(hours as i64)]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_text_names_the_old_product() {
        for extra in Extra::ALL {
            for language in Language::ALL {
                let text = extra.text(language);
                assert!(!text.contains("AnyPS5"), "{extra:?} {language:?}: {text}");
            }
        }
    }

    #[test]
    fn every_text_exists_in_both_languages() {
        for extra in Extra::ALL {
            for language in Language::ALL {
                assert!(!extra.text(language).is_empty(), "{extra:?} {language:?}");
            }
        }
    }

    #[test]
    fn playtime_reads_calmly_in_both_languages() {
        let en = |s| playtime_text(s, Language::English);
        assert_eq!(en(0), "Under 1 min");
        assert_eq!(en(59), "Under 1 min");
        assert_eq!(en(60), "1 min");
        assert_eq!(en(45 * 60 + 59), "45 min");
        assert_eq!(en(3600), "1 h");
        assert_eq!(en(3 * 3600 + 20 * 60), "3 h 20 min");
        assert_eq!(en(9 * 3600 + 59 * 60 + 59), "9 h 59 min");
        // From ten hours the minutes are noise.
        assert_eq!(en(10 * 3600 + 59 * 60), "10 h");
        assert_eq!(en(12 * 3600 + 34 * 60), "12 h");
        assert_eq!(en(112 * 3600 + 1800), "112 h");
        assert_eq!(
            playtime_text(3 * 3600 + 20 * 60, Language::Turkish),
            "3 sa 20 dk"
        );
        assert_eq!(playtime_text(45 * 60, Language::Turkish), "45 dk");
        assert_eq!(playtime_text(5, Language::Turkish), "1 dk altı");
    }

    #[test]
    fn placeholders_match_between_languages() {
        for extra in Extra::ALL {
            assert_eq!(
                extra.english().matches("%@").count(),
                extra.turkish().matches("%@").count(),
                "{extra:?}"
            );
            for text in [
                extra.german(),
                extra.spanish(),
                extra.french(),
                extra.portuguese(),
            ] {
                assert_eq!(
                    extra.english().matches("%@").count(),
                    text.expect("translated").matches("%@").count(),
                    "{extra:?}"
                );
            }
        }
    }
}
