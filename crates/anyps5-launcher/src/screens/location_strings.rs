//! The words of the unlocked games location and of moving the unlocked games (docs/DESIGN.md 5.7).
//! English and Turkish like the rest.

use std::path::Path;

use anyps5_core::location::Refusal;

use crate::ui::i18n::{Key, Language, byte_text};

fn pick(language: Language, english: &'static str, turkish: &'static str) -> &'static str {
    match language {
        Language::English => english,
        Language::Turkish => turkish,
        Language::German => german(english),
        Language::Spanish => spanish(english),
        Language::French => french(english),
        Language::Portuguese => portuguese(english),
    }
}

/// The German text of an English one (the English text is the key); English when none is there.
fn german(english: &'static str) -> &'static str {
    match english {
        "Unlocked Games Location" => "Speicherort freigeschalteter Spiele",
        "Choose a Folder for Unlocked Games" => "Ordner für freigeschaltete Spiele wählen",
        "Use Default Location" => "Standardort verwenden",
        "Next to your games" => "Neben deinen Spielen",
        "Not found" => "Nicht gefunden",
        "free" => "frei",
        "Unlocked games are copied in full into a .AnyPS5 folder here. Your dumps are never changed." => {
            "Freigeschaltete Spiele werden hier vollständig in einen .AnyPS5-Ordner kopiert. Deine Dumps bleiben unverändert."
        }
        "This is a network location. Copying is slower, and games run over the network." => {
            "Das ist ein Netzwerkort. Kopieren dauert länger, und Spiele laufen über das Netzwerk."
        }
        "This folder is not available, so its games show as locked. Connect the drive, or choose another folder." => {
            "Dieser Ordner ist nicht verfügbar, deshalb erscheinen seine Spiele als gesperrt. Schließ das Laufwerk an oder wähle einen anderen Ordner."
        }
        "Unlocking and playing wait until the move is done. Saves move with their game." => {
            "Freischalten und Spielen warten, bis das Verschieben fertig ist. Spielstände ziehen mit ihrem Spiel um."
        }
        "Cancel Move" => "Verschieben abbrechen",
        "Continue Moving" => "Verschieben fortsetzen",
        "Leave Games Where They Are" => "Spiele am alten Ort lassen",
        "They stay in the old location, unused. You unlock them again after that." => {
            "Sie bleiben ungenutzt am alten Ort. Danach schaltest du sie noch einmal frei."
        }
        "That folder is not there. Choose another one." => {
            "Dieser Ordner ist nicht da. Wähle einen anderen."
        }
        "Choose a folder outside your games folder. Your dumps are never written to." => {
            "Wähle einen Ordner außerhalb deines Spieleordners. In deine Dumps wird nie geschrieben."
        }
        "Choose a folder outside the .AnyPS5 folders." => {
            "Wähle einen Ordner außerhalb der .AnyPS5-Ordner."
        }
        "Cannot write to that folder. Choose another one." => {
            "In diesen Ordner kann nicht geschrieben werden. Wähle einen anderen."
        }
        "Close the running game first." => "Beende zuerst das laufende Spiel.",
        "Wait until unlocking is done." => "Warte, bis das Freischalten fertig ist.",
        "Finish or cancel the move first." => "Schließ das Verschieben zuerst ab oder brich es ab.",
        "Unlocked games moved." => "Freigeschaltete Spiele verschoben.",
        "Your unlocked games are being moved. Try again when that is done." => {
            "Deine freigeschalteten Spiele werden gerade verschoben. Versuch es noch einmal, wenn das fertig ist."
        }
        other => other,
    }
}

/// The Spanish text of an English one (the English text is the key); English when none is there.
fn spanish(english: &'static str) -> &'static str {
    match english {
        "Unlocked Games Location" => "Ubicación de los juegos desbloqueados",
        "Choose a Folder for Unlocked Games" => "Elegir carpeta para los juegos desbloqueados",
        "Use Default Location" => "Usar la ubicación predeterminada",
        "Next to your games" => "Junto a tus juegos",
        "Not found" => "No encontrada",
        "free" => "libres",
        "Unlocked games are copied in full into a .AnyPS5 folder here. Your dumps are never changed." => {
            "Los juegos desbloqueados se copian completos en una carpeta .AnyPS5 aquí. Tus dumps nunca se modifican."
        }
        "This is a network location. Copying is slower, and games run over the network." => {
            "Esta es una ubicación de red. Copiar es más lento y los juegos se ejecutan a través de la red."
        }
        "This folder is not available, so its games show as locked. Connect the drive, or choose another folder." => {
            "Esta carpeta no está disponible, por eso sus juegos aparecen bloqueados. Conecta la unidad o elige otra carpeta."
        }
        "Unlocking and playing wait until the move is done. Saves move with their game." => {
            "Desbloquear y jugar esperan a que termine el traslado. Las partidas guardadas se mueven con su juego."
        }
        "Cancel Move" => "Cancelar traslado",
        "Continue Moving" => "Continuar traslado",
        "Leave Games Where They Are" => "Dejar los juegos donde están",
        "They stay in the old location, unused. You unlock them again after that." => {
            "Se quedan sin usar en la ubicación anterior. Después los desbloqueas de nuevo."
        }
        "That folder is not there. Choose another one." => "Esa carpeta no existe. Elige otra.",
        "Choose a folder outside your games folder. Your dumps are never written to." => {
            "Elige una carpeta fuera de tu carpeta de juegos. Nunca se escribe en tus dumps."
        }
        "Choose a folder outside the .AnyPS5 folders." => {
            "Elige una carpeta fuera de las carpetas .AnyPS5."
        }
        "Cannot write to that folder. Choose another one." => {
            "No se puede escribir en esa carpeta. Elige otra."
        }
        "Close the running game first." => "Cierra primero el juego en ejecución.",
        "Wait until unlocking is done." => "Espera a que termine el desbloqueo.",
        "Finish or cancel the move first." => "Termina o cancela primero el traslado.",
        "Unlocked games moved." => "Juegos desbloqueados trasladados.",
        "Your unlocked games are being moved. Try again when that is done." => {
            "Se están trasladando tus juegos desbloqueados. Inténtalo de nuevo cuando termine."
        }
        other => other,
    }
}

/// The French text of an English one (the English text is the key); English when none is there.
fn french(english: &'static str) -> &'static str {
    match english {
        "Unlocked Games Location" => "Emplacement des jeux déverrouillés",
        "Choose a Folder for Unlocked Games" => "Choisir un dossier pour les jeux déverrouillés",
        "Use Default Location" => "Utiliser l’emplacement par défaut",
        "Next to your games" => "À côté de tes jeux",
        "Not found" => "Introuvable",
        "free" => "libres",
        "Unlocked games are copied in full into a .AnyPS5 folder here. Your dumps are never changed." => {
            "Les jeux déverrouillés sont copiés en entier dans un dossier .AnyPS5 ici. Tes dumps ne sont jamais modifiés."
        }
        "This is a network location. Copying is slower, and games run over the network." => {
            "C’est un emplacement réseau. La copie est plus lente et les jeux tournent via le réseau."
        }
        "This folder is not available, so its games show as locked. Connect the drive, or choose another folder." => {
            "Ce dossier n’est pas disponible, ses jeux apparaissent donc comme verrouillés. Branche le disque ou choisis un autre dossier."
        }
        "Unlocking and playing wait until the move is done. Saves move with their game." => {
            "Le déverrouillage et le jeu attendent la fin du déplacement. Les sauvegardes suivent leur jeu."
        }
        "Cancel Move" => "Annuler le déplacement",
        "Continue Moving" => "Reprendre le déplacement",
        "Leave Games Where They Are" => "Laisser les jeux où ils sont",
        "They stay in the old location, unused. You unlock them again after that." => {
            "Ils restent inutilisés à l’ancien emplacement. Tu les déverrouilles ensuite de nouveau."
        }
        "That folder is not there. Choose another one." => {
            "Ce dossier n’existe pas. Choisis-en un autre."
        }
        "Choose a folder outside your games folder. Your dumps are never written to." => {
            "Choisis un dossier en dehors de ton dossier des jeux. Rien n’est jamais écrit dans tes dumps."
        }
        "Choose a folder outside the .AnyPS5 folders." => {
            "Choisis un dossier en dehors des dossiers .AnyPS5."
        }
        "Cannot write to that folder. Choose another one." => {
            "Impossible d’écrire dans ce dossier. Choisis-en un autre."
        }
        "Close the running game first." => "Ferme d’abord le jeu en cours.",
        "Wait until unlocking is done." => "Attends la fin du déverrouillage.",
        "Finish or cancel the move first." => "Termine ou annule d’abord le déplacement.",
        "Unlocked games moved." => "Jeux déverrouillés déplacés.",
        "Your unlocked games are being moved. Try again when that is done." => {
            "Tes jeux déverrouillés sont en cours de déplacement. Réessaie quand ce sera terminé."
        }
        other => other,
    }
}

/// The Portuguese text of an English one (the English text is the key); English when none is there.
fn portuguese(english: &'static str) -> &'static str {
    match english {
        "Unlocked Games Location" => "Local dos jogos desbloqueados",
        "Choose a Folder for Unlocked Games" => "Escolher pasta para os jogos desbloqueados",
        "Use Default Location" => "Usar o local padrão",
        "Next to your games" => "Ao lado dos seus jogos",
        "Not found" => "Não encontrado",
        "free" => "livres",
        "Unlocked games are copied in full into a .AnyPS5 folder here. Your dumps are never changed." => {
            "Os jogos desbloqueados são copiados por inteiro para uma pasta .AnyPS5 aqui. Seus dumps nunca são alterados."
        }
        "This is a network location. Copying is slower, and games run over the network." => {
            "Este é um local de rede. A cópia é mais lenta e os jogos rodam pela rede."
        }
        "This folder is not available, so its games show as locked. Connect the drive, or choose another folder." => {
            "Esta pasta não está disponível, por isso seus jogos aparecem como bloqueados. Conecte a unidade ou escolha outra pasta."
        }
        "Unlocking and playing wait until the move is done. Saves move with their game." => {
            "Desbloquear e jogar aguardam o fim da mudança. Os jogos salvos acompanham o jogo."
        }
        "Cancel Move" => "Cancelar mudança",
        "Continue Moving" => "Continuar mudança",
        "Leave Games Where They Are" => "Deixar os jogos onde estão",
        "They stay in the old location, unused. You unlock them again after that." => {
            "Eles ficam sem uso no local antigo. Depois você os desbloqueia de novo."
        }
        "That folder is not there. Choose another one." => "Essa pasta não existe. Escolha outra.",
        "Choose a folder outside your games folder. Your dumps are never written to." => {
            "Escolha uma pasta fora da sua pasta de jogos. Nada é gravado nos seus dumps."
        }
        "Choose a folder outside the .AnyPS5 folders." => {
            "Escolha uma pasta fora das pastas .AnyPS5."
        }
        "Cannot write to that folder. Choose another one." => {
            "Não é possível gravar nessa pasta. Escolha outra."
        }
        "Close the running game first." => "Feche primeiro o jogo em execução.",
        "Wait until unlocking is done." => "Aguarde o fim do desbloqueio.",
        "Finish or cancel the move first." => "Conclua ou cancele a mudança primeiro.",
        "Unlocked games moved." => "Jogos desbloqueados movidos.",
        "Your unlocked games are being moved. Try again when that is done." => {
            "Seus jogos desbloqueados estão sendo movidos. Tente de novo quando terminar."
        }
        other => other,
    }
}

/// The title of the setting row.
pub fn title(language: Language) -> &'static str {
    pick(
        language,
        "Unlocked Games Location",
        "Kilidi Açılan Oyunların Konumu",
    )
}

/// The title of the folder dialog.
pub fn dialog(language: Language) -> &'static str {
    pick(
        language,
        "Choose a Folder for Unlocked Games",
        "Kilidi Açılan Oyunlar İçin Klasör Seç",
    )
}

/// "Use Default Location": back to the games folder.
pub fn use_default(language: Language) -> &'static str {
    pick(language, "Use Default Location", "Varsayılan Konumu Kullan")
}

/// Where the default is, in the detail of the row.
fn default_place(language: Language) -> &'static str {
    pick(language, "Next to your games", "Oyunlarının yanında")
}

