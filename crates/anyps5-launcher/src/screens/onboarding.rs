//! First start (docs/DESIGN.md 5.15): language, look, controller, games, online and a last page.
//!
//! A calm, centred setup over the live theme backdrop, like a console's first start: progress at
//! the top, the question in the middle, and one footer where Back, the step's one primary action
//! and the hints always stand in the same places. The model (`app::model::onboarding`) owns the
//! steps and the focus; this file paints them and reports taps.

use egui::epaint::{Mesh, Vertex, WHITE_UV};
use egui::{
    Align, Align2, Context, Id, Painter, Pos2, Rect, Role, Sense, Shape, Ui, WidgetInfo, pos2, vec2,
};

use crate::app::model::{PadSeen, Step, Target};
use crate::screens::controllers::button_label;
use crate::screens::controllers_strings::Pads;
use crate::screens::feel_strings::FeelText;
use crate::screens::ingame::InGameText;
use crate::screens::legal::{Legal, consent_line};
use crate::screens::onboarding_strings::OnboardingText as OT;
use crate::screens::online_strings::OnlineText;
use crate::screens::{ScreenCx, footer_hints, place};
use crate::ui::anim::{AnimExt, Motion, lerp};
use crate::ui::background::{Background, SLOT_CARDS, STILL_TIME_CARD};
use crate::ui::chrome::{self, onboarding_dots};
use crate::ui::components::{
    ConsoleButton, HintSet, PadGlyph, Switch, corner, hint_bar_set_size, interact,
    paint_focus_stroke, paint_hairline, paint_surface,
};
use crate::ui::glass::{GlassStyle, paint_glass};
use crate::ui::i18n::{Arg, Key, Language};
use crate::ui::icons::Icon;
use crate::ui::pad_hub;
use crate::ui::pad_info::Family;
use crate::ui::text;
use crate::ui::theme::{self, Theme, black, color, fade, mix, pad, radius, size, space, ty};

/// Steps slide in from this far to the side.
const STEP_SLIDE: f32 = 40.0;
/// The scrim over the backdrop: black at the top, a little more at the bottom where the footer is.
const SCRIM: (f32, f32) = (0.30, 0.52);
/// The widest a subtitle runs.
const SUBTITLE_WIDTH: f32 = 680.0;
/// The widest a title runs before it wraps.
const TITLE_WIDTH: f32 = 1100.0;
/// Where the progress dots stand below the top.
const DOTS_TOP: f32 = 48.0;
/// Where in the free room above and below the tallest page the content sits: a little above the
/// middle. Every page starts at the same height, so the title never moves between pages.
const CENTRE_BIAS: f32 = 0.42;
/// The tallest a page gets, title to last control, in dp (Look, in German).
const TALLEST: f32 = 540.0;
/// Neighbouring cards and chips of one group: the control gap.
const CARD_GAP: f32 = space::M;
/// The look cards in a row: six looks make two rows of three.
pub const THEMES_PER_ROW: usize = 3;
/// The theme cards, and the columns every row of the Look page shares.
const THEME_CARD: (f32, f32) = (312.0, 120.0);
/// The name chip on a look card: its height, and the dark it lies on (white ink on it reads on
/// the lightest look too).
const NAME_CHIP_HEIGHT: f32 = 32.0;
const NAME_CHIP_FILL: f32 = 0.55;
/// What the controller page, the games page and the lists are wide: two theme columns and a gap.
const LIST_WIDTH: f32 = 2.0 * THEME_CARD.0 + CARD_GAP;
/// The consent sentence wraps wider than the list above it, so it does not end on a lone word.
const CONSENT_WIDTH: f32 = 760.0;
const SWITCH_CHIP_HEIGHT: f32 = 56.0;
const CONFIRM_CARD: (f32, f32) = (THEME_CARD.0, 104.0);
const FACE_BUTTON: f32 = 44.0;
const BUTTON_MIN: f32 = 180.0;
const CHECK_GLYPH: f32 = 20.0;
/// The language list: its width, a row's height (a comfortable target), and how far a row's lit
/// shape stands in from the card, so the ring stays inside the card.
const LANGUAGE_LIST_WIDTH: f32 = 420.0;
const LANGUAGE_ROW: f32 = 56.0;
const LANGUAGE_ROW_INSET: f32 = 6.0;
const ACCENT_DOT: f32 = 6.0;
const STATUS_GLYPH: f32 = 24.0;
const ROW_GLYPH: f32 = 22.0;
/// Game tiles of the games page: as many as fit the list's width.
const TILE: f32 = 80.0;
const MAX_TILES: usize = 7;
/// A control that is not there (no pad), and a card that is not the chosen one.
const DIMMED: f32 = 0.4;
const UNCHOSEN: f32 = 0.72;
/// Black over the material of the glass cards.
const GLASS_BLACK: f32 = 0.38;

/// What a press on the onboarding asks the model to do.
enum Press {
    Tap(Target),
    Back,
}

/// Where a step stands on the screen.
#[derive(Clone, Copy)]
struct Page {
    /// Centre line, shifted while the step slides.
    x: f32,
    /// Top of the title.
    top: f32,
    /// Only the step the model is on takes presses.
    active: bool,
    language: Language,
    accent: egui::Color32,
}

