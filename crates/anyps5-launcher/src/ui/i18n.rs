//! Ported from utkuhalis/AnyPS5-ARM (Localization.swift, GPL-2.0-only) and changed since; see README "Credits".
//! Localisation (docs/DESIGN.md section 8): English, Turkish, German, Spanish, French and Brazilian
//! Portuguese, 213 keys generated from the
//! macOS launcher's `Localization.swift` by `tools/gen-i18n.mjs`, with the Windows replacements
//! of section 9.3 already applied.
//!
//! Lookup is `current language table, then English, then the key name`. Because both tables are
//! complete `match`es the fallbacks can never fire; they are kept in [`Language::text`] only as the
//! documented contract.

include!("i18n_table.rs");
include!("i18n_german.rs");
include!("i18n_spanish.rs");
include!("i18n_french.rs");
include!("i18n_portuguese.rs");

/// Interface languages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Language {
    Turkish,
    #[default]
    English,
    German,
    Spanish,
    French,
    /// Brazilian Portuguese (`pt-BR`).
    Portuguese,
}

impl Language {
    pub const ALL: [Language; 6] = [
        Language::German,
        Language::English,
        Language::Spanish,
        Language::French,
        Language::Portuguese,
        Language::Turkish,
    ];

    /// Preference value (`"tr"`, `"en"`, `"de"`, `"es"`, `"fr"`, `"pt"`).
    pub const fn code(self) -> &'static str {
        match self {
            Language::Turkish => "tr",
            Language::English => "en",
            Language::German => "de",
            Language::Spanish => "es",
            Language::French => "fr",
            Language::Portuguese => "pt",
        }
    }

    pub fn from_code(code: &str) -> Option<Language> {
        match code {
            "tr" => Some(Language::Turkish),
            "en" => Some(Language::English),
            "de" => Some(Language::German),
            "es" => Some(Language::Spanish),
            "fr" => Some(Language::French),
            "pt" => Some(Language::Portuguese),
            _ => None,
        }
    }

    /// Name in its own language.
    pub const fn display_name(self) -> &'static str {
        match self {
            Language::Turkish => "Türkçe",
            Language::English => "English",
            Language::German => "Deutsch",
            Language::Spanish => "Español",
            Language::French => "Français",
            Language::Portuguese => "Português (Brasil)",
        }
    }

    /// The console language a game starts in when its own setting follows the launcher (Spain's
    /// Spanish; [`Language::console_language_for`] picks Latin America's from the system locale).
    pub const fn console_language(self) -> u32 {
        match self {
            Language::Turkish => 19,
            Language::English => 1,
            Language::German => 4,
            Language::Spanish => 3,
            Language::French => 2,
            Language::Portuguese => 17,
        }
    }

    /// [`Language::console_language`] with Latin American Spanish (20) for a Latin American system
    /// locale. The one place that mirrors `anyps5_core::settings::Language::console_language_for`.
    pub fn console_language_for(self, system_locale: &str) -> u32 {
        let core = match self {
            Language::Turkish => anyps5_core::settings::Language::Turkish,
            Language::English => anyps5_core::settings::Language::English,
            Language::German => anyps5_core::settings::Language::German,
            Language::Spanish => anyps5_core::settings::Language::Spanish,
            Language::French => anyps5_core::settings::Language::French,
            Language::Portuguese => anyps5_core::settings::Language::Portuguese,
        };
        u32::try_from(core.console_language_for(system_locale)).unwrap_or(1)
    }

    /// The language of the OS UI (`tr`, `de`, `es`, `fr`, `pt`), else English.
    pub fn system() -> Language {
        match sys_locale::get_locale() {
            Some(locale) => Language::from_locale(&locale),
            None => Language::English,
        }
    }

    /// The language a locale tag (`de-AT`, `tr_TR`, `pt-BR`) asks for; English for any other.
    pub fn from_locale(tag: &str) -> Language {
        match anyps5_core::settings::Language::from_locale(tag) {
            anyps5_core::settings::Language::Turkish => Language::Turkish,
            anyps5_core::settings::Language::English => Language::English,
            anyps5_core::settings::Language::German => Language::German,
            anyps5_core::settings::Language::Spanish => Language::Spanish,
            anyps5_core::settings::Language::French => Language::French,
            anyps5_core::settings::Language::Portuguese => Language::Portuguese,
        }
    }

    /// True when the language writes the decimal separator as a comma (`8,39`), false for the
    /// point (`8.39`). Every language but English.
    pub const fn decimal_comma(self) -> bool {
        !matches!(self, Language::English)
    }

    /// Looks a key up in this language.
    pub const fn text(self, key: Key) -> &'static str {
        if !cfg!(windows)
            && let Some(text) = elsewhere(self, key)
        {
            return text;
        }
        if let Some(text) = ours(self, key) {
            return text;
        }
        match self {
            Language::Turkish => key.turkish(),
            Language::English => key.english(),
            Language::German => match key.german() {
                Some(text) => text,
                None => key.english(),
            },
            Language::Spanish => match key.spanish() {
                Some(text) => text,
                None => key.english(),
            },
            Language::French => match key.french() {
                Some(text) => text,
                None => key.english(),
            },
            Language::Portuguese => match key.portuguese() {
                Some(text) => text,
                None => key.english(),
            },
        }
    }

    /// Looks a key up and substitutes `%d` / `%@` in order. `%%` yields a percent sign.
    pub fn format(self, key: Key, args: &[Arg<'_>]) -> String {
        substitute(self.text(key), args)
    }

    /// Locale tag for dates (`en-US`, `tr-TR`, `de-DE`, `es-ES`, `fr-FR`, `pt-BR`).
    pub const fn locale(self) -> &'static str {
        match self {
            Language::Turkish => "tr-TR",
            Language::English => "en-US",
            Language::German => "de-DE",
            Language::Spanish => "es-ES",
            Language::French => "fr-FR",
            Language::Portuguese => "pt-BR",
        }
    }
}