/// What the row shows: the place and what is free there, or that it is not there.
pub fn detail(
    custom: Option<&Path>,
    free: Option<u64>,
    missing: bool,
    language: Language,
) -> String {
    let place = custom.map_or_else(
        || default_place(language).to_owned(),
        |folder| folder.display().to_string(),
    );
    if missing {
        return format!(
            "{place} \u{b7} {}",
            pick(language, "Not found", "Bulunamadı")
        );
    }
    match free {
        Some(free) => format!(
            "{place} \u{b7} {} {}",
            byte_text(free.min(i64::MAX as u64) as i64, language),
            pick(language, "free", "boş")
        ),
        None => place,
    }
}

/// The note under the row when the copies sit next to the dumps.
pub fn note_default(language: Language) -> &'static str {
    language.text(Key::InstalledSize)
}

/// The note under the row for another folder.
pub fn note_custom(language: Language) -> &'static str {
    pick(
        language,
        "Unlocked games are copied in full into a .AnyPS5 folder here. Your dumps are never changed.",
        "Kilidi açılan oyunlar buradaki .AnyPS5 klasörüne tam kopya olarak yazılır. Dökümlerin hiç değişmez.",
    )
}

/// The note under the row for a network drive.
pub fn note_network(language: Language) -> &'static str {
    pick(
        language,
        "This is a network location. Copying is slower, and games run over the network.",
        "Bu bir ağ konumu. Kopyalama daha yavaştır ve oyunlar ağ üzerinden çalışır.",
    )
}