/// Paints the onboarding and applies its presses.
pub fn show(ui: &mut Ui, cx: &mut ScreenCx<'_>) {
    let screen = cx.ctx.content_rect();
    let painter = ui.painter().clone();
    paint_scrim(&painter, screen);
    note_pad(cx);

    let language = cx.language();
    let flow = cx.model.onboarding_flow();
    let steps = flow.len();
    let position = cx.model.onboarding_position();
    let current = cx.model.onboarding.step;
    // The live button check needs a repaint with every pad event, on its step only.
    pad_hub::set_watched(current == Step::Controller);
    let mut presses = Vec::new();

    // Progress: the dots, and the same in words under them.
    let dots_width = chrome::onboarding_dots_width(steps);
    let dots_y = screen.min.y + DOTS_TOP;
    place(
        ui,
        Rect::from_min_size(
            pos2(screen.center().x - dots_width * 0.5, dots_y),
            vec2(dots_width, 5.0),
        ),
        |ui| {
            ui.spacing_mut().interact_size.y = 0.0;
            onboarding_dots(ui, steps, position)
        },
    );
    let progress = OT::StepOf.format(
        language,
        &[Arg::Int(position as i64 + 1), Arg::Int(steps as i64)],
    );
    let label = text::paint(
        &painter,
        pos2(screen.center().x, dots_y + 5.0 + space::M),
        Align2::CENTER_TOP,
        ty::DETAIL,
        &progress,
        color::text_note(),
    );
    ui.interact(
        label.expand(space::XS),
        Id::unique("onboarding-progress"),
        Sense::hover(),
    )
    .widget_info(|| WidgetInfo::labeled(Role::Label, true, progress.as_str()));

    // The footer stands level with the hint bar.
    let hints = HintSet::BASIC.without_back();
    let (_, hints_height) = hint_bar_set_size(&painter, language, hints);
    let footer_y = screen.max.y - pad::HINT_Y - hints_height * 0.5;
    // Every page starts at the same height: the free room between the progress and the footer,
    // less the tallest page, a little above the middle.
    let band = (
        dots_y + 5.0 + space::M + ty::DETAIL.line_height() + space::XL,
        footer_y - size::BUTTON_MIN_HEIGHT * 0.5 - space::XL,
    );
    let top = band.0 + ((band.1 - band.0 - TALLEST) * CENTRE_BIAS).max(0.0);

    // The steps slide and fade as the step number animates.
    let animated = cx
        .ctx
        .animate_to("onboarding-step", position as f32, Motion::Layout);
    let accent = cx.accent();
    let slide = if cx.ctx.reduced_motion() {
        0.0
    } else {
        STEP_SLIDE
    };
    for (index, step) in flow.iter().enumerate() {
        let d = animated - index as f32;
        if d.abs() >= 1.0 {
            continue;
        }
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(screen).id_salt(index));
        child.set_opacity((1.0 - d.abs()) * ui.painter().opacity());
        let page = Page {
            x: screen.center().x - slide * d,
            top,
            active: *step == current && index == position,
            language,
            accent,
        };
        match step {
            Step::Language => language_step(&mut child, cx, page, &mut presses),
            Step::Look => look_step(&mut child, cx, page, &mut presses),
            Step::Controller => controller_step(&mut child, cx, page, &mut presses),
            Step::Games => games_step(&mut child, cx, page, &mut presses),
            Step::Online => online_step(&mut child, cx, page, &mut presses),
            Step::Done => done_step(&mut child, cx, page),
        }
    }

    // The footer does not slide with the pages: the buttons of the step the player is on stand
    // where the last page had them.
    footer(
        ui,
        cx,
        screen.center().x,
        footer_y,
        &actions(cx, current, language),
        &mut presses,
    );

    // Back stands at the leading edge of the footer on every step but the first; the hint bar
    // then leaves out its own Back.
    if position > 0 {
        let button = ConsoleButton::new(language.text(Key::Back)).icon(Icon::CARET_LEFT);
        let width = button.width(&painter);
        let rect = Rect::from_min_size(
            pos2(
                screen.min.x + pad::HOME_X,
                footer_y - size::BUTTON_MIN_HEIGHT * 0.5,
            ),
            vec2(width, size::BUTTON_MIN_HEIGHT),
        );
        if place(ui, rect, |ui| button.show(ui)).clicked() {
            presses.push(Press::Back);
        }
    }
    footer_hints(ui, screen, language, hints);

    for press in presses {
        match press {
            Press::Tap(target) => cx.model.tap_onboarding(target),
            Press::Back => cx.model.onboarding_back(),
        }
    }
}

/// Tells the model which controller is connected, so it can add its step.
fn note_pad(cx: &mut ScreenCx<'_>) {
    let snap = pad_hub::snapshot();
    let pad = snap
        .pads
        .iter()
        .find(|p| p.active)
        .or(snap.pads.first())
        .map(|p| PadSeen {
            name: p.name.clone(),
            family: p.family,
        });
    cx.model.note_pad(pad);
}

/// Black at the top, a little more at the bottom.
fn paint_scrim(painter: &Painter, rect: Rect) {
    let mut mesh = Mesh::default();
    let (from, to) = (theme::scrim(SCRIM.0), theme::scrim(SCRIM.1));
    for (pos, colour) in [
        (rect.left_top(), from),
        (rect.right_top(), from),
        (rect.left_bottom(), to),
        (rect.right_bottom(), to),
    ] {
        mesh.vertices.push(Vertex {
            pos,
            uv: WHITE_UV,
            color: colour,
        });
    }
    mesh.indices.extend_from_slice(&[0, 1, 2, 1, 3, 2]);
    painter.add(Shape::mesh(mesh));
}

// ---------------------------------------------------------------------------------------------
// Building blocks
// ---------------------------------------------------------------------------------------------

/// Centred wrapped text with the leading a paragraph needs (1.4).
fn centred(
    painter: &Painter,
    style: theme::TextStyle,
    text: &str,
    ink: egui::Color32,
    width: f32,
    rows: usize,
    leading: f32,
) -> std::sync::Arc<egui::Galley> {
    let mut job = egui::text::LayoutJob::simple(text.to_owned(), style.font(), ink, width);
    job.wrap.max_rows = rows;
    job.halign = Align::Center;
    for section in &mut job.sections {
        section.format.line_height = Some(style.px() * leading);
        section.format.extra_letter_spacing = style.tracking;
    }
    painter.layout_job(job)
}

