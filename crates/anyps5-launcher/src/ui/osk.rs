//! The on-screen keyboard for every text row (docs/DESIGN.md 5.22): a pad has no keys, so a text
//! row that is selected by the pad (or anywhere in Gaming Mode) opens this card instead of the
//! caret. Keys form a grid the pad walks; Select types the key, Back cancels, Options deletes.
//!
//! The session lives in the egui context, so a row can open it and read its text back without the
//! screens knowing anything about it: [`open`] starts it, [`take_output`] returns what was typed
//! (and whether Done ended it), [`command`] feeds it the pad and [`show`] draws it over the page.

use egui::{Area, Context, Id, Order, Rect, UiBuilder, pos2, vec2};

use crate::ui::anim::{AnimExt, Motion};
use crate::ui::components::{ConsoleButton, ControlSize, Variant, paint_scrim, paint_shadow};
use crate::ui::glass::{GlassStyle, paint_glass};
use crate::ui::i18n::Language;
use crate::ui::input::Command;
use crate::ui::sound::Cue;
use crate::ui::text;
use crate::ui::theme::{self, color, radius, shadows, size, space, ty};

const COLUMNS: usize = 10;
const PAD: f32 = space::XL;
const SCRIM: f32 = 0.55;
const RISE: f32 = 12.0;
/// The most characters of the typed text shown (the end is what the player needs).
const SHOWN: usize = 64;

const LETTERS: [&str; 3] = ["qwertyuiop", "asdfghjkl-", "zxcvbnm._@"];
const SYMBOLS: [&str; 3] = ["1234567890", "/:;()&=+#%", "!?*,~^\\|'\""];

#[derive(Clone, Debug)]
struct Session {
    target: Id,
    title: String,
    text: String,
    row: usize,
    col: usize,
    shift: bool,
    symbols: bool,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            target: Id::NULL,
            title: String::new(),
            text: String::new(),
            row: 0,
            col: 0,
            shift: false,
            symbols: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Key {
    Shift,
    Symbols,
    Space,
    Delete,
    Done,
}

const CONTROLS: [Key; 5] = [Key::Shift, Key::Symbols, Key::Space, Key::Delete, Key::Done];

fn session_id() -> Id {
    Id::unique("osk-session")
}

fn output_id(target: Id) -> Id {
    Id::unique(("osk-output", target))
}

/// Whether the launcher runs in Gaming Mode (Steam Deck, gamescope): read once.
pub fn gaming_mode() -> bool {
    static GAMING: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *GAMING.get_or_init(|| anyps5_core::platform::gaming_mode(&anyps5_core::platform::process_vars))
}

static CONSOLE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Console Mode is on (the preference, read at start): the pad is the device in use from the
/// first frame, as in Gaming Mode, until a key or the pointer says otherwise.
pub fn set_console_mode(on: bool) {
    CONSOLE.store(on, std::sync::atomic::Ordering::Relaxed);
}

pub fn console_mode() -> bool {
    CONSOLE.load(std::sync::atomic::Ordering::Relaxed)
}

/// Whether text rows open the keyboard instead of the caret: the pad is the device in use, or the
/// launcher runs in Gaming Mode. Set once a frame by the shell.
pub fn set_wanted(ctx: &Context, wanted: bool) {
    ctx.data_mut(|d| d.insert_temp(Id::unique("osk-wanted"), wanted));
}

pub fn wanted(ctx: &Context) -> bool {
    ctx.data(|d| d.get_temp::<bool>(Id::unique("osk-wanted")))
        .unwrap_or(false)
}

pub fn is_open(ctx: &Context) -> bool {
    ctx.data(|d| d.get_temp::<Session>(session_id())).is_some()
}

/// Opens the keyboard for the text `target` shows.
pub fn open(ctx: &Context, target: Id, title: &str, text: &str) {
    if is_open(ctx) {
        return;
    }
    let session = Session {
        target,
        title: title.to_owned(),
        text: text.to_owned(),
        row: 1,
        col: 4,
        shift: false,
        symbols: false,
    };
    ctx.data_mut(|d| d.insert_temp(session_id(), session));
    ctx.request_repaint();
}

/// What was typed for `target` since the last call, and whether Done ended it.
pub fn take_output(ctx: &Context, target: Id) -> Option<(String, bool)> {
    ctx.data_mut(|d| d.remove_temp::<(String, bool)>(output_id(target)))
}

/// The cue to play for the last key a click pressed.
pub fn take_cue(ctx: &Context) -> Option<Cue> {
    ctx.data_mut(|d| d.remove_temp::<Option<Cue>>(Id::unique("osk-cue")))
        .flatten()
}

fn rows(symbols: bool) -> &'static [&'static str; 3] {
    if symbols { &SYMBOLS } else { &LETTERS }
}