/// The OS UI locale tag (`pt-BR`), empty when it cannot be read.
pub fn system_locale() -> String {
    sys_locale::get_locale().unwrap_or_default()
}

/// Strings AnyPlay words its own way, in place of the generated table's: they speak of the
/// player's games and game dumps, never of a platform's name.
const fn ours(language: Language, key: Key) -> Option<&'static str> {
    Some(match (language, key) {
        (Language::English, Key::WelcomeSubtitle) => {
            "Play your games on your PC, and with your friends. A few quick steps first."
        }
        (Language::English, Key::ChooseFolderSubtitle) => {
            "Choose the folder that holds your game dumps. Subfolders are scanned too, and new games show up on their own."
        }
        (Language::English, Key::EmptyLibraryText) => {
            "Put game dumps in your games folder and they show up here on their own."
        }
        (Language::English, Key::AboutText) => {
            "AnyPlay Launcher runs your games on Windows with AnyPS5."
        }
        (Language::Turkish, Key::WelcomeSubtitle) => {
            "Oyunlarını PC'nde ve arkadaşlarınla oyna. Önce birkaç kısa adım."
        }
        (Language::Turkish, Key::ChooseFolderSubtitle) => {
            "Oyun dump'larının olduğu klasörü seç. Alt klasörleri de tararım, yeni oyunları kendim bulurum."
        }
        (Language::Turkish, Key::EmptyLibraryText) => {
            "Oyun klasörüne oyun dump'larını koy, burada kendiliğinden görünsünler."
        }
        (Language::Turkish, Key::AboutText) => {
            "AnyPlay Launcher oyunlarını AnyPS5 ile Windows'ta çalıştırır."
        }
        (Language::English, Key::DiagnosisUnsupported) => {
            "Not supported yet. Send a report from Options > Log so it can be added. Function: %@"
        }
        (Language::English, Key::DiagnosisGpuLost) => {
            "The GPU stopped responding. Update your graphics driver, then start the game again."
        }
        (Language::English, Key::DiagnosisOutOfMemory) => {
            "The game ran out of memory. Close other programs, then start the game again."
        }
        (Language::English, Key::DiagnosisEngineError) => {
            "Engine error. Start the game again. If it keeps happening, send a report from Options > Log. Details: %@"
        }
        (Language::English, Key::DiagnosisCrashed) => {
            "The game crashed. Start the game again. If it keeps happening, send a report from Options > Log. (%@)"
        }
        (Language::English, Key::ReportFailed) => {
            "Could not save the report. Check that the disk has space, then try again."
        }
        (Language::Turkish, Key::DiagnosisUnsupported) => {
            "Henüz desteklenmiyor. Eklenebilmesi için Seçenekler > Günlük'ten rapor gönder. Fonksiyon: %@"
        }
        (Language::Turkish, Key::DiagnosisGpuLost) => {
            "GPU ile bağlantı koptu. Grafik sürücünü güncelle, sonra oyunu yeniden başlat."
        }
        (Language::Turkish, Key::DiagnosisOutOfMemory) => {
            "Bellek yetersiz kaldı. Diğer programları kapat, sonra oyunu yeniden başlat."
        }
        (Language::Turkish, Key::DiagnosisEngineError) => {
            "Motor hatası. Oyunu yeniden başlat. Tekrarlarsa Seçenekler > Günlük'ten rapor gönder. Ayrıntı: %@"
        }
        (Language::Turkish, Key::DiagnosisCrashed) => {
            "Oyun çöktü. Oyunu yeniden başlat. Tekrarlarsa Seçenekler > Günlük'ten rapor gönder. (%@)"
        }
        (Language::Turkish, Key::ReportFailed) => {
            "Rapor kaydedilemedi. Diskte yer olduğunu kontrol edip yeniden dene."
        }
        (Language::English, Key::NoGamesFound) => {
            "No games in this folder. Choose another, or add dumps to it."
        }
        (Language::Turkish, Key::NoGamesFound) => {
            "Bu klasörde oyun yok. Başka bir klasör seç ya da içine oyun dump'ları ekle."
        }
        _ => return None,
    })
}

