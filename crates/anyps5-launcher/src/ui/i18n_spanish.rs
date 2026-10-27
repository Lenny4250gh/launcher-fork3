// Spanish texts for the core table (`i18n_table.rs`). A key without one falls back to English.
// Sentence case, not English Title Case (first word and proper names only); neutral Spanish,
// the player is addressed as "tú" in the imperative (docs/i18n/es.md).
impl Key {
    /// The Spanish text of a key; `None` falls back to English.
    #[allow(unreachable_patterns)]
    const fn spanish(self) -> Option<&'static str> {
        Some(match self {
            Key::Welcome => "Te damos la bienvenida a AnyPlay",
            Key::WelcomeSubtitle => {
                "Juega tus juegos en el PC y con tus amigos. Antes, unos pasos rápidos."
            }
            Key::ChooseLanguage => "Idioma",
            Key::ChooseTheme => "Elige un tema",
            Key::ChooseThemeSubtitle => {
                "Puedes cambiarlo después en Ajustes o usar tu propia imagen o video."
            }
            Key::ChooseFolder => "¿Dónde están tus juegos?",
            Key::ChooseFolderSubtitle => {
                "Elige la carpeta con tus dumps de juegos. También se buscan las subcarpetas y los juegos nuevos aparecen solos."
            }
            Key::PickFolder => "Elegir carpeta",
            Key::ChangeFolder => "Cambiar carpeta",
            Key::GamesFound => "%d juegos encontrados",
            Key::NoGamesFound => "No hay juegos en esta carpeta. Elige otra o añade dumps.",
            Key::ContinueAction => "Continuar",
            Key::Back => "Atrás",
            Key::Start => "Iniciar",
            Key::Skip => "Omitir",
            Key::Games => "Juegos",
            Key::Library => "Biblioteca de juegos",
            Key::Settings => "Ajustes",
            Key::Play => "Jugar",
            Key::Unlock => "Desbloquear",
            Key::Unlocking => "Desbloqueando",
            Key::Cancel => "Cancelar",
            Key::Queued => "En cola",
            Key::Update => "Actualizar",
            Key::Retry => "Reintentar",
            Key::Options => "Opciones",
            Key::Locked => "Bloqueado",
            Key::Ready => "Listo para jugar",
            Key::UpdateAvailable => "Se puede volver a instalar con la nueva versión",
            Key::Failed => "No se pudo desbloquear",
            Key::Starting => "Iniciando…",
            Key::StartingText => {
                "El juego se está abriendo, espera un momento. El primer inicio puede tardar unos minutos mientras el juego se prepara para Windows."
            }
            Key::Running => "Jugando",
            Key::StepUnwrap => "Abriendo los archivos del juego",
            Key::StepRelink => "Convirtiendo para Windows",
            Key::StepCopy => "Preparando el juego",
            Key::StepBundle => "Empaquetando",
            Key::StepFinish => "Últimos retoques",
            Key::TitleID => "ID del título",
            Key::Version => "Versión",
            Key::Size => "Tamaño",
            Key::Status => "Estado",
            Key::Folder => "Carpeta",
            Key::GameSettings => "Ajustes del juego",
            Key::Resolution => "Resolución",
            Key::ResolutionDefault => "Predeterminada del juego",
            Key::Display => "Pantalla",
            Key::Windowed => "Ventana",
            Key::Fullscreen => "Pantalla completa",
            Key::WindowSize => "Tamaño de ventana",
            Key::Vsync => "Sincronización vertical",
            Key::On => "Activado",
            Key::Off => "Desactivado",
            Key::Overlay => "Indicador de rendimiento",
            Key::OverlayOff => "Desactivado",
            Key::OverlayTopLeft => "Arriba a la izquierda",
            Key::OverlayTopRight => "Arriba a la derecha",
            Key::OverlayBottomLeft => "Abajo a la izquierda",
            Key::OverlayBottomRight => "Abajo a la derecha",
            Key::GameLanguage => "Idioma del juego",
            Key::FollowLauncher => "Igual que el launcher",
            Key::Reinstall => "Desbloquear de nuevo",
            Key::ShowInFinder => "Mostrar en el Explorador",
            Key::OpenLog => "Abrir registro",
            Key::Uninstall => "Quitar instalación",
            Key::ClearShaderCache => "Vaciar caché de shaders",
            Key::UninstallConfirm => {
                "Tus archivos del juego se conservan y el juego vuelve a aparecer como bloqueado."
            }
            Key::UninstallDone => "Instalación quitada",
            Key::CacheCleared => "Caché de shaders vaciada",
            Key::Appearance => "Apariencia",
            Key::Theme => "Tema",
            Key::Accent => "Color de énfasis",
            Key::Background => "Fondo",
            Key::BackgroundTheme => "Animación del tema",
            Key::BackgroundImage => "Mi imagen",
            Key::BackgroundVideo => "Mi video",
            Key::ChooseFile => "Elegir archivo",
            Key::GameArtwork => "Mostrar la ilustración del juego seleccionado",
            Key::GlassWindow => "Ventana de cristal",
            Key::Motion => "Animaciones",
            Key::MotionFull => "Completas",
            Key::MotionReduced => "Reducidas",
            Key::Sounds => "Sonidos de la interfaz",
            Key::Language => "Idioma",
            Key::Storage => "Biblioteca",
            Key::Rescan => "Buscar de nuevo",
            Key::InstalledSize => {
                "Los juegos desbloqueados se guardan junto a tus dumps y comparten sus archivos si la unidad lo permite. Elige otra carpeta para tenerlos en otra unidad."
            }
            Key::About => "Acerca de",
            Key::AboutText => "AnyPlay Launcher inicia tus juegos con AnyPS5 en Windows.",
            Key::ToolkitVersion => "Toolkit",
            Key::ProfileName => "Nombre del perfil",
            Key::Behaviour => "Comportamiento",
            Key::HideWhilePlaying => "Ocultar el launcher mientras juegas",
            Key::RosettaMissing => "Los juegos se ejecutan con Rosetta, que este Mac aún no tiene.",
            Key::RosettaInstall => "Instalar Rosetta",
            Key::RosettaInstalled => "Rosetta está instalado",
            Key::MoveToApplications => "Mover AnyPS5 a Aplicaciones",
            Key::MoveToApplicationsText => {
                "macOS abre las apps descargadas desde una copia bloqueada y de solo lectura. Permite moverla a Aplicaciones y volver a abrirla para que no vuelva a estorbar."
            }
            Key::MoveNow => "Mover y volver a abrir",
            Key::Later => "Más tarde",
            Key::NoToolkit => {
                "Esta copia no incluye herramientas de conversión. Usa el instalador de la versión publicada."
            }
            Key::EmptyLibraryTitle => "Aún no hay juegos",
            Key::EmptyLibraryText => {
                "Pon dumps de juegos en tu carpeta de juegos y aparecerán aquí solos."
            }
            Key::Scanning => "Buscando juegos",
            Key::AllGames => "Todos los juegos",
            Key::SortName => "Por nombre",
            Key::PlayTime => "Jugado por última vez",
            Key::ThemeCosmic => "Cósmico",
            Key::ThemeAurora => "Aurora",
            Key::ThemeSunset => "Atardecer",
            Key::HintMove => "Mover",
            Key::HintSelect => "Seleccionar",
            Key::HintBack => "Atrás",
            Key::HintOptions => "Opciones",
            Key::UnlockFailed => "No se pudo desbloquear",
            Key::StartFailed => "El juego no se pudo iniciar",
            Key::GameEnded => "El juego se cerró",
            Key::LogTitle => "Registro",
            Key::Close => "Cerrar",
            Key::Quit => "Salir",
            Key::Confirm => "Confirmar",
            Key::UnreadableDump => "Este dump está cifrado o dañado y no se puede abrir.",
            Key::Controls => "Controles",
            Key::ControlsHint => {
                "Elige una fila y pulsa la tecla que quieras asignarle. Esc cancela. Un mando conectado mantiene sus propios botones."
            }
            Key::PressKey => "Pulsa una tecla…",
            Key::ResetControls => "Restablecer teclas predeterminadas",
            Key::NoKey => "Ninguna",
            Key::Dpad => "Cruceta",
            Key::LeftStick => "Stick izquierdo",
            Key::RightStick => "Stick derecho",
            Key::TouchpadLeft => "Panel táctil (izquierda)",
            Key::TouchpadRight => "Panel táctil (derecha)",
            Key::DirectionUp => "Arriba",
            Key::DirectionDown => "Abajo",
            Key::DirectionLeft => "Izquierda",
            Key::DirectionRight => "Derecha",
            Key::KeySpace => "Espacio",
            Key::KeyEnter => "Intro",
            Key::MouseLeft => "Clic izquierdo",
            Key::MouseRight => "Clic derecho",
            Key::MouseMiddle => "Clic central",
            Key::MouseBack => "Ratón 4",
            Key::MouseForward => "Ratón 5",
            Key::WheelUp => "Rueda ↑",
            Key::WheelDown => "Rueda ↓",
            Key::Trophies => "Trofeos",
            Key::TrophyHidden => "Trofeo oculto",
            Key::TrophyHiddenDetail => "Mostrará qué es cuando lo consigas.",
            Key::TrophyLocked => "No conseguido",
            Key::GradePlatinum => "Platino",
            Key::GradeGold => "Oro",
            Key::GradeSilver => "Plata",
            Key::GradeBronze => "Bronce",
            Key::Advanced => "Avanzado",
            Key::AdvancedDetail => "Sticks, ratón, teclado, caché, compartir",
            Key::AdvancedTitle => "Ajustes avanzados",
            Key::MakeReport => "Crear informe",
            Key::MakeReportDetail => "Para adjuntar en Discord o GitHub",
            Key::ScreenshotsFolder => "Carpeta de capturas",
            Key::ShowScreenshots => "Mostrar capturas",
            Key::FontsFolder => "Carpeta de fuentes del sistema",
            Key::FontsNone => "Ninguna",
            Key::FontsReset => "Quitar la carpeta de fuentes",
            Key::FontsHint => {
                "Los juegos que piden las fuentes del sistema de la consola usan los archivos de esta carpeta."
            }
            Key::SectionSticks => "Sticks del mando",
            Key::LeftDeadzone => "Zona muerta del stick izquierdo",
            Key::RightDeadzone => "Zona muerta del stick derecho",
            Key::DeadzoneDetail => {
                "Cuánto del recorrido cuenta como centro; evita que un stick se desvíe solo"
            }
            Key::SectionMouse => "Ratón como cámara (botón central o F7)",
            Key::MouseSensitivity => "Sensibilidad del ratón",
            Key::MouseSensitivityDetail => "Más alta gira del todo con menos movimiento",
            Key::MouseVertical => "Velocidad vertical",
            Key::MouseVerticalDetail => "Acelera mirar arriba y abajo frente a girar",
            Key::MouseCurve => "Curva de respuesta",
            Key::MouseCurveDetail => "Por encima de 1 da más precisión a baja velocidad",
            Key::MouseMinimum => "Inclinación mínima",
            Key::MouseMinimumDetail => "Súbela si un movimiento lento no mueve la cámara",
            Key::MouseMaxSpeed => "Velocidad máxima",
            Key::MouseMaxSpeedDetail => {
                "Mantiene un movimiento rápido a inclinación máxima un poco más"
            }
            Key::MouseSmoothing => "Suavizado",
            Key::MouseSmoothingDetail => {
                "Suaviza el movimiento irregular; desactivado sigue cada lectura"
            }
            Key::MouseDecay => "Centrar tras",
            Key::MouseDecayDetail => "Cuánto tarda la cámara en parar después de que pare el ratón",
            Key::SectionKeyboard => "Teclado",
            Key::StickModifierScale => "Intensidad del movimiento lento",
            Key::StickModifierScaleDetail => {
                "Cuánto se inclinan los sticks mientras mantienes el movimiento lento (se asigna en Controles)"
            }
            Key::KeyboardRamp => "Rampa del stick",
            Key::KeyboardRampDetail => "Cuánto tarda una tecla en inclinar el stick del todo",
            Key::TriggerRamp => "Rampa del gatillo",
            Key::TriggerRampDetail => {
                "Cuánto tarda una tecla en pulsar L2/R2 del todo, para juegos de carreras"
            }
            Key::SectionGraphics => "Gráficos",
            Key::ShaderCache => "Caché de shaders",
            Key::ShaderCacheDetail => {
                "Si está desactivada, los shaders se compilan de nuevo en cada inicio"
            }
            Key::SectionShare => "Compartir",
            Key::ExportSettings => "Exportar ajustes",
            Key::ExportSettingsDetail => "Teclas y todos los ajustes en un archivo",
            Key::ImportSettings => "Importar ajustes",
            Key::ImportSettingsDetail => "Aplica un archivo de ajustes compartido",
            Key::ResetAdvanced => "Restablecer ajustes avanzados",
            Key::ReportSaved => "Informe guardado",
            Key::ReportFailed => {
                "No se pudo guardar el informe. Comprueba que haya espacio en la unidad e inténtalo de nuevo."
            }
            Key::SettingsExported => "Ajustes exportados",
            Key::SettingsImported => "Ajustes importados",
            Key::SettingsImportedOther => "Ajustes de %@ importados",
            Key::SettingsFileFailed => "Este no es un archivo de ajustes de AnyPlay",
            Key::StickModifier => "Movimiento lento",
            Key::DiagnosisMissingModule => {
                "Falta un módulo: %@. Desbloquear el juego de nuevo puede solucionarlo."
            }
            Key::DiagnosisUnsupported => {
                "Aún no es compatible. Envía un informe desde Opciones > Registro para que se pueda añadir. Función: %@"
            }
            Key::DiagnosisGpuLost => {
                "La GPU dejó de responder. Actualiza el controlador de la tarjeta gráfica y vuelve a iniciar el juego."
            }
            Key::DiagnosisOutOfMemory => {
                "El juego se quedó sin memoria. Cierra otros programas y vuelve a iniciar el juego."
            }
            Key::DiagnosisEngineError => {
                "Error del motor. Vuelve a iniciar el juego. Si sigue pasando, envía un informe desde Opciones > Registro. Detalles: %@"
            }
            Key::DiagnosisCrashed => {
                "El juego se cerró por un error. Vuelve a iniciarlo. Si sigue pasando, envía un informe desde Opciones > Registro. (%@)"
            }
            Key::TrophyEarned => "Trofeo conseguido",
            _ => return None,
        })
    }
}