/// The characters of a grid row, capitals when `shift` is on.
fn row_keys(symbols: bool, shift: bool, row: usize) -> Vec<String> {
    rows(symbols)[row]
        .chars()
        .map(|c| {
            if shift && !symbols {
                c.to_uppercase().collect()
            } else {
                c.to_string()
            }
        })
        .collect()
}

fn columns_of(row: usize) -> usize {
    if row == 3 { CONTROLS.len() } else { COLUMNS }
}

/// Where the cursor lands in `row` when it comes from column `col` of another row.
fn column_in(row: usize, from_row: usize, col: usize) -> usize {
    let (to, from) = (columns_of(row), columns_of(from_row));
    if to == from {
        col
    } else {
        ((col * to + from / 2) / from).min(to - 1)
    }
}

enum Outcome {
    Keep,
    Done,
}

impl Session {
    fn press(&mut self, row: usize, col: usize) -> Outcome {
        if row == 3 {
            match CONTROLS[col] {
                Key::Shift => self.shift = !self.shift,
                Key::Symbols => {
                    self.symbols = !self.symbols;
                    self.shift = false;
                }
                Key::Space => self.text.push(' '),
                Key::Delete => {
                    self.text.pop();
                }
                Key::Done => return Outcome::Done,
            }
        } else if let Some(key) = row_keys(self.symbols, self.shift, row).get(col) {
            self.text.push_str(key);
            // A capital is for one letter.
            self.shift = false;
        }
        Outcome::Keep
    }
}

fn store(ctx: &Context, session: Session, done: bool) {
    ctx.data_mut(|d| {
        d.insert_temp(output_id(session.target), (session.text.clone(), done));
        if done {
            d.remove_temp::<Session>(session_id());
        } else {
            d.insert_temp(session_id(), session);
        }
    });
    ctx.request_repaint();
}

/// A pad or key command while the keyboard is open. It takes every command and answers with the
/// cue to play; `None` when the keyboard is not open.
pub fn command(ctx: &Context, command: Command) -> Option<Cue> {
    let mut session = ctx.data(|d| d.get_temp::<Session>(session_id()))?;
    let (row, col) = (session.row, session.col);
    match command {
        Command::Back => {
            ctx.data_mut(|d| d.remove_temp::<Session>(session_id()));
            return Some(Cue::Back);
        }
        Command::Options => {
            if session.text.pop().is_some() {
                store(ctx, session, false);
            }
            return Some(Cue::Move);
        }
        Command::Select => {
            let done = matches!(session.press(row, col), Outcome::Done);
            store(ctx, session, done);
            return Some(if done { Cue::Select } else { Cue::Move });
        }
        Command::Left => {
            let n = columns_of(row);
            session.col = (col + n - 1) % n;
        }
        Command::Right => session.col = (col + 1) % columns_of(row),
        Command::Up => {
            let to = (row + 3) % 4;
            session.row = to;
            session.col = column_in(to, row, col);
        }
        Command::Down => {
            let to = (row + 1) % 4;
            session.row = to;
            session.col = column_in(to, row, col);
        }
    }
    ctx.data_mut(|d| d.insert_temp(session_id(), session));
    Some(Cue::Move)
}

struct Words {
    shift: &'static str,
    letters: &'static str,
    symbols: &'static str,
    space: &'static str,
    delete: &'static str,
    done: &'static str,
    empty: &'static str,
    hint: &'static str,
}

fn words(language: Language) -> Words {
    match language {
        Language::English => Words {
            shift: "Shift",
            letters: "abc",
            symbols: "123",
            space: "Space",
            delete: "Delete",
            done: "Done",
            empty: "Type with the keys",
            hint: "Select types a key. Options deletes. Back cancels.",
        },
        Language::German => Words {
            shift: "Umschalt",
            letters: "abc",
            symbols: "123",
            space: "Leerzeichen",
            delete: "Löschen",
            done: "Fertig",
            empty: "Mit den Tasten tippen",
            hint: "Auswählen tippt eine Taste. Optionen löscht. Zurück bricht ab.",
        },
        Language::Spanish => Words {
            shift: "Mayús",
            letters: "abc",
            symbols: "123",
            space: "Espacio",
            delete: "Borrar",
            done: "Listo",
            empty: "Escribe con las teclas",
            hint: "Seleccionar escribe una tecla. Opciones borra. Atrás cancela.",
        },
        Language::French => Words {
            shift: "Maj",
            letters: "abc",
            symbols: "123",
            space: "Espace",
            delete: "Effacer",
            done: "Terminé",
            empty: "Saisis avec les touches",
            hint: "Sélectionner tape une touche. Options efface. Retour annule.",
        },
        Language::Portuguese => Words {
            shift: "Maiús",
            letters: "abc",
            symbols: "123",
            space: "Espaço",
            delete: "Apagar",
            done: "Pronto",
            empty: "Digite com as teclas",
            hint: "Selecionar digita uma tecla. Opções apaga. Voltar cancela.",
        },
        Language::Turkish => Words {
            shift: "Shift",
            letters: "abc",
            symbols: "123",
            space: "Boşluk",
            delete: "Sil",
            done: "Bitti",
            empty: "Tuşlarla yaz",
            hint: "Seç tuşu yazar. Seçenekler siler. Geri iptal eder.",
        },
    }
}

