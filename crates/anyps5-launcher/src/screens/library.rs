//! The library grid (docs/DESIGN.md 5.10): every game as a tile, six to a row, opened from the
//! library button. Arrows move (up and down by a row), select or a click picks, back and options
//! close. The host decides what each tile shows (icon, state overlay, state label) and what picking
//! means.

use egui::{Align2, Color32, Context, Rect, Sense, Ui, UiBuilder, pos2, vec2};

use crate::screens::clamped;
use crate::screens::scroll::Follow;
use crate::screens::side::commands;
use crate::ui::Shell;
use crate::ui::anim::AnimExt;
use crate::ui::chrome::{TileImage, TileOverlay, TileStyle, fullscreen_panel, paint_tile};
use crate::ui::components::{ConsoleButton, ControlSize};
use crate::ui::glass::GlassStyle;
use crate::ui::i18n::{Key, Language};
use crate::ui::icons::Icon;
use crate::ui::input::{self, Command, InputEvent};
use crate::ui::sound::Cue;
use crate::ui::text;
use crate::ui::theme::{color, pad, size, ty};

/// Columns when the width is not known yet; the grid fills the width it gets (`columns_for`).
pub const COLUMNS: usize = 6;
const COLUMN_GAP: f32 = 26.0;
const ROW_GAP: f32 = 30.0;
const GRID_PADDING: f32 = 14.0;
/// Between the state line and the line of what players found.
const COMPAT_GAP: f32 = 4.0;

/// As many tiles as fit the width, at least three.
pub fn columns_for(width: f32) -> usize {
    let fit = ((width + COLUMN_GAP) / (size::LIBRARY_TILE + COLUMN_GAP)).floor();
    (fit.max(0.0) as usize).max(3)
}

/// The state line under a tile: a glyph and a word in the state's colour.
#[derive(Clone, Debug)]
pub struct StateText {
    pub icon: Icon,
    pub text: String,
    pub tint: Color32,
    /// The plain "Ready": kept for the accessible name, not drawn under the tile.
    pub quiet: bool,
}

/// One game of the grid.
#[derive(Clone, Debug)]
pub struct LibraryEntry {
    pub name: String,
    pub icon: Option<TileImage>,
    pub overlay: TileOverlay,
    pub state: StateText,
    /// What players found, in words under the state ("Runs well"); nothing when it is unknown.
    pub compat: Option<StateText>,
}

/// What the host has to do after a frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LibraryAction {
    /// Select this game and close the grid.
    Pick(usize),
    /// No games: close the grid and ask for the games folder.
    ChooseFolder,
    /// Close without a pick.
    Close,
    /// The bar: the next order, the next filter, the search page, the storage page, and
    /// "show all games" from the empty page.
    NextSort,
    NextFilter,
    OpenSearch,
    OpenStorage,
    ClearFilters,
}

/// What the bar above the grid says. Its stops, left to right, are the order, the filter, the
/// search and the storage page.
#[derive(Clone, Debug, Default)]
pub struct BarData {
    pub sort: String,
    pub filter: String,
    /// What was typed; the search button then says so.
    pub query: String,
    pub search: String,
    pub storage: String,
    /// A filter or a search hides games: an empty grid then offers to show them all.
    pub narrowed: bool,
    pub nothing_title: String,
    pub nothing_text: String,
    pub clear: String,
}

/// The stops of the bar.
pub const BAR_STOPS: usize = 4;

/// Where the focus sits on the library page.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum Zone {
    #[default]
    Grid,
    Bar(usize),
}

/// The games, borrowed for one frame.
pub struct LibraryData<'a> {
    pub entries: &'a [LibraryEntry],
    pub language: Language,
    pub accent: Color32,
    /// A games folder is set: the empty grid offers to change it, not to pick one.
    pub folder_set: bool,
    /// The games are being looked for: the empty grid asks for nothing yet.
    pub scanning: bool,
    pub bar: BarData,
}

/// The grid overlay.
#[derive(Default)]
pub struct LibraryGrid {
    open: bool,
    closing: bool,
    index: usize,
    /// Columns of the last frame, so the arrows move by the rows the player sees.
    columns: usize,
    zone: Zone,
    /// The stop of the bar the focus returns to.
    bar_stop: usize,
}

