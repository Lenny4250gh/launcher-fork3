// German texts for the core table (`i18n_table.rs`). A key without one falls back to English.
// Capitalisation follows German rules (nouns and sentence starts), not English Title Case;
// the player is addressed as "du" (docs/DESIGN.md section 8).
impl Key {
    /// The German text of a key; `None` falls back to English.
    #[allow(unreachable_patterns)]
    const fn german(self) -> Option<&'static str> {
        Some(match self {
            Key::Welcome => "Willkommen bei AnyPlay",
            Key::WelcomeSubtitle => {
                "Spiele deine Spiele am PC und mit deinen Freunden. Vorher noch ein paar kurze Schritte."
            }
            Key::ChooseLanguage => "Sprache",
            Key::ChooseTheme => "Design wählen",
            Key::ChooseThemeSubtitle => {
                "Du kannst es später in den Einstellungen ändern oder ein eigenes Bild oder Video nutzen."
            }
            Key::ChooseFolder => "Wo sind deine Spiele?",
            Key::ChooseFolderSubtitle => {
                "Wähle den Ordner mit deinen Spiele-Dumps. Unterordner werden mit durchsucht, neue Spiele erscheinen von selbst."
            }
            Key::PickFolder => "Ordner wählen",
            Key::ChangeFolder => "Ordner ändern",
            Key::GamesFound => "%d Spiele gefunden",
            Key::NoGamesFound => {
                "In diesem Ordner sind keine Spiele. Wähle einen anderen oder lege Dumps hinein."
            }
            Key::ContinueAction => "Weiter",
            Key::Back => "Zurück",
            Key::Start => "Starten",
            Key::Skip => "Überspringen",
            Key::Games => "Spiele",
            Key::Library => "Spielebibliothek",
            Key::Settings => "Einstellungen",
            Key::Play => "Spielen",
            Key::Unlock => "Freischalten",
            Key::Unlocking => "Wird freigeschaltet",
            Key::Cancel => "Abbrechen",
            Key::Queued => "In der Warteschlange",
            Key::Update => "Aktualisieren",
            Key::Retry => "Erneut versuchen",
            Key::Options => "Optionen",
            Key::Locked => "Gesperrt",
            Key::Ready => "Bereit zum Spielen",
            Key::UpdateAvailable => "Kann mit der neuen Version erneut installiert werden",
            Key::Failed => "Freischalten fehlgeschlagen",
            Key::Starting => "Wird gestartet …",
            Key::StartingText => {
                "Das Spiel öffnet sich, bitte warte. Der erste Start kann ein paar Minuten dauern, während das Spiel für Windows vorbereitet wird."
            }
            Key::Running => "Läuft",
            Key::StepUnwrap => "Spieldateien werden geöffnet",
            Key::StepRelink => "Für Windows umwandeln",
            Key::StepCopy => "Spiel wird vorbereitet",
            Key::StepBundle => "Wird verpackt",
            Key::StepFinish => "Letzte Handgriffe",
            Key::TitleID => "Titel-ID",
            Key::Version => "Version",
            Key::Size => "Größe",
            Key::Status => "Status",
            Key::Folder => "Ordner",
            Key::GameSettings => "Spieleinstellungen",
            Key::Resolution => "Auflösung",
            Key::ResolutionDefault => "Standard des Spiels",
            Key::Display => "Anzeige",
            Key::Windowed => "Fenster",
            Key::Fullscreen => "Vollbild",
            Key::WindowSize => "Fenstergröße",
            Key::Vsync => "Vertikale Synchronisation",
            Key::On => "An",
            Key::Off => "Aus",
            Key::Overlay => "Leistungsanzeige",
            Key::OverlayOff => "Aus",
            Key::OverlayTopLeft => "Oben links",
            Key::OverlayTopRight => "Oben rechts",
            Key::OverlayBottomLeft => "Unten links",
            Key::OverlayBottomRight => "Unten rechts",
            Key::GameLanguage => "Spielsprache",
            Key::FollowLauncher => "Wie im Launcher",
            Key::Reinstall => "Erneut freischalten",
            Key::ShowInFinder => "Im Explorer zeigen",
            Key::OpenLog => "Protokoll öffnen",
            Key::Uninstall => "Installation entfernen",
            Key::ClearShaderCache => "Shader-Cache leeren",
            Key::UninstallConfirm => {
                "Deine Spieldateien bleiben erhalten, das Spiel erscheint wieder als gesperrt."
            }
            Key::UninstallDone => "Installation entfernt",
            Key::CacheCleared => "Shader-Cache geleert",
            Key::Appearance => "Darstellung",
            Key::Theme => "Design",
            Key::Accent => "Akzentfarbe",
            Key::Background => "Hintergrund",
            Key::BackgroundTheme => "Design-Animation",
            Key::BackgroundImage => "Mein Bild",
            Key::BackgroundVideo => "Mein Video",
            Key::ChooseFile => "Datei wählen",
            Key::GameArtwork => "Artwork des gewählten Spiels zeigen",
            Key::GlassWindow => "Glasfenster",
            Key::Motion => "Animationen",
            Key::MotionFull => "Voll",
            Key::MotionReduced => "Reduziert",
            Key::Sounds => "Oberflächen-Sounds",
            Key::Language => "Sprache",
            Key::Storage => "Bibliothek",
            Key::Rescan => "Erneut suchen",
            Key::InstalledSize => {
                "Freigeschaltete Spiele liegen neben deinen Dumps und teilen sich ihre Dateien, wo das Laufwerk es erlaubt. Wähle einen anderen Ordner, um sie auf einem anderen Laufwerk zu halten."
            }
            Key::About => "Über",
            Key::AboutText => "AnyPlay Launcher startet deine Spiele mit AnyPS5 unter Windows.",
            Key::ToolkitVersion => "Toolkit",
            Key::ProfileName => "Profilname",
            Key::Behaviour => "Verhalten",
            Key::HideWhilePlaying => "Launcher beim Spielen ausblenden",
            Key::RosettaMissing => {
                "Spiele laufen über Rosetta, das auf diesem Mac noch nicht installiert ist."
            }
            Key::RosettaInstall => "Rosetta installieren",
            Key::RosettaInstalled => "Rosetta ist installiert",
            Key::MoveToApplications => "AnyPS5 nach Programme verschieben",
            Key::MoveToApplicationsText => {
                "macOS öffnet heruntergeladene Apps aus einer gesperrten, schreibgeschützten Kopie. Erlaube das Verschieben nach Programme und das erneute Öffnen, damit es nie wieder stört."
            }
            Key::MoveNow => "Verschieben und neu öffnen",
            Key::Later => "Später",
            Key::NoToolkit => {
                "Diese Kopie hat keine Umwandlungswerkzeuge. Nutze den Installer aus dem Release."
            }
            Key::EmptyLibraryTitle => "Noch keine Spiele",
            Key::EmptyLibraryText => {
                "Lege Spiele-Dumps in deinen Spieleordner, dann erscheinen sie hier von selbst."
            }
            Key::Scanning => "Suche nach Spielen",
            Key::AllGames => "Alle Spiele",
            Key::SortName => "Nach Name",
            Key::PlayTime => "Zuletzt gespielt",
            Key::ThemeCosmic => "Kosmos",
            Key::ThemeAurora => "Aurora",
            Key::ThemeSunset => "Sonnenuntergang",
            Key::HintMove => "Bewegen",
            Key::HintSelect => "Auswählen",
            Key::HintBack => "Zurück",
            Key::HintOptions => "Optionen",
            Key::UnlockFailed => "Freischalten fehlgeschlagen",
            Key::StartFailed => "Das Spiel konnte nicht starten",
            Key::GameEnded => "Das Spiel wurde beendet",
            Key::LogTitle => "Protokoll",
            Key::Close => "Schließen",
            Key::Quit => "Beenden",
            Key::Confirm => "Bestätigen",
            Key::UnreadableDump => {
                "Dieser Dump ist verschlüsselt oder beschädigt und lässt sich nicht öffnen."
            }
            Key::Controls => "Steuerung",
            Key::ControlsHint => {
                "Wähle eine Zeile und drücke die gewünschte Taste. Esc bricht ab. Ein verbundener Controller behält seine eigenen Tasten."
            }
            Key::PressKey => "Taste drücken …",
            Key::ResetControls => "Auf Standardtasten zurücksetzen",
            Key::NoKey => "Keine",
            Key::Dpad => "Steuerkreuz",
            Key::LeftStick => "Linker Stick",
            Key::RightStick => "Rechter Stick",
            Key::TouchpadLeft => "Touchpad (links)",
            Key::TouchpadRight => "Touchpad (rechts)",
            Key::DirectionUp => "Hoch",
            Key::DirectionDown => "Runter",
            Key::DirectionLeft => "Links",
            Key::DirectionRight => "Rechts",
            Key::KeySpace => "Leertaste",
            Key::KeyEnter => "Eingabe",
            Key::MouseLeft => "Linksklick",
            Key::MouseRight => "Rechtsklick",
            Key::MouseMiddle => "Mittelklick",
            Key::MouseBack => "Maus 4",
            Key::MouseForward => "Maus 5",
            Key::WheelUp => "Rad ↑",
            Key::WheelDown => "Rad ↓",
            Key::Trophies => "Trophäen",
            Key::TrophyHidden => "Versteckte Trophäe",
            Key::TrophyHiddenDetail => "Zeigt erst, was es ist, wenn du sie erhältst.",
            Key::TrophyLocked => "Nicht erhalten",
            Key::GradePlatinum => "Platin",
            Key::GradeGold => "Gold",
            Key::GradeSilver => "Silber",
            Key::GradeBronze => "Bronze",
            Key::Advanced => "Erweitert",
            Key::AdvancedDetail => "Sticks, Maus, Tastatur, Cache, Teilen",
            Key::AdvancedTitle => "Erweiterte Einstellungen",
            Key::MakeReport => "Bericht erstellen",
            Key::MakeReportDetail => "Zum Anhängen auf Discord oder GitHub",
            Key::ScreenshotsFolder => "Screenshot-Ordner",
            Key::ShowScreenshots => "Screenshots zeigen",
            Key::FontsFolder => "Ordner für Systemschriften",
            Key::FontsNone => "Keiner",
            Key::FontsReset => "Schriftenordner entfernen",
            Key::FontsHint => {
                "Spiele, die die Systemschriften der Konsole anfordern, nutzen die Dateien in diesem Ordner."
            }
            Key::SectionSticks => "Controller-Sticks",
            Key::LeftDeadzone => "Totzone linker Stick",
            Key::RightDeadzone => "Totzone rechter Stick",
            Key::DeadzoneDetail => {
                "Wie viel des Wegs als Mitte gilt; verhindert einen driftenden Stick"
            }
            Key::SectionMouse => "Maus als Kamera (mittlere Taste oder F7)",
            Key::MouseSensitivity => "Mausempfindlichkeit",
            Key::MouseSensitivityDetail => "Höher dreht mit weniger Bewegung voll",
            Key::MouseVertical => "Vertikale Geschwindigkeit",
            Key::MouseVerticalDetail => {
                "Beschleunigt Hoch- und Runterschauen gegenüber dem Drehen"
            }
            Key::MouseCurve => "Ansprechkurve",
            Key::MouseCurveDetail => "Über 1 steuert sich bei niedriger Geschwindigkeit feiner",
            Key::MouseMinimum => "Mindestausschlag",
            Key::MouseMinimumDetail => {
                "Erhöhe es, wenn langsame Bewegung die Kamera nicht bewegt"
            }
            Key::MouseMaxSpeed => "Höchstgeschwindigkeit",
            Key::MouseMaxSpeedDetail => {
                "Hält einen schnellen Ruck etwas länger auf vollem Ausschlag"
            }
            Key::MouseSmoothing => "Glättung",
            Key::MouseSmoothingDetail => "Glättet zittrige Bewegung; aus folgt jeder Meldung",
            Key::MouseDecay => "Mitte nach",
            Key::MouseDecayDetail => "Wie bald die Kamera stoppt, nachdem die Maus stoppt",
            Key::SectionKeyboard => "Tastatur",
            Key::StickModifierScale => "Stärke der langsamen Bewegung",
            Key::StickModifierScaleDetail => {
                "Wie weit die Sticks ausschlagen, solange die langsame Bewegung gehalten wird (in Steuerung festlegen)"
            }
            Key::KeyboardRamp => "Stick-Anlauf",
            Key::KeyboardRampDetail => {
                "Wie lange eine Taste braucht, um den Stick voll auszulenken"
            }
            Key::TriggerRamp => "Trigger-Anlauf",
            Key::TriggerRampDetail => {
                "Wie lange eine Taste braucht, um L2/R2 voll zu drücken, für Rennspiele"
            }
            Key::SectionGraphics => "Grafik",
            Key::ShaderCache => "Shader-Cache",
            Key::ShaderCacheDetail => "Wenn aus, werden Shader bei jedem Start neu kompiliert",
            Key::SectionShare => "Teilen",
            Key::ExportSettings => "Einstellungen exportieren",
            Key::ExportSettingsDetail => "Tasten und alle Einstellungen in einer Datei",
            Key::ImportSettings => "Einstellungen importieren",
            Key::ImportSettingsDetail => "Übernimmt eine geteilte Einstellungsdatei",
            Key::ResetAdvanced => "Erweiterte Einstellungen zurücksetzen",
            Key::ReportSaved => "Bericht gespeichert",
            Key::ReportFailed => {
                "Der Bericht konnte nicht gespeichert werden. Prüfe, ob auf dem Laufwerk Platz ist, und versuche es erneut."
            }
            Key::SettingsExported => "Einstellungen exportiert",
            Key::SettingsImported => "Einstellungen importiert",
            Key::SettingsImportedOther => "Einstellungen von %@ importiert",
            Key::SettingsFileFailed => "Das ist keine AnyPlay-Einstellungsdatei",
            Key::StickModifier => "Langsame Bewegung",
            Key::DiagnosisMissingModule => {
                "Fehlendes Modul: %@. Erneutes Freischalten des Spiels kann helfen."
            }
            Key::DiagnosisUnsupported => {
                "Noch nicht unterstützt. Sende einen Bericht über Optionen > Protokoll, damit es ergänzt werden kann. Funktion: %@"
            }
            Key::DiagnosisGpuLost => {
                "Die GPU reagiert nicht mehr. Aktualisiere deinen Grafiktreiber und starte das Spiel erneut."
            }
            Key::DiagnosisOutOfMemory => {
                "Dem Spiel ging der Speicher aus. Schließe andere Programme und starte das Spiel erneut."
            }
            Key::DiagnosisEngineError => {
                "Engine-Fehler. Starte das Spiel erneut. Passiert es weiter, sende einen Bericht über Optionen > Protokoll. Details: %@"
            }
            Key::DiagnosisCrashed => {
                "Das Spiel ist abgestürzt. Starte es erneut. Passiert es weiter, sende einen Bericht über Optionen > Protokoll. (%@)"
            }
            Key::TrophyEarned => "Trophäe erhalten",
            _ => return None,
        })
    }
}
