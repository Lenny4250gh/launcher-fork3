//! The trophy overview (docs/DESIGN.md 5.21): the trophies of every game together. A head with the
//! total, then one row per game that has a trophy list: its picture, name and latest trophy, the
//! count and a bar. The game with the newest trophy comes first. Up and down walk the rows, select
//! opens the game's hub, back and options leave.
//!
//! The screen knows nothing of the model: the host hands it the rows ([`OverviewData`]) and carries
//! out what it answers with ([`OverviewAction`]).

use egui::{
    Align2, Color32, Context, Id, Rect, Role, Sense, Ui, UiBuilder, WidgetInfo, pos2, vec2,
};

use crate::screens::hub_strings::HubText as H;
use crate::screens::scroll::Follow;
use crate::screens::side::commands;
use crate::screens::{BACK_ROW, back_button, clamped, footer_hints};
use crate::ui::Shell;
use crate::ui::anim::{AnimExt, Motion};
use crate::ui::chrome::{TileImage, TileStyle, fullscreen_panel, paint_tile};
use crate::ui::components::{self, HintSet, paint_grade_counts, paint_progress};
use crate::ui::glass::GlassStyle;
use crate::ui::i18n::{Arg, Language};
use crate::ui::icons::Icon;
use crate::ui::input::{Command, InputEvent};
use crate::ui::sound::Cue;
use crate::ui::state_page::{self, StatePage};
use crate::ui::text;
use crate::ui::theme::{color, pad, radius, size, space, ty};

const LIST_MARGIN: f32 = 8.0;
const EDGE_FADE: f32 = 20.0;
const ROW_PAD: f32 = 18.0;
const ROW_ART: f32 = 56.0;
const ROW_GAP: f32 = 10.0;
const FIGURE_WIDTH: f32 = 260.0;
const BAR_HEIGHT: f32 = 6.0;

/// One game of the overview.
#[derive(Clone, Debug)]
pub struct OverviewRow {
    pub title_id: String,
    pub name: String,
    pub art: Option<TileImage>,
    pub earned: usize,
    pub total: usize,
    pub fraction: f32,
    /// Platinum first.
    pub grades: [u32; 4],
    /// `Latest on 3 Oct`, or the words for nothing earned yet.
    pub latest: String,
    /// What a screen reader says.
    pub label: String,
}

pub struct OverviewData<'a> {
    pub language: Language,
    pub accent: Color32,
    /// `120 trophies in 8 games`; empty while nothing is known.
    pub summary: String,
    /// The trophy lists are still being read.
    pub loading: bool,
    pub rows: &'a [OverviewRow],
}

/// What the host has to do after a frame.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OverviewAction {
    Close,
    /// Open the Game Hub of this game (a title ID).
    OpenGame(String),
}

#[derive(Default)]
pub struct OverviewScreen {
    open: bool,
    closing: bool,
    index: usize,
    back_clicked: bool,
}

impl OverviewScreen {
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

    pub fn is_active(&self) -> bool {
        self.open || self.closing
    }

    pub fn set_focus(&mut self, index: usize) {
        self.index = index;
    }

    pub fn show(
        &mut self,
        shell: &mut Shell,
        ctx: &Context,
        events: &[InputEvent],
        data: &OverviewData<'_>,
    ) -> Vec<OverviewAction> {
        let mut actions = Vec::new();
        if !self.is_active() {
            return actions;
        }
        let count = data.rows.len();
        self.index = self.index.min(count.saturating_sub(1));
        if self.open {
            for command in commands(events) {
                match command {
                    Command::Back | Command::Options => actions.push(OverviewAction::Close),
                    Command::Select => {
                        if let Some(row) = data.rows.get(self.index) {
                            shell.play(Cue::Select);
                            actions.push(OverviewAction::OpenGame(row.title_id.clone()));
                        }
                    }
                    Command::Up | Command::Down => {
                        let next = if command == Command::Up {
                            self.index.saturating_sub(1)
                        } else {
                            (self.index + 1).min(count.saturating_sub(1))
                        };
                        if next != self.index {
                            self.index = next;
                            shell.play(Cue::Move);
                        } else {
                            shell.play(Cue::Bump);
                        }
                    }
                    Command::Left | Command::Right => {}
                }
            }
        }
        let open = self.open;
        let mut clicked = None;
        let drawn = fullscreen_panel(
            ctx,
            "trophy-overview",
            open,
            GlassStyle::fullscreen_panel(),
            |ui, _| self.draw(ui, data, open, &mut clicked),
        );
        if drawn.is_none() {
            self.closing = false;
        }
        if std::mem::take(&mut self.back_clicked) && open {
            shell.play(Cue::Back);
            actions.push(OverviewAction::Close);
        }
        if let (true, Some(i)) = (open, clicked)
            && let Some(row) = data.rows.get(i)
        {
            self.index = i;
            shell.play(Cue::Select);
            actions.push(OverviewAction::OpenGame(row.title_id.clone()));
        }
        actions
    }

