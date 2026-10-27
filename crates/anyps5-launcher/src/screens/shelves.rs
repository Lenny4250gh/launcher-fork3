//! The shelves of the home screen (docs/DESIGN.md 5.3.1): below the tile row, at most two small
//! rows of games, "Friends Playing Now", "Recently Played" and "New". The tile row and the selected
//! game's hero stay the focus: a shelf is a quiet way to jump, drawn only when the window has the
//! room for it.

use egui::{Align2, CursorIcon, Id, Rect, Role, Sense, Ui, WidgetInfo, pos2, vec2};

use crate::app::library_view::{Shelf, ShelfKind};
use crate::screens::home::tile_style;
use crate::screens::library_strings::LibraryText as T;
use crate::screens::{ScreenCx, clamped};
use crate::ui::a11y;
use crate::ui::anim::{AnimExt, Motion};
use crate::ui::chrome::{self, TileOverlay};
use crate::ui::focus::HomeFocus;
use crate::ui::icons::Icon;
use crate::ui::input;
use crate::ui::text;
use crate::ui::theme::{color, pad, size, ty};

/// The sizes an icon on a shelf can have, largest first: the shelf takes the largest the room under
/// the tile row allows, so it never looks squeezed (the tile row's own icons are 118 dp).
const TILES: [f32; 4] = [96.0, 80.0, 68.0, 56.0];
/// The room between two icons.
const ICON_GAP: f32 = 24.0;
/// How much wider than its icon a caption may be (it stays inside the room between two icons).
const CAPTION_EXTRA: f32 = 16.0;
/// Between the label and the icons, and between the icons and their captions.
const LABEL_GAP: f32 = 10.0;
const CAPTION_GAP: f32 = 8.0;
/// Between two shelves.
const SHELF_GAP: f32 = 56.0;
/// From the tile row down to the shelves.
const ABOVE: f32 = 18.0;
/// The room kept free between the shelves and the hero's title.
pub const BELOW: f32 = 16.0;

/// One line of a caption: cut where the width ends, not at the last word that fits, so a long name
/// keeps as much of itself as the icon is wide ("Ember Cartogra...", not "Ember ...").
fn one_line(
    painter: &egui::Painter,
    text: &str,
    color: egui::Color32,
    width: f32,
) -> std::sync::Arc<egui::Galley> {
    let style = ty::STATE_LABEL_SMALL;
    let mut job = egui::text::LayoutJob::simple(text.to_owned(), style.font(), color, width);
    job.wrap.max_rows = 1;
    job.wrap.break_anywhere = true;
    job.sections[0].format.line_height = Some(style.line_height());
    painter.layout_job(job)
}

fn caption_line() -> f32 {
    ty::STATE_LABEL_SMALL.line_height()
}

/// The height of the shelves: the label, the icons and two lines of captions.
pub fn height(tile: f32) -> f32 {
    ty::INFO_OVERLINE.line_height() + LABEL_GAP + tile + CAPTION_GAP + 2.0 * caption_line()
}

/// The largest icon size whose shelves fit in `room` (dp, from [`top`] down), if any does.
pub fn tile_for(room: f32) -> Option<f32> {
    TILES.into_iter().find(|tile| height(*tile) <= room)
}

/// Where the shelves start, from the top of the window.
pub fn top(screen: Rect) -> f32 {
    screen.min.y + pad::HOME_TOP + size::ICON_BUTTON + 34.0 + size::TILE_LARGE + 24.0 + ABOVE
}

/// The most games one shelf shows when `shelves` of them share the width of the window; fewer
/// rather than a squeezed row.
pub fn cap(screen: Rect, shelves: usize, tile: f32) -> usize {
    if shelves == 0 {
        return 0;
    }
    let width = screen.width() - 2.0 * pad::HERO_X - SHELF_GAP * (shelves - 1) as f32;
    let pitch = tile + ICON_GAP;
    (((width / shelves as f32 + ICON_GAP) / pitch).floor() as usize)
        .min(crate::app::library_view::SHELF_MAX)
}

fn label(kind: ShelfKind) -> T {
    match kind {
        ShelfKind::Friends => T::ShelfFriends,
        ShelfKind::Recent => T::ShelfRecent,
        ShelfKind::New => T::ShelfNew,
    }
}