/// Where an arrow command moves the focus in a grid of `count` tiles.
pub fn move_index(index: usize, command: Command, count: usize, columns: usize) -> usize {
    let last = count.saturating_sub(1);
    match command {
        Command::Left => index.saturating_sub(1),
        Command::Right => (index + 1).min(last),
        Command::Up => index.saturating_sub(columns),
        Command::Down => (index + columns).min(last),
        _ => index,
    }
}

impl LibraryGrid {
    /// Opens with the focus on the host's selected game.
    pub fn open(&mut self, selected: usize) {
        self.open = true;
        self.closing = false;
        self.index = selected;
        self.zone = Zone::Grid;
    }

    pub fn close(&mut self) {
        if self.open {
            self.open = false;
            self.closing = true;
        }
    }

    /// Leaves without an exit: another panel takes the screen over.
    pub fn dismiss(&mut self) {
        self.open = false;
        self.closing = false;
    }

    pub fn is_active(&self) -> bool {
        self.open || self.closing
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn show(
        &mut self,
        shell: &mut Shell,
        ctx: &Context,
        events: &[InputEvent],
        data: &LibraryData<'_>,
    ) -> Vec<LibraryAction> {
        let mut actions = Vec::new();
        if !self.is_active() {
            return actions;
        }
        let count = data.entries.len();
        self.index = self.index.min(count.saturating_sub(1));
        if self.open {
            for command in commands(events) {
                if let Zone::Bar(stop) = self.zone {
                    match command {
                        Command::Left | Command::Right => {
                            let next = if command == Command::Left {
                                stop.saturating_sub(1)
                            } else {
                                (stop + 1).min(BAR_STOPS - 1)
                            };
                            self.bar_stop = next;
                            if next != stop {
                                self.zone = Zone::Bar(next);
                                shell.play(Cue::Move);
                            } else {
                                shell.play(Cue::Bump);
                            }
                        }
                        Command::Down if count > 0 => {
                            self.zone = Zone::Grid;
                            shell.play(Cue::Move);
                        }
                        Command::Up | Command::Down => shell.play(Cue::Bump),
                        Command::Select => {
                            shell.play(Cue::Select);
                            actions.push(bar_action(stop));
                        }
                        Command::Back | Command::Options => {
                            shell.play(Cue::Close);
                            self.close();
                            actions.push(LibraryAction::Close);
                        }
                    }
                    if !self.open {
                        break;
                    }
                    continue;
                }
                match command {
                    Command::Up if self.index < self.columns.max(1) && data.bar_visible() => {
                        self.zone = Zone::Bar(self.bar_stop);
                        shell.play(Cue::Move);
                    }
                    Command::Left | Command::Right | Command::Up | Command::Down => {
                        let columns = if self.columns == 0 {
                            COLUMNS
                        } else {
                            self.columns
                        };
                        let next = move_index(self.index, command, count, columns);
                        if next != self.index {
                            self.index = next;
                            shell.play(Cue::Move);
                        } else {
                            shell.play(Cue::Bump);
                        }
                    }
                    Command::Select if count > 0 => {
                        shell.play(Cue::Select);
                        self.close();
                        actions.push(LibraryAction::Pick(self.index));
                    }
                    Command::Select if data.bar.narrowed => {
                        shell.play(Cue::Select);
                        actions.push(LibraryAction::ClearFilters);
                    }
                    Command::Select if !data.scanning => {
                        shell.play(Cue::Select);
                        self.close();
                        actions.push(LibraryAction::ChooseFolder);
                    }
                    Command::Select => {}
                    Command::Back | Command::Options => {
                        shell.play(Cue::Close);
                        self.close();
                        actions.push(LibraryAction::Close);
                    }
                }
                if !self.open {
                    break;
                }
            }
        }
        let mut clicked = None;
        let mut bar_clicked = None;
        let mut clear = false;
        let mut folder = false;
        let mut columns = self.columns;
        let index = self.index;
        let zone = self.zone;
        let open = self.open;
        let drawn = fullscreen_panel(
            ctx,
            "library-grid",
            open,
            GlassStyle::fullscreen_panel(),
            |ui, _| {
                draw(
                    ui,
                    data,
                    (index, zone),
                    &mut clicked,
                    &mut folder,
                    &mut columns,
                    (&mut bar_clicked, &mut clear),
                )
            },
        );
        self.columns = columns;
        if drawn.is_none() {
            self.closing = false;
        }
        if let (true, Some(stop)) = (open, bar_clicked) {
            self.bar_stop = stop;
            self.zone = Zone::Bar(stop);
            shell.play(Cue::Select);
            actions.push(bar_action(stop));
        }
        if open && clear {
            shell.play(Cue::Select);
            actions.push(LibraryAction::ClearFilters);
        }
        if open && folder {
            shell.play(Cue::Select);
            self.close();
            actions.push(LibraryAction::ChooseFolder);
        }
        if let (true, Some(i)) = (open, clicked) {
            self.index = i;
            shell.play(Cue::Select);
            self.close();
            actions.push(LibraryAction::Pick(i));
        }
        actions
    }
}

fn bar_action(stop: usize) -> LibraryAction {
    match stop {
        0 => LibraryAction::NextSort,
        1 => LibraryAction::NextFilter,
        2 => LibraryAction::OpenSearch,
        _ => LibraryAction::OpenStorage,
    }
}

impl LibraryData<'_> {
    /// The bar is there once there are games to order, filter and search, or a filter hides them.
    fn bar_visible(&self) -> bool {
        !self.entries.is_empty() || self.bar.narrowed
    }
}