/// The few strings that name the system. The table is generated with Windows in them; on every
/// other system these take their place.
const fn elsewhere(language: Language, key: Key) -> Option<&'static str> {
    if cfg!(target_os = "macos") {
        return match (language, key) {
            (Language::English, Key::StartingText) => Some(
                "The game is opening, please wait. The first start can take a few minutes while Rosetta translates the game and its shaders compile.",
            ),
            (Language::English, Key::StepRelink) => Some("Converting for macOS"),
            (Language::English, Key::ShowInFinder) => Some("Show in Finder"),
            (Language::English, Key::AboutText) => {
                Some("AnyPlay Launcher runs your games on macOS with AnyPS5, Rosetta and MoltenVK.")
            }
            (Language::Turkish, Key::StartingText) => Some(
                "Oyun açılıyor, lütfen bekle. İlk açılış, Rosetta oyunu çevirirken ve gölgelendiriciler derlenirken birkaç dakika sürebilir.",
            ),
            (Language::Turkish, Key::StepRelink) => Some("macOS için çevriliyor"),
            (Language::Turkish, Key::ShowInFinder) => Some("Finder’da göster"),
            (Language::Turkish, Key::AboutText) => Some(
                "AnyPlay Launcher oyunlarını AnyPS5, Rosetta ve MoltenVK ile macOS'ta çalıştırır.",
            ),
            (Language::Spanish, Key::StartingText) => Some(
                "El juego se está abriendo, espera un momento. El primer inicio puede tardar unos minutos mientras Rosetta traduce el juego y se compilan los shaders.",
            ),
            (Language::Spanish, Key::StepRelink) => Some("Convirtiendo para macOS"),
            (Language::Spanish, Key::ShowInFinder) => Some("Mostrar en el Finder"),
            (Language::Spanish, Key::AboutText) => {
                Some("AnyPlay Launcher inicia tus juegos en macOS con AnyPS5, Rosetta y MoltenVK.")
            }
            (Language::French, Key::StartingText) => Some(
                "Le jeu s’ouvre, patiente un instant. Le premier lancement peut prendre quelques minutes, le temps que Rosetta traduise le jeu et que les shaders soient compilés.",
            ),
            (Language::French, Key::StepRelink) => Some("Conversion pour macOS"),
            (Language::French, Key::ShowInFinder) => Some("Afficher dans le Finder"),
            (Language::French, Key::AboutText) => {
                Some("AnyPlay Launcher lance tes jeux sous macOS avec AnyPS5, Rosetta et MoltenVK.")
            }
            (Language::Portuguese, Key::StartingText) => Some(
                "O jogo está abrindo, aguarde. O primeiro início pode levar alguns minutos enquanto o Rosetta traduz o jogo e os shaders são compilados.",
            ),
            (Language::Portuguese, Key::StepRelink) => Some("Convertendo para o macOS"),
            (Language::Portuguese, Key::ShowInFinder) => Some("Mostrar no Finder"),
            (Language::Portuguese, Key::AboutText) => Some(
                "O AnyPlay Launcher inicia seus jogos no macOS com o AnyPS5, Rosetta e MoltenVK.",
            ),
            _ => None,
        };
    }
    Some(match (language, key) {
        (Language::English, Key::StartingText) => {
            "The game is opening, please wait. The first start can take a few minutes while the game gets ready for Linux."
        }
        (Language::English, Key::StepRelink) => "Converting for Linux",
        (Language::English, Key::ShowInFinder) => "Show in file manager",
        (Language::English, Key::AboutText) => {
            "AnyPlay Launcher runs your games on Linux with AnyPS5."
        }
        (Language::Turkish, Key::StartingText) => {
            "Oyun açılıyor, lütfen bekle. İlk açılış, oyun Linux için hazırlanırken birkaç dakika sürebilir."
        }
        (Language::Turkish, Key::StepRelink) => "Linux için çevriliyor",
        (Language::Turkish, Key::ShowInFinder) => "Dosya yöneticisinde göster",
        (Language::Turkish, Key::AboutText) => {
            "AnyPlay Launcher oyunlarını AnyPS5 ile Linux'ta çalıştırır."
        }
        (Language::Spanish, Key::StartingText) => {
            "El juego se está abriendo, espera un momento. El primer inicio puede tardar unos minutos mientras el juego se prepara para Linux."
        }
        (Language::Spanish, Key::StepRelink) => "Convirtiendo para Linux",
        (Language::Spanish, Key::ShowInFinder) => "Mostrar en el gestor de archivos",
        (Language::Spanish, Key::AboutText) => {
            "AnyPlay Launcher inicia tus juegos en Linux con AnyPS5."
        }
        (Language::French, Key::StartingText) => {
            "Le jeu s’ouvre, patiente un instant. Le premier lancement peut prendre quelques minutes, le temps de préparer le jeu pour Linux."
        }
        (Language::French, Key::StepRelink) => "Conversion pour Linux",
        (Language::French, Key::ShowInFinder) => "Afficher dans le gestionnaire de fichiers",
        (Language::French, Key::AboutText) => {
            "AnyPlay Launcher lance tes jeux sous Linux avec AnyPS5."
        }
        (Language::Portuguese, Key::StartingText) => {
            "O jogo está abrindo, aguarde. O primeiro início pode levar alguns minutos enquanto o jogo é preparado para o Linux."
        }
        (Language::Portuguese, Key::StepRelink) => "Convertendo para o Linux",
        (Language::Portuguese, Key::ShowInFinder) => "Mostrar no gerenciador de arquivos",
        (Language::Portuguese, Key::AboutText) => {
            "O AnyPlay Launcher inicia seus jogos no Linux com o AnyPS5."
        }
        _ => return None,
    })
}

