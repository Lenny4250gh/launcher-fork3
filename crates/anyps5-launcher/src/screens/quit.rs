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
    /// An unlocked copy is being removed.
    Removing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuitAction {
    /// Stop the unlocks, remember where they were and close.
    Pause,
    /// Cancel the unlocks and close.
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
                "A game is still being converted. Pause only works while the launcher is open: if you close it now, the progress cannot be saved and the conversion starts over.",
                "Bir oyun hâlâ dönüştürülüyor. Duraklatma yalnızca başlatıcı açıkken çalışır: şimdi kapatırsan ilerleme kaydedilemez ve dönüştürme baştan başlar.",
                "Ein Spiel wird noch umgewandelt. Pause funktioniert nur bei geöffnetem Launcher: Wenn du jetzt schließt, kann der Fortschritt nicht gespeichert werden und die Umwandlung beginnt von vorn.",
                "Un juego aún se está convirtiendo. La pausa solo funciona con el launcher abierto: si lo cierras ahora, el progreso no se puede guardar y la conversión empezará de cero.",
                "Un jeu est encore en cours de conversion. La pause ne fonctionne que lorsque le launcher est ouvert : si tu le fermes maintenant, la progression ne peut pas être enregistrée et la conversion repartira de zéro.",
                "Um jogo ainda está sendo convertido. A pausa só funciona com o launcher aberto: se você fechar agora, o progresso não pode ser salvo e a conversão recomeça do zero.",
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

/// "Pause and quit" / "Quit and lose progress" / "Go back".
#[derive(Default)]
pub struct QuitPanel {
    side: SideState,
}

impl QuitPanel {
    /// Opens on "Go back", the first row: closing loses the progress.
    pub fn open(&mut self) {
        self.side.open_at(0);
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
            actions.push(if *row == "pause" {
                QuitAction::Pause
            } else if *row == "cancel" {
                QuitAction::Cancel
            } else {
                QuitAction::Stay
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
            QuitKind::Unlocking => vec![
                Row::command("stay", pick(
                        language,
                        ["Go back", "Geri dön", "Zurück", "Volver", "Retour", "Voltar"],
                    ))
                    .icon(Icon::ARROW_LEFT)
                    .refusal(),
                Row::command(
                    "cancel",
                    pick(
                        language,
                        [
                            "Quit and lose progress",
                            "Çık ve ilerlemeyi kaybet",
                            "Beenden und Fortschritt verlieren",
                            "Salir y perder el progreso",
                            "Quitter et perdre la progression",
                            "Sair e perder o progresso",
                        ],
                    ),
                )
                .icon(Icon::X)
                .destructive(),
            ],
            QuitKind::Removing => vec![
                Row::command("stay", pick(
                        language,
                        ["Go back", "Geri dön", "Zurück", "Volver", "Retour", "Voltar"],
                    ))
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
