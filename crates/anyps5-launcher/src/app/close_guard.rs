//! Closing the window while the launcher is converting (unlocking), installing or uninstalling:
//! the player is asked first. An unlock can be stopped, so the question offers to go back; a
//! removal cannot be stopped once it started, and the message says so.

use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};

use crate::app::model::Model;
use crate::ui::i18n::Language;

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

/// Asks, blocking, and returns whether the window may close.
pub fn confirm_close(busy: Busy, language: Language) -> bool {
    let (title, text, yes, back) = texts(busy, language);
    let result = MessageDialog::new()
        .set_level(MessageLevel::Warning)
        .set_title(title)
        .set_description(text)
        .set_buttons(MessageButtons::OkCancelCustom(
            yes.to_owned(),
            back.to_owned(),
        ))
        .show();
    matches!(result, MessageDialogResult::Custom(ref c) if c == yes)
        || matches!(result, MessageDialogResult::Ok)
}
