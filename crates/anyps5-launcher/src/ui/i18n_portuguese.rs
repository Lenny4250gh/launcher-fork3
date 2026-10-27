// Brazilian Portuguese texts for the core table (`i18n_table.rs`). A key without one falls back to English.
// Sentence case, not English Title Case (first word and proper names only); the player is
// addressed as "você" in the imperative of "você" (docs/i18n/pt-BR.md).
impl Key {
    /// The Brazilian Portuguese text of a key; `None` falls back to English.
    #[allow(unreachable_patterns)]
    const fn portuguese(self) -> Option<&'static str> {
        Some(match self {
            Key::Welcome => "Boas-vindas ao AnyPlay",
            Key::WelcomeSubtitle => {
                "Jogue seus jogos no PC e com seus amigos. Antes, alguns passos rápidos."
            }
            Key::ChooseLanguage => "Idioma",
            Key::ChooseTheme => "Escolha um tema",
            Key::ChooseThemeSubtitle => {
                "Você pode mudar depois em Configurações ou usar sua própria imagem ou vídeo."
            }
            Key::ChooseFolder => "Onde estão seus jogos?",
            Key::ChooseFolderSubtitle => {
                "Escolha a pasta com seus dumps de jogos. As subpastas também são verificadas e os jogos novos aparecem sozinhos."
            }
            Key::PickFolder => "Escolher pasta",
            Key::ChangeFolder => "Mudar pasta",
            Key::GamesFound => "%d jogos encontrados",
            Key::NoGamesFound => "Nenhum jogo nesta pasta. Escolha outra ou adicione dumps a ela.",
            Key::ContinueAction => "Continuar",
            Key::Back => "Voltar",
            Key::Start => "Iniciar",
            Key::Skip => "Pular",
            Key::Games => "Jogos",
            Key::Library => "Biblioteca de jogos",
            Key::Settings => "Configurações",
            Key::Play => "Jogar",
            Key::Unlock => "Desbloquear",
            Key::Unlocking => "Desbloqueando",
            Key::Cancel => "Cancelar",
            Key::Queued => "Na fila",
            Key::Update => "Atualizar",
            Key::Retry => "Tentar de novo",
            Key::Options => "Opções",
            Key::Locked => "Bloqueado",
            Key::Ready => "Pronto para jogar",
            Key::UpdateAvailable => "Pode ser instalado de novo com a nova versão",
            Key::Failed => "Não foi possível desbloquear",
            Key::Starting => "Iniciando…",
            Key::StartingText => {
                "O jogo está abrindo, aguarde. O primeiro início pode levar alguns minutos enquanto o jogo é preparado para o Windows."
            }
            Key::Running => "Jogando",
            Key::StepUnwrap => "Abrindo os arquivos do jogo",
            Key::StepRelink => "Convertendo para o Windows",
            Key::StepCopy => "Preparando o jogo",
            Key::StepBundle => "Empacotando",
            Key::StepFinish => "Últimos ajustes",
            Key::TitleID => "ID do título",
            Key::Version => "Versão",
            Key::Size => "Tamanho",
            Key::Status => "Status",
            Key::Folder => "Pasta",
            Key::GameSettings => "Configurações do jogo",
            Key::Resolution => "Resolução",
            Key::ResolutionDefault => "Padrão do jogo",
            Key::Display => "Tela",
            Key::Windowed => "Janela",
            Key::Fullscreen => "Tela cheia",
            Key::WindowSize => "Tamanho da janela",
            Key::Vsync => "Sincronização vertical",
            Key::On => "Ativado",
            Key::Off => "Desativado",
            Key::Overlay => "Indicador de desempenho",
            Key::OverlayOff => "Desativado",
            Key::OverlayTopLeft => "Canto superior esquerdo",
            Key::OverlayTopRight => "Canto superior direito",
            Key::OverlayBottomLeft => "Canto inferior esquerdo",
            Key::OverlayBottomRight => "Canto inferior direito",
            Key::GameLanguage => "Idioma do jogo",
            Key::FollowLauncher => "Igual ao launcher",
            Key::Reinstall => "Desbloquear de novo",
            Key::ShowInFinder => "Mostrar no Explorador",
            Key::OpenLog => "Abrir registro",
            Key::Uninstall => "Remover instalação",
            Key::ClearShaderCache => "Limpar cache de shaders",
            Key::UninstallConfirm => {
                "Os arquivos do seu jogo são mantidos e o jogo volta a aparecer como bloqueado."
            }
            Key::UninstallDone => "Instalação removida",
            Key::CacheCleared => "Cache de shaders limpo",
            Key::Appearance => "Aparência",
            Key::Theme => "Tema",
            Key::Accent => "Cor de destaque",
            Key::Background => "Plano de fundo",
            Key::BackgroundTheme => "Animação do tema",
            Key::BackgroundImage => "Minha imagem",
            Key::BackgroundVideo => "Meu vídeo",
            Key::ChooseFile => "Escolher arquivo",
            Key::GameArtwork => "Mostrar a arte do jogo selecionado",
            Key::GlassWindow => "Janela de vidro",
            Key::Motion => "Animações",
            Key::MotionFull => "Completas",
            Key::MotionReduced => "Reduzidas",
            Key::Sounds => "Sons da interface",
            Key::Language => "Idioma",
            Key::Storage => "Biblioteca",
            Key::Rescan => "Verificar de novo",
            Key::InstalledSize => {
                "Os jogos desbloqueados ficam ao lado dos seus dumps e compartilham os arquivos quando o disco permite. Escolha outra pasta para mantê-los em outro disco."
            }
            Key::About => "Sobre",
            Key::AboutText => "O AnyPlay Launcher inicia seus jogos com o AnyPS5 no Windows.",
            Key::ToolkitVersion => "Toolkit",
            Key::ProfileName => "Nome do perfil",
            Key::Behaviour => "Comportamento",
            Key::HideWhilePlaying => "Ocultar o launcher durante o jogo",
            Key::RosettaMissing => "Os jogos rodam pelo Rosetta, que este Mac ainda não tem.",
            Key::RosettaInstall => "Instalar o Rosetta",
            Key::RosettaInstalled => "O Rosetta está instalado",
            Key::MoveToApplications => "Mover o AnyPS5 para Aplicativos",
            Key::MoveToApplicationsText => {
                "O macOS abre apps baixados a partir de uma cópia bloqueada, somente leitura. Permita movê-lo para Aplicativos e abri-lo de novo, para que isso nunca mais atrapalhe."
            }
            Key::MoveNow => "Mover e reabrir",
            Key::Later => "Mais tarde",
            Key::NoToolkit => {
                "Esta cópia não tem ferramentas de conversão. Use o instalador da versão publicada."
            }
            Key::EmptyLibraryTitle => "Nenhum jogo ainda",
            Key::EmptyLibraryText => {
                "Coloque dumps de jogos na sua pasta de jogos e eles aparecerão aqui sozinhos."
            }
            Key::Scanning => "Procurando jogos",
            Key::AllGames => "Todos os jogos",
            Key::SortName => "Por nome",
            Key::PlayTime => "Jogado por último",
            Key::ThemeCosmic => "Cósmico",
            Key::ThemeAurora => "Aurora",
            Key::ThemeSunset => "Pôr do sol",
            Key::HintMove => "Mover",
            Key::HintSelect => "Selecionar",
            Key::HintBack => "Voltar",
            Key::HintOptions => "Opções",
            Key::UnlockFailed => "Não foi possível desbloquear",
            Key::StartFailed => "Não foi possível iniciar o jogo",
            Key::GameEnded => "O jogo foi fechado",
            Key::LogTitle => "Registro",
            Key::Close => "Fechar",
            Key::Quit => "Sair",
            Key::Confirm => "Confirmar",
            Key::UnreadableDump => {
                "Este dump está criptografado ou danificado e não pode ser aberto."
            }
            Key::Controls => "Controles",
            Key::ControlsHint => {
                "Escolha uma linha e pressione a tecla que deseja usar. Esc cancela. Um controle conectado mantém seus próprios botões."
            }
            Key::PressKey => "Pressione uma tecla…",
            Key::ResetControls => "Restaurar teclas padrão",
            Key::NoKey => "Nenhuma",
            Key::Dpad => "Direcional",
            Key::LeftStick => "Analógico esquerdo",
            Key::RightStick => "Analógico direito",
            Key::TouchpadLeft => "Touchpad (esquerda)",
            Key::TouchpadRight => "Touchpad (direita)",
            Key::DirectionUp => "Cima",
            Key::DirectionDown => "Baixo",
            Key::DirectionLeft => "Esquerda",
            Key::DirectionRight => "Direita",
            Key::KeySpace => "Espaço",
            Key::KeyEnter => "Enter",
            Key::MouseLeft => "Clique esquerdo",
            Key::MouseRight => "Clique direito",
            Key::MouseMiddle => "Clique do meio",
            Key::MouseBack => "Mouse 4",
            Key::MouseForward => "Mouse 5",
            Key::WheelUp => "Roda ↑",
            Key::WheelDown => "Roda ↓",
            Key::Trophies => "Troféus",
            Key::TrophyHidden => "Troféu oculto",
            Key::TrophyHiddenDetail => "Mostra o que é quando você o conquistar.",
            Key::TrophyLocked => "Não conquistado",
            Key::GradePlatinum => "Platina",
            Key::GradeGold => "Ouro",
            Key::GradeSilver => "Prata",
            Key::GradeBronze => "Bronze",
            Key::Advanced => "Avançado",
            Key::AdvancedDetail => "Analógicos, mouse, teclado, cache, compartilhamento",
            Key::AdvancedTitle => "Configurações avançadas",
            Key::MakeReport => "Criar relatório",
            Key::MakeReportDetail => "Para anexar no Discord ou no GitHub",
            Key::ScreenshotsFolder => "Pasta de capturas de tela",
            Key::ShowScreenshots => "Mostrar capturas de tela",
            Key::FontsFolder => "Pasta de fontes do sistema",
            Key::FontsNone => "Nenhuma",
            Key::FontsReset => "Remover a pasta de fontes",
            Key::FontsHint => {
                "Jogos que pedem as fontes do sistema do console usam os arquivos desta pasta."
            }
            Key::SectionSticks => "Analógicos do controle",
            Key::LeftDeadzone => "Zona morta do analógico esquerdo",
            Key::RightDeadzone => "Zona morta do analógico direito",
            Key::DeadzoneDetail => {
                "Quanto do curso conta como centro; evita que um analógico se desvie sozinho"
            }
            Key::SectionMouse => "Mouse como câmera (botão do meio ou F7)",
            Key::MouseSensitivity => "Sensibilidade do mouse",
            Key::MouseSensitivityDetail => "Mais alta gira por completo com menos movimento",
            Key::MouseVertical => "Velocidade vertical",
            Key::MouseVerticalDetail => "Acelera olhar para cima e para baixo em relação a girar",
            Key::MouseCurve => "Curva de resposta",
            Key::MouseCurveDetail => "Acima de 1 dá mais precisão em baixa velocidade",
            Key::MouseMinimum => "Inclinação mínima",
            Key::MouseMinimumDetail => "Aumente se um movimento lento não mexer a câmera",
            Key::MouseMaxSpeed => "Velocidade máxima",
            Key::MouseMaxSpeedDetail => {
                "Mantém um movimento rápido na inclinação máxima por um pouco mais de tempo"
            }
            Key::MouseSmoothing => "Suavização",
            Key::MouseSmoothingDetail => {
                "Suaviza movimentos irregulares; desativada, segue cada leitura"
            }
            Key::MouseDecay => "Centralizar após",
            Key::MouseDecayDetail => {
                "Quanto tempo a câmera leva para parar depois que o mouse para"
            }
            Key::SectionKeyboard => "Teclado",
            Key::StickModifierScale => "Intensidade do movimento lento",
            Key::StickModifierScaleDetail => {
                "Quanto os analógicos se inclinam enquanto o movimento lento está pressionado (defina em Controles)"
            }
            Key::KeyboardRamp => "Rampa do analógico",
            Key::KeyboardRampDetail => {
                "Quanto tempo uma tecla leva para inclinar o analógico por completo"
            }
            Key::TriggerRamp => "Rampa do gatilho",
            Key::TriggerRampDetail => {
                "Quanto tempo uma tecla leva para pressionar L2/R2 por completo, para jogos de corrida"
            }
            Key::SectionGraphics => "Gráficos",
            Key::ShaderCache => "Cache de shaders",
            Key::ShaderCacheDetail => "Desativado, os shaders são compilados de novo a cada início",
            Key::SectionShare => "Compartilhamento",
            Key::ExportSettings => "Exportar configurações",
            Key::ExportSettingsDetail => "Teclas e todas as configurações em um arquivo",
            Key::ImportSettings => "Importar configurações",
            Key::ImportSettingsDetail => "Aplica um arquivo de configurações compartilhado",
            Key::ResetAdvanced => "Restaurar configurações avançadas",
            Key::ReportSaved => "Relatório salvo",
            Key::ReportFailed => {
                "Não foi possível salvar o relatório. Verifique se há espaço no disco e tente de novo."
            }
            Key::SettingsExported => "Configurações exportadas",
            Key::SettingsImported => "Configurações importadas",
            Key::SettingsImportedOther => "Configurações de %@ importadas",
            Key::SettingsFileFailed => "Este não é um arquivo de configurações do AnyPlay",
            Key::StickModifier => "Movimento lento",
            Key::DiagnosisMissingModule => {
                "Módulo ausente: %@. Desbloquear o jogo de novo pode resolver."
            }
            Key::DiagnosisUnsupported => {
                "Ainda sem suporte. Envie um relatório em Opções > Registro para que seja adicionado. Função: %@"
            }
            Key::DiagnosisGpuLost => {
                "A GPU parou de responder. Atualize o driver da placa de vídeo e inicie o jogo de novo."
            }
            Key::DiagnosisOutOfMemory => {
                "O jogo ficou sem memória. Feche outros programas e inicie o jogo de novo."
            }
            Key::DiagnosisEngineError => {
                "Erro do motor. Inicie o jogo de novo. Se continuar, envie um relatório em Opções > Registro. Detalhes: %@"
            }
            Key::DiagnosisCrashed => {
                "O jogo travou. Inicie-o de novo. Se continuar, envie um relatório em Opções > Registro. (%@)"
            }
            Key::TrophyEarned => "Troféu conquistado",
            _ => return None,
        })
    }
}