/// The same, left-aligned, for text inside a row.
fn flowing(
    painter: &Painter,
    style: theme::TextStyle,
    text: &str,
    ink: egui::Color32,
    width: f32,
) -> std::sync::Arc<egui::Galley> {
    let mut job = egui::text::LayoutJob::simple(text.to_owned(), style.font(), ink, width);
    job.wrap.max_rows = 3;
    for section in &mut job.sections {
        section.format.line_height = Some(style.px() * 1.4);
    }
    painter.layout_job(job)
}

/// Title and subtitle of a step, centred; returns the y at which the step's content starts.
/// `sentence_rows` is the room the sentence keeps at the least.
fn heading(
    painter: &Painter,
    page: Page,
    title: &str,
    subtitle: &str,
    sentence_rows: usize,
) -> f32 {
    let title = centred(
        painter,
        ty::DISPLAY,
        title,
        color::text(),
        TITLE_WIDTH,
        2,
        1.15,
    );
    let height = title.size().y;
    painter.galley(pos2(page.x, page.top), title, color::text());
    let sub_y = page.top + height + space::M;
    let sub = centred(
        painter,
        ty::CATEGORY,
        subtitle,
        color::text_detail(),
        SUBTITLE_WIDTH,
        3,
        1.4,
    );
    // A page whose words change while the focus moves keeps room for the longest sentence, so
    // nothing under it moves.
    let sub_height = sub
        .size()
        .y
        .max(sentence_rows as f32 * ty::CATEGORY.px() * 1.4);
    painter.galley(pos2(page.x, sub_y), sub, color::text_detail());
    sub_y + sub_height + space::XXL
}

/// A small caption over a group of choices, centred.
fn caption(painter: &Painter, page: Page, y: f32, words: &str) -> f32 {
    text::paint(
        painter,
        pos2(page.x, y),
        Align2::CENTER_TOP,
        ty::OVERLINE,
        &text::to_upper(words, page.language),
        color::text_detail(),
    );
    y + ty::OVERLINE.line_height() + space::L
}

/// The left edge of `count` equal items with `gap` between, centred on the page.
fn row_start(page: Page, count: usize, width: f32, gap: f32) -> f32 {
    let total = count as f32 * width + count.saturating_sub(1) as f32 * gap;
    page.x - total * 0.5
}

/// What a footer button is.
struct Action<'a> {
    label: &'a str,
    icon: Icon,
    target: Target,
}

/// The buttons of a step's footer: the first is the step's one primary action. The model says
/// what each `Target` does; this says how it reads.
fn actions(cx: &ScreenCx<'_>, step: Step, language: Language) -> Vec<Action<'static>> {
    let go = |label: &'static str, target| Action {
        label,
        icon: Icon::ARROW_RIGHT,
        target,
    };
    let pick = |label: &'static str, target| Action {
        label,
        icon: Icon::FOLDER,
        target,
    };
    match step {
        Step::Language | Step::Look | Step::Controller => {
            vec![go(language.text(Key::ContinueAction), Target::Primary)]
        }
        Step::Games => {
            let folder = cx.model.prefs.games_folder.is_some();
            if cx.model.onboarding_games_pick_first() {
                // No folder, or a folder with no games in it: choosing is what the player came for.
                if folder {
                    vec![
                        pick(language.text(Key::ChangeFolder), Target::Primary),
                        go(language.text(Key::ContinueAction), Target::Secondary),
                    ]
                } else {
                    vec![
                        pick(language.text(Key::PickFolder), Target::Primary),
                        go(OT::NotNow.text(language), Target::Secondary),
                    ]
                }
            } else {
                vec![
                    go(language.text(Key::ContinueAction), Target::Primary),
                    pick(language.text(Key::ChangeFolder), Target::Secondary),
                ]
            }
        }
        Step::Online => vec![
            Action {
                label: OnlineText::SignIn.text(language),
                icon: Icon::SIGN_IN,
                target: Target::SignIn,
            },
            Action {
                label: OnlineText::CreateAccount.text(language),
                icon: Icon::USER_PLUS,
                target: Target::CreateAccount,
            },
            go(OT::NotNow.text(language), Target::NotNow),
        ],
        Step::Done => vec![go(language.text(Key::Start), Target::Primary)],
    }
}