/// The note under the row when the folder or its drive is gone.
pub fn note_missing(language: Language) -> &'static str {
    pick(
        language,
        "This folder is not available, so its games show as locked. Connect the drive, or choose another folder.",
        "Bu klasör kullanılamıyor, bu yüzden oyunları kilitli görünür. Sürücüyü tak ya da başka bir klasör seç.",
    )
}

/// The row while the copies move: "Moving 3 of 7 · 42%".
pub fn moving(done: usize, total: usize, percent: u32, language: Language) -> String {
    match language {
        Language::English => format!(
            "Moving {} of {total} \u{b7} {percent}%",
            (done + 1).min(total)
        ),
        Language::Turkish => format!(
            "{} / {total} taşınıyor \u{b7} %{percent}",
            (done + 1).min(total)
        ),
        Language::German => format!(
            "{} von {total} wird verschoben \u{b7} {percent} %",
            (done + 1).min(total)
        ),
        Language::Spanish => format!(
            "Trasladando {} de {total} \u{b7} {percent}\u{a0}%",
            (done + 1).min(total)
        ),
        Language::French => format!(
            "Déplacement de {} sur {total} \u{b7} {percent}\u{a0}%",
            (done + 1).min(total)
        ),
        Language::Portuguese => format!(
            "Movendo {} de {total} \u{b7} {percent}%",
            (done + 1).min(total)
        ),
    }
}

