//! The body of "What's New" for a big release: a hero line and the headlines in sections, each
//! with an icon, instead of the list of every change. It shows once after the launcher updates to
//! the release (see `PanelHost::frame`) and again from About, "Show Again".
//!
//! The content is the table `SECTIONS` below, limited to what shipped in 0.3.0. An item that
//! did not ship is deleted from the table; nothing else changes.

use egui::{Align2, Color32, Rect, ScrollArea, Ui, UiBuilder, pos2, vec2};

use crate::ui::i18n::Language;
use crate::ui::icons::Icon;
use crate::ui::text;
use crate::ui::theme::{color, radius, space, ty};

/// The release this body presents. A newer release shows the list of its changes again.
pub const RELEASE: &str = anyps5_core::settings::WHATS_NEW_RELEASE;

/// English, Turkish and German side by side.
type Triple = (&'static str, &'static str, &'static str);

struct Item {
    icon: Icon,
    title: Triple,
    note: Triple,
}

struct Section {
    title: Triple,
    items: &'static [Item],
}

const SECTIONS: &[Section] = &[
    Section {
        title: ("Play together", "Birlikte oyna", "Zusammen spielen"),
        items: &[
            Item {
                icon: Icon::MICROPHONE,
                title: (
                    "Voice in games",
                    "Oyunlarda sesli sohbet",
                    "Sprache im Spiel",
                ),
                note: (
                    "Push to talk while a game is in front, a Talk Button on the pad, Open Mic and Reduce Background Noise.",
                    "Oyun öndeyken bas-konuş, kolda konuşma düğmesi, Açık Mikrofon ve Arka Plan Gürültüsünü Azalt.",
                    "Push-to-Talk, während ein Spiel vorne ist, eine Sprechtaste am Controller, offenes Mikrofon und Hintergrundgeräusche verringern.",
                ),
            },
            Item {
                icon: Icon::EXPORT,
                title: ("Go live", "Canlı yayın", "Live gehen"),
                note: (
                    "Stream to Twitch, YouTube or any RTMP address from the launcher (Windows).",
                    "Başlatıcıdan Twitch, YouTube veya herhangi bir RTMP adresine yayın yap (Windows).",
                    "Streame aus dem Launcher zu Twitch, YouTube oder einer RTMP-Adresse (Windows).",
                ),
            },
            Item {
                icon: Icon::BELL,
                title: (
                    "Notification Center",
                    "Bildirim Merkezi",
                    "Mitteilungszentrale",
                ),
                note: (
                    "Everything that happened in one list, your friends' online status and an invite link.",
                    "Olan her şey tek listede, arkadaşlarının çevrimiçi durumu ve bir davet bağlantısı.",
                    "Alles, was passiert ist, in einer Liste, der Online-Status deiner Freunde und ein Einladungslink.",
                ),
            },
            Item {
                icon: Icon::SMILEY,
                title: ("Emoji button", "Emoji düğmesi", "Emoji-Taste"),
                note: (
                    "Pick an emoji right from the message box.",
                    "Emojiyi doğrudan mesaj kutusundan seç.",
                    "Wähle ein Emoji direkt im Nachrichtenfeld.",
                ),
            },
        ],
    },
    Section {
        title: ("Your library", "Kitaplığın", "Deine Bibliothek"),
        items: &[
            Item {
                icon: Icon::CHECK_CIRCLE,
                title: (
                    "What players found",
                    "Oyuncuların bulguları",
                    "Was Spieler herausgefunden haben",
                ),
                note: (
                    "Every game shows Runs well, Runs with issues, Doesn't start or LAN, as reported by players.",
                    "Her oyun, oyuncuların bildirdiği şekilde Sorunsuz çalışıyor, Sorunlarla çalışıyor, Başlamıyor veya LAN gösterir.",
                    "Jedes Spiel zeigt, was Spieler melden: Läuft gut, Läuft mit Problemen, Startet nicht oder LAN.",
                ),
            },
            Item {
                icon: Icon::MAGNIFYING_GLASS,
                title: (
                    "Sort, filter, search",
                    "Sırala, süz, ara",
                    "Sortieren, filtern, suchen",
                ),
                note: (
                    "Storage per game, shelves on Home and a toast when new games show up.",
                    "Oyun başına depolama, ana ekranda raflar ve yeni oyunlar geldiğinde bir bildirim.",
                    "Speicher pro Spiel, Regale auf dem Startbildschirm und ein Hinweis, wenn neue Spiele auftauchen.",
                ),
            },
            Item {
                icon: Icon::SQUARES_FOUR,
                title: ("Game Hub", "Oyun Merkezi", "Spielzentrale"),
                note: (
                    "A trophy overview and your play time this week, all in one place.",
                    "Kupa özeti ve bu haftaki oynama süren, hepsi tek yerde.",
                    "Eine Trophäenübersicht und deine Spielzeit diese Woche, alles an einem Ort.",
                ),
            },
        ],
    },
    Section {
        title: ("Make it yours", "Sana göre", "Mach es zu deinem"),
        items: &[
            Item {
                icon: Icon::GAME_CONTROLLER,
                title: (
                    "Controller Center",
                    "Kontrolcü Merkezi",
                    "Controller-Zentrale",
                ),
                note: (
                    "Set up your controllers in one place and remap keys.",
                    "Kontrolcülerini tek yerden kur ve tuşları yeniden ata.",
                    "Richte deine Controller an einem Ort ein und belege Tasten neu.",
                ),
            },
            Item {
                icon: Icon::PALETTE,
                title: ("Looks and sound", "Görünüm ve ses", "Aussehen und Klang"),
                note: (
                    "The Game Colors theme, volumes per kind of sound, Larger Text and High Contrast.",
                    "Oyun Renkleri teması, ses türüne göre seviyeler, Daha Büyük Metin ve Yüksek Karşıtlık.",
                    "Das Thema Spielfarben, Lautstärke je Klangart, Größerer Text und Hoher Kontrast.",
                ),
            },
            Item {
                icon: Icon::TRANSLATE,
                title: ("German", "Almanca", "Deutsch"),
                note: (
                    "The launcher now speaks German.",
                    "Başlatıcı artık Almanca konuşuyor.",
                    "Der Launcher spricht jetzt Deutsch.",
                ),
            },
        ],
    },
    Section {
        title: (
            "Everywhere you play",
            "Her oynadığın yerde",
            "Überall, wo du spielst",
        ),
        items: &[
            Item {
                icon: Icon::STEAM_LOGO,
                title: (
                    "Steam Deck and Linux",
                    "Steam Deck ve Linux",
                    "Steam Deck und Linux",
                ),
                note: (
                    "Gaming Mode with an on-screen keyboard, linked copies of unlocked games and a Flatpak.",
                    "Ekran klavyeli Oyun Modu, kilidi açılmış oyunların bağlı kopyaları ve bir Flatpak.",
                    "Gaming-Modus mit Bildschirmtastatur, verknüpfte Kopien entsperrter Spiele und ein Flatpak.",
                ),
            },
            Item {
                icon: Icon::APP_WINDOW,
                title: ("On the Mac", "Mac'te", "Auf dem Mac"),
                note: (
                    "Add to Steam, and the PS button.",
                    "Steam'e ekle ve PS düğmesi.",
                    "Zu Steam hinzufügen und die PS-Taste.",
                ),
            },
            Item {
                icon: Icon::STETHOSCOPE,
                title: ("Send Report", "Rapor gönder", "Bericht senden"),
                note: (
                    "After a crash, send a report so we can fix what broke.",
                    "Bir çökmeden sonra, bozulanı düzeltebilmemiz için rapor gönder.",
                    "Sende nach einem Absturz einen Bericht, damit wir den Fehler beheben können.",
                ),
            },
        ],
    },
];

fn pick(triple: Triple, language: Language) -> &'static str {
    match language {
        Language::Turkish => triple.1,
        Language::German => triple.2,
        Language::English => triple.0,
    }
}