/// Paints the shelves and reports the presses as the focus they ask for.
pub fn show(
    ui: &mut Ui,
    cx: &mut ScreenCx<'_>,
    top: f32,
    tile: f32,
    shelves: &[Shelf],
    presses: &mut Vec<HomeFocus>,
) {
    let ctx = cx.ctx.clone();
    let painter = ui.painter().clone();
    let language = cx.language();
    let accent = cx.accent();
    let interactive = cx.model.overlay.is_none() && cx.model.starting.is_none();
    let ring_on = interactive && matches!(cx.model.focus, HomeFocus::Shelf(_));
    let line = caption_line();
    let pitch = tile + ICON_GAP;
    let caption_width = tile + CAPTION_EXTRA;
    let mut x = ctx.content_rect().min.x + pad::HERO_X;
    let mut flat = 0usize;
    for shelf in shelves {
        let title = text::to_upper(label(shelf.kind).text(language), language);
        text::paint(
            &painter,
            pos2(x, top),
            Align2::LEFT_TOP,
            ty::INFO_OVERLINE,
            &title,
            color::text_overline(),
        );
        let icons_top = top + ty::INFO_OVERLINE.line_height() + LABEL_GAP;
        for item in &shelf.items {
            let index = flat;
            flat += 1;
            let Some(game) = cx.model.game(&item.game).cloned() else {
                continue;
            };
            let rect = Rect::from_min_size(pos2(x, icons_top), vec2(tile, tile));
            let cell = Rect::from_min_size(
                rect.min,
                vec2(caption_width, tile + CAPTION_GAP + 2.0 * line),
            );
            x += pitch;
            let focused = ring_on && cx.model.focus == HomeFocus::Shelf(index);
            let ring_t = ctx.focus_t(("shelf-ring", index), focused);
            let response = ui.interact(cell, Id::unique(("shelf-item", index)), Sense::click());
            let who = match (&item.friend, item.join.is_some()) {
                (Some(friend), true) => format!(
                    "{}, {}, {}",
                    game.name,
                    T::ShelfFriendPlays.format(language, &[crate::ui::i18n::Arg::Str(friend)]),
                    T::Join.text(language)
                ),
                (Some(friend), false) => format!(
                    "{}, {}",
                    game.name,
                    T::ShelfFriendPlays.format(language, &[crate::ui::i18n::Arg::Str(friend)])
                ),
                (None, _) => format!("{}, {}", game.name, label(shelf.kind).text(language)),
            };
            response.widget_info(|| WidgetInfo::labeled(Role::Button, interactive, who.as_str()));
            a11y::follow(&ctx, response.id, focused);
            let hovered = interactive && input::hovered(&ctx, &response);
            if hovered {
                ctx.set_cursor_icon(CursorIcon::PointingHand);
            }
            if interactive && response.clicked() {
                presses.push(HomeFocus::Shelf(index));
            }
            let mut style = tile_style(cx, &game, tile, accent);
            // A row of padlocks would be noise; the state of a game is on its page.
            style.overlay = TileOverlay::None;
            style.focus = ring_t;
            style.ring = ring_t > 0.002;
            style.hover = ctx.animate_to(("shelf-hover", index), f32::from(hovered), Motion::Hover);
            chrome::paint_tile(&painter, rect, &style);

            // The name, then who plays it or the way to join.
            let ink = if focused {
                color::text()
            } else {
                color::text_detail()
            };
            let name = one_line(&painter, &game.name, ink, caption_width);
            // A name cut to the caption is read in full while the pointer or the focus is on it.
            crate::ui::reveal::full_text(
                &ctx,
                response.id,
                cell,
                &game.name,
                hovered || (focused && input::focus_visible(&ctx)),
                name.elided,
            );
            let name_y = rect.max.y + CAPTION_GAP;
            painter.galley(pos2(rect.min.x, name_y), name, ink);
            let second_y = name_y + line;
            if let Some(friend) = &item.friend {
                if item.join.is_some() {
                    painter.text(
                        pos2(rect.min.x + 6.0, second_y + line * 0.5),
                        Align2::CENTER_CENTER,
                        Icon::USERS.glyph(),
                        Icon::USERS.font(12.0, true),
                        color::text_active(),
                    );
                    let join = clamped(
                        &painter,
                        ty::STATE_LABEL_SMALL,
                        T::Join.text(language),
                        color::text_active(),
                        caption_width - 20.0,
                        1,
                    );
                    painter.galley(
                        pos2(rect.min.x + 20.0, second_y),
                        join,
                        color::text_active(),
                    );
                } else {
                    let who = one_line(&painter, friend, color::state_neutral(), caption_width);
                    painter.galley(pos2(rect.min.x, second_y), who, color::state_neutral());
                }
            }
        }
        x += SHELF_GAP - ICON_GAP;
    }
}
