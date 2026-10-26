//! The body of "What's New" for a big release: a hero line and the headlines in sections, each
//! with an icon, instead of the list of every change. It shows once after the launcher updates to
//! the release (see `PanelHost::frame`) and again from About, "Show Again".
//!
//! The content is the table `SECTIONS` below, written from `docs/releases/v0.3.0.md`. An item that
//! did not ship is deleted from the table; nothing else changes.

use egui::{Align2, Color32, Rect, ScrollArea, Ui, UiBuilder, pos2, vec2};

use crate::ui::i18n::Language;
use crate::ui::icons::Icon;
use crate::ui::text;
use crate::ui::theme::{color, radius, space, ty};

/// The release this body presents. A newer release shows the list of its changes again.
pub const RELEASE: &str = anyps5_core::settings::WHATS_NEW_RELEASE;

/// English and Turkish side by side; German is looked up by the English text in `german`, and any
/// other language reads English until it has its own.
type Pair = (&'static str, &'static str);

struct Item {
    icon: Icon,
    title: Pair,
    note: Pair,
}

struct Section {
    title: Pair,
    items: &'static [Item],
}

const SECTIONS: &[Section] = &[
    Section {
        title: ("Play together", "Birlikte oyna"),
        items: &[
            Item {
                icon: Icon::SQUARES_FOUR,
                title: ("Game Hub", "Oyun Merkezi"),
                note: (
                    "Everything about a game on one page.",
                    "Bir oyunla ilgili her şey tek sayfada.",
                ),
            },
            Item {
                icon: Icon::USERS,
                title: ("Looking for Group", "Grup Arama"),
                note: (
                    "Find players for a game, and share your captures with friends.",
                    "Bir oyun için oyuncu bul ve ekran görüntülerini arkadaşlarınla paylaş.",
                ),
            },
            Item {
                icon: Icon::EXPORT,
                title: ("Live broadcast", "Canlı yayın"),
                note: (
                    "Go live to Twitch or YouTube from the launcher.",
                    "Başlatıcıdan Twitch veya YouTube'da canlı yayın yap.",
                ),
            },
            Item {
                icon: Icon::MICROPHONE,
                title: ("Better voice chat", "Daha iyi sesli sohbet"),
                note: (
                    "Push-to-talk and noise suppression.",
                    "Bas-konuş ve gürültü engelleme.",
                ),
            },
            Item {
                icon: Icon::SPARKLE,
                title: ("Emoji in messages", "Mesajlarda emoji"),
                note: (
                    "An emoji button in the message box.",
                    "Mesaj kutusunda bir emoji düğmesi.",
                ),
            },
        ],
    },
    Section {
        title: ("Your launcher", "Başlatıcın"),
        items: &[
            Item {
                icon: Icon::ENVELOPE_SIMPLE,
                title: ("Notification Center", "Bildirim Merkezi"),
                note: (
                    "Everything that happened, in one list.",
                    "Olan her şey tek bir listede.",
                ),
            },
            Item {
                icon: Icon::GAME_CONTROLLER,
                title: ("Controller Center", "Kontrolcü Merkezi"),
                note: (
                    "Set up and test your controllers in one place.",
                    "Kontrolcülerini tek yerden kur ve dene.",
                ),
            },
            Item {
                icon: Icon::CHECK_CIRCLE,
                title: ("Compatibility", "Uyumluluk"),
                note: (
                    "See how well a game runs in the library, on Home and when a new game appears.",
                    "Bir oyunun ne kadar iyi çalıştığını kitaplıkta, ana ekranda ve yeni oyun göründüğünde gör.",
                ),
            },
            Item {
                icon: Icon::PALETTE,
                title: ("Make it yours", "Senin tarzın"),
                note: (
                    "The Game Colors theme, sound volumes, Larger Text and High Contrast.",
                    "Oyun Renkleri teması, ses seviyeleri, Daha Büyük Metin ve Yüksek Karşıtlık.",
                ),
            },
            Item {
                icon: Icon::TRANSLATE,
                title: ("More languages", "Daha fazla dil"),
                note: (
                    "German and more, so the launcher speaks your language.",
                    "Almanca ve daha fazlası: başlatıcı senin dilini konuşur.",
                ),
            },
        ],
    },
    Section {
        title: ("Everywhere you play", "Her oynadığın yerde"),
        items: &[
            Item {
                icon: Icon::STEAM_LOGO,
                title: ("Steam Deck and Linux", "Steam Deck ve Linux"),
                note: (
                    "Steam Deck support, a Flatpak, and game folders on other drives through symlinks.",
                    "Steam Deck desteği, bir Flatpak ve sembolik bağlantılarla diğer sürücülerdeki oyun klasörleri.",
                ),
            },
            Item {
                icon: Icon::APP_WINDOW,
                title: ("Mac: Add to Steam", "Mac: Steam'e ekle"),
                note: (
                    "Add the launcher and your games to Steam on the Mac.",
                    "Mac'te başlatıcıyı ve oyunlarını Steam'e ekle.",
                ),
            },
        ],
    },
    Section {
        title: ("Steadier", "Daha kararlı"),
        items: &[Item {
            icon: Icon::STETHOSCOPE,
            title: ("Watchdog and crash reports", "Bekçi ve çökme raporları"),
            note: (
                "A stuck launcher is noticed, and crash reports help us fix what broke.",
                "Takılan başlatıcı fark edilir, çökme raporları bozulanı düzeltmemize yardım eder.",
            ),
        }],
    },
];