/// One format argument.
#[derive(Clone, Copy, Debug)]
pub enum Arg<'a> {
    Int(i64),
    Str(&'a str),
}

impl From<i64> for Arg<'_> {
    fn from(value: i64) -> Self {
        Arg::Int(value)
    }
}

impl From<usize> for Arg<'_> {
    fn from(value: usize) -> Self {
        Arg::Int(value as i64)
    }
}

impl<'a> From<&'a str> for Arg<'a> {
    fn from(value: &'a str) -> Self {
        Arg::Str(value)
    }
}

/// Replaces `%d` and `%@` placeholders with `args` in order.
pub fn substitute(template: &str, args: &[Arg<'_>]) -> String {
    let mut out = String::with_capacity(template.len() + 16);
    let mut next = args.iter();
    let mut chars = template.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '%' {
            out.push(c);
            continue;
        }
        match chars.peek() {
            Some('d') | Some('@') => {
                chars.next();
                match next.next() {
                    Some(Arg::Int(v)) => out.push_str(&v.to_string()),
                    Some(Arg::Str(s)) => out.push_str(s),
                    None => {}
                }
            }
            Some('%') => {
                chars.next();
                out.push('%');
            }
            _ => out.push('%'),
        }
    }
    out
}

impl Key {
    /// Keys that only make sense on macOS (Rosetta, move to Applications). They stay in the table
    /// for completeness but no Windows screen shows them.
    pub const fn is_macos_only(self) -> bool {
        matches!(
            self,
            Key::RosettaMissing
                | Key::RosettaInstall
                | Key::RosettaInstalled
                | Key::MoveToApplications
                | Key::MoveToApplicationsText
                | Key::MoveNow
                | Key::Later
        )
    }
}