/// The note under the row while the copies move.
pub fn note_moving(language: Language) -> &'static str {
    pick(
        language,
        "Unlocking and playing wait until the move is done. Saves move with their game.",
        "Taşıma bitene kadar kilit açma ve oynama bekler. Kayıtlar oyunlarıyla birlikte taşınır.",
    )
}

pub fn cancel_move(language: Language) -> &'static str {
    pick(language, "Cancel Move", "Taşımayı İptal Et")
}

pub fn continue_move(language: Language) -> &'static str {
    pick(language, "Continue Moving", "Taşımaya Devam Et")
}

pub fn leave(language: Language) -> &'static str {
    pick(
        language,
        "Leave Games Where They Are",
        "Oyunları Olduğu Yerde Bırak",
    )
}

pub fn leave_detail(language: Language) -> &'static str {
    pick(
        language,
        "They stay in the old location, unused. You unlock them again after that.",
        "Eski konumda kullanılmadan kalırlar. Sonra yeniden kilidini açarsın.",
    )
}

/// The note under the row when a move stopped before it was done.
pub fn note_left(count: usize, from: &Path, language: Language) -> String {
    let from = from.display();
    match (language, count) {
        (Language::English, 0) => format!(
            "Some unlocked games are still in {from}. Unlocking and playing wait until they are moved."
        ),
        (Language::English, 1) => format!(
            "1 unlocked game is still in {from}. Unlocking and playing wait until it is moved."
        ),
        (Language::English, n) => format!(
            "{n} unlocked games are still in {from}. Unlocking and playing wait until they are moved."
        ),
        (Language::Turkish, 0) => format!(
            "Kilidi açılan bazı oyunlar hâlâ {from} içinde. Taşınana kadar kilit açma ve oynama bekler."
        ),
        (Language::Turkish, n) => format!(
            "Kilidi açılan {n} oyun hâlâ {from} içinde. Taşınana kadar kilit açma ve oynama bekler."
        ),
        (Language::German, 0) => format!(
            "Einige freigeschaltete Spiele liegen noch in {from}. Freischalten und Spielen warten, bis sie verschoben sind."
        ),
        (Language::German, 1) => format!(
            "1 freigeschaltetes Spiel liegt noch in {from}. Freischalten und Spielen warten, bis es verschoben ist."
        ),
        (Language::German, n) => format!(
            "{n} freigeschaltete Spiele liegen noch in {from}. Freischalten und Spielen warten, bis sie verschoben sind."
        ),
        (Language::Spanish, 0) => format!(
            "Algunos juegos desbloqueados siguen en {from}. Desbloquear y jugar esperan hasta que se trasladen."
        ),
        (Language::Spanish, 1) => format!(
            "1 juego desbloqueado sigue en {from}. Desbloquear y jugar esperan hasta que se traslade."
        ),
        (Language::Spanish, n) => format!(
            "{n} juegos desbloqueados siguen en {from}. Desbloquear y jugar esperan hasta que se trasladen."
        ),
        (Language::French, 0) => format!(
            "Certains jeux déverrouillés sont encore dans {from}. Le déverrouillage et le jeu attendent qu’ils soient déplacés."
        ),
        (Language::French, 1) => format!(
            "1 jeu déverrouillé est encore dans {from}. Le déverrouillage et le jeu attendent qu’il soit déplacé."
        ),
        (Language::French, n) => format!(
            "{n} jeux déverrouillés sont encore dans {from}. Le déverrouillage et le jeu attendent qu’ils soient déplacés."
        ),
        (Language::Portuguese, 0) => format!(
            "Alguns jogos desbloqueados ainda estão em {from}. Desbloquear e jogar aguardam até eles serem movidos."
        ),
        (Language::Portuguese, 1) => format!(
            "1 jogo desbloqueado ainda está em {from}. Desbloquear e jogar aguardam até ele ser movido."
        ),
        (Language::Portuguese, n) => format!(
            "{n} jogos desbloqueados ainda estão em {from}. Desbloquear e jogar aguardam até eles serem movidos."
        ),
    }
}