fn pick(pair: Pair, language: Language) -> &'static str {
    match language {
        Language::Turkish => pair.1,
        Language::German => german(pair.0).unwrap_or(pair.0),
        _ => pair.0,
    }
}

/// German for an English text of the table; `None` reads English.
fn german(english: &str) -> Option<&'static str> {
    Some(match english {
        "Play together" => "Zusammen spielen",
        "Game Hub" => "Spiele-Hub",
        "Everything about a game on one page." => "Alles zu einem Spiel auf einer Seite.",
        "Looking for Group" => "Mitspieler finden",
        "Find players for a game, and share your captures with friends." => {
            "Finde Mitspieler für ein Spiel und teile deine Aufnahmen mit Freunden."
        }
        "Live broadcast" => "Live-Übertragung",
        "Go live to Twitch or YouTube from the launcher." => {
            "Gehe direkt aus dem Launcher auf Twitch oder YouTube live."
        }
        "Better voice chat" => "Besserer Sprachchat",
        "Push-to-talk and noise suppression." => "Push-to-Talk und Rauschunterdrückung.",
        "Emoji in messages" => "Emoji in Nachrichten",
        "An emoji button in the message box." => "Eine Emoji-Taste im Nachrichtenfeld.",
        "Your launcher" => "Dein Launcher",
        "Notification Center" => "Benachrichtigungen",
        "Everything that happened, in one list." => "Alles, was passiert ist, in einer Liste.",
        "Controller Center" => "Controller-Zentrale",
        "Set up and test your controllers in one place." => {
            "Richte deine Controller an einem Ort ein und teste sie."
        }
        "Compatibility" => "Kompatibilität",
        "See how well a game runs in the library, on Home and when a new game appears." => {
            "Sieh in der Bibliothek, auf der Startseite und bei neuen Spielen, wie gut ein Spiel läuft."
        }
        "Make it yours" => "Ganz nach deinem Geschmack",
        "The Game Colors theme, sound volumes, Larger Text and High Contrast." => {
            "Das Design Spielfarben, Lautstärken, größerer Text und hoher Kontrast."
        }
        "More languages" => "Mehr Sprachen",
        "German and more, so the launcher speaks your language." => {
            "Deutsch und mehr, damit der Launcher deine Sprache spricht."
        }
        "Everywhere you play" => "Überall, wo du spielst",
        "Steam Deck and Linux" => "Steam Deck und Linux",
        "Steam Deck support, a Flatpak, and game folders on other drives through symlinks." => {
            "Unterstützung für das Steam Deck, ein Flatpak und Spielordner auf anderen Laufwerken über Symlinks."
        }
        "Mac: Add to Steam" => "Mac: Zu Steam hinzufügen",
        "Add the launcher and your games to Steam on the Mac." => {
            "Füge auf dem Mac den Launcher und deine Spiele zu Steam hinzu."
        }
        "Steadier" => "Stabiler",
        "Watchdog and crash reports" => "Wächter und Absturzberichte",
        "A stuck launcher is noticed, and crash reports help us fix what broke." => {
            "Ein hängender Launcher wird bemerkt, und Absturzberichte helfen uns, Fehler zu beheben."
        }
        "Show again" => "Erneut anzeigen",
        "The biggest update yet." => "Das bisher größte Update.",
        "Play together, talk, go live and make the launcher yours." => {
            "Spiele zusammen, sprich miteinander, geh live und mach den Launcher zu deinem."
        }
        _ => return None,
    })
}

fn hero(language: Language) -> (&'static str, &'static str) {
    pick_two(
        language,
        (
            "The biggest update yet.",
            "Play together, talk, go live and make the launcher yours.",
        ),
        (
            "Şimdiye dek en büyük güncelleme.",
            "Birlikte oyna, konuş, canlı yayın yap ve başlatıcıyı kendine göre ayarla.",
        ),
    )
}

fn pick_two(
    language: Language,
    en: (&'static str, &'static str),
    tr: (&'static str, &'static str),
) -> (&'static str, &'static str) {
    match language {
        Language::Turkish => tr,
        Language::German => (german(en.0).unwrap_or(en.0), german(en.1).unwrap_or(en.1)),
        _ => en,
    }
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
    pick(("Show again", "Yeniden göster"), language)
}