fn hero(language: Language) -> (&'static str, &'static str) {
    let headline: Triple = (
        "The biggest update yet.",
        "Şimdiye dek en büyük güncelleme.",
        "Das bisher größte Update.",
    );
    let lead: Triple = (
        "Talk in games, go live, see what runs well and make the launcher yours.",
        "Oyunlarda konuş, canlı yayın yap, neyin iyi çalıştığını gör ve başlatıcıyı kendine göre ayarla.",
        "Sprich im Spiel, geh live, sieh, was gut läuft, und mach den Launcher zu deinem.",
    );
    (pick(headline, language), pick(lead, language))
}

/// The widest the content gets: past this the hero and cards stay readable instead of stretching.
const MAX_WIDTH: f32 = 1180.0;
/// Two columns of cards from this width on.
const TWO_COLUMNS: f32 = 980.0;
const CARD_PAD: f32 = space::L;
const ICON_BOX: f32 = 48.0;

/// Paints the hero and the sections in `body`, scrolling when they are taller. `offset` is the
/// scroll position the pad asked for (`Some`), and gets the position the area ended at.
pub fn show(
    ui: &mut Ui,
    body: Rect,
    language: Language,
    version: &str,
    accent: Color32,
    offset: &mut f32,
    set_offset: bool,
) {
    let mut child = ui.new_child(UiBuilder::new().max_rect(body));
    let mut area = ScrollArea::vertical()
        .id_salt("welcome-scroll")
        .auto_shrink([false, false]);
    if set_offset {
        area = area.vertical_scroll_offset(*offset);
    }
    let output = area.show(&mut child, |ui| {
        let width = ui.available_width().min(MAX_WIDTH);
        let painter = ui.painter().clone();
        let columns = if width >= TWO_COLUMNS { 2 } else { 1 };
        let gap = space::M;
        let card_w = (width - gap * (columns as f32 - 1.0)) / columns as f32;
        let text_w = card_w - 2.0 * CARD_PAD - ICON_BOX - space::M;

        // Lay everything out first: the height of the content is the sum of what is painted.
        let (headline, lead) = hero(language);
        let overline = text::layout(
            &painter,
            ty::OVERLINE,
            &format!(
                "ANYPLAY LAUNCHER {}",
                version.trim_start_matches('v').to_uppercase()
            ),
            color::text_caption(),
            width,
        );
        let headline = text::layout(&painter, ty::DISPLAY, headline, color::text(), width);
        let lead = text::layout(
            &painter,
            ty::BODY,
            lead,
            color::text_subtitle(),
            width.min(760.0),
        );

        struct Card {
            title: std::sync::Arc<egui::Galley>,
            note: std::sync::Arc<egui::Galley>,
            icon: Icon,
            height: f32,
        }
        let sections: Vec<(std::sync::Arc<egui::Galley>, Vec<Vec<Card>>)> = SECTIONS
            .iter()
            .map(|section| {
                let caption = text::layout(
                    &painter,
                    ty::OVERLINE,
                    &pick(section.title, language).to_uppercase(),
                    color::text_caption(),
                    width,
                );
                let cards: Vec<Card> = section
                    .items
                    .iter()
                    .map(|item| {
                        let title = text::layout(
                            &painter,
                            ty::ROW_TITLE,
                            pick(item.title, language),
                            color::text(),
                            text_w,
                        );
                        let note = text::layout(
                            &painter,
                            ty::DETAIL_LARGE,
                            pick(item.note, language),
                            color::text_detail(),
                            text_w,
                        );
                        let content = title.size().y + space::XS + note.size().y;
                        Card {
                            title,
                            note,
                            icon: item.icon,
                            height: 2.0 * CARD_PAD + content.max(ICON_BOX),
                        }
                    })
                    .collect();
                let rows: Vec<Vec<Card>> = {
                    let mut rows: Vec<Vec<Card>> = Vec::new();
                    for card in cards {
                        match rows.last_mut() {
                            Some(row) if row.len() < columns => row.push(card),
                            _ => rows.push(vec![card]),
                        }
                    }
                    rows
                };
                (caption, rows)
            })
            .collect();

        let hero_h = overline.size().y
            + space::M
            + headline.size().y
            + space::M
            + lead.size().y
            + space::XXL;
        let mut total = hero_h;
        for (caption, rows) in &sections {
            total += caption.size().y + space::M;
            for row in rows {
                total += row.iter().map(|c| c.height).fold(0.0, f32::max) + gap;
            }
            total += space::XXL - gap;
        }
        total += space::XL;

        let (rect, _) =
            ui.allocate_exact_size(vec2(ui.available_width(), total), egui::Sense::hover());
        let x = rect.min.x;
        let mut y = rect.min.y;
        let ink_x = |painter: &egui::Painter, galley: std::sync::Arc<egui::Galley>, y: f32| {
            painter.galley(pos2(x, y), galley, Color32::WHITE);
        };
        let h = overline.size().y;
        ink_x(&painter, overline, y);
        y += h + space::M;
        let h = headline.size().y;
        ink_x(&painter, headline, y);
        y += h + space::M;
        let h = lead.size().y;
        ink_x(&painter, lead, y);
        y += h + space::XXL;

        for (caption, rows) in sections {
            let h = caption.size().y;
            ink_x(&painter, caption, y);
            y += h + space::M;
            for row in rows {
                let row_h = row.iter().map(|c| c.height).fold(0.0, f32::max);
                for (column, card) in row.into_iter().enumerate() {
                    let cx = x + column as f32 * (card_w + gap);
                    let card_rect = Rect::from_min_size(pos2(cx, y), vec2(card_w, row_h));
                    painter.rect_filled(
                        card_rect,
                        crate::ui::components::corner(radius::CARD),
                        color::surface(),
                    );
                    let icon_rect = Rect::from_min_size(
                        pos2(cx + CARD_PAD, y + CARD_PAD),
                        vec2(ICON_BOX, ICON_BOX),
                    );
                    painter.rect_filled(
                        icon_rect,
                        crate::ui::components::corner(radius::ROW),
                        crate::ui::theme::fade(accent, 0.18),
                    );
                    painter.text(
                        icon_rect.center(),
                        Align2::CENTER_CENTER,
                        card.icon.glyph(),
                        card.icon.font(24.0, true),
                        accent,
                    );
                    let tx = cx + CARD_PAD + ICON_BOX + space::M;
                    let title_h = card.title.size().y;
                    painter.galley(pos2(tx, y + CARD_PAD), card.title, Color32::WHITE);
                    painter.galley(
                        pos2(tx, y + CARD_PAD + title_h + space::XS),
                        card.note,
                        Color32::WHITE,
                    );
                }
                y += row_h + gap;
            }
            y += space::XXL - gap;
        }
    });
    *offset = output.state.clamped_offset().y;
}

/// The About row that opens the showcase again: its detail.
pub fn show_again(language: Language) -> &'static str {
    pick(
        ("Show again", "Yeniden göster", "Erneut anzeigen"),
        language,
    )
}