/// The footer's buttons, centred on `x` and level with Back and the hints.
fn footer(
    ui: &mut Ui,
    cx: &ScreenCx<'_>,
    x: f32,
    y: f32,
    actions: &[Action<'_>],
    presses: &mut Vec<Press>,
) {
    let painter = ui.painter().clone();
    let focus = cx.model.onboarding_target();
    let buttons: Vec<ConsoleButton<'_>> = actions
        .iter()
        .enumerate()
        .map(|(index, action)| {
            ConsoleButton::new(action.label)
                .icon(action.icon)
                .prominent(index == 0)
                .min_width(BUTTON_MIN)
                .focused(focus == action.target)
        })
        .collect();
    let widths: Vec<f32> = buttons.iter().map(|b| b.width(&painter)).collect();
    let total: f32 = widths.iter().sum::<f32>() + size::CONTROL_GAP * (widths.len() - 1) as f32;
    let mut left = x - total * 0.5;
    for ((button, width), action) in buttons.into_iter().zip(widths).zip(actions) {
        let rect = Rect::from_min_size(
            pos2(left, y - size::BUTTON_MIN_HEIGHT * 0.5),
            vec2(width, size::BUTTON_MIN_HEIGHT),
        );
        if place(ui, rect, |ui| button.show(ui)).clicked() {
            presses.push(Press::Tap(action.target));
        }
        left += width + size::CONTROL_GAP;
    }
}

/// A row with a label, an optional detail and a switch: one setting. Returns whether it was
/// clicked and the height it took.
#[allow(clippy::too_many_arguments)]
fn switch_chip(
    ui: &mut Ui,
    origin: Pos2,
    width: f32,
    label: &str,
    detail: Option<&str>,
    on: bool,
    focused: bool,
    active: bool,
    accent: egui::Color32,
) -> (bool, f32) {
    let painter = ui.painter().clone();
    let text_width = width - 2.0 * space::L - size::SWITCH.0 - space::L;
    let detail_galley =
        detail.map(|d| flowing(&painter, ty::NOTE, d, color::text_detail(), text_width));
    let content = ty::ROW_TITLE.line_height()
        + detail_galley
            .as_ref()
            .map_or(0.0, |g| space::XS + g.size().y);
    let height = (content + 2.0 * space::M + space::XS).max(SWITCH_CHIP_HEIGHT);
    let rect = Rect::from_min_size(origin, vec2(width, height));
    let it = place(ui, rect, |ui| {
        interact(ui, rect.size(), focused && active, active, None)
    });
    it.response
        .widget_info(|| WidgetInfo::selected(Role::Switch, true, on, label));
    let painted = it.painted(&painter);
    paint_surface(
        &painter,
        painted,
        radius::CARD,
        color::surface_row(),
        it.hover,
        it.focus,
        it.press,
        size::FOCUS_RING_LARGE,
    );
    paint_hairline(&painter, painted, radius::CARD, color::hairline_card());
    let text_x = painted.min.x + space::L;
    let top = painted.center().y - content * 0.5;
    text::paint(
        &painter,
        pos2(text_x, top),
        Align2::LEFT_TOP,
        ty::ROW_TITLE,
        label,
        color::text(),
    );
    if let Some(galley) = detail_galley {
        painter.galley(
            pos2(text_x, top + ty::ROW_TITLE.line_height() + space::XS),
            galley,
            color::text_detail(),
        );
    }
    let switch = Rect::from_center_size(
        pos2(
            painted.max.x - space::L - size::SWITCH.0 * 0.5,
            painted.center().y,
        ),
        vec2(size::SWITCH.0, size::SWITCH.1),
    );
    let inner = place(ui, switch, |ui| {
        Switch::new(on, accent)
            .announce(false)
            .focus_amount(it.focus)
            .show(ui)
    });
    (active && (it.response.clicked() || inner.clicked()), height)
}

/// One row of a list card.
struct ListRow<'a> {
    icon: Icon,
    title: &'a str,
    detail: &'a str,
}