    fn draw(
        &mut self,
        ui: &mut Ui,
        data: &OverviewData<'_>,
        open: bool,
        clicked: &mut Option<usize>,
    ) {
        let language = data.language;
        let screen = ui.max_rect();
        let area = screen.shrink2(vec2(pad::TROPHIES.0, pad::TROPHIES.1));
        if back_button(ui, area, language) {
            self.back_clicked = true;
        }
        footer_hints(ui, screen, language, HintSet::BASIC.without_back());
        let painter = ui.painter().clone();
        let head = pos2(area.min.x, area.min.y + BACK_ROW);
        let overline = ty::TROPHIES_OVERLINE;
        let title = ty::TROPHY_LIST_TITLE;
        text::paint(
            &painter,
            head,
            Align2::LEFT_TOP,
            overline,
            &text::to_upper(H::OverviewOverline.text(language), language),
            color::text_overline(),
        );
        let title_y = head.y + overline.line_height() + 8.0;
        text::paint(
            &painter,
            pos2(head.x, title_y),
            Align2::LEFT_TOP,
            title,
            H::OverviewTitle.text(language),
            color::text(),
        );
        let mut bottom = title_y + title.line_height();
        if !data.summary.is_empty() {
            bottom += space::S;
            text::paint(
                &painter,
                pos2(head.x, bottom),
                Align2::LEFT_TOP,
                ty::ROW_INFO,
                &data.summary,
                color::text_subtitle(),
            );
            bottom += ty::ROW_INFO.line_height();
        }
        ui.interact(
            Rect::from_min_max(head, pos2(area.max.x, bottom)),
            Id::unique("overview-head"),
            Sense::hover(),
        )
        .widget_info(|| {
            WidgetInfo::labeled(
                Role::Label,
                true,
                format!("{}. {}", H::OverviewTitle.text(language), data.summary),
            )
        });
        let list_top = bottom + space::XL;
        if data.rows.is_empty() {
            let (icon, title, note) = if data.loading {
                (Icon::CIRCLE_DASHED, H::OverviewLoading.text(language), None)
            } else {
                (
                    Icon::TROPHY,
                    H::OverviewEmptyTitle.text(language),
                    Some(H::OverviewEmptyNote.text(language)),
                )
            };
            let page = StatePage {
                icon,
                title,
                note,
                loading: data.loading,
                action: None,
            };
            let room = Rect::from_min_max(pos2(area.min.x, list_top), area.max);
            state_page::show(ui, room, &page, false);
            return;
        }
        let list = Rect::from_min_max(
            pos2(area.min.x - LIST_MARGIN, list_top - LIST_MARGIN),
            pos2(area.max.x + LIST_MARGIN, area.max.y),
        );
        let mut child = ui.new_child(UiBuilder::new().max_rect(list));
        self.draw_rows(&mut child, data, open, clicked);
    }

