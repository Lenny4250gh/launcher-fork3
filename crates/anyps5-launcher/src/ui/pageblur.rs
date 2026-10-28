//! The page behind a card, blurred (docs/DESIGN.md 5.9.1).
//!
//! A question stands on a soft blur of the page it covers (the tiles, the hero, the text), not on
//! the theme's backdrop alone. egui draws the page in one pass, so the page cannot be blurred while
//! it is drawn; instead the window is captured ONCE when the card opens, shrunk to an eighth,
//! blurred on the CPU (a few thousand pixels) and kept as a small texture that the card's veil
//! stretches over the screen. While the card is open nothing is recomputed: a frame costs one
//! textured quad. The texture is dropped when the card is gone.
//!
//! Until the capture arrives (one or two frames) and under High Contrast the caller keeps its flat
//! scrim.

use egui::{
    Color32, ColorImage, Context, Event, Id, Painter, Rect, TextureHandle, TextureOptions,
    UserData, ViewportCommand, pos2,
};

/// How much the page is shrunk before it is blurred (the blur's strength grows with it).
const SHRINK: usize = 8;
/// Box blur radius, in shrunk pixels, and the number of passes (two boxes are close to a gaussian).
const RADIUS: usize = 3;
const PASSES: usize = 2;
/// How dark the blurred page is under the card (1 = as it is).
const DIM: f32 = 0.60;
/// Seconds for the blur to fade in once the capture has arrived.
const READY_FADE: f32 = 0.15;

/// Marks the capture this module asked for, so no other screenshot is taken for its own.
struct Tag;

#[derive(Clone, Default)]
struct Held {
    texture: Option<TextureHandle>,
    asked: bool,
}

/// Paints the blurred page over `rect` for the card `id`; `v` is the card's openness.
pub fn paint(ctx: &Context, painter: &Painter, rect: Rect, id: Id, open: bool, v: f32) {
    if crate::ui::theme::high_contrast() {
        return;
    }
    let key = id.with("page-blur");
    let mut held = ctx.data(|d| d.get_temp::<Held>(key)).unwrap_or_default();
    if open && !held.asked {
        // The card is at about 0 on its first frame, so the capture shows the page alone.
        held.asked = true;
        ctx.send_viewport_cmd(ViewportCommand::Screenshot(UserData::new(Tag)));
        ctx.request_repaint();
    }
    if held.asked && held.texture.is_none() {
        let shot = ctx.input(|i| {
            i.events.iter().find_map(|event| match event {
                Event::Screenshot {
                    user_data, image, ..
                } if user_data.data.as_ref().is_some_and(|d| d.is::<Tag>()) => {
                    Some(image.clone())
                }
                _ => None,
            })
        });
        if let Some(image) = shot {
            held.texture = Some(ctx.load_texture(
                "page-blur",
                blurred(&image),
                TextureOptions::LINEAR,
            ));
        }
    }
    ctx.data_mut(|d| d.insert_temp(key, held.clone()));
    let Some(texture) = &held.texture else {
        return;
    };
    let ready = ctx.animate_bool_with_time(key.with("ready"), true, READY_FADE);
    let alpha = (v * ready).clamp(0.0, 1.0);
    if alpha <= 0.0 {
        return;
    }
    let a = (alpha * 255.0).round();
    let k = (a * DIM).round() as u8;
    // Premultiplied: the image fades in and dims at once.
    let tint = Color32::from_rgba_premultiplied(k, k, k, a as u8);
    painter.image(
        texture.id(),
        rect,
        Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
        tint,
    );
}

/// Drops what the card kept (call once its exit is over).
pub fn release(ctx: &Context, id: Id) {
    let key = id.with("page-blur");
    if ctx.data(|d| d.get_temp::<Held>(key)).is_some() {
        ctx.data_mut(|d| d.remove::<Held>(key));
    }
}

/// The capture shrunk to an eighth and blurred: averages a few samples of each block, then two
/// separable box passes with clamped edges.
fn blurred(src: &ColorImage) -> ColorImage {
    let [w, h] = src.size;
    let sw = (w / SHRINK).max(1);
    let sh = (h / SHRINK).max(1);
    let mut px = vec![[0.0_f32; 3]; sw * sh];
    for y in 0..sh {
        let (y0, y1) = (y * h / sh, ((y + 1) * h / sh).max(y * h / sh + 1));
        for x in 0..sw {
            let (x0, x1) = (x * w / sw, ((x + 1) * w / sw).max(x * w / sw + 1));
            let mut sum = [0.0_f32; 3];
            let mut n = 0.0_f32;
            for sy in (y0..y1).step_by(2) {
                for sx in (x0..x1).step_by(2) {
                    let c = src.pixels[sy.min(h - 1) * w + sx.min(w - 1)].to_array();
                    sum[0] += f32::from(c[0]);
                    sum[1] += f32::from(c[1]);
                    sum[2] += f32::from(c[2]);
                    n += 1.0;
                }
            }
            px[y * sw + x] = [sum[0] / n, sum[1] / n, sum[2] / n];
        }
    }
    for _ in 0..PASSES {
        px = box_pass(&px, sw, sh, true);
        px = box_pass(&px, sw, sh, false);
    }
    let mut rgba = Vec::with_capacity(sw * sh * 4);
    for p in &px {
        rgba.push(p[0].round().clamp(0.0, 255.0) as u8);
        rgba.push(p[1].round().clamp(0.0, 255.0) as u8);
        rgba.push(p[2].round().clamp(0.0, 255.0) as u8);
        rgba.push(255);
    }
    ColorImage::from_rgba_unmultiplied([sw, sh], &rgba)
}

/// One box blur along a row (`horizontal`) or a column, edges clamped.
fn box_pass(src: &[[f32; 3]], w: usize, h: usize, horizontal: bool) -> Vec<[f32; 3]> {
    let mut out = vec![[0.0_f32; 3]; src.len()];
    let (lines, len) = if horizontal { (h, w) } else { (w, h) };
    let at = |line: usize, i: usize| {
        if horizontal {
            line * w + i
        } else {
            i * w + line
        }
    };
    let taps = (2 * RADIUS + 1) as f32;
    for line in 0..lines {
        for i in 0..len {
            let mut sum = [0.0_f32; 3];
            for d in 0..=2 * RADIUS {
                let j = (i + d).saturating_sub(RADIUS).min(len - 1);
                let p = src[at(line, j)];
                sum[0] += p[0];
                sum[1] += p[1];
                sum[2] += p[2];
            }
            out[at(line, i)] = [sum[0] / taps, sum[1] / taps, sum[2] / taps];
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_flat_page_stays_flat() {
        let src = ColorImage::from_rgba_unmultiplied([64, 32], &[40, 80, 120, 255].repeat(64 * 32));
        let out = blurred(&src);
        assert_eq!(out.size, [8, 4]);
        assert!(out.pixels.iter().all(|c| c.to_array() == [40, 80, 120, 255]));
    }

    #[test]
    fn a_tiny_window_still_gives_one_pixel() {
        let src = ColorImage::from_rgba_unmultiplied([3, 2], &[10, 10, 10, 255].repeat(6));
        assert_eq!(blurred(&src).size, [1, 1]);
    }
}