/// A glass card holding rows separated by hairlines: a list of facts, not a card per fact.
/// Returns the y under it.
fn list_card(
    ctx: &Context,
    painter: &Painter,
    page: Page,
    y: f32,
    width: f32,
    rows: &[ListRow<'_>],
) -> f32 {
    let text_x_offset = space::XL + ROW_GLYPH + space::M;
    let text_width = width - text_x_offset - space::XL;
    let laid: Vec<_> = rows
        .iter()
        .map(|row| {
            flowing(
                painter,
                ty::NOTE,
                row.detail,
                color::text_detail(),
                text_width,
            )
        })
        .collect();
    let heights: Vec<f32> = laid
        .iter()
        .map(|g| ty::ROW_TITLE.line_height() + space::XS + g.size().y + 2.0 * space::L)
        .collect();
    let card = Rect::from_min_size(
        pos2(page.x - width * 0.5, y),
        vec2(width, heights.iter().sum()),
    );
    paint_glass(
        ctx,
        painter,
        card,
        GlassStyle::glass(radius::CARD, GLASS_BLACK),
    );
    let mut top = card.min.y;
    for (index, ((row, galley), height)) in rows.iter().zip(laid).zip(heights).enumerate() {
        if index > 0 {
            painter.line_segment(
                [
                    pos2(card.min.x + space::XL, top),
                    pos2(card.max.x - space::XL, top),
                ],
                egui::Stroke::new(1.0, color::hairline_glass()),
            );
        }
        painter.text(
            pos2(
                card.min.x + space::XL + ROW_GLYPH * 0.5,
                top + space::L + ty::ROW_TITLE.line_height() * 0.5,
            ),
            Align2::CENTER_CENTER,
            row.icon.glyph(),
            row.icon.font(ROW_GLYPH, false),
            color::text_label(),
        );
        let text_x = card.min.x + text_x_offset;
        text::paint(
            painter,
            pos2(text_x, top + space::L),
            Align2::LEFT_TOP,
            ty::ROW_TITLE,
            row.title,
            color::text(),
        );
        painter.galley(
            pos2(
                text_x,
                top + space::L + ty::ROW_TITLE.line_height() + space::XS,
            ),
            galley,
            color::text_detail(),
        );
        top += height;
    }
    card.max.y
}

// ---------------------------------------------------------------------------------------------
// Steps
// ---------------------------------------------------------------------------------------------

fn language_step(ui: &mut Ui, cx: &mut ScreenCx<'_>, page: Page, presses: &mut Vec<Press>) {
    let painter = ui.painter().clone();
    let language = page.language;
    let y = heading(
        &painter,
        page,
        language.text(Key::Welcome),
        language.text(Key::WelcomeSubtitle),
        2,
    );
    let focus = cx.model.onboarding_target();
    // One calm list: the languages' own names, one to a row, in a single glass card.
    let count = Language::ALL.len();
    let card = Rect::from_min_size(
        pos2(page.x - LANGUAGE_LIST_WIDTH * 0.5, y),
        vec2(LANGUAGE_LIST_WIDTH, count as f32 * LANGUAGE_ROW),
    );
    paint_glass(
        cx.ctx,
        &painter,
        card,
        GlassStyle::glass(radius::CARD, GLASS_BLACK),
    );
    for (index, row_language) in Language::ALL.iter().enumerate() {
        let rect = Rect::from_min_size(
            pos2(card.min.x, card.min.y + index as f32 * LANGUAGE_ROW),
            vec2(LANGUAGE_LIST_WIDTH, LANGUAGE_ROW),
        );
        // A hairline between two rows, left out beside the chosen one, whose lit shape is the edge.
        let beside_chosen = *row_language == language
            || index
                .checked_sub(1)
                .is_some_and(|i| Language::ALL[i] == language);
        if index > 0 && !beside_chosen {
            painter.line_segment(
                [
                    pos2(rect.min.x + space::XL, rect.min.y),
                    pos2(rect.max.x - space::XL, rect.min.y),
                ],
                egui::Stroke::new(1.0, color::hairline_glass()),
            );
        }
        let response = place(ui, rect, |ui| {
            language_row(
                ui,
                *row_language,
                *row_language == language,
                focus == Target::Language(index),
                page.active,
            )
        });
        if page.active && response.clicked() {
            presses.push(Press::Tap(Target::Language(index)));
        }
    }
}

/// One row of the language list: the language's own name, and a check at the trailing edge when
/// it is the chosen one. The chosen row is lit (`surface_hover`, name in full ink) so the choice
/// never rests on tone alone; the ring shows only for keys and the pad.
pub(crate) fn language_row(
    ui: &mut Ui,
    language: Language,
    chosen: bool,
    focused: bool,
    active: bool,
) -> egui::Response {
    let name = language.display_name();
    let it = interact(
        ui,
        vec2(LANGUAGE_LIST_WIDTH, LANGUAGE_ROW),
        focused && active,
        active,
        None,
    );
    it.response
        .widget_info(|| WidgetInfo::selected(Role::RadioButton, true, chosen, name));
    let painter = ui.painter();
    let t = ui.ctx().choice_t((it.response.id, "chosen"), chosen);
    // Concentric with the card: its radius less the inset.
    let lit = it.painted(painter).shrink(LANGUAGE_ROW_INSET);
    paint_surface(
        painter,
        lit,
        radius::CARD - LANGUAGE_ROW_INSET,
        fade(color::surface_hover(), t),
        it.hover,
        it.focus,
        it.press,
        size::FOCUS_RING,
    );
    let ink = mix(color::text_label(), color::text(), t);
    text::paint(
        painter,
        pos2(lit.min.x + space::XL - LANGUAGE_ROW_INSET, lit.center().y),
        Align2::LEFT_CENTER,
        ty::THEME_CARD_NAME,
        name,
        ink,
    );
    if t > 0.01 {
        let icon = Icon::CHECK_CIRCLE;
        painter.text(
            pos2(
                lit.max.x - space::XL + LANGUAGE_ROW_INSET - CHECK_GLYPH * 0.5,
                lit.center().y,
            ),
            Align2::CENTER_CENTER,
            icon.glyph(),
            icon.font(CHECK_GLYPH, true),
            fade(color::text(), t),
        );
    }
    it.response
}

fn look_step(ui: &mut Ui, cx: &mut ScreenCx<'_>, page: Page, presses: &mut Vec<Press>) {
    let ctx = cx.ctx.clone();
    let painter = ui.painter().clone();
    let language = page.language;
    let mut y = heading(
        &painter,
        page,
        OT::LookTitle.text(language),
        OT::LookSubtitle.text(language),
        0,
    );
    let focus = cx.model.onboarding_target();
    let current = cx.model.theme();
    let opacity = painter.opacity();
    let (width, preview_height) = THEME_CARD;
    let column_height = preview_height;
    let per_row = THEMES_PER_ROW;
    let left = row_start(page, per_row, width, CARD_GAP);
    let rows = Theme::ALL.len().div_ceil(per_row);
    let top = y;
    for (index, theme) in Theme::ALL.iter().enumerate() {
        let (column_index, row_index) = (index % per_row, index / per_row);
        let y = top + row_index as f32 * (column_height + CARD_GAP);
        let focused = page.active && focus == Target::Theme(index);
        let chosen = current == *theme;
        let t = ctx.choice_t(("onboarding-theme", index), chosen);
        let resting_left = left + column_index as f32 * (width + CARD_GAP);
        let column = Rect::from_min_size(pos2(resting_left, y), vec2(width, column_height));
        let preview = Rect::from_min_size(column.min, vec2(width, preview_height));
        let card_radius = radius::THEME_CARD;
        // The chosen card is live, the others hold a still frame.
        let time = if chosen {
            STILL_TIME_CARD + ctx.clock() as f32
        } else {
            STILL_TIME_CARD
        };
        let mut layer = Background::new(SLOT_CARDS + theme.index(), *theme, time);
        layer.radius = card_radius;
        layer.alpha = lerp(UNCHOSEN, 1.0, t) * opacity;
        layer.paint(&painter, preview);
        let ring = ctx.focus_t(("onboarding-theme-ring", index), focused);
        paint_focus_stroke(&painter, preview, card_radius, size::FOCUS_RING_LARGE, ring);
        // The live card moves; under Reduced motion it holds still and nothing repaints.
        if chosen && !ctx.reduced_motion() {
            ctx.request_repaint();
        }

        // The name sits on the card in a dark chip, so it reads on the bright looks too.
        let style = if chosen {
            ty::THEME_CARD_NAME_FOCUSED
        } else {
            ty::THEME_CARD_NAME
        };
        let shown = lerp(UNCHOSEN, 1.0, t) * opacity;
        let title = theme.title(language);
        let name_width = text::width(&painter, style, title);
        let chip = Rect::from_min_size(
            pos2(
                preview.min.x + space::M,
                preview.max.y - space::M - NAME_CHIP_HEIGHT,
            ),
            vec2(
                space::M + name_width + space::M + 2.0 * ACCENT_DOT + space::M,
                NAME_CHIP_HEIGHT,
            ),
        );
        painter.rect_filled(
            chip,
            corner(NAME_CHIP_HEIGHT * 0.5),
            fade(black(NAME_CHIP_FILL), shown),
        );
        let name_y = chip.center().y;
        let name = text::paint(
            &painter,
            pos2(chip.min.x + space::M, name_y),
            Align2::LEFT_CENTER,
            style,
            title,
            fade(egui::Color32::WHITE, shown),
        );
        painter.circle_filled(
            pos2(name.max.x + space::M + ACCENT_DOT, name_y),
            ACCENT_DOT,
            fade(theme.default_accent().color(), shown),
        );
        // The check sits on the card, like the language cards', over a dark disc so it reads on
        // the bright themes too.
        if t > 0.01 {
            let icon = Icon::CHECK_CIRCLE;
            let at = pos2(
                preview.max.x - space::L - CHECK_GLYPH * 0.5,
                preview.min.y + space::L + CHECK_GLYPH * 0.5,
            );
            painter.circle_filled(at, CHECK_GLYPH * 0.5 + space::XS, black(0.4 * t));
            painter.text(
                at,
                Align2::CENTER_CENTER,
                icon.glyph(),
                icon.font(CHECK_GLYPH, true),
                fade(egui::Color32::WHITE, t),
            );
        }

        if page.active {
            let response = ui.interact(
                column,
                Id::unique(("onboarding-theme-card", index)),
                Sense::click(),
            );
            let label = theme.title(language);
            response.widget_info(|| WidgetInfo::selected(Role::RadioButton, true, chosen, label));
            crate::ui::a11y::follow(&ctx, response.id, focused);
            if crate::ui::input::hovered(&ctx, &response) {
                ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
            }
            if response.clicked() {
                presses.push(Press::Tap(Target::Theme(index)));
            }
        }
    }
    y = top + rows as f32 * column_height + (rows - 1) as f32 * CARD_GAP + space::XXL;

    // Three switches under the cards, in the same columns.
    y = caption(&painter, page, y, OT::Accessibility.text(language));
    let prefs = &cx.model.prefs;
    let chips = [
        (
            Target::LargerText,
            FeelText::LargerText.text(language),
            prefs.larger_text,
        ),
        (
            Target::HighContrast,
            FeelText::HighContrast.text(language),
            prefs.high_contrast,
        ),
        (
            Target::ReducedMotion,
            OT::ReduceMotion.text(language),
            prefs.reduced_motion,
        ),
    ];
    for (index, (target, label, on)) in chips.into_iter().enumerate() {
        let (clicked, _) = switch_chip(
            ui,
            pos2(left + index as f32 * (width + CARD_GAP), y),
            width,
            label,
            None,
            on,
            focus == target,
            page.active,
            page.accent,
        );
        if clicked {
            presses.push(Press::Tap(target));
        }
    }
}

/// The label of a face button for a pad family ("Cross", "A").
fn face_name(family: Family, button: &str) -> &'static str {
    button_label(family, button)
}

