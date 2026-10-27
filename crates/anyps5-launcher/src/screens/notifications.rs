//! The Notification Center (docs/DESIGN.md 9.12): a full-screen area with everything the launcher
//! told the player on its own, newest first: friend requests, invitations, messages, trophies,
//! updates, Play Together and notices. Each entry says what it is and when, and has its own
//! button (Accept, Join, Open) and Dismiss. "Clear All" empties the list.
//!
//! Up and down move between the entries (and from the first one to Clear All), left and right
//! between the buttons of an entry, select runs the focused button, back closes. The screen is
//! pure: the host hands it the entries as [`Item`]s and runs the [`NotifyAction`]s it returns.

use std::collections::HashSet;

use anyps5_core::notifications::Kind;
use egui::{
    Align, Align2, Color32, Context, Layout, Rect, Role, Sense, Ui, UiBuilder, WidgetInfo, pos2,
    vec2,
};

use crate::screens::clamped;
use crate::screens::notification_strings::NotifyText as NT;
use crate::screens::online::ago_text;
use crate::screens::scroll::Follow;
use crate::screens::side::commands;
use crate::ui::Shell;
use crate::ui::anim::AnimExt;
use crate::ui::chrome::fullscreen_panel;
use crate::ui::components::{self, ConsoleButton, ControlSize, HintSet};
use crate::ui::glass::GlassStyle;
use crate::ui::i18n::{Arg, Language};
use crate::ui::icons::Icon;
use crate::ui::input::{self, Command, InputEvent};
use crate::ui::sound::Cue;
use crate::ui::state_page::{self, StatePage};
use crate::ui::text;
use crate::ui::theme::{color, fade, pad, radius, size, space, ty};

/// Room around the scrolling rows for the focused row's lift.
const LIST_MARGIN: f32 = 8.0;
/// The widest the list gets, so a line stays readable on a wide screen.
const LIST_MAX: f32 = 920.0;
/// The disc behind the glyph of an entry, and the glyph.
const DISC: f32 = 44.0;
const GLYPH: f32 = 20.0;
/// Inside a row: the edge, and the gap between the disc, the words and the buttons.
const ROW_EDGE: f32 = 20.0;
const ROW_GAP: f32 = 16.0;

/// One entry as the screen draws it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub id: u64,
    pub kind: Kind,
    pub text: String,
    pub at: u64,
    /// The label of the entry's own button, when there is something to do.
    pub primary: Option<NT>,
}

/// What the screen is given every frame.
pub struct NotifyData<'a> {
    pub items: &'a [Item],
    pub language: Language,
    /// Seconds since the epoch, and the local UTC offset.
    pub now: i64,
    pub utc_offset: i32,
}

/// What the host has to do after a frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotifyAction {
    Close,
    /// Run the main button of an entry.
    Run(u64),
    Dismiss(u64),
    ClearAll,
}

/// Where the focus stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stop {
    Clear,
    /// A button of a row: its index, and whether it is the main one.
    Row {
        index: usize,
        primary: bool,
    },
}

#[derive(Default)]
pub struct NotificationsScreen {
    open: bool,
    closing: bool,
    stop: Option<Stop>,
    /// The entries that were not read before this visit: they carry a mark until the area closes.
    fresh: HashSet<u64>,
}

impl NotificationsScreen {
    /// Opens on the newest entry. `fresh` are the entries not read yet.
    pub fn open(&mut self, fresh: HashSet<u64>) {
        self.open = true;
        self.closing = false;
        self.stop = None;
        self.fresh = fresh;
    }

    pub fn close(&mut self) {
        if self.open {
            self.open = false;
            self.closing = true;
        }
    }