/// Decimal-unit file size (1 GB = 10^9 bytes) with the language's decimal separator, e.g. `8,39 GB`
/// in Turkish and `8.39 GB` in English. Negative input means "not measured yet" and yields an ellipsis.
pub fn byte_text(bytes: i64, language: Language) -> String {
    if bytes < 0 {
        return "…".to_owned();
    }
    // French counts in octets (o, Ko, Mo, Go, To); the others keep the English letters.
    let units: [&str; 5] = if language == Language::French {
        ["octets", "Ko", "Mo", "Go", "To"]
    } else {
        ["bytes", "KB", "MB", "GB", "TB"]
    };
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1000.0 && unit < units.len() - 1 {
        value /= 1000.0;
        unit += 1;
    }
    if unit == 0 {
        return format!("{bytes} {}", units[0]);
    }
    let text = if value >= 100.0 {
        format!("{value:.0}")
    } else if value >= 10.0 {
        format!("{value:.1}")
    } else {
        format!("{value:.2}")
    };
    let text = if language.decimal_comma() {
        text.replace('.', ",")
    } else {
        text
    };
    format!("{text} {}", units[unit])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_213_keys_exist_in_both_languages() {
        assert_eq!(Key::ALL.len(), 213);
        for key in Key::ALL {
            for language in Language::ALL {
                assert!(
                    !language.text(key).is_empty(),
                    "{} is empty in {language:?}",
                    key.name()
                );
            }
        }
    }

    #[cfg(not(any(windows, target_os = "macos")))]
    #[test]
    fn system_names_follow_the_system() {
        assert_eq!(
            Language::English.text(Key::StepRelink),
            "Converting for Linux"
        );
        assert_eq!(
            Language::Turkish.text(Key::ShowInFinder),
            "Dosya yöneticisinde göster"
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn macos_names_do_not_claim_linux_or_windows_support() {
        assert_eq!(
            Language::English.text(Key::StepRelink),
            "Converting for macOS"
        );
        assert_eq!(Language::English.text(Key::ShowInFinder), "Show in Finder");
    }

    #[cfg(windows)]
    #[test]
    fn windows_replacements_are_applied() {
        assert_eq!(
            Language::English.text(Key::WelcomeSubtitle),
            "Play your games on your PC, and with your friends. A few quick steps first."
        );
        assert_eq!(
            Language::English.text(Key::StepRelink),
            "Converting for Windows"
        );
        assert_eq!(
            Language::Turkish.text(Key::StepRelink),
            "Windows için çevriliyor"
        );
        assert_eq!(
            Language::English.text(Key::ShowInFinder),
            "Show in Explorer"
        );
        assert_eq!(
            Language::English.text(Key::AboutText),
            "AnyPlay Launcher runs your games on Windows with AnyPS5."
        );
    }

    #[test]
    fn no_visible_string_uses_the_old_name() {
        for key in Key::ALL {
            if key.is_macos_only() {
                continue;
            }
            for language in Language::ALL {
                let text = language.text(key);
                assert!(
                    !text.contains("AnyPS5 Launcher"),
                    "{} ({language:?}): {text}",
                    key.name()
                );
                // AnyPS5 is still the engine (the About text) and the store folder of games.
                if !matches!(key, Key::AboutText | Key::InstalledSize) {
                    assert!(
                        !text.contains("AnyPS5"),
                        "{} ({language:?}) names AnyPS5: {text}",
                        key.name()
                    );
                }
            }
        }
    }

    #[test]
    fn no_visible_string_names_the_platform() {
        for key in Key::ALL {
            for language in Language::ALL {
                let text = language.text(key).replace("AnyPS5", "");
                assert!(
                    !text.contains("PS5") && !text.contains("PlayStation"),
                    "{} ({language:?}) names the platform: {text}",
                    key.name()
                );
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn no_visible_string_still_names_the_mac() {
        for key in Key::ALL {
            if key.is_macos_only() {
                continue;
            }
            for language in Language::ALL {
                let text = language.text(key);
                for word in ["Mac", "Finder", "Rosetta", "APFS", "MoltenVK"] {
                    assert!(
                        !text.contains(word),
                        "{} ({language:?}) still says {word}: {text}",
                        key.name()
                    );
                }
            }
        }
    }

    #[test]
    fn format_substitutes_in_order() {
        assert_eq!(
            Language::English.format(Key::GamesFound, &[3usize.into()]),
            "3 games found"
        );
        assert_eq!(
            Language::Turkish.format(Key::GamesFound, &[3usize.into()]),
            "3 oyun bulundu"
        );
        assert_eq!(
            Language::English.format(Key::DiagnosisMissingModule, &["libX.dll".into()]),
            "Missing module: libX.dll. Unlocking the game again may fix it."
        );
        assert_eq!(substitute("100%% of %d", &[Arg::Int(7)]), "100% of 7");
    }

    #[test]
    fn every_core_key_has_a_text_in_every_translated_language() {
        for key in Key::ALL {
            let placeholders = |text: &str| text.matches("%@").count() + text.matches("%d").count();
            for (language, text) in [
                ("German", key.german()),
                ("Spanish", key.spanish()),
                ("French", key.french()),
                ("Portuguese", key.portuguese()),
            ] {
                let text = text.unwrap_or_else(|| panic!("{} has no {language} text", key.name()));
                assert!(!text.trim().is_empty(), "{} ({language})", key.name());
                assert_eq!(
                    placeholders(text),
                    placeholders(key.english()),
                    "{} ({language}) changes its placeholders",
                    key.name()
                );
            }
        }
    }

    #[test]
    fn the_system_locale_picks_the_language() {
        assert_eq!(Language::from_locale("de-AT"), Language::German);
        assert_eq!(Language::from_locale("DE_CH"), Language::German);
        assert_eq!(Language::from_locale("tr-TR"), Language::Turkish);
        assert_eq!(Language::from_locale("es-MX"), Language::Spanish);
        assert_eq!(Language::from_locale("fr_CA"), Language::French);
        assert_eq!(Language::from_locale("pt-BR"), Language::Portuguese);
        assert_eq!(Language::from_locale("pt-PT"), Language::Portuguese);
        assert_eq!(Language::from_locale("it-IT"), Language::English);
        assert_eq!(
            Language::German.format(Key::GamesFound, &[3usize.into()]),
            "3 Spiele gefunden"
        );
        assert_eq!(
            Language::Spanish.format(Key::GamesFound, &[3usize.into()]),
            "3 juegos encontrados"
        );
        assert_eq!(
            Language::French.format(Key::GamesFound, &[3usize.into()]),
            "3 jeux trouvés"
        );
        assert_eq!(
            Language::Portuguese.format(Key::GamesFound, &[3usize.into()]),
            "3 jogos encontrados"
        );
    }

    #[test]
    fn console_languages_match_the_platform_numbers() {
        assert_eq!(Language::Spanish.console_language(), 3);
        assert_eq!(Language::French.console_language(), 2);
        assert_eq!(Language::Portuguese.console_language(), 17);
        assert_eq!(Language::Spanish.console_language_for("es-MX"), 20);
        assert_eq!(Language::Spanish.console_language_for("es-ES"), 3);
        for language in Language::ALL {
            let core = crate::screens::convert::language_to_core(language);
            assert_eq!(
                i64::from(language.console_language()),
                i64::from(core.console_language()),
                "{language:?}"
            );
        }
    }

    #[test]
    fn language_codes_round_trip() {
        for language in Language::ALL {
            assert_eq!(Language::from_code(language.code()), Some(language));
        }
        assert_eq!(Language::from_code("de"), Some(Language::German));
        assert_eq!(Language::from_code("es"), Some(Language::Spanish));
        assert_eq!(Language::from_code("fr"), Some(Language::French));
        assert_eq!(Language::from_code("pt"), Some(Language::Portuguese));
        assert_eq!(Language::from_code("xx"), None);
    }

    #[test]
    fn byte_text_uses_decimal_units() {
        assert_eq!(byte_text(8_390_000_000, Language::Turkish), "8,39 GB");
        assert_eq!(byte_text(8_390_000_000, Language::German), "8,39 GB");
        assert_eq!(byte_text(8_390_000_000, Language::Spanish), "8,39 GB");
        assert_eq!(byte_text(8_390_000_000, Language::Portuguese), "8,39 GB");
        assert_eq!(byte_text(8_390_000_000, Language::French), "8,39 Go");
        assert_eq!(byte_text(999, Language::French), "999 octets");
        assert_eq!(byte_text(8_390_000_000, Language::English), "8.39 GB");
        assert_eq!(byte_text(-1, Language::English), "…");
        assert_eq!(byte_text(999, Language::English), "999 bytes");
        assert_eq!(byte_text(12_300_000, Language::English), "12.3 MB");
    }
}
