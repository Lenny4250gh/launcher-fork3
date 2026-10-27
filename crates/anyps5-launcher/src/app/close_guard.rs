//! Closing the window while the launcher is converting (unlocking), installing or uninstalling:
//! the player is asked first, in the launcher's own panel (never an OS dialog). An unlock can be stopped, so the question offers to go back; a
//! removal cannot be stopped once it started, and the message says so.

use egui::{Color32, Context};

use crate::app::model::Model;
use crate::screens::dialog;
use crate::screens::rows::{self, Change, Row};
use crate::screens::side::{self, SideState, SideView};
use crate::ui::i18n::Language;
use crate::ui::icons::Icon;
use crate::ui::input::InputEvent;
use crate::ui::Shell;

/// What is going on that closing would interrupt.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Busy {
    Unlocking,
    /// A removal is running (it cannot be cancelled), possibly together with unlocks.
    Uninstalling,
}

impl Model {
    pub fn busy_for_close(&self) -> Option<Busy> {
        if !self.uninstalling.is_empty() {
            Some(Busy::Uninstalling)
        } else if !self.progress.is_empty() || !self.queue.is_empty() || !self.cancelling.is_empty()
        {
            Some(Busy::Unlocking)
        } else {
            None
        }
    }
}

/// `(title, message, close anyway, go back)`.
fn texts(
    busy: Busy,
    language: Language,
) -> (&'static str, &'static str, &'static str, &'static str) {
    use Busy::*;
    use Language::*;
    match (busy, language) {
        (Unlocking, Turkish) => ("Kapatılsın mı?", "Bir oyun hazırlanıyor veya yükleniyor. Şimdi kapatırsan işlem durur. Emin misin?", "Evet, eminim", "Geri dön"),
        (Unlocking, German) => ("Wirklich schließen?", "Ein Spiel wird gerade umgewandelt oder installiert. Wenn du jetzt schließt, wird der Vorgang abgebrochen. Bist du sicher?", "Ja, ich bin sicher", "Zurück"),
        (Unlocking, Spanish) => ("¿Cerrar ahora?", "Se está convirtiendo o instalando un juego. Si cierras ahora, el proceso se detendrá. ¿Seguro?", "Sí, estoy seguro", "Volver"),
        (Unlocking, French) => ("Fermer maintenant ?", "Un jeu est en cours de conversion ou d’installation. Si tu fermes maintenant, le processus sera interrompu. Sûr ?", "Oui, je suis sûr", "Retour"),
        (Unlocking, Portuguese) => ("Fechar agora?", "Um jogo está sendo convertido ou instalado. Se fechar agora, o processo será interrompido. Tem certeza?", "Sim, tenho certeza", "Voltar"),
        (Unlocking, English) => ("Close now?", "A game is being unlocked or installed. If you close now, the work is stopped. Are you sure?", "Yes, I'm sure", "Go back"),
        (Uninstalling, Turkish) => ("Kaldırma sürüyor", "Bir oyun kaldırılıyor. Kaldırma işlemi İPTAL EDİLEMEZ. Şimdi kapatırsan oyun yarım kalabilir. Yine de kapatılsın mı?", "Yine de kapat", "Geri dön"),
        (Uninstalling, German) => ("Deinstallation läuft", "Ein Spiel wird gerade deinstalliert. Die Deinstallation kann NICHT abgebrochen werden. Wenn du jetzt schließt, bleibt sie möglicherweise unvollständig. Trotzdem schließen?", "Trotzdem schließen", "Zurück"),
        (Uninstalling, Spanish) => ("Desinstalación en curso", "Se está desinstalando un juego. La desinstalación NO se puede cancelar. Si cierras ahora, puede quedar incompleta. ¿Cerrar de todos modos?", "Cerrar de todos modos", "Volver"),
        (Uninstalling, French) => ("Désinstallation en cours", "Un jeu est en cours de désinstallation. La désinstallation ne peut PAS être annulée. Si tu fermes maintenant, elle risque d’être incomplète. Fermer quand même ?", "Fermer quand même", "Retour"),
        (Uninstalling, Portuguese) => ("Desinstalação em andamento", "Um jogo está sendo desinstalado. A desinstalação NÃO pode ser cancelada. Se fechar agora, ela pode ficar incompleta. Fechar mesmo assim?", "Fechar mesmo assim", "Voltar"),
        (Uninstalling, English) => ("Uninstall in progress", "A game is being uninstalled. An uninstall can NOT be cancelled. If you close now it may be left incomplete. Close anyway?", "Close anyway", "Go back"),
    }
}

/// What the player answered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuitAction {
    /// Close the launcher anyway.
    Quit,
    /// Back to the launcher.
    Keep,
}

/// The question before the launcher closes in the middle of work: "Yes, I'm sure" / "Go back".
#[derive(Default)]
pub struct QuitPanel {
    side: SideState,
    busy: Option<Busy>,
}

impl QuitPanel {
    /// Opens on "Go back": Enter must never interrupt the work by accident.
    pub fn open(&mut self, busy: Busy) {
        self.busy = Some(busy);
        self.side.open_at(1);
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
            actions.push(if *row == "quit" {
                QuitAction::Quit
            } else {
                QuitAction::Keep
            });
        }
    }

    pub fn show(
        &mut self,
        shell: &mut Shell,
        ctx: &Context,
        events: &[InputEvent],
        language: Language,
        accent: Color32,
    ) -> Vec<QuitAction> {
        let mut actions = Vec::new();
        if !self.side.is_active() {
            return actions;
        }
        let (title, text, yes, back) = texts(self.busy.unwrap_or(Busy::Unlocking), language);
        let build = || {
            vec![
                Row::command("quit", yes).icon(Icon::POWER).destructive(),
                Row::command("keep", back).icon(Icon::PLAY).refusal(),
            ]
        };
        if self.side.is_open() {
            let rows = build();
            let driven = dialog::drive(shell, &mut self.side.nav, &rows, &side::commands(events));
            for change in &driven.changes {
                self.handle(change, &mut actions);
            }
            if driven.close {
                self.side.close();
                actions.push(QuitAction::Keep);
            }
        }
        let mut rows = build();
        let view = SideView {
            title,
            subtitle: Some(text),
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
                    actions.push(QuitAction::Keep);
                }
            }
            Some(_) => {}
            None => self.side.finished(),
        }
        actions
    }
}