    pub fn is_active(&self) -> bool {
        self.open || self.closing
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Where the focus stands, made valid for the entries there are.
    fn stop(&self, items: &[Item]) -> Option<Stop> {
        if items.is_empty() {
            return None;
        }
        Some(
            match self.stop.unwrap_or(Stop::Row {
                index: 0,
                primary: true,
            }) {
                Stop::Clear => Stop::Clear,
                Stop::Row { index, primary } => {
                    let index = index.min(items.len() - 1);
                    Stop::Row {
                        index,
                        primary: primary && items[index].primary.is_some(),
                    }
                }
            },
        )
    }

    /// One command. Returns the sound and what to do.
    fn step(&mut self, command: Command, items: &[Item]) -> (Option<Cue>, Option<NotifyAction>) {
        let Some(stop) = self.stop(items) else {
            return match command {
                Command::Back | Command::Options => (Some(Cue::Close), Some(NotifyAction::Close)),
                _ => (None, None),
            };
        };
        let last = items.len() - 1;
        let landing = |index: usize, primary: bool| Stop::Row {
            index,
            primary: primary && items[index].primary.is_some(),
        };
        let next = match (stop, command) {
            (_, Command::Back | Command::Options) => {
                return (Some(Cue::Close), Some(NotifyAction::Close));
            }
            (Stop::Clear, Command::Down) => Some(landing(0, true)),
            (Stop::Row { index: 0, .. }, Command::Up) => Some(Stop::Clear),
            (Stop::Row { index, primary }, Command::Up) => Some(landing(index - 1, primary)),
            (Stop::Row { index, primary }, Command::Down) if index < last => {
                Some(landing(index + 1, primary))
            }
            (
                Stop::Row {
                    index,
                    primary: true,
                },
                Command::Right,
            ) => Some(Stop::Row {
                index,
                primary: false,
            }),
            (
                Stop::Row {
                    index,
                    primary: false,
                },
                Command::Left,
            ) if items[index].primary.is_some() => Some(Stop::Row {
                index,
                primary: true,
            }),
            (Stop::Clear, Command::Select) => {
                return (Some(Cue::Select), Some(NotifyAction::ClearAll));
            }
            (Stop::Row { index, primary }, Command::Select) => {
                let id = items[index].id;
                return (
                    Some(Cue::Select),
                    Some(if primary {
                        NotifyAction::Run(id)
                    } else {
                        NotifyAction::Dismiss(id)
                    }),
                );
            }
            _ => None,
        };
        match next {
            Some(next) => {
                self.stop = Some(next);
                (Some(Cue::Move), None)
            }
            None if command.is_direction() => (Some(Cue::Bump), None),
            None => (None, None),
        }
    }

    pub fn show(
        &mut self,
        shell: &mut Shell,
        ctx: &Context,
        events: &[InputEvent],
        data: &NotifyData<'_>,
    ) -> Vec<NotifyAction> {
        let mut actions = Vec::new();
        if !self.is_active() {
            return actions;
        }
        if self.open {
            for command in commands(events) {
                let (cue, action) = self.step(command, data.items);
                if let Some(cue) = cue {
                    shell.play(cue);
                }
                if let Some(action) = action {
                    if action == NotifyAction::Close {
                        self.close();
                    }
                    actions.push(action);
                }
            }
        }
        let open = self.open;
        let stop = self.stop(data.items).filter(|_| open);
        let mut clicked: Vec<(Stop, NotifyAction)> = Vec::new();
        let mut closed = false;
        let drawn = fullscreen_panel(
            ctx,
            "notifications",
            open,
            GlassStyle::fullscreen_panel(),
            |ui, _| self.draw(ui, data, stop, &mut clicked, &mut closed),
        );
        if drawn.is_none() {
            self.closing = false;
            self.fresh.clear();
        }
        if open {
            for (at, action) in clicked {
                self.stop = Some(at);
                shell.play(Cue::Select);
                actions.push(action);
            }
            if closed {
                shell.play(Cue::Close);
                self.close();
                actions.push(NotifyAction::Close);
            }
        }
        actions
    }

    fn draw(
        &self,
        ui: &mut Ui,
        data: &NotifyData<'_>,
        stop: Option<Stop>,
        clicked: &mut Vec<(Stop, NotifyAction)>,
        closed: &mut bool,
    ) {
        let language = data.language;
        let screen = ui.max_rect();
        let area = screen.shrink2(vec2(pad::TROPHIES.0, pad::TROPHIES.1));
        let painter = ui.painter().clone();

        // The hint bar stands bottom right like on Home; the list ends above it.
        let (_, hint_height) =
            components::hint_bar_enter_size(&painter, language, HintSet::BASIC, None);
        crate::screens::footer_hints(ui, screen, language, HintSet::BASIC);

        let width = area.width().min(LIST_MAX);
        let header = Rect::from_min_size(area.min, vec2(width, ControlSize::Small.height() + 12.0));
        // The way out for the pointer, at the end of the title row; Clear All stands beside it.
        let (edge, leave) = crate::screens::close_button(ui, header, language);
        *closed |= leave;
        text::paint(
            &painter,
            pos2(header.min.x, header.center().y),
            Align2::LEFT_CENTER,
            ty::TROPHY_LIST_TITLE,
            NT::Title.text(language),
            color::text(),
        );
        if !data.items.is_empty() {
            let label = NT::ClearAll.text(language);
            let button_width = text::width(&painter, ty::CHIP, label)
                + 2.0 * ControlSize::Small.text_pad()
                + ControlSize::Small.glyph(true)
                + 12.0;
            let mut bar = ui.new_child(
                UiBuilder::new()
                    .max_rect(Rect::from_min_size(
                        pos2(
                            edge - space::M - button_width - 8.0,
                            header.center().y - 16.0,
                        ),
                        vec2(button_width + 8.0, 32.0),
                    ))
                    .layout(Layout::right_to_left(Align::Center)),
            );
            if ConsoleButton::new(label)
                .icon(Icon::BROOM)
                .size(ControlSize::Small)
                .focused(stop == Some(Stop::Clear))
                .show(&mut bar)
                .clicked()
            {
                clicked.push((Stop::Clear, NotifyAction::ClearAll));
            }
        }

        let list = Rect::from_min_max(
            pos2(
                area.min.x - LIST_MARGIN,
                header.max.y + space::XL - LIST_MARGIN,
            ),
            pos2(
                area.min.x + width + LIST_MARGIN,
                screen.max.y - pad::HINT_Y - hint_height - space::M,
            ),
        );
        let mut child = ui.new_child(UiBuilder::new().max_rect(list));
        if data.items.is_empty() {
            let page = StatePage {
                icon: Icon::BELL,
                title: NT::EmptyTitle.text(language),
                note: Some(NT::EmptyBody.text(language)),
                loading: false,
                action: None,
            };
            state_page::show(&mut child, list, &page, false);
            return;
        }
        self.draw_rows(&mut child, data, stop, clicked);
    }

    fn draw_rows(
        &self,
        ui: &mut Ui,
        data: &NotifyData<'_>,
        stop: Option<Stop>,
        clicked: &mut Vec<(Stop, NotifyAction)>,
    ) {
        let ctx = ui.ctx().clone();
        let id = ui.make_persistent_id("notification-rows");
        let focus_row = match stop {
            Some(Stop::Row { index, .. }) => Some(index),
            // The header is on screen whenever the list is at the top.
            _ => None,
        };
        let (follow, scroll_to) = Follow::begin(&ctx, id, focus_row);
        let mut area = egui::ScrollArea::vertical()
            .id_salt(id)
            .auto_shrink([false, false])
            .scroll_bar_visibility(
                egui::containers::scroll_area::ScrollBarVisibility::AlwaysHidden,
            );
        if let Some(offset) = scroll_to {
            area = area.vertical_scroll_offset(offset);
        }
        let mut geometry = Vec::with_capacity(data.items.len());
        let output = area.show(ui, |ui| {
            egui::Frame::NONE
                .inner_margin(egui::Margin::same(LIST_MARGIN as i8))
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = size::ROW_GAP;
                    let origin = ui.cursor().min.y;
                    for (i, item) in data.items.iter().enumerate() {
                        let focus = match stop {
                            Some(Stop::Row { index, primary }) if index == i => Some(primary),
                            _ => None,
                        };
                        let rect = self.draw_row(ui, data, item, i, focus, clicked);
                        geometry.push((rect.min.y - origin, rect.height()));
                    }
                });
        });
        follow.end(
            &ctx,
            geometry,
            output.content_size.y,
            output.inner_rect.height(),
            output.state.clamped_offset().y,
        );
    }

    /// One row. `focus` is `Some(primary)` when a button of the row holds the focus.
    fn draw_row(
        &self,
        ui: &mut Ui,
        data: &NotifyData<'_>,
        item: &Item,
        index: usize,
        focus: Option<bool>,
        clicked: &mut Vec<(Stop, NotifyAction)>,
    ) -> Rect {
        let ctx = ui.ctx().clone();
        let language = data.language;
        let painter = ui.painter().clone();
        let width = ui.available_width();

        // The buttons first: they decide the room the words have.
        let dismiss = NT::Dismiss.text(language);
        let button_width = |label: &str| {
            text::width(&painter, ty::CHIP, label) + 2.0 * ControlSize::Small.text_pad() + 4.0
        };
        let mut buttons_width = button_width(dismiss);
        if let Some(primary) = item.primary {
            buttons_width += size::CONTROL_GAP + button_width(primary.text(language));
        }
        let text_x = ROW_EDGE + DISC + ROW_GAP;
        let text_width = (width - text_x - ROW_GAP - buttons_width - ROW_EDGE).max(120.0);

        let kind = kind_text(item.kind).text(language);
        let when = ago_text(data.now, item.at, data.utc_offset, language);
        let caption = format!("{kind} · {when}");
        let caption_galley = clamped(
            &painter,
            ty::DETAIL,
            &caption,
            Color32::PLACEHOLDER,
            text_width,
            1,
        );
        let body = clamped(
            &painter,
            ty::ROW_TITLE,
            &item.text,
            Color32::PLACEHOLDER,
            text_width,
            2,
        );
        let block = caption_galley.size().y + 4.0 + body.size().y;
        let height = (DISC.max(block) + 2.0 * 16.0).max(size::CONTROL_L + 2.0 * space::S);
        let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        if !ui.is_rect_visible(rect) {
            return rect;
        }
        let fresh = self.fresh.contains(&item.id);
        let name = NT::EntryName.format(
            language,
            &[Arg::Str(kind), Arg::Str(&item.text), Arg::Str(&when)],
        );
        response.widget_info(|| WidgetInfo::labeled(Role::Label, true, name.as_str()));
        crate::ui::a11y::follow(&ctx, response.id, focus.is_some());

        let focus_t = ctx.focus_t(("notification-row-focus", item.id), focus.is_some());
        let hover_t = ctx.animate_to(
            ("notification-row-hover", item.id),
            f32::from(input::hovered(&ctx, &response)),
            crate::ui::anim::Motion::Hover,
        );
        components::paint_surface(
            &painter,
            rect,
            radius::TROPHY_ROW,
            color::surface_row(),
            hover_t,
            focus_t,
            0.0,
            // The ring belongs to the button that holds the focus, not to the row.
            0.0,
        );

        // The disc with the glyph of the kind.
        let disc = Rect::from_center_size(
            pos2(rect.min.x + ROW_EDGE + DISC * 0.5, rect.center().y),
            vec2(DISC, DISC),
        );
        painter.circle_filled(disc.center(), DISC * 0.5, fade(Color32::WHITE, 0.08));
        painter.text(
            disc.center(),
            Align2::CENTER_CENTER,
            kind_icon(item.kind).glyph(),
            kind_icon(item.kind).font(GLYPH, fresh),
            if fresh {
                color::text()
            } else {
                color::text_detail()
            },
        );

        // The words: what and when, then the sentence. New entries are brighter and carry a dot.
        let x = rect.min.x + text_x;
        let top = rect.center().y - block * 0.5;
        let caption_height = caption_galley.size().y;
        painter.galley(pos2(x, top), caption_galley, color::text_caption());
        let ink = if fresh {
            color::text()
        } else {
            color::text_label()
        };
        let body_cut = body.elided;
        painter.galley(pos2(x, top + caption_height + 4.0), body, ink);
        // A sentence cut to two lines is read in full while the pointer or the focus is on the
        // entry (the chip of DESIGN.md 3.13).
        crate::ui::reveal::full_text(
            &ctx,
            response.id,
            rect,
            &item.text,
            input::hovered(&ctx, &response) || (focus.is_some() && input::focus_visible(&ctx)),
            body_cut,
        );
        if fresh {
            painter.circle_filled(
                pos2(rect.min.x + ROW_EDGE * 0.5, rect.center().y),
                3.0,
                color::text(),
            );
        }

        // The buttons, right to left: Dismiss at the edge, the main one before it.
        let strip = Rect::from_min_max(
            pos2(
                rect.max.x - ROW_EDGE - buttons_width - 8.0,
                rect.center().y - 16.0,
            ),
            pos2(rect.max.x - ROW_EDGE + 8.0, rect.center().y + 16.0),
        );
        let mut buttons = ui.new_child(
            UiBuilder::new()
                .max_rect(strip)
                .layout(Layout::right_to_left(Align::Center)),
        );
        buttons.spacing_mut().item_spacing.x = size::CONTROL_GAP;
        buttons.add_space(8.0);
        if ConsoleButton::new(dismiss)
            .size(ControlSize::Small)
            .focused(focus == Some(false))
            .show(&mut buttons)
            .clicked()
        {
            clicked.push((
                Stop::Row {
                    index,
                    primary: false,
                },
                NotifyAction::Dismiss(item.id),
            ));
        }
        if let Some(primary) = item.primary
            && ConsoleButton::new(primary.text(language))
                .size(ControlSize::Small)
                .focused(focus == Some(true))
                .show(&mut buttons)
                .clicked()
        {
            clicked.push((
                Stop::Row {
                    index,
                    primary: true,
                },
                NotifyAction::Run(item.id),
            ));
        }
        rect
    }
}