#[allow(clippy::too_many_arguments)]
fn draw(
    ui: &mut Ui,
    data: &LibraryData<'_>,
    (focus, zone): (usize, Zone),
    clicked: &mut Option<usize>,
    folder: &mut bool,
    columns_out: &mut usize,
    (bar_clicked, clear): (&mut Option<usize>, &mut bool),
) {
    let ctx = ui.ctx().clone();
    let painter = ui.painter().clone();
    let language = data.language;
    let area = ui.max_rect().shrink2(vec2(pad::LIBRARY.0, pad::LIBRARY.1));

    // Header: the title, the count, the hints on the right.
    let title_h = ty::SCREEN_TITLE.line_height();
    let title = text::paint(
        &painter,
        pos2(area.min.x, area.min.y + title_h * 0.5),
        Align2::LEFT_CENTER,
        ty::SCREEN_TITLE,
        language.text(Key::Library),
        color::text(),
    );
    if !data.entries.is_empty() {
        text::paint(
            &painter,
            pos2(title.max.x + 8.0 + 8.0, area.min.y + title_h * 0.5 + 2.0),
            Align2::LEFT_CENTER,
            ty::LIBRARY_COUNT,
            &data.entries.len().to_string(),
            color::text_count(),
        );
    }
    let header = Rect::from_min_size(area.min, vec2(area.width(), title_h));
    crate::screens::right_aligned_hints(ui, header, language);

    // The bar under the header: order, filter, search, storage. A button stays 12 dp from the next.
    let mut bar_bottom = header.max.y;
    if data.bar_visible() {
        let top = header.max.y + size::CONTROL_GAP;
        let bar = Rect::from_min_size(
            pos2(area.min.x, top),
            vec2(area.width(), ControlSize::Medium.height()),
        );
        bar_bottom = bar.max.y;
        let mut child = ui.new_child(
            UiBuilder::new()
                .max_rect(bar)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        child.spacing_mut().item_spacing.x = size::CONTROL_GAP;
        let search_label = if data.bar.query.trim().is_empty() {
            data.bar.search.clone()
        } else {
            data.bar.query.clone()
        };
        let stops = [
            (&data.bar.sort, Icon::ARROWS_DOWN_UP),
            (&data.bar.filter, Icon::LIST),
            (&search_label, Icon::MAGNIFYING_GLASS),
            (&data.bar.storage, Icon::HARD_DRIVE),
        ];
        for (i, (label, icon)) in stops.into_iter().enumerate() {
            let shown = text::middle_truncate(&painter, ty::CHOICE_VALUE, label, 320.0);
            let response = ConsoleButton::new(&shown)
                .icon(icon)
                .size(ControlSize::Medium)
                .accessible_name(label.clone())
                .focused(zone == Zone::Bar(i))
                .show(&mut child);
            if response.clicked() {
                *bar_clicked = Some(i);
            }
        }
    }

    // Nothing shown: with a filter or a search the page says so and offers all games; without
    // games it uses the words of the empty home, in the middle under the header.
    if data.entries.is_empty() {
        let middle = pos2(area.center().x, area.center().y);
        if data.bar.narrowed {
            text::paint(
                &painter,
                middle,
                Align2::CENTER_BOTTOM,
                ty::EMPTY_TITLE,
                &data.bar.nothing_title,
                color::text(),
            );
            let body = crate::screens::wrapped_centered(
                &painter,
                ty::BODY,
                &data.bar.nothing_text,
                color::text_empty(),
                area.width().min(720.0),
                3,
            );
            let body_height = body.size().y;
            painter.galley(pos2(middle.x, middle.y + 16.0), body, color::text_empty());
            let button = Rect::from_center_size(
                pos2(
                    middle.x,
                    middle.y + 16.0 + body_height + 24.0 + size::BUTTON_MIN_HEIGHT * 0.5,
                ),
                vec2(300.0, size::BUTTON_MIN_HEIGHT),
            );
            let mut child = ui.new_child(
                UiBuilder::new()
                    .max_rect(button)
                    .layout(egui::Layout::top_down(egui::Align::Center)),
            );
            if ConsoleButton::new(&data.bar.clear)
                .prominent(true)
                .focused(zone == Zone::Grid)
                .show(&mut child)
                .clicked()
            {
                *clear = true;
            }
            return;
        }
        text::paint(
            &painter,
            middle,
            Align2::CENTER_BOTTOM,
            ty::EMPTY_TITLE,
            language.text(Key::EmptyLibraryTitle),
            color::text(),
        );
        let body = crate::screens::wrapped_centered(
            &painter,
            ty::BODY,
            language.text(Key::EmptyLibraryText),
            color::text_empty(),
            area.width().min(720.0),
            3,
        );
        let body_height = body.size().y;
        painter.galley(pos2(middle.x, middle.y + 16.0), body, color::text_empty());
        // The one next action: the same button as the empty home.
        let button = Rect::from_center_size(
            pos2(
                middle.x,
                middle.y + 16.0 + body_height + 24.0 + size::BUTTON_MIN_HEIGHT * 0.5,
            ),
            vec2(260.0, size::BUTTON_MIN_HEIGHT),
        );
        let mut child = ui.new_child(
            UiBuilder::new()
                .max_rect(button)
                .layout(egui::Layout::top_down(egui::Align::Center)),
        );
        let title = language.text(if data.folder_set {
            Key::ChangeFolder
        } else {
            Key::PickFolder
        });
        // The only control of the page: it holds the focus, so a key or the pad sees what Select does.
        if !data.scanning
            && ConsoleButton::new(title)
                .icon(Icon::FOLDER)
                .prominent(true)
                .focused(true)
                .show(&mut child)
                .clicked()
        {
            *folder = true;
        }
        return;
    }

    // The grid.
    // The first column stands on the title's edge; the padding is room for the focus ring.
    let list = Rect::from_min_max(
        pos2(area.min.x - GRID_PADDING, bar_bottom + 26.0),
        pos2(area.max.x + GRID_PADDING, area.max.y),
    );
    let mut child = ui.new_child(UiBuilder::new().max_rect(list));
    let id = child.make_persistent_id("library-scroll");
    let columns = columns_for(area.width());
    *columns_out = columns;
    let focus_row = focus / columns;
    let (follow, scroll_to) = Follow::begin(&ctx, id, Some(focus_row));
    let mut area_scroll = egui::ScrollArea::vertical()
        .id_salt(id)
        .auto_shrink([false, false])
        .scroll_bar_visibility(egui::containers::scroll_area::ScrollBarVisibility::AlwaysHidden);
    if let Some(offset) = scroll_to {
        area_scroll = area_scroll.vertical_scroll_offset(offset);
    }
    let tile = size::LIBRARY_TILE;
    let name_style = |focused: bool| if focused { ty::FACT } else { ty::SENTENCE };
    // A second line under the state says what players found; every cell has room for it once
    // any game has one.
    let any_compat = data.entries.iter().any(|e| e.compat.is_some());
    let line = ty::STATE_LABEL_SMALL.line_height();
    let cell_h = tile
        + 10.0
        + ty::SENTENCE.line_height()
        + 10.0
        + line
        + if any_compat { COMPAT_GAP + line } else { 0.0 };
    let rows = data.entries.len().div_ceil(columns);
    let mut geometry = Vec::with_capacity(rows);
    let output = area_scroll.show(&mut child, |ui| {
        let top = ui.cursor().min.y + GRID_PADDING;
        let (content, _) = ui.allocate_exact_size(
            vec2(
                ui.available_width(),
                2.0 * GRID_PADDING + rows as f32 * cell_h + rows.saturating_sub(1) as f32 * ROW_GAP,
            ),
            Sense::hover(),
        );
        let _ = content;
        for row in 0..rows {
            let y = top + row as f32 * (cell_h + ROW_GAP);
            geometry.push((y - top, cell_h));
        }
        for (i, entry) in data.entries.iter().enumerate() {
            let (row, col) = (i / columns, i % columns);
            let x = area.min.x + col as f32 * (tile + COLUMN_GAP);
            let y = top + row as f32 * (cell_h + ROW_GAP);
            let cell = Rect::from_min_size(pos2(x, y), vec2(tile, cell_h));
            let focused = i == focus && zone == Zone::Grid;
            let response = ui.interact(cell, id.with(("cell", i)), Sense::click());
            crate::ui::a11y::follow(&ctx, response.id, focused);
            response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::Role::Button,
                    true,
                    match &entry.compat {
                        Some(c) => format!("{}, {}, {}", entry.name, entry.state.text, c.text),
                        None => format!("{}, {}", entry.name, entry.state.text),
                    },
                )
            });
            if input::hovered(&ctx, &response) {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
            if response.clicked() {
                *clicked = Some(i);
            }
            if !ui.is_rect_visible(cell) {
                continue;
            }
            let painter = ui.painter().clone();
            let t = ui.ctx().focus_t(("library-focus", i), focused);
            let mut style = TileStyle::new(data.accent);
            style.image = entry.icon;
            style.overlay = entry.overlay;
            style.focus = t;
            style.ring = true;
            style.time = ui.ctx().clock();
            style.hover = ui.ctx().animate_to(
                ("library-hover", i),
                f32::from(input::hovered(&ctx, &response)),
                crate::ui::anim::Motion::Hover,
            );
            paint_tile(
                &painter,
                Rect::from_min_size(cell.min, vec2(tile, tile)),
                &style,
            );
            // The name: one line, the focused tile's up to two so a long name reads in full (its state
            // line moves down inside the cell's own gap, nothing else shifts).
            let ns = name_style(focused);
            let ink = if focused {
                color::text()
            } else {
                color::text_detail()
            };
            let name_g = clamped(
                &painter,
                ns,
                &entry.name,
                ink,
                tile,
                if focused { 2 } else { 1 },
            );
            let extra = (name_g.size().y - ty::SENTENCE.line_height()).max(0.0);
            let ny = cell.min.y + tile + 10.0;
            painter.galley(pos2(cell.min.x, ny), name_g, ink);
            let mut sy = ny + ty::SENTENCE.line_height() + extra + 10.0;
            let mut lines = Vec::new();
            if !entry.state.quiet {
                lines.push(&entry.state);
            }
            if let Some(c) = &entry.compat {
                lines.push(c);
            }
            // The word, in the ink of its state; the glyph says it again for those who do not
            // read the colour.
            for state in lines {
                painter.text(
                    pos2(cell.min.x + 6.0, sy + line * 0.5),
                    Align2::CENTER_CENTER,
                    state.icon.glyph(),
                    state.icon.font(12.0, true),
                    state.tint,
                );
                let label_g = clamped(
                    &painter,
                    ty::STATE_LABEL_SMALL,
                    &state.text,
                    state.tint,
                    tile - 20.0,
                    1,
                );
                painter.galley(pos2(cell.min.x + 20.0, sy), label_g, state.tint);
                sy += line + COMPAT_GAP;
            }
        }
    });
    follow.end(
        &ctx,
        geometry,
        output.content_size.y,
        output.inner_rect.height(),
        output.state.clamped_offset().y,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrows_move_by_one_and_by_a_row() {
        assert_eq!(move_index(0, Command::Right, 9, 6), 1);
        assert_eq!(move_index(0, Command::Left, 9, 6), 0, "no wrap");
        assert_eq!(move_index(1, Command::Down, 9, 6), 7);
        assert_eq!(move_index(7, Command::Up, 9, 6), 1);
        assert_eq!(move_index(8, Command::Down, 9, 6), 8, "last tile stays");
        // A wider grid moves by its own row.
        assert_eq!(move_index(1, Command::Down, 20, 8), 9);
    }

    #[test]
    fn the_grid_fills_its_width() {
        assert_eq!(columns_for(6.0 * 196.0 + 5.0 * 26.0), 6);
        assert_eq!(columns_for(1740.0), 7);
        assert_eq!(columns_for(400.0), 3, "never fewer than three");
        assert_eq!(
            move_index(4, Command::Down, 9, 6),
            8,
            "clamped to the last tile"
        );
        assert_eq!(move_index(8, Command::Right, 9, 6), 8);
        assert_eq!(move_index(0, Command::Right, 0, 6), 0, "empty grid");
    }
}