/// Draws the keyboard when it is open, and while it leaves.
pub fn show(ctx: &Context, language: Language) {
    let current = ctx.data(|d| d.get_temp::<Session>(session_id()));
    let open = current.is_some();
    let v = ctx.animate_from_to(
        "osk-open",
        0.0,
        if open { 1.0 } else { 0.0 },
        Motion::Overlay,
    );
    if v <= 0.001 && !open {
        return;
    }
    let last = ctx.data(|d| d.get_temp::<Session>(Id::unique("osk-last")));
    let Some(session) = current.or(last) else {
        return;
    };
    ctx.data_mut(|d| d.insert_temp(Id::unique("osk-last"), session.clone()));
    let v = v.clamp(0.0, 1.0);
    let words = words(language);
    let screen = ctx.content_rect();
    let gap = size::CONTROL_GAP;
    let key_h = ControlSize::Medium.height();
    let field_h = ControlSize::Large.height();
    let card_w = (screen.width() - 2.0 * space::XL).min(760.0);
    let inner = card_w - 2.0 * PAD;
    let key_w = ((inner - gap * (COLUMNS as f32 - 1.0)) / COLUMNS as f32).floor();
    let title_h = ty::ROW_TITLE.line_height();
    let note_h = ty::NOTE.line_height();
    let card_h = PAD * 2.0
        + title_h
        + space::S
        + field_h
        + space::L
        + 4.0 * key_h
        + 3.0 * gap
        + space::L
        + note_h;
    let rise = if ctx.reduced_motion() {
        0.0
    } else {
        (1.0 - v) * RISE
    };
    let rect = Rect::from_center_size(screen.center() + vec2(0.0, rise), vec2(card_w, card_h));
    let mut presses: Vec<(usize, usize)> = Vec::new();

    Area::new(Id::unique("osk-area"))
        .order(Order::Foreground)
        .fixed_pos(screen.min)
        .interactable(true)
        .show(ctx, |ui| {
            ui.expand_to_include_rect(screen);
            let painter = ui.painter().clone();
            let mut faded = painter.clone();
            faded.set_opacity(v);
            paint_scrim(&painter, screen, SCRIM * v);
            paint_shadow(
                &painter,
                rect,
                radius::LARGE,
                egui::epaint::Shadow {
                    color: theme::fade(shadows::toast().color, v),
                    ..shadows::toast()
                },
            );
            let mut style = GlassStyle::glass(radius::LARGE, 0.6);
            style.black *= v;
            style.alpha = v;
            paint_glass(ctx, &painter, rect, style);

            let x = rect.min.x + PAD;
            let mut y = rect.min.y + PAD;
            text::paint(
                &faded,
                pos2(x, y),
                egui::Align2::LEFT_TOP,
                ty::ROW_TITLE,
                &session.title,
                color::text_label(),
            );
            y += title_h + space::S;

            // The typed text, its end, with a caret.
            let field = Rect::from_min_size(pos2(x, y), vec2(inner, field_h));
            faded.rect_filled(field, field_h * 0.5, color::surface());
            let at = pos2(field.min.x + space::XL, field.center().y);
            if session.text.is_empty() {
                text::paint(
                    &faded,
                    at,
                    egui::Align2::LEFT_CENTER,
                    ty::BODY,
                    words.empty,
                    color::text_note(),
                );
            } else {
                let chars: Vec<char> = session.text.chars().collect();
                let tail: String = chars[chars.len().saturating_sub(SHOWN)..].iter().collect();
                text::paint(
                    &faded,
                    at,
                    egui::Align2::LEFT_CENTER,
                    ty::BODY,
                    &format!("{tail}|"),
                    color::text(),
                );
            }
            ui.interact(field, Id::unique("osk-field"), egui::Sense::hover())
                .widget_info(|| {
                    egui::WidgetInfo::labeled(egui::Role::TextInput, true, session.title.as_str())
                });
            y += field_h + space::L;

            let mut child = ui.new_child(UiBuilder::new().max_rect(rect));
            child.set_opacity(v);
            for row in 0..4 {
                let top = y + row as f32 * (key_h + gap);
                if row < 3 {
                    for (col, label) in row_keys(session.symbols, session.shift, row)
                        .iter()
                        .enumerate()
                    {
                        let slot = Rect::from_min_size(
                            pos2(x + col as f32 * (key_w + gap), top),
                            vec2(key_w, key_h),
                        );
                        let focused = open && session.row == row && session.col == col;
                        if key(&mut child, slot, label, focused, Variant::Secondary) {
                            presses.push((row, col));
                        }
                    }
                } else {
                    let span = (COLUMNS / CONTROLS.len()) as f32;
                    let slot_w = span * key_w + (span - 1.0) * gap;
                    for (col, control) in CONTROLS.iter().enumerate() {
                        let slot = Rect::from_min_size(
                            pos2(x + col as f32 * (slot_w + gap), top),
                            vec2(slot_w, key_h),
                        );
                        let label = match control {
                            Key::Shift => words.shift,
                            Key::Symbols if session.symbols => words.letters,
                            Key::Symbols => words.symbols,
                            Key::Space => words.space,
                            Key::Delete => words.delete,
                            Key::Done => words.done,
                        };
                        let variant = if *control == Key::Done {
                            Variant::Primary
                        } else {
                            Variant::Secondary
                        };
                        let focused = open && session.row == 3 && session.col == col;
                        if key(&mut child, slot, label, focused, variant) {
                            presses.push((3, col));
                        }
                    }
                }
            }
            let note_y = y + 4.0 * key_h + 3.0 * gap + space::L;
            text::paint(
                &faded,
                pos2(rect.center().x, note_y),
                egui::Align2::CENTER_TOP,
                ty::NOTE,
                words.hint,
                color::text_detail(),
            );
        });

    // Clicks on keys, applied once the area is drawn.
    if open {
        for (row, col) in presses {
            let Some(mut current) = ctx.data(|d| d.get_temp::<Session>(session_id())) else {
                break;
            };
            current.row = row;
            current.col = col;
            let done = matches!(current.press(row, col), Outcome::Done);
            store(ctx, current, done);
            let cue = if done { Cue::Select } else { Cue::Move };
            ctx.data_mut(|d| d.insert_temp(Id::unique("osk-cue"), Some(cue)));
        }
    }
}

