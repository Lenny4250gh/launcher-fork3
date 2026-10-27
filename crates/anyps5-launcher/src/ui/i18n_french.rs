// French texts for the core table (`i18n_table.rs`). A key without one falls back to English.
// Sentence case, not English Title Case (first word and proper names only); the player is
// addressed as "tu" in the imperative; no-break spaces (\u{a0}) before : ; ! ? % and in « » (docs/i18n/fr.md).
impl Key {
    /// The French text of a key; `None` falls back to English.
    #[allow(unreachable_patterns)]
    const fn french(self) -> Option<&'static str> {
        Some(match self {
            Key::Welcome => "Bienvenue sur AnyPlay",
            Key::WelcomeSubtitle => {
                "Joue à tes jeux sur PC et avec tes amis. Quelques étapes rapides d’abord."
            }
            Key::ChooseLanguage => "Langue",
            Key::ChooseTheme => "Choisis un thème",
            Key::ChooseThemeSubtitle => {
                "Tu pourras le changer plus tard dans les paramètres ou utiliser ta propre image ou vidéo."
            }
            Key::ChooseFolder => "Où sont tes jeux\u{a0}?",
            Key::ChooseFolderSubtitle => {
                "Choisis le dossier qui contient tes dumps de jeux. Les sous-dossiers sont aussi analysés et les nouveaux jeux apparaissent tout seuls."
            }
            Key::PickFolder => "Choisir le dossier",
            Key::ChangeFolder => "Changer de dossier",
            Key::GamesFound => "%d jeux trouvés",
            Key::NoGamesFound => {
                "Aucun jeu dans ce dossier. Choisis-en un autre ou ajoutes-y des dumps."
            }
            Key::ContinueAction => "Continuer",
            Key::Back => "Retour",
            Key::Start => "Démarrer",
            Key::Skip => "Passer",
            Key::Games => "Jeux",
            Key::Library => "Bibliothèque de jeux",
            Key::Settings => "Paramètres",
            Key::Play => "Jouer",
            Key::Unlock => "Déverrouiller",
            Key::Unlocking => "Déverrouillage",
            Key::Cancel => "Annuler",
            Key::Queued => "En attente",
            Key::Update => "Mettre à jour",
            Key::Retry => "Réessayer",
            Key::Options => "Options",
            Key::Locked => "Verrouillé",
            Key::Ready => "Prêt à jouer",
            Key::UpdateAvailable => "Peut être réinstallé avec la nouvelle version",
            Key::Failed => "Déverrouillage impossible",
            Key::Starting => "Démarrage…",
            Key::StartingText => {
                "Le jeu s’ouvre, patiente un instant. Le premier lancement peut prendre quelques minutes, le temps de préparer le jeu pour Windows."
            }
            Key::Running => "En jeu",
            Key::StepUnwrap => "Ouverture des fichiers du jeu",
            Key::StepRelink => "Conversion pour Windows",
            Key::StepCopy => "Préparation du jeu",
            Key::StepBundle => "Empaquetage",
            Key::StepFinish => "Dernières retouches",
            Key::TitleID => "ID du titre",
            Key::Version => "Version",
            Key::Size => "Taille",
            Key::Status => "État",
            Key::Folder => "Dossier",
            Key::GameSettings => "Paramètres du jeu",
            Key::Resolution => "Résolution",
            Key::ResolutionDefault => "Par défaut du jeu",
            Key::Display => "Affichage",
            Key::Windowed => "Fenêtre",
            Key::Fullscreen => "Plein écran",
            Key::WindowSize => "Taille de la fenêtre",
            Key::Vsync => "Synchronisation verticale",
            Key::On => "Activé",
            Key::Off => "Désactivé",
            Key::Overlay => "Indicateur de performances",
            Key::OverlayOff => "Désactivé",
            Key::OverlayTopLeft => "En haut à gauche",
            Key::OverlayTopRight => "En haut à droite",
            Key::OverlayBottomLeft => "En bas à gauche",
            Key::OverlayBottomRight => "En bas à droite",
            Key::GameLanguage => "Langue du jeu",
            Key::FollowLauncher => "Comme le launcher",
            Key::Reinstall => "Déverrouiller à nouveau",
            Key::ShowInFinder => "Afficher dans l’Explorateur",
            Key::OpenLog => "Ouvrir le journal",
            Key::Uninstall => "Supprimer l’installation",
            Key::ClearShaderCache => "Vider le cache des shaders",
            Key::UninstallConfirm => {
                "Tes fichiers de jeu sont conservés et le jeu apparaît de nouveau comme verrouillé."
            }
            Key::UninstallDone => "Installation supprimée",
            Key::CacheCleared => "Cache des shaders vidé",
            Key::Appearance => "Apparence",
            Key::Theme => "Thème",
            Key::Accent => "Couleur d’accentuation",
            Key::Background => "Arrière-plan",
            Key::BackgroundTheme => "Animation du thème",
            Key::BackgroundImage => "Mon image",
            Key::BackgroundVideo => "Ma vidéo",
            Key::ChooseFile => "Choisir un fichier",
            Key::GameArtwork => "Afficher l’illustration du jeu sélectionné",
            Key::GlassWindow => "Fenêtre en verre",
            Key::Motion => "Animations",
            Key::MotionFull => "Complètes",
            Key::MotionReduced => "Réduites",
            Key::Sounds => "Sons de l’interface",
            Key::Language => "Langue",
            Key::Storage => "Bibliothèque",
            Key::Rescan => "Analyser à nouveau",
            Key::InstalledSize => {
                "Les jeux déverrouillés se trouvent à côté de tes dumps et partagent leurs fichiers quand le disque le permet. Choisis un autre dossier pour les garder sur un autre disque."
            }
            Key::About => "À propos",
            Key::AboutText => "AnyPlay Launcher lance tes jeux avec AnyPS5 sous Windows.",
            Key::ToolkitVersion => "Toolkit",
            Key::ProfileName => "Nom du profil",
            Key::Behaviour => "Comportement",
            Key::HideWhilePlaying => "Masquer le launcher pendant le jeu",
            Key::RosettaMissing => "Les jeux fonctionnent via Rosetta, que ce Mac n’a pas encore.",
            Key::RosettaInstall => "Installer Rosetta",
            Key::RosettaInstalled => "Rosetta est installé",
            Key::MoveToApplications => "Déplacer AnyPS5 dans Applications",
            Key::MoveToApplicationsText => {
                "macOS ouvre les apps téléchargées depuis une copie verrouillée en lecture seule. Autorise le déplacement dans Applications et la réouverture, pour que cela ne te gêne plus."
            }
            Key::MoveNow => "Déplacer et rouvrir",
            Key::Later => "Plus tard",
            Key::NoToolkit => {
                "Cette copie n’a pas d’outils de conversion. Utilise l’installateur de la version publiée."
            }
            Key::EmptyLibraryTitle => "Aucun jeu pour l’instant",
            Key::EmptyLibraryText => {
                "Place des dumps de jeux dans ton dossier de jeux et ils apparaîtront ici tout seuls."
            }
            Key::Scanning => "Recherche de jeux",
            Key::AllGames => "Tous les jeux",
            Key::SortName => "Par nom",
            Key::PlayTime => "Dernière partie",
            Key::ThemeCosmic => "Cosmique",
            Key::ThemeAurora => "Aurore",
            Key::ThemeSunset => "Crépuscule",
            Key::HintMove => "Déplacer",
            Key::HintSelect => "Sélectionner",
            Key::HintBack => "Retour",
            Key::HintOptions => "Options",
            Key::UnlockFailed => "Déverrouillage impossible",
            Key::StartFailed => "Le jeu n’a pas pu démarrer",
            Key::GameEnded => "Le jeu s’est fermé",
            Key::LogTitle => "Journal",
            Key::Close => "Fermer",
            Key::Quit => "Quitter",
            Key::Confirm => "Confirmer",
            Key::UnreadableDump => "Ce dump est chiffré ou endommagé et ne peut pas être ouvert.",
            Key::Controls => "Commandes",
            Key::ControlsHint => {
                "Choisis une ligne, puis appuie sur la touche voulue. Échap annule. Une manette connectée garde ses propres boutons."
            }
            Key::PressKey => "Appuie sur une touche…",
            Key::ResetControls => "Rétablir les touches par défaut",
            Key::NoKey => "Aucune",
            Key::Dpad => "Croix directionnelle",
            Key::LeftStick => "Stick gauche",
            Key::RightStick => "Stick droit",
            Key::TouchpadLeft => "Pavé tactile (gauche)",
            Key::TouchpadRight => "Pavé tactile (droite)",
            Key::DirectionUp => "Haut",
            Key::DirectionDown => "Bas",
            Key::DirectionLeft => "Gauche",
            Key::DirectionRight => "Droite",
            Key::KeySpace => "Espace",
            Key::KeyEnter => "Entrée",
            Key::MouseLeft => "Clic gauche",
            Key::MouseRight => "Clic droit",
            Key::MouseMiddle => "Clic central",
            Key::MouseBack => "Souris 4",
            Key::MouseForward => "Souris 5",
            Key::WheelUp => "Molette ↑",
            Key::WheelDown => "Molette ↓",
            Key::Trophies => "Trophées",
            Key::TrophyHidden => "Trophée caché",
            Key::TrophyHiddenDetail => "Il se révélera quand tu l’obtiendras.",
            Key::TrophyLocked => "Non obtenu",
            Key::GradePlatinum => "Platine",
            Key::GradeGold => "Or",
            Key::GradeSilver => "Argent",
            Key::GradeBronze => "Bronze",
            Key::Advanced => "Avancé",
            Key::AdvancedDetail => "Sticks, souris, clavier, cache, partage",
            Key::AdvancedTitle => "Paramètres avancés",
            Key::MakeReport => "Créer un rapport",
            Key::MakeReportDetail => "À joindre sur Discord ou GitHub",
            Key::ScreenshotsFolder => "Dossier des captures",
            Key::ShowScreenshots => "Afficher les captures",
            Key::FontsFolder => "Dossier des polices système",
            Key::FontsNone => "Aucun",
            Key::FontsReset => "Retirer le dossier des polices",
            Key::FontsHint => {
                "Les jeux qui demandent les polices système de la console utilisent les fichiers de ce dossier."
            }
            Key::SectionSticks => "Sticks de la manette",
            Key::LeftDeadzone => "Zone morte du stick gauche",
            Key::RightDeadzone => "Zone morte du stick droit",
            Key::DeadzoneDetail => {
                "Part de la course considérée comme centrée\u{a0}; évite qu’un stick dérive"
            }
            Key::SectionMouse => "Souris comme caméra (bouton central ou F7)",
            Key::MouseSensitivity => "Sensibilité de la souris",
            Key::MouseSensitivityDetail => "Plus haut, on tourne à fond avec moins de mouvement",
            Key::MouseVertical => "Vitesse verticale",
            Key::MouseVerticalDetail => {
                "Accélère le regard vers le haut et le bas par rapport à la rotation"
            }
            Key::MouseCurve => "Courbe de réponse",
            Key::MouseCurveDetail => "Au-dessus de 1, plus de précision à basse vitesse",
            Key::MouseMinimum => "Inclinaison minimale",
            Key::MouseMinimumDetail => "Augmente-la si un mouvement lent ne bouge pas la caméra",
            Key::MouseMaxSpeed => "Vitesse maximale",
            Key::MouseMaxSpeedDetail => {
                "Garde un geste rapide à pleine inclinaison un peu plus longtemps"
            }
            Key::MouseSmoothing => "Lissage",
            Key::MouseSmoothingDetail => {
                "Lisse les mouvements saccadés\u{a0}; désactivé, chaque relevé est suivi"
            }
            Key::MouseDecay => "Recentrer après",
            Key::MouseDecayDetail => "Délai avant l’arrêt de la caméra une fois la souris immobile",
            Key::SectionKeyboard => "Clavier",
            Key::StickModifierScale => "Amplitude du mouvement lent",
            Key::StickModifierScaleDetail => {
                "Inclinaison des sticks tant que le mouvement lent est maintenu (à définir dans Commandes)"
            }
            Key::KeyboardRamp => "Montée du stick",
            Key::KeyboardRampDetail => "Temps qu’une touche met à incliner le stick à fond",
            Key::TriggerRamp => "Montée de la gâchette",
            Key::TriggerRampDetail => {
                "Temps qu’une touche met à enfoncer L2/R2 à fond, pour les jeux de course"
            }
            Key::SectionGraphics => "Graphismes",
            Key::ShaderCache => "Cache des shaders",
            Key::ShaderCacheDetail => "Désactivé, les shaders sont recompilés à chaque démarrage",
            Key::SectionShare => "Partage",
            Key::ExportSettings => "Exporter les paramètres",
            Key::ExportSettingsDetail => "Touches et tous les paramètres dans un fichier",
            Key::ImportSettings => "Importer les paramètres",
            Key::ImportSettingsDetail => "Applique un fichier de paramètres partagé",
            Key::ResetAdvanced => "Réinitialiser les paramètres avancés",
            Key::ReportSaved => "Rapport enregistré",
            Key::ReportFailed => {
                "Le rapport n’a pas pu être enregistré. Vérifie qu’il reste de la place sur le disque et réessaie."
            }
            Key::SettingsExported => "Paramètres exportés",
            Key::SettingsImported => "Paramètres importés",
            Key::SettingsImportedOther => "Paramètres de %@ importés",
            Key::SettingsFileFailed => "Ce n’est pas un fichier de paramètres AnyPlay",
            Key::StickModifier => "Mouvement lent",
            Key::DiagnosisMissingModule => {
                "Module manquant\u{a0}: %@. Déverrouiller à nouveau le jeu peut régler le problème."
            }
            Key::DiagnosisUnsupported => {
                "Pas encore pris en charge. Envoie un rapport via Options > Journal pour qu’il soit ajouté. Fonction\u{a0}: %@"
            }
            Key::DiagnosisGpuLost => {
                "Le GPU ne répond plus. Mets à jour ton pilote graphique et relance le jeu."
            }
            Key::DiagnosisOutOfMemory => {
                "Le jeu a manqué de mémoire. Ferme d’autres programmes et relance le jeu."
            }
            Key::DiagnosisEngineError => {
                "Erreur du moteur. Relance le jeu. Si cela continue, envoie un rapport via Options > Journal. Détails\u{a0}: %@"
            }
            Key::DiagnosisCrashed => {
                "Le jeu a planté. Relance-le. Si cela continue, envoie un rapport via Options > Journal. (%@)"
            }
            Key::TrophyEarned => "Trophée obtenu",
            _ => return None,
        })
    }
}