fn kind_text(kind: Kind) -> NT {
    match kind {
        Kind::FriendRequest => NT::KindFriendRequest,
        Kind::Invitation => NT::KindInvitation,
        Kind::Message => NT::KindMessage,
        Kind::Trophy => NT::KindTrophy,
        Kind::Update => NT::KindUpdate,
        Kind::Together => NT::KindTogether,
        Kind::System => NT::KindSystem,
    }
}

/// The glyph of a kind.
pub fn kind_icon(kind: Kind) -> Icon {
    match kind {
        Kind::FriendRequest => Icon::USER_PLUS,
        Kind::Invitation => Icon::USERS,
        Kind::Message => Icon::ENVELOPE_SIMPLE,
        Kind::Trophy => Icon::TROPHY,
        Kind::Update => Icon::DOWNLOAD_SIMPLE,
        Kind::Together => Icon::GAME_CONTROLLER,
        Kind::System => Icon::BELL,
    }
}

/// Sample entries for the tests.
#[cfg(test)]
pub mod sample {
    use super::*;

    pub fn items(now: u64) -> Vec<Item> {
        let item = |id, kind, text: &str, ago, primary| Item {
            id,
            kind,
            text: text.to_owned(),
            at: now.saturating_sub(ago),
            primary,
        };
        vec![
            item(
                6,
                Kind::FriendRequest,
                "Mira wants to be your friend",
                40,
                Some(NT::Accept),
            ),
            item(
                5,
                Kind::Together,
                "Jonas is playing Hollow Orchard. Join?",
                6 * 60,
                Some(NT::Join),
            ),
            item(
                4,
                Kind::Message,
                "Mira: are you free tonight? We could finish the last chapter",
                25 * 60,
                Some(NT::Open),
            ),
            item(
                3,
                Kind::Trophy,
                "Hollow Orchard: Night Harvest",
                3 * 3600,
                Some(NT::Open),
            ),
            item(
                2,
                Kind::Update,
                "AnyPlay Launcher 0.3.0 is ready to install",
                20 * 3600,
                Some(NT::Open),
            ),
            item(1, Kind::System, "Missed call from Jonas", 3 * 86_400, None),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn screen(items: &[Item]) -> NotificationsScreen {
        let mut screen = NotificationsScreen::default();
        screen.open(items.iter().map(|i| i.id).collect());
        screen
    }

    #[test]
    fn up_from_the_first_entry_reaches_clear_all_and_down_comes_back() {
        let items = sample::items(1_000_000);
        let mut screen = screen(&items);
        assert_eq!(screen.step(Command::Up, &items).0, Some(Cue::Move));
        assert_eq!(screen.stop(&items), Some(Stop::Clear));
        assert_eq!(
            screen.step(Command::Select, &items).1,
            Some(NotifyAction::ClearAll)
        );
        screen.step(Command::Down, &items);
        assert_eq!(
            screen.stop(&items),
            Some(Stop::Row {
                index: 0,
                primary: true
            })
        );
    }

    #[test]
    fn left_and_right_move_between_the_buttons_of_an_entry() {
        let items = sample::items(1_000_000);
        let mut screen = screen(&items);
        assert_eq!(
            screen.step(Command::Select, &items).1,
            Some(NotifyAction::Run(6))
        );
        screen.step(Command::Right, &items);
        assert_eq!(
            screen.step(Command::Select, &items).1,
            Some(NotifyAction::Dismiss(6))
        );
        screen.step(Command::Left, &items);
        assert_eq!(
            screen.step(Command::Select, &items).1,
            Some(NotifyAction::Run(6))
        );
        // At the edge a direction bumps.
        assert_eq!(screen.step(Command::Left, &items).0, Some(Cue::Bump));
    }

    #[test]
    fn an_entry_without_a_main_button_only_dismisses() {
        let items = sample::items(1_000_000);
        let mut screen = screen(&items);
        for _ in 0..items.len() - 1 {
            screen.step(Command::Down, &items);
        }
        assert_eq!(
            screen.stop(&items),
            Some(Stop::Row {
                index: items.len() - 1,
                primary: false
            })
        );
        assert_eq!(
            screen.step(Command::Select, &items).1,
            Some(NotifyAction::Dismiss(1))
        );
        // Down at the end bumps.
        assert_eq!(screen.step(Command::Down, &items).0, Some(Cue::Bump));
    }

    #[test]
    fn with_nothing_left_only_back_does_anything() {
        let mut screen = NotificationsScreen::default();
        screen.open(HashSet::new());
        assert_eq!(screen.step(Command::Select, &[]), (None, None));
        assert_eq!(screen.step(Command::Back, &[]).1, Some(NotifyAction::Close));
    }

    #[test]
    fn the_focus_stays_valid_when_entries_go() {
        let items = sample::items(1_000_000);
        let mut screen = screen(&items);
        for _ in 0..items.len() - 1 {
            screen.step(Command::Down, &items);
        }
        assert_eq!(
            screen.stop(&items[..2]),
            Some(Stop::Row {
                index: 1,
                primary: false
            })
        );
    }
}
