//! The Game Hub (docs/DESIGN.md 5.21): one full-screen page per game, opened from the home hero and
//! from the library. The game's art and name on top with the one primary action (Play, or Back to
//! Game while it runs); under them what the player may want to know about the game: trophies,
//! play time, this game's captures, friends who play it, Play Together, what players found and the
//! game's own music.
//!
//! The screen knows nothing of the model. The host hands it everything as words and pictures
//! ([`HubData`]) and carries out the [`HubAction`]s it answers with. What the screen keeps is the
//! open state and where the focus is. Arrows walk the buttons in reading order, select presses,
//! back and options leave; a click does the same.

use egui::{
    Align, Align2, Color32, Context, Id, Layout, Rect, Role, Sense, Ui, UiBuilder, pos2, vec2,
};

use crate::screens::hub_strings::HubText as H;
use crate::screens::side::commands;
use crate::screens::{BACK_ROW, back_button, clamped, footer_hints};
use crate::ui::Shell;
use crate::ui::chrome::{TileImage, TileStyle, fullscreen_panel, paint_tile};
use crate::ui::components::{
    ConsoleButton, ControlSize, HintSet, paint_grade_counts, paint_progress,
};
use crate::ui::glass::GlassStyle;
use crate::ui::i18n::Language;
use crate::ui::icons::Icon;
use crate::ui::input::{Command, InputEvent};
use crate::ui::sound::Cue;
use crate::ui::text;
use crate::ui::theme::{color, pad, size, space, ty};

/// The game's picture beside its name.
const ART: f32 = 144.0;
/// The room between the blocks, in both directions.
const GAP: f32 = space::XL;
/// The height of the first row (trophies, play time, captures) and of the second.
const ROW_ONE: f32 = 184.0;
const ROW_TWO: f32 = 116.0;
const COLUMNS: usize = 4;
const THUMB_WIDTH: f32 = 112.0;
const THUMB_HEIGHT: f32 = 63.0;
const BAR_HEIGHT: f32 = 6.0;
/// How many pictures the captures block can show.
pub const THUMBS: usize = 4;

/// What the host has to do after a frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubAction {
    /// Back or options: leave the page.
    Close,
    /// Play, or Back to Game, or whatever the game's state offers.
    Primary,
    /// The game's own trophy list.
    OpenTrophies,
    /// All games' trophies together.
    OpenOverview,
    /// The Captures area.
    OpenCaptures,
}

/// A value with a sentence under it; either may be missing.
#[derive(Clone, Debug, Default)]
pub struct Block {
    pub value: Option<String>,
    pub note: Option<String>,
}

impl Block {
    pub fn note(note: impl Into<String>) -> Block {
        Block {
            value: None,
            note: Some(note.into()),
        }
    }

    pub fn value(value: impl Into<String>, note: Option<String>) -> Block {
        Block {
            value: Some(value.into()),
            note,
        }
    }
}

/// The trophies block: the figure, the way to all of them and the latest one.
#[derive(Clone, Debug)]
pub struct TrophyBlock {
    /// `12 of 40`; empty for a game without a trophy list.
    pub figure: String,
    /// How far, 0 to 1; `None` without a list.
    pub fraction: Option<f32>,
    /// Platinum first.
    pub grades: [u32; 4],
    pub note: String,
}

/// The captures block: how many, and the newest pictures.
#[derive(Clone, Debug, Default)]
pub struct CaptureBlock {
    /// `5 captures`; `None` when there are none.
    pub count: Option<String>,
    /// The newest first, up to [`THUMBS`]; `None` is a picture still on its way.
    pub thumbs: Vec<Option<TileImage>>,
    pub empty: String,
}

pub struct HubData<'a> {
    pub language: Language,
    pub accent: Color32,
    pub name: &'a str,
    /// Under the name: the version and the state, one line.
    pub subtitle: String,
    pub art: Option<TileImage>,
    pub primary_title: String,
    pub primary_icon: Icon,
    /// The primary action is one the player may take now (not while the game starts).
    pub primary_ready: bool,
    pub trophies: TrophyBlock,
    /// The trophy list can be opened (the game has one).
    pub has_trophy_list: bool,
    pub time: Block,
    pub captures: CaptureBlock,
    pub friends: Block,
    pub together: Block,
    pub compat: Block,
    pub music: Block,
}

/// A button of the page, in reading order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Stop {
    Primary,
    TrophyList,
    Overview,
    Captures,
}

impl Stop {
    fn action(self) -> HubAction {
        match self {
            Stop::Primary => HubAction::Primary,
            Stop::TrophyList => HubAction::OpenTrophies,
            Stop::Overview => HubAction::OpenOverview,
            Stop::Captures => HubAction::OpenCaptures,
        }
    }
}