/// A face button as the player knows it: the PlayStation shapes, or the letter.
fn paint_face(
    painter: &Painter,
    centre: Pos2,
    diameter: f32,
    family: Family,
    button: &str,
    ink: egui::Color32,
) {
    let glyph = diameter * 0.5;
    match (family, button) {
        (Family::PlayStation, "South") => PadGlyph::Cross.paint(painter, centre, glyph, ink),
        (Family::PlayStation, "East") => PadGlyph::Circle.paint(painter, centre, glyph, ink),
        (Family::PlayStation, "West") => PadGlyph::Square.paint(painter, centre, glyph, ink),
        (Family::PlayStation, _) => PadGlyph::Triangle.paint(painter, centre, glyph, ink),
        _ => {
            text::paint(
                painter,
                centre,
                Align2::CENTER_CENTER,
                ty::ITEM_TITLE,
                face_name(family, button),
                ink,
            );
        }
    }
}

fn controller_step(ui: &mut Ui, cx: &mut ScreenCx<'_>, page: Page, presses: &mut Vec<Press>) {
    let ctx = cx.ctx.clone();
    let painter = ui.painter().clone();
    let language = page.language;
    let (name, family, connected) = cx.model.onboarding_pad().map_or(
        (String::new(), Family::PlayStation, false),
        |(pad, connected)| (pad.name.clone(), pad.family, connected),
    );
    let subtitle = if connected {
        OT::PadConnected.format(language, &[Arg::Str(&name)])
    } else {
        OT::PadLost.text(language).to_owned()
    };
    let mut y = heading(&painter, page, OT::PadTitle.text(language), &subtitle, 0);

    // The button check: the four face buttons light while the player presses them.
    let live = pad_hub::snapshot().live;
    let gap = space::M;
    let left = page.x - (4.0 * FACE_BUTTON + 3.0 * gap) * 0.5;
    for (index, button) in ["West", "North", "East", "South"].into_iter().enumerate() {
        let centre = pos2(
            left + FACE_BUTTON * 0.5 + index as f32 * (FACE_BUTTON + gap),
            y + FACE_BUTTON * 0.5,
        );
        let lit = ctx.flag_t(
            ("onboarding-face", index),
            connected && live.is_down(button),
            Motion::Press,
        );
        let rest = if connected { 1.0 } else { DIMMED };
        painter.circle_filled(
            centre,
            FACE_BUTTON * 0.5,
            fade(mix(color::surface(), color::primary(), lit), rest),
        );
        painter.circle_stroke(
            centre,
            FACE_BUTTON * 0.5,
            egui::Stroke::new(1.0, fade(color::hairline_control(), rest)),
        );
        paint_face(
            &painter,
            centre,
            FACE_BUTTON,
            family,
            button,
            fade(mix(color::text_label(), color::on_primary(), lit), rest),
        );
    }
    y += FACE_BUTTON + space::XL;

    // Which face button confirms.
    let focus = cx.model.onboarding_target();
    let circle = crate::screens::controllers::confirm_is_circle(&cx.model.prefs);
    let left = row_start(page, 2, CONFIRM_CARD.0, CARD_GAP);
    for (index, (this, other)) in [("South", "East"), ("East", "South")]
        .into_iter()
        .enumerate()
    {
        let rect = Rect::from_min_size(
            pos2(left + index as f32 * (CONFIRM_CARD.0 + CARD_GAP), y),
            vec2(CONFIRM_CARD.0, CONFIRM_CARD.1),
        );
        let title = Pads::ConfirmWith.format(language, &[Arg::Str(face_name(family, this))]);
        let (response, painted, t) = place(ui, rect, |ui| {
            chrome::choice_card(
                ui,
                rect.size(),
                circle == (index == 1),
                focus == Target::Confirm(index),
                page.active,
                &title,
            )
        });
        let badge = pos2(
            painted.min.x + space::XL + FACE_BUTTON * 0.5,
            painted.center().y,
        );
        painter.circle_stroke(
            badge,
            FACE_BUTTON * 0.5,
            egui::Stroke::new(1.0, color::hairline_control()),
        );
        paint_face(&painter, badge, FACE_BUTTON, family, this, color::text());
        let text_x = badge.x + FACE_BUTTON * 0.5 + space::L;
        let block = ty::ROW_TITLE.line_height() + space::XS + ty::NOTE.line_height();
        let top = painted.center().y - block * 0.5;
        text::paint(
            &painter,
            pos2(text_x, top),
            Align2::LEFT_TOP,
            ty::ROW_TITLE,
            &title,
            mix(color::text_label(), color::text(), t),
        );
        text::paint(
            &painter,
            pos2(text_x, top + ty::ROW_TITLE.line_height() + space::XS),
            Align2::LEFT_TOP,
            ty::NOTE,
            &OT::PadGoesBack.format(language, &[Arg::Str(face_name(family, other))]),
            color::text_detail(),
        );
        if page.active && response.clicked() {
            presses.push(Press::Tap(Target::Confirm(index)));
        }
    }
    // The Home button is another question than the confirm button: a group of its own.
    y += CONFIRM_CARD.1 + space::XL;

    let (clicked, _) = switch_chip(
        ui,
        pos2(page.x - LIST_WIDTH * 0.5, y),
        LIST_WIDTH,
        InGameText::Setting.text(language),
        Some(InGameText::SettingDetail.text(language)),
        cx.model.prefs.ps_button,
        focus == Target::HomeButton,
        page.active,
        page.accent,
    );
    if clicked {
        presses.push(Press::Tap(Target::HomeButton));
    }
}