/// A toast: why the picked folder cannot be used.
pub fn refusal(why: &Refusal, language: Language) -> &'static str {
    match why {
        Refusal::NotAbsolute | Refusal::NotAFolder => pick(
            language,
            "That folder is not there. Choose another one.",
            "Bu klasör yok. Başka birini seç.",
        ),
        Refusal::InsideGamesFolder => pick(
            language,
            "Choose a folder outside your games folder. Your dumps are never written to.",
            "Oyun klasörünün dışında bir klasör seç. Dökümlere hiçbir şey yazılmaz.",
        ),
        Refusal::InsideStore => pick(
            language,
            "Choose a folder outside the .AnyPS5 folders.",
            ".AnyPS5 klasörlerinin dışında bir klasör seç.",
        ),
        Refusal::NotWritable(_) => pick(
            language,
            "Cannot write to that folder. Choose another one.",
            "Bu klasöre yazılamıyor. Başka birini seç.",
        ),
    }
}

/// A toast: the location cannot change right now.
pub fn busy_playing(language: Language) -> &'static str {
    pick(
        language,
        "Close the running game first.",
        "Önce çalışan oyunu kapat.",
    )
}

pub fn busy_unlocking(language: Language) -> &'static str {
    pick(
        language,
        "Wait until unlocking is done.",
        "Kilit açma bitene kadar bekle.",
    )
}

pub fn busy_moving(language: Language) -> &'static str {
    pick(
        language,
        "Finish or cancel the move first.",
        "Önce taşımayı bitir ya da iptal et.",
    )
}

/// A toast: the games moved.
pub fn moved(language: Language) -> &'static str {
    pick(
        language,
        "Unlocked games moved.",
        "Kilidi açılan oyunlar taşındı.",
    )
}

