//! The question before the window closes while games are being unlocked or removed: pause the
//! unlocks and go on later, cancel them, or stay. A removal can be neither paused nor cancelled,
//! so the question only says so. A side panel of the usual family, like the question before a
//! running game is closed.

use egui::Color32;
use egui::Context;

use crate::screens::dialog;
use crate::screens::rows::{self, Change, Row};
use crate::screens::side::{self, SideState, SideView};
use crate::ui::Shell;
use crate::ui::i18n::Language;
use crate::ui::icons::Icon;
use crate::ui::input::InputEvent;

/// What is under way when the window is asked to close.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuitKind {
    /// A game is being unlocked (converted and installed), or waits for it.
    Unlocking,
    /// A game is paused by hand (and maybe others run): closing loses its progress.
    Paused,
    /// An unlocked copy is being removed.
    Removing,
    /// The small Cancel button of one unlock was pressed: the work done so far is lost.
    CancelUnlock,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuitAction {
    /// Mark the unlocks as paused and close: Resume starts them again from the beginning.
    Pause,
    /// Cancel the unlocks and close (the progress is lost).
    Cancel,
    /// Back to the launcher.
    Stay,
}

/// English, Turkish, German, Spanish, French, Brazilian Portuguese.
fn pick(language: Language, texts: [&'static str; 6]) -> &'static str {
    match language {
        Language::English => texts[0],
        Language::Turkish => texts[1],
        Language::German => texts[2],
        Language::Spanish => texts[3],
        Language::French => texts[4],
        Language::Portuguese => texts[5],
    }
}

fn title(kind: QuitKind, language: Language) -> &'static str {
    match kind {
        QuitKind::Unlocking => pick(
            language,
            [
                "Close while installing?",
                "Kurulum sürerken kapatılsın mı?",
                "Während der Installation schließen?",
                "¿Cerrar durante la instalación?",
                "Fermer pendant l’installation ?",
                "Fechar durante a instalação?",
            ],
        ),
        QuitKind::Paused => pick(
            language,
            [
                "Close with a paused game?",
                "Duraklatılmış oyunla kapatılsın mı?",
                "Mit pausiertem Spiel schließen?",
                "¿Cerrar con un juego en pausa?",
                "Fermer avec un jeu en pause ?",
                "Fechar com um jogo em pausa?",
            ],
        ),
        QuitKind::CancelUnlock => pick(
            language,
            [
                "Cancel this unlock?",
                "Bu kurulum iptal edilsin mi?",
                "Diese Installation abbrechen?",
                "¿Cancelar esta instalación?",
                "Annuler cette installation ?",
                "Cancelar esta instalação?",
            ],
        ),
        QuitKind::Removing => pick(
            language,
            [
                "A removal is under way",
                "Bir kaldırma sürüyor",
                "Eine Entfernung läuft",
                "Hay una eliminación en curso",
                "Une suppression est en cours",
                "Uma remoção está em andamento",
            ],
        ),
    }
}

fn ask(kind: QuitKind, language: Language) -> &'static str {
    match kind {
        QuitKind::Unlocking => pick(
            language,
            [
                "A game is still being converted. Closing the launcher stops it and the progress is lost. With Pause and quit it is marked as paused, and Resume starts it again from the beginning.",
                "Bir oyun hâlâ dönüştürülüyor. Başlatıcıyı kapatmak onu durdurur ve ilerleme kaybolur. Duraklat ve çık ile oyun duraklatılmış olarak işaretlenir ve Devam Et onu baştan başlatır.",
                "Ein Spiel wird noch umgewandelt. Wenn du den Launcher schließt, wird es gestoppt und der Fortschritt geht verloren. Mit Pausieren und beenden wird es als pausiert markiert, und Fortsetzen beginnt von vorn.",
                "Un juego aún se está convirtiendo. Cerrar el launcher lo detiene y el progreso se pierde. Con Pausar y salir queda marcado como en pausa, y Reanudar empieza desde el principio.",
                "Un jeu est encore en cours de conversion. Fermer le launcher l’arrête et la progression est perdue. Avec Mettre en pause et quitter, il est marqué en pause, et Reprendre repart du début.",
                "Um jogo ainda está sendo convertido. Fechar o launcher o interrompe e o progresso é perdido. Com Pausar e sair, ele fica marcado como em pausa, e Retomar recomeça do início.",
            ],
        ),
        QuitKind::Paused => pick(
            language,
            [
                "A game is paused. Pause only works while the launcher is open: the progress cannot be saved, so if you close it the conversion is cancelled and starts again from the beginning.",
                "Bir oyun duraklatıldı. Duraklatma yalnızca başlatıcı açıkken çalışır: ilerleme kaydedilemez, kapatırsan dönüştürme iptal edilir ve baştan başlar.",
                "Ein Spiel ist pausiert. Pause funktioniert nur bei geöffnetem Launcher: Der Fortschritt kann nicht gespeichert werden. Wenn du schließt, wird die Umwandlung abgebrochen und beginnt von vorn.",
                "Un juego está en pausa. La pausa solo funciona con el launcher abierto: el progreso no se puede guardar, así que si lo cierras la conversión se cancela y empieza desde el principio.",
                "Un jeu est en pause. La pause ne fonctionne que lorsque le launcher est ouvert : la progression ne peut pas être enregistrée, donc si tu fermes, la conversion est annulée et repart du début.",
                "Um jogo está em pausa. A pausa só funciona com o launcher aberto: o progresso não pode ser salvo, então se você fechar a conversão é cancelada e recomeça do início.",
            ],
        ),
        QuitKind::CancelUnlock => pick(
            language,
            [
                "The work done so far is lost. To stop and go on later, use Pause instead.",
                "Şimdiye kadarki iş kaybolur. Durdurup sonra devam etmek için bunun yerine Duraklat’ı kullan.",
                "Die bisherige Arbeit geht verloren. Zum Anhalten und späteren Fortsetzen nutze stattdessen Pausieren.",
                "El trabajo hecho hasta ahora se pierde. Para parar y seguir más tarde, usa Pausar.",
                "Le travail déjà fait est perdu. Pour arrêter et reprendre plus tard, utilise plutôt Pause.",
                "O trabalho feito até agora é perdido. Para parar e continuar depois, use Pausar.",
            ],
        ),
        QuitKind::Removing => pick(
            language,
            [
                "A copy is being removed. A removal cannot be paused or cancelled: wait until it is done.",
                "Bir kopya kaldırılıyor. Kaldırma duraklatılamaz veya iptal edilemez: bitmesini bekle.",
                "Eine Kopie wird entfernt. Das Entfernen lässt sich weder pausieren noch abbrechen: Warte, bis es fertig ist.",
                "Se está eliminando una copia. Una eliminación no se puede pausar ni cancelar: espera a que termine.",
                "Une copie est en cours de suppression. Une suppression ne peut être ni mise en pause ni annulée : attends qu’elle soit terminée.",
                "Uma cópia está sendo removida. Uma remoção não pode ser pausada nem cancelada: aguarde até terminar.",
            ],
        ),
    }
}