fn games_step(ui: &mut Ui, cx: &mut ScreenCx<'_>, page: Page, presses: &mut Vec<Press>) {
    let ctx = cx.ctx.clone();
    let painter = ui.painter().clone();
    let language = page.language;
    let mut y = heading(
        &painter,
        page,
        language.text(Key::ChooseFolder),
        language.text(Key::ChooseFolderSubtitle),
        0,
    );
    let focus = cx.model.onboarding_target();
    let folder = cx.model.prefs.games_folder.clone();
    let accent = page.accent;
    let scanning = cx.model.scanning;
    let found = cx.model.games.len();

    // The status card: what is there, or that nothing is yet.
    let status_line = ty::ROW_TITLE.line_height();
    let path_line = ty::DETAIL.line_height();
    let content_height = status_line + space::XS + path_line;
    let card = Rect::from_min_size(
        pos2(page.x - LIST_WIDTH * 0.5, y),
        vec2(LIST_WIDTH, content_height + 2.0 * space::XL),
    );
    paint_glass(
        &ctx,
        &painter,
        card,
        GlassStyle::glass(radius::CARD, GLASS_BLACK),
    );
    let (icon, fill, tint) = match (&folder, scanning, found) {
        (None, _, _) => (Icon::FOLDER_SIMPLE_DASHED, false, color::text_detail()),
        (_, true, _) => (Icon::MAGNIFYING_GLASS, false, color::text_label()),
        (_, _, 0) => (Icon::FOLDER_SIMPLE_DASHED, false, color::text_detail()),
        _ => (Icon::CHECK_CIRCLE, true, color::text_label()),
    };
    painter.text(
        pos2(card.min.x + space::XL + STATUS_GLYPH * 0.5, card.center().y),
        Align2::CENTER_CENTER,
        icon.glyph(),
        icon.font(STATUS_GLYPH, fill),
        tint,
    );
    let text_x = card.min.x + space::XL + STATUS_GLYPH + space::M;
    let (status, path) = match &folder {
        None => (
            OT::NoFolderTitle.text(language).to_owned(),
            OT::NoFolderDetail.text(language).to_owned(),
        ),
        Some(folder) => (
            if scanning {
                language.text(Key::Scanning).to_owned()
            } else if found == 0 {
                language.text(Key::NoGamesFound).to_owned()
            } else {
                cx.model.games_found_text()
            },
            text::middle_truncate(
                &painter,
                ty::DETAIL,
                &folder.to_string_lossy(),
                card.max.x - space::XL - text_x,
            ),
        ),
    };
    let text_top = card.center().y - content_height * 0.5;
    text::paint(
        &painter,
        pos2(text_x, text_top),
        Align2::LEFT_TOP,
        ty::ROW_TITLE,
        &status,
        color::text(),
    );
    text::paint(
        &painter,
        pos2(text_x, text_top + status_line + space::XS),
        Align2::LEFT_TOP,
        ty::DETAIL,
        &path,
        color::text_detail(),
    );
    y = card.max.y + space::XL;

    if folder.is_some() {
        // The first games, as many as fit the list's width, centred.
        let shown = cx.model.games.len().min(MAX_TILES);
        let side = TILE;
        if shown > 0 {
            let left = row_start(page, shown, side, space::M);
            for (index, game) in cx.model.games.clone().iter().take(shown).enumerate() {
                let rect = Rect::from_min_size(
                    pos2(left + index as f32 * (side + space::M), y),
                    vec2(side, side),
                );
                let style = crate::screens::home::tile_style(cx, game, side, accent);
                chrome::paint_tile(&painter, rect, &style);
            }
            y += side + space::XL;
        }
        let (clicked, _) = switch_chip(
            ui,
            pos2(page.x - LIST_WIDTH * 0.5, y),
            LIST_WIDTH,
            crate::screens::prepare_strings::setting(language),
            Some(crate::screens::prepare_strings::setting_detail(language)),
            cx.model.prefs.prepare_games,
            focus == Target::Prepare,
            page.active,
            accent,
        );
        if clicked {
            presses.push(Press::Tap(Target::Prepare));
        }
    }
}