    fn draw_rows(
        &mut self,
        ui: &mut Ui,
        data: &OverviewData<'_>,
        open: bool,
        clicked: &mut Option<usize>,
    ) {
        let ctx = ui.ctx().clone();
        let id = ui.make_persistent_id("overview-rows");
        let focus = open.then_some(self.index.min(data.rows.len().saturating_sub(1)));
        let (follow, scroll_to) = Follow::begin(&ctx, id, focus);
        let mut area = egui::ScrollArea::vertical()
            .id_salt(id)
            .auto_shrink([false, false])
            .scroll_bar_visibility(
                egui::containers::scroll_area::ScrollBarVisibility::AlwaysHidden,
            );
        if let Some(offset) = scroll_to {
            area = area.vertical_scroll_offset(offset);
        }
        let height_id = id.with("row-height");
        let row_height: f32 = ctx.data(|d| d.get_temp(height_id)).unwrap_or(90.0);
        let clip = ui.clip_rect().intersect(ui.max_rect()).shrink(LIST_MARGIN);
        let mut geometry = Vec::with_capacity(data.rows.len());
        let output = area.show(ui, |ui| {
            egui::Frame::NONE
                .inner_margin(egui::Margin::same(LIST_MARGIN as i8))
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = ROW_GAP;
                    let origin = ui.cursor().min.y;
                    for (i, row) in data.rows.iter().enumerate() {
                        let top = ui.cursor().min.y;
                        let edge = (top - clip.min.y).min(clip.max.y - (top + row_height));
                        let visible = ((edge + EDGE_FADE) / EDGE_FADE).clamp(0.0, 1.0);
                        let rect = ui
                            .scope(|ui| {
                                ui.multiply_opacity(visible);
                                self.draw_row(ui, row, data, open && i == self.index, i)
                            })
                            .inner;
                        ctx.data_mut(|d| d.insert_temp(height_id, rect.height()));
                        geometry.push((rect.min.y - origin, rect.height()));
                        let response =
                            ui.interact(rect, id.with(("overview-row", i)), Sense::click());
                        response.widget_info(|| {
                            WidgetInfo::labeled(Role::Button, true, row.label.as_str())
                        });
                        if response.clicked() {
                            *clicked = Some(i);
                        }
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

    fn draw_row(
        &self,
        ui: &mut Ui,
        row: &OverviewRow,
        data: &OverviewData<'_>,
        focused: bool,
        index: usize,
    ) -> Rect {
        let ctx = ui.ctx().clone();
        let painter = ui.painter().clone();
        let width = ui.available_width();
        let name_x = ROW_PAD + ROW_ART + 18.0;
        let text_w = (width - name_x - FIGURE_WIDTH - 2.0 * ROW_PAD).max(120.0);
        let name = clamped(
            &painter,
            ty::TROPHY_NAME,
            &row.name,
            Color32::PLACEHOLDER,
            text_w,
            1,
        );
        let latest = clamped(
            &painter,
            ty::DETAIL_LARGE,
            &row.latest,
            Color32::PLACEHOLDER,
            text_w,
            1,
        );
        let block = name.size().y + 6.0 + latest.size().y;
        let height = ROW_ART.max(block) + 28.0;
        let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        if !ui.is_rect_visible(rect) {
            return rect;
        }
        let t = ctx.focus_t(("overview-row-focus", index), focused);
        let under_pointer = ctx.pointer_hover_pos().is_some_and(|p| rect.contains(p));
        let hover_t = ctx.animate_to(
            ("overview-row-hover", index),
            f32::from(under_pointer),
            Motion::Hover,
        );
        components::paint_surface(
            &painter,
            rect,
            radius::TROPHY_ROW,
            color::surface_trophy_row(),
            hover_t,
            t,
            0.0,
            size::FOCUS_RING,
        );
        let art = Rect::from_center_size(
            pos2(rect.min.x + ROW_PAD + ROW_ART * 0.5, rect.center().y),
            vec2(ROW_ART, ROW_ART),
        );
        let mut style = TileStyle::new(data.accent);
        style.image = row.art;
        paint_tile(&painter, art, &style);
        let x = rect.min.x + name_x;
        let top = rect.center().y - block * 0.5;
        let name_h = name.size().y;
        painter.galley(pos2(x, top), name, color::text());
        painter.galley(pos2(x, top + name_h + 6.0), latest, color::text_detail());

        // The count with its bar, at the trailing edge.
        let right = rect.max.x - ROW_PAD;
        let left = right - FIGURE_WIDTH + space::XL;
        let figure = H::TrophyCount.format(
            data.language,
            &[Arg::Int(row.earned as i64), Arg::Int(row.total as i64)],
        );
        let figure_w = text::tabular_width(&painter, ty::INFO_VALUE, &figure);
        let figure_y = rect.center().y - (ty::INFO_VALUE.line_height() + 8.0 + BAR_HEIGHT) * 0.5;
        text::paint_tabular(
            &painter,
            pos2(right - figure_w, figure_y),
            ty::INFO_VALUE,
            &figure,
            color::text(),
        );
        if row.grades.iter().any(|n| *n > 0) {
            paint_grade_counts(&painter, pos2(left, figure_y + 2.0), row.grades);
        }
        let bar = Rect::from_min_size(
            pos2(left, figure_y + ty::INFO_VALUE.line_height() + 8.0),
            vec2(right - left, BAR_HEIGHT),
        );
        paint_progress(&painter, bar, row.fraction, data.accent);
        rect
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_screen_starts_closed_and_fades_out_when_closed() {
        let mut screen = OverviewScreen::default();
        assert!(!screen.is_active());
        screen.open();
        assert!(screen.is_open());
        screen.close();
        assert!(!screen.is_open() && screen.is_active());
    }
}