fn key(ui: &mut egui::Ui, slot: Rect, label: &str, focused: bool, variant: Variant) -> bool {
    let mut cell = ui.new_child(
        UiBuilder::new()
            .max_rect(slot)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    ConsoleButton::new(label)
        .size(ControlSize::Medium)
        .variant(variant)
        .focused(focused)
        .min_width(slot.width())
        .show(&mut cell)
        .clicked()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session() -> Session {
        Session {
            target: Id::unique("t"),
            title: "Name".into(),
            text: String::new(),
            row: 0,
            col: 0,
            shift: false,
            symbols: false,
        }
    }

    #[test]
    fn keys_type_and_a_capital_is_for_one_letter() {
        let mut s = session();
        s.press(0, 0);
        s.shift = true;
        s.press(1, 0);
        s.press(1, 1);
        assert_eq!(s.text, "qAs");
        assert!(!s.shift);
    }

    #[test]
    fn controls_delete_space_symbols_and_done() {
        let mut s = session();
        s.text = "ab".into();
        s.press(3, 3);
        assert_eq!(s.text, "a");
        s.press(3, 2);
        assert_eq!(s.text, "a ");
        s.press(3, 1);
        assert!(s.symbols);
        s.press(0, 0);
        assert_eq!(s.text, "a 1");
        assert!(matches!(s.press(3, 4), Outcome::Done));
    }

    #[test]
    fn the_cursor_keeps_its_place_between_rows_of_different_length() {
        assert_eq!(column_in(3, 0, 0), 0);
        assert_eq!(column_in(3, 0, 9), 4);
        assert_eq!(column_in(0, 3, 4), 8);
        assert_eq!(column_in(1, 0, 7), 7);
    }

    #[test]
    fn the_pad_walks_types_and_cancels() {
        let ctx = Context::default();
        let target = Id::unique("row");
        open(&ctx, target, "Cloud", "ab");
        assert!(is_open(&ctx));
        // Row 1 -> row 2 -> the control row, on Space.
        command(&ctx, Command::Down);
        command(&ctx, Command::Down);
        command(&ctx, Command::Select);
        assert_eq!(take_output(&ctx, target), Some(("ab ".into(), false)));
        // Delete is next to it.
        command(&ctx, Command::Right);
        command(&ctx, Command::Select);
        assert_eq!(take_output(&ctx, target), Some(("ab".into(), false)));
        command(&ctx, Command::Back);
        assert!(!is_open(&ctx));
        assert!(command(&ctx, Command::Select).is_none());
    }
}