fn online_step(ui: &mut Ui, cx: &mut ScreenCx<'_>, page: Page, presses: &mut Vec<Press>) {
    let ctx = cx.ctx.clone();
    let painter = ui.painter().clone();
    let language = page.language;
    let mut y = heading(
        &painter,
        page,
        OT::OnlineTitle.text(language),
        OT::OnlineSubtitle.text(language),
        0,
    );
    // The defaults, in plain words, before anyone signs in.
    let rows = [
        ListRow {
            icon: Icon::LOCK,
            title: OT::FactPrivateTitle.text(language),
            detail: OT::FactPrivateDetail.text(language),
        },
        ListRow {
            icon: Icon::EYE_SLASH,
            title: OnlineText::AppearOffline.text(language),
            detail: OT::FactOfflineDetail.text(language),
        },
        ListRow {
            icon: Icon::HARD_DRIVE,
            title: OT::FactLocalTitle.text(language),
            detail: OT::FactLocalDetail.text(language),
        },
    ];
    y = list_card(&ctx, &painter, page, y, LIST_WIDTH, &rows) + space::XL;

    // What creating an account means agreeing to, then where signing in waits later.
    let focus = cx.model.onboarding_target();
    let linked = match focus {
        Target::Terms => Some(Legal::Terms),
        Target::Privacy => Some(Legal::Privacy),
        _ => None,
    };
    let (legal, height) = consent_line(
        ui,
        pos2(page.x, y),
        CONSENT_WIDTH,
        language,
        page.active,
        true,
        linked.filter(|_| page.active),
    );
    match legal {
        Some(Legal::Terms) => presses.push(Press::Tap(Target::Terms)),
        Some(Legal::Privacy) => presses.push(Press::Tap(Target::Privacy)),
        None => {}
    }
    text::paint(
        &painter,
        pos2(page.x, y + height + space::S),
        Align2::CENTER_TOP,
        ty::DETAIL,
        OT::OnlineLater.text(language),
        color::text_note(),
    );
}

fn done_step(ui: &mut Ui, cx: &mut ScreenCx<'_>, page: Page) {
    let ctx = cx.ctx.clone();
    let painter = ui.painter().clone();
    let language = page.language;
    let y = heading(
        &painter,
        page,
        OT::DoneTitle.text(language),
        OT::DoneSubtitle.text(language),
        0,
    );
    let pad_found = cx.model.onboarding_pad().is_some();
    let home_button = pad_found && cx.model.prefs.ps_button;
    let folder = cx.model.prefs.games_folder.is_some();
    let mut rows = Vec::new();
    // A sign-in the player asked for comes first: it happens when Start is pressed.
    match cx.model.onboarding_sign_in() {
        Some(false) => rows.push(ListRow {
            icon: Icon::SIGN_IN,
            title: OT::DoneNextSignIn.text(language),
            detail: OT::DoneSignInDetail.text(language),
        }),
        Some(true) => rows.push(ListRow {
            icon: Icon::USER_PLUS,
            title: OT::DoneNextCreate.text(language),
            detail: OT::DoneSignInDetail.text(language),
        }),
        None => {}
    }
    // Steam owns the buttons in Gaming Mode: no row about them there.
    if !cx.model.onboarding_gaming_mode() {
        rows.push(if home_button {
            ListRow {
                icon: Icon::GAME_CONTROLLER,
                title: OT::DoneHomeTitle.text(language),
                detail: OT::DoneHomeDetail.text(language),
            }
        } else {
            ListRow {
                icon: Icon::GAME_CONTROLLER,
                title: OT::DonePadTitle.text(language),
                detail: OT::DonePadDetail.text(language),
            }
        });
    }
    rows.push(ListRow {
        icon: Icon::GEAR_SIX,
        title: OT::DoneSettingsTitle.text(language),
        detail: OT::DoneSettingsDetail.text(language),
    });
    rows.push(if folder {
        ListRow {
            icon: Icon::SQUARES_FOUR,
            title: OT::DoneGamesTitle.text(language),
            detail: OT::DoneGamesDetail.text(language),
        }
    } else {
        ListRow {
            icon: Icon::FOLDER,
            title: OT::DoneNoFolderTitle.text(language),
            detail: OT::DoneNoFolderDetail.text(language),
        }
    });
    list_card(&ctx, &painter, page, y, LIST_WIDTH, &rows);
}