/// The buttons the page has for this data, in the order the arrows walk them.
pub(crate) fn stops(data: &HubData<'_>) -> Vec<Stop> {
    let mut stops = vec![Stop::Primary];
    if data.has_trophy_list {
        stops.push(Stop::TrophyList);
    }
    stops.push(Stop::Overview);
    if data.captures.count.is_some() {
        stops.push(Stop::Captures);
    }
    stops
}

/// Where the focus goes for a command, or `None` when it stays (the end of the list).
pub(crate) fn step(index: usize, count: usize, command: Command) -> Option<usize> {
    let next = match command {
        Command::Up | Command::Left => index.checked_sub(1)?,
        Command::Down | Command::Right => index + 1,
        _ => return None,
    };
    (next < count).then_some(next)
}

#[derive(Default)]
pub struct HubScreen {
    open: bool,
    closing: bool,
    /// The focused button, by its place in [`stops`].
    index: usize,
    back_clicked: bool,
}

impl HubScreen {
    pub fn open(&mut self) {
        self.open = true;
        self.closing = false;
        self.index = 0;
    }

    pub fn close(&mut self) {
        if self.open {
            self.open = false;
            self.closing = true;
        }
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Open, or still fading out: the host keeps calling `show`.
    pub fn is_active(&self) -> bool {
        self.open || self.closing
    }

    /// Puts the focus on a place (the gallery shows a focused button).
    pub fn set_focus(&mut self, index: usize) {
        self.index = index;
    }

    pub fn show(
        &mut self,
        shell: &mut Shell,
        ctx: &Context,
        events: &[InputEvent],
        data: &HubData<'_>,
    ) -> Vec<HubAction> {
        let mut actions = Vec::new();
        if !self.is_active() {
            return actions;
        }
        let list = stops(data);
        self.index = self.index.min(list.len() - 1);
        if self.open {
            for command in commands(events) {
                match command {
                    Command::Back | Command::Options => actions.push(HubAction::Close),
                    Command::Select => {
                        shell.play(Cue::Select);
                        actions.push(list[self.index].action());
                    }
                    _ => match step(self.index, list.len(), command) {
                        Some(next) => {
                            self.index = next;
                            shell.play(Cue::Move);
                        }
                        None => {
                            shell.play(Cue::Bump);
                        }
                    },
                }
            }
        }
        let open = self.open;
        let focus = list[self.index];
        let mut pressed = None;
        let drawn = fullscreen_panel(
            ctx,
            "game-hub",
            open,
            GlassStyle::fullscreen_panel(),
            |ui, _| self.draw(ui, data, open.then_some(focus), &mut pressed),
        );
        if drawn.is_none() {
            self.closing = false;
        }
        if std::mem::take(&mut self.back_clicked) && open {
            shell.play(Cue::Back);
            actions.push(HubAction::Close);
        }
        if let (true, Some(stop)) = (open, pressed) {
            if let Some(at) = list.iter().position(|s| *s == stop) {
                self.index = at;
            }
            shell.play(Cue::Select);
            actions.push(stop.action());
        }
        actions
    }

    fn draw(
        &mut self,
        ui: &mut Ui,
        data: &HubData<'_>,
        focus: Option<Stop>,
        pressed: &mut Option<Stop>,
    ) {
        let language = data.language;
        let screen = ui.max_rect();
        let area = screen.shrink2(vec2(pad::TROPHIES.0, pad::TROPHIES.1));
        if back_button(ui, area, language) {
            self.back_clicked = true;
        }
        footer_hints(ui, screen, language, HintSet::BASIC.without_back());
        let painter = ui.painter().clone();

        let origin = pos2(area.min.x, area.min.y + BACK_ROW);
        let width = area.width();
        let header_bottom = self.draw_header(ui, &painter, origin, width, data, focus, pressed);
        self.draw_grid(
            ui,
            &painter,
            pos2(area.min.x, header_bottom + GAP),
            width,
            data,
            focus,
            pressed,
        );
        ui.interact(area, Id::unique("hub-name"), Sense::hover())
            .widget_info(|| {
                egui::WidgetInfo::labeled(
                    Role::Label,
                    true,
                    H::ScreenName.format(language, &[crate::ui::i18n::Arg::Str(data.name)]),
                )
            });
    }

    /// The art, the name and the primary button. Returns the bottom edge.
    #[allow(clippy::too_many_arguments)]
    fn draw_header(
        &self,
        ui: &mut Ui,
        painter: &egui::Painter,
        origin: egui::Pos2,
        width: f32,
        data: &HubData<'_>,
        focus: Option<Stop>,
        pressed: &mut Option<Stop>,
    ) -> f32 {
        let language = data.language;
        let art = Rect::from_min_size(origin, vec2(ART, ART));
        let mut style = TileStyle::new(data.accent);
        style.image = data.art;
        paint_tile(painter, art, &style);

        let x = art.max.x + 26.0;
        let text_width = (origin.x + width - x).max(200.0);
        let overline = ty::TROPHIES_OVERLINE;
        let title = ty::TROPHY_LIST_TITLE;
        let sub = ty::ROW_INFO;
        let mut y = origin.y;
        text::paint(
            painter,
            pos2(x, y),
            Align2::LEFT_TOP,
            overline,
            &text::to_upper(H::Overline.text(language), language),
            color::text_overline(),
        );
        y += overline.line_height() + 8.0;
        let galley = clamped(
            painter,
            title,
            data.name,
            Color32::PLACEHOLDER,
            text_width,
            1,
        );
        let title_h = galley.size().y;
        painter.galley(pos2(x, y), galley, color::text());
        y += title_h + 4.0;
        if !data.subtitle.is_empty() {
            let galley = clamped(
                painter,
                sub,
                &data.subtitle,
                Color32::PLACEHOLDER,
                text_width,
                1,
            );
            let h = galley.size().y;
            painter.galley(pos2(x, y), galley, color::text_subtitle());
            y += h;
        }
        y += space::L;
        let button_rect = Rect::from_min_size(pos2(x, y), vec2(text_width.min(420.0), 54.0));
        let mut cell = ui.new_child(
            UiBuilder::new()
                .max_rect(button_rect)
                .layout(Layout::left_to_right(Align::Center)),
        );
        let response = ConsoleButton::new(&data.primary_title)
            .icon(data.primary_icon)
            .icon_fill(true)
            .prominent(data.primary_ready)
            .min_width(size::PLAY_MIN_WIDTH)
            .focused(focus == Some(Stop::Primary))
            .show(&mut cell);
        if response.clicked() {
            *pressed = Some(Stop::Primary);
        }
        art.max.y.max(button_rect.max.y)
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_grid(
        &self,
        ui: &mut Ui,
        painter: &egui::Painter,
        origin: egui::Pos2,
        width: f32,
        data: &HubData<'_>,
        focus: Option<Stop>,
        pressed: &mut Option<Stop>,
    ) {
        let language = data.language;
        let column = (width - GAP * (COLUMNS as f32 - 1.0)) / COLUMNS as f32;
        let cell = |col: usize, span: usize, top: f32, height: f32| {
            Rect::from_min_size(
                pos2(origin.x + col as f32 * (column + GAP), top),
                vec2(column * span as f32 + GAP * (span as f32 - 1.0), height),
            )
        };
        let row_one = origin.y;
        let row_two = origin.y + ROW_ONE + GAP;

        // Trophies.
        let rect = cell(0, 1, row_one, ROW_ONE);
        let mut y = overline(painter, rect, H::Trophies.text(language), language);
        let t = &data.trophies;
        let figure = if t.figure.is_empty() {
            "\u{2013}"
        } else {
            &t.figure
        };
        let galley = text::paint_tabular(
            painter,
            pos2(rect.min.x, y),
            ty::PANEL_TITLE,
            figure,
            color::text(),
        );
        y = galley.max.y + space::S;
        if let Some(fraction) = t.fraction {
            paint_progress(
                painter,
                Rect::from_min_size(pos2(rect.min.x, y), vec2(rect.width(), BAR_HEIGHT)),
                fraction,
                data.accent,
            );
            y += BAR_HEIGHT + space::S;
            if t.grades.iter().any(|n| *n > 0) {
                paint_grade_counts(painter, pos2(rect.min.x, y), t.grades);
                y += ty::CHIP.line_height() + space::XS;
            }
        }
        note(painter, rect, y, &t.note, 2);
        let mut buttons = Rect::from_min_size(
            pos2(rect.min.x, rect.max.y - 32.0),
            vec2(rect.width(), 32.0),
        );
        buttons.min.y = rect.max.y - 32.0;
        let mut row = ui.new_child(
            UiBuilder::new()
                .max_rect(buttons)
                .layout(Layout::left_to_right(Align::Center)),
        );
        row.spacing_mut().item_spacing.x = size::CONTROL_GAP;
        if data.has_trophy_list
            && ConsoleButton::new(H::TrophyList.text(language))
                .size(ControlSize::Small)
                .focused(focus == Some(Stop::TrophyList))
                .show(&mut row)
                .clicked()
        {
            *pressed = Some(Stop::TrophyList);
        }
        if ConsoleButton::new(H::AllGames.text(language))
            .size(ControlSize::Small)
            .focused(focus == Some(Stop::Overview))
            .show(&mut row)
            .clicked()
        {
            *pressed = Some(Stop::Overview);
        }

        // Play time.
        let rect = cell(1, 1, row_one, ROW_ONE);
        block(
            painter,
            rect,
            H::PlayTime.text(language),
            &data.time,
            language,
        );

        // Captures: how many and the newest pictures.
        let rect = cell(2, 2, row_one, ROW_ONE);
        let mut y = overline(painter, rect, H::Captures.text(language), language);
        let c = &data.captures;
        match &c.count {
            Some(count) => {
                let h = text::paint(
                    painter,
                    pos2(rect.min.x, y),
                    Align2::LEFT_TOP,
                    ty::PANEL_TITLE,
                    count,
                    color::text(),
                )
                .height();
                y += h + space::S;
                let fit = ((rect.width() + space::M) / (THUMB_WIDTH + space::M))
                    .floor()
                    .max(1.0) as usize;
                for (i, thumb) in c.thumbs.iter().take(fit.min(THUMBS)).enumerate() {
                    let thumb_rect = Rect::from_min_size(
                        pos2(rect.min.x + i as f32 * (THUMB_WIDTH + space::M), y),
                        vec2(THUMB_WIDTH, THUMB_HEIGHT),
                    );
                    let mut style = TileStyle::new(data.accent);
                    style.image = *thumb;
                    paint_thumb(painter, thumb_rect, &style);
                }
                let buttons = Rect::from_min_size(
                    pos2(rect.min.x, rect.max.y - 32.0),
                    vec2(rect.width(), 32.0),
                );
                let mut row = ui.new_child(
                    UiBuilder::new()
                        .max_rect(buttons)
                        .layout(Layout::left_to_right(Align::Center)),
                );
                if ConsoleButton::new(H::SeeAll.text(language))
                    .size(ControlSize::Small)
                    .focused(focus == Some(Stop::Captures))
                    .show(&mut row)
                    .clicked()
                {
                    *pressed = Some(Stop::Captures);
                }
            }
            None => {
                note(painter, rect, y, &c.empty, 3);
            }
        }

        // The second row.
        block(
            painter,
            cell(0, 1, row_two, ROW_TWO),
            H::Friends.text(language),
            &data.friends,
            language,
        );
        block(
            painter,
            cell(1, 1, row_two, ROW_TWO),
            H::PlayTogether.text(language),
            &data.together,
            language,
        );
        block(
            painter,
            cell(2, 1, row_two, ROW_TWO),
            H::Compatibility.text(language),
            &data.compat,
            language,
        );
        block(
            painter,
            cell(3, 1, row_two, ROW_TWO),
            H::GameMusic.text(language),
            &data.music,
            language,
        );
    }
}

/// A thumbnail: the picture rounded, a plain tile while it loads.
fn paint_thumb(painter: &egui::Painter, rect: Rect, style: &TileStyle) {
    paint_tile(painter, rect, style);
}

/// The small caption of a block; returns where its content starts.
fn overline(painter: &egui::Painter, rect: Rect, label: &str, language: Language) -> f32 {
    let style = ty::TROPHIES_OVERLINE;
    text::paint(
        painter,
        rect.min,
        Align2::LEFT_TOP,
        style,
        &text::to_upper(label, language),
        color::text_overline(),
    );
    rect.min.y + style.line_height() + space::S
}

/// A sentence under a value, `rows` rows at most.
fn note(painter: &egui::Painter, rect: Rect, y: f32, text: &str, rows: usize) {
    if text.is_empty() {
        return;
    }
    let galley = clamped(
        painter,
        ty::NOTE,
        text,
        Color32::PLACEHOLDER,
        rect.width(),
        rows,
    );
    painter.galley(pos2(rect.min.x, y), galley, color::text_detail());
}

/// A block of the second kind: caption, value, sentence.
fn block(painter: &egui::Painter, rect: Rect, label: &str, block: &Block, language: Language) {
    let mut y = overline(painter, rect, label, language);
    if let Some(value) = &block.value {
        let galley = clamped(
            painter,
            ty::ITEM_TITLE,
            value,
            Color32::PLACEHOLDER,
            rect.width(),
            1,
        );
        let h = galley.size().y;
        painter.galley(pos2(rect.min.x, y), galley, color::text());
        y += h + space::XS;
    }
    if let Some(sentence) = &block.note {
        note(painter, rect, y, sentence, 3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_arrows_walk_the_buttons_and_stop_at_the_ends() {
        assert_eq!(step(0, 3, Command::Down), Some(1));
        assert_eq!(step(0, 3, Command::Right), Some(1));
        assert_eq!(step(2, 3, Command::Down), None, "no wrap at the end");
        assert_eq!(step(0, 3, Command::Up), None, "no wrap at the start");
        assert_eq!(step(2, 3, Command::Left), Some(1));
        assert_eq!(step(1, 3, Command::Select), None);
    }

    #[test]
    fn a_new_screen_is_closed_and_fades_out() {
        let mut screen = HubScreen::default();
        assert!(!screen.is_active());
        screen.open();
        assert!(screen.is_open());
        screen.close();
        assert!(!screen.is_open() && screen.is_active());
    }
}