/// A toast: the move stopped with games left in the old location.
pub fn stopped(left: usize, language: Language) -> String {
    match language {
        Language::English => format!(
            "{left} {} left in the old location. Continue the move in Settings.",
            if left == 1 { "game was" } else { "games were" }
        ),
        Language::Turkish => {
            format!("{left} oyun eski konumunda kaldı. Taşımaya Ayarlar'dan devam et.")
        }
        Language::German => format!(
            "{left} {} am alten Ort. Setz das Verschieben in den Einstellungen fort.",
            if left == 1 {
                "Spiel liegt noch"
            } else {
                "Spiele liegen noch"
            }
        ),
        Language::Spanish => format!(
            "{left} {} en la ubicación anterior. Continúa el traslado en Ajustes.",
            if left == 1 {
                "juego sigue"
            } else {
                "juegos siguen"
            }
        ),
        Language::French => format!(
            "{left} {} à l’ancien emplacement. Reprends le déplacement dans Paramètres.",
            if left <= 1 {
                "jeu est resté"
            } else {
                "jeux sont restés"
            }
        ),
        Language::Portuguese => format!(
            "{left} {} no local antigo. Continue a mudança em Configurações.",
            if left <= 1 {
                "jogo ficou"
            } else {
                "jogos ficaram"
            }
        ),
    }
}

/// A toast: the new drive is full.
pub fn no_space(needed: u64, free: u64, language: Language) -> String {
    let (needed, free) = (
        byte_text(needed.min(i64::MAX as u64) as i64, language),
        byte_text(free.min(i64::MAX as u64) as i64, language),
    );
    match language {
        Language::English => format!(
            "The new drive is full: the next game needs {needed}, {free} free. Free some space, then continue."
        ),
        Language::Turkish => format!(
            "Yeni sürücü dolu: sıradaki oyun {needed} istiyor, {free} boş. Yer açıp devam et."
        ),
        Language::German => format!(
            "Das neue Laufwerk ist voll: Das nächste Spiel braucht {needed}, frei sind {free}. Schaffe Platz und mach dann weiter."
        ),
        Language::Spanish => format!(
            "La unidad nueva está llena: el siguiente juego necesita {needed} y hay {free} libres. Libera espacio y continúa."
        ),
        Language::French => format!(
            "Le nouveau disque est plein\u{a0}: le prochain jeu demande {needed}, {free} libres. Libère de l’espace, puis continue."
        ),
        Language::Portuguese => format!(
            "A nova unidade está cheia: o próximo jogo precisa de {needed}, e há {free} livres. Libere espaço e continue."
        ),
    }
}

/// The failure of a play or unlock command while games are still being moved.
pub fn waits_for_move(language: Language) -> &'static str {
    pick(
        language,
        "Your unlocked games are being moved. Try again when that is done.",
        "Kilidi açılan oyunların taşınıyor. Bitince yeniden dene.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_row_names_the_place_and_the_free_space() {
        let en = Language::English;
        assert_eq!(
            detail(None, Some(3_000_000_000), false, en),
            "Next to your games \u{b7} 3.00 GB free"
        );
        assert_eq!(
            detail(Some(Path::new("E:\\Unlocked")), None, true, en),
            "E:\\Unlocked \u{b7} Not found"
        );
        assert_eq!(detail(Some(Path::new("E:\\U")), None, false, en), "E:\\U");
    }

    #[test]
    fn counts_agree_with_their_nouns() {
        let from = Path::new("D:\\Old");
        assert!(note_left(1, from, Language::English).starts_with("1 unlocked game is "));
        assert!(note_left(0, from, Language::English).starts_with("Some unlocked games"));
        assert!(note_left(3, from, Language::English).starts_with("3 unlocked games are "));
        assert!(stopped(1, Language::English).starts_with("1 game was "));
        for language in Language::ALL {
            for count in [0, 1, 2, 7] {
                assert!(note_left(count, from, language).contains("D:\\Old"));
                assert!(!stopped(count, language).is_empty());
            }
        }
        assert!(note_left(1, from, Language::Spanish).starts_with("1 juego desbloqueado sigue "));
        assert!(
            note_left(2, from, Language::Spanish).starts_with("2 juegos desbloqueados siguen ")
        );
        assert!(note_left(1, from, Language::French).starts_with("1 jeu d"));
        assert!(note_left(2, from, Language::French).starts_with("2 jeux d"));
        assert!(
            note_left(1, from, Language::Portuguese).starts_with("1 jogo desbloqueado ainda está ")
        );
        assert!(
            note_left(2, from, Language::Portuguese)
                .starts_with("2 jogos desbloqueados ainda estão ")
        );
        assert_eq!(
            moving(0, 7, 4, Language::English),
            "Moving 1 of 7 \u{b7} 4%"
        );
        assert_eq!(
            moving(7, 7, 100, Language::English),
            "Moving 7 of 7 \u{b7} 100%"
        );
    }
}