/// "Cancel and quit" / "Go back".
#[derive(Default)]
pub struct QuitPanel {
    side: SideState,
}

impl QuitPanel {
    /// Opens on the safe answer: the first row, where pausing loses nothing; for the question
    /// about one unlock the last one, which keeps it going.
    pub fn open(&mut self, kind: QuitKind) {
        self.side
            .open_at(usize::from(kind == QuitKind::CancelUnlock));
    }

    pub fn close(&mut self) {
        self.side.close();
    }

    pub fn dismiss(&mut self) {
        self.side.dismiss();
    }

    pub fn is_active(&self) -> bool {
        self.side.is_active()
    }

    fn handle(&mut self, change: &Change, actions: &mut Vec<QuitAction>) {
        if let Change::Run { row } = change {
            self.side.close();
            actions.push(match *row {
                "pause" => QuitAction::Pause,
                "cancel" => QuitAction::Cancel,
                _ => QuitAction::Stay,
            });
        }
    }

    pub fn show(
        &mut self,
        shell: &mut Shell,
        ctx: &Context,
        events: &[InputEvent],
        kind: QuitKind,
        language: Language,
        accent: Color32,
    ) -> Vec<QuitAction> {
        let mut actions = Vec::new();
        if !self.side.is_active() {
            return actions;
        }
        let build = || match kind {
            QuitKind::Unlocking | QuitKind::Paused => vec![
                Row::command(
                    "pause",
                    pick(
                        language,
                        [
                            "Pause and quit",
                            "Duraklat ve çık",
                            "Pausieren und beenden",
                            "Pausar y salir",
                            "Mettre en pause et quitter",
                            "Pausar e sair",
                        ],
                    ),
                )
                .icon(Icon::PAUSE),
                Row::command(
                    "cancel",
                    pick(
                        language,
                        [
                            "Cancel and quit",
                            "İptal et ve çık",
                            "Abbrechen und beenden",
                            "Cancelar y salir",
                            "Annuler et quitter",
                            "Cancelar e sair",
                        ],
                    ),
                )
                .icon(Icon::X)
                .destructive(),
                Row::command(
                    "stay",
                    pick(
                        language,
                        [
                            "Go back",
                            "Geri dön",
                            "Zurück",
                            "Volver",
                            "Retour",
                            "Voltar",
                        ],
                    ),
                )
                .icon(Icon::ARROW_LEFT)
                .refusal(),
            ],
            QuitKind::CancelUnlock => vec![
                Row::command(
                    "cancel",
                    pick(
                        language,
                        [
                            "Cancel unlock",
                            "Kurulumu iptal et",
                            "Installation abbrechen",
                            "Cancelar instalación",
                            "Annuler l’installation",
                            "Cancelar instalação",
                        ],
                    ),
                )
                .icon(Icon::X)
                .destructive(),
                Row::command(
                    "stay",
                    pick(
                        language,
                        [
                            "Keep going",
                            "Sürdür",
                            "Weitermachen",
                            "Seguir",
                            "Continuer",
                            "Continuar",
                        ],
                    ),
                )
                .icon(Icon::ARROW_LEFT)
                .refusal(),
            ],
            QuitKind::Removing => vec![
                Row::command(
                    "stay",
                    pick(
                        language,
                        [
                            "Go back",
                            "Geri dön",
                            "Zurück",
                            "Volver",
                            "Retour",
                            "Voltar",
                        ],
                    ),
                )
                .icon(Icon::ARROW_LEFT)
                .refusal(),
            ],
        };
        if self.side.is_open() {
            let rows = build();
            let driven = dialog::drive(shell, &mut self.side.nav, &rows, &side::commands(events));
            for change in &driven.changes {
                self.handle(change, &mut actions);
            }
            if driven.close {
                self.side.close();
                actions.push(QuitAction::Stay);
            }
        }
        let mut rows = build();
        let view = SideView {
            title: title(kind, language),
            subtitle: Some(ask(kind, language)),
            question: true,
            list_id: "quit-rows",
            nav: &self.side.nav,
            accent,
            language,
            force: None,
        };
        match side::draw(ctx, self.side.is_open(), &view, &mut rows) {
            Some(drawn) if self.side.is_open() => {
                let (changes, _) =
                    rows::apply_mouse(shell, &mut self.side.nav, &rows, drawn.events);
                for change in &changes {
                    self.handle(change, &mut actions);
                }
                if drawn.back_clicked {
                    self.side.close();
                    actions.push(QuitAction::Stay);
                }
            }
            Some(_) => {}
            None => self.side.finished(),
        }
        actions
    }
}
