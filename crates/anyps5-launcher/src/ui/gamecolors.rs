//! Game Colors (docs/DESIGN.md 3.13): a theme whose three tones and accent come from the art of the
//! selected game.
//!
//! [`dominant`] finds up to three colours in a small copy of the art, [`look_from`] turns them into
//! a palette for the backdrop shader and an accent, and clamps both so the text that sits on them
//! keeps the ratios of `ui::contrast`: the lit tone is never brighter than the lit tone of Aurora,
//! the brightest built-in theme the table measures, and the accent is never darker than Blue,
//! the darkest built-in accent. Pure arithmetic: the decoding and the thread are in
//! `app::game_colors`.

use egui::Color32;

use crate::ui::theme::{Palette, rgb};

/// The most light the lit tone (the middle tone lifted a third of the way to the bright one) may have, as WCAG
/// relative luminance: what Aurora's lit tone has, which the contrast table passes.
pub const LIT_MAX: f64 = 0.21;
/// The least light an accent may have: Blue's, the darkest built-in accent. A lighter accent only
/// gains on every backdrop; its ink is chosen by `readable_ink`.
pub const ACCENT_MIN: f64 = 0.33;
/// How far toward the bright tone the shader lifts the middle one behind text in the Cosmic and
/// Aurora patterns (the 0.35 of `ui::contrast`).
const LIT_MIX: f32 = 0.35;
/// The brightest channel of the deep tone.
const DEEP_MAX: f32 = 0.10;
/// Hue buckets of the histogram.
const BUCKETS: usize = 12;
/// A pixel is colourful when it is at least this saturated and this bright.
const MIN_SATURATION: f32 = 0.22;
const MIN_VALUE: f32 = 0.18;

/// What a game's art gives the theme.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Look {
    pub palette: Palette,
    pub accent: Color32,
}

impl Look {
    /// The look of a game without art, or whose art has no colour to take: Cosmic's.
    pub const fn fallback() -> Look {
        Look {
            palette: crate::ui::theme::Theme::Cosmic.palette(),
            accent: Color32::from_rgb(102, 163, 255),
        }
    }
}

fn linear(v: f32) -> f64 {
    let v = f64::from(v.clamp(0.0, 1.0));
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

/// WCAG relative luminance of a display-referred colour.
pub fn luminance(c: [f32; 3]) -> f64 {
    0.2126 * linear(c[0]) + 0.7152 * linear(c[1]) + 0.0722 * linear(c[2])
}

fn mixed(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [0, 1, 2].map(|i| a[i] + (b[i] - a[i]) * t)
}

fn scaled(c: [f32; 3], k: f32) -> [f32; 3] {
    c.map(|v| (v * k).clamp(0.0, 1.0))
}

/// `c` scaled so that its brightest channel is `value`.
fn with_value(c: [f32; 3], value: f32) -> [f32; 3] {
    let top = c[0].max(c[1]).max(c[2]);
    if top <= 0.001 {
        [value; 3]
    } else {
        scaled(c, value / top)
    }
}

fn hsv(c: [f32; 3]) -> (f32, f32, f32) {
    let (max, min) = (c[0].max(c[1]).max(c[2]), c[0].min(c[1]).min(c[2]));
    let delta = max - min;
    let hue = if delta <= 0.0 {
        0.0
    } else if max == c[0] {
        ((c[1] - c[2]) / delta).rem_euclid(6.0)
    } else if max == c[1] {
        (c[2] - c[0]) / delta + 2.0
    } else {
        (c[0] - c[1]) / delta + 4.0
    };
    let saturation = if max <= 0.0 { 0.0 } else { delta / max };
    (hue * 60.0, saturation, max)
}

/// Up to three colours that carry the picture, most prominent first: the hue buckets of its
/// colourful pixels, weighted by saturation and brightness, at least two buckets (60 degrees)
/// apart. A picture without colour (greys, black) gives its average, or nothing when it is empty.
pub fn dominant(image: &image::RgbaImage) -> Vec<[f32; 3]> {
    let mut sum = [[0.0f32; 3]; BUCKETS];
    let mut weight = [0.0f32; BUCKETS];
    let mut all = [0.0f32; 3];
    let mut count = 0.0f32;
    for pixel in image.pixels() {
        if pixel[3] < 128 {
            continue;
        }
        let c = [
            f32::from(pixel[0]) / 255.0,
            f32::from(pixel[1]) / 255.0,
            f32::from(pixel[2]) / 255.0,
        ];
        for (a, v) in all.iter_mut().zip(c) {
            *a += v;
        }
        count += 1.0;
        let (hue, saturation, value) = hsv(c);
        if saturation < MIN_SATURATION || value < MIN_VALUE {
            continue;
        }
        let bucket = ((hue / 360.0 * BUCKETS as f32) as usize).min(BUCKETS - 1);
        let w = saturation * value;
        for (s, v) in sum[bucket].iter_mut().zip(c) {
            *s += v * w;
        }
        weight[bucket] += w;
    }
    if count == 0.0 {
        return Vec::new();
    }
    let mut order: Vec<usize> = (0..BUCKETS).filter(|&b| weight[b] > 0.0).collect();
    order.sort_by(|&a, &b| weight[b].total_cmp(&weight[a]));
    let mut picked: Vec<usize> = Vec::new();
    for bucket in order {
        let apart = picked.iter().all(|&p| {
            let gap = bucket.abs_diff(p);
            gap.min(BUCKETS - gap) >= 2
        });
        if apart {
            picked.push(bucket);
        }
        if picked.len() == 3 {
            break;
        }
    }
    if picked.is_empty() {
        return vec![all.map(|v| v / count)];
    }
    picked
        .into_iter()
        .map(|b| sum[b].map(|v| v / weight[b]))
        .collect()
}

/// Brings a palette inside the limits of the contrast table: the deep tone stays dark and the lit
/// tone stays at or below [`LIT_MAX`].
pub fn clamp_palette(mut palette: Palette) -> Palette {
    let top = palette.deep.iter().copied().fold(0.0f32, f32::max);
    if top > DEEP_MAX {
        palette.deep = scaled(palette.deep, DEEP_MAX / top);
    }
    let lit = |k: f32| {
        luminance(mixed(
            scaled(palette.middle, k),
            scaled(palette.bright, k),
            LIT_MIX,
        ))
    };
    if lit(1.0) > LIT_MAX {
        let (mut low, mut high) = (0.0f32, 1.0f32);
        for _ in 0..20 {
            let k = (low + high) * 0.5;
            if lit(k) > LIT_MAX {
                high = k;
            } else {
                low = k;
            }
        }
        palette.middle = scaled(palette.middle, low);
        palette.bright = scaled(palette.bright, low);
    }
    palette
}

/// An accent from a colour: full brightness, and lifted toward white until it has [`ACCENT_MIN`].
pub fn accent_from(colour: [f32; 3]) -> Color32 {
    let mut c = with_value(colour, 1.0);
    let mut steps = 0;
    while luminance(c) < ACCENT_MIN && steps < 24 {
        c = mixed(c, [1.0; 3], 0.08);
        steps += 1;
    }
    rgb(c[0], c[1], c[2])
}

/// The look of a picture's colours (the result of [`dominant`]): deep from the first colour, the
/// middle tone between the first and the second, the bright tone from the second and the third;
/// with fewer colours the ones there are stand in. `None` for no colour at all.
pub fn look_from(colours: &[[f32; 3]]) -> Option<Look> {
    let first = *colours.first()?;
    let second = colours.get(1).copied().unwrap_or(first);
    let third = colours.get(2).copied().unwrap_or(second);
    let palette = clamp_palette(Palette {
        deep: with_value(first, DEEP_MAX * 0.8),
        middle: with_value(mixed(first, second, 0.5), 0.46),
        bright: with_value(mixed(second, third, 0.5), 1.0),
    });
    Some(Look {
        palette,
        accent: accent_from(first),
    })
}

/// The look of a picture: [`dominant`] then [`look_from`].
pub fn look_of(image: &image::RgbaImage) -> Option<Look> {
    look_from(&dominant(image))
}

/// Moves a palette toward another by `k` (0 stays, 1 arrives), tone by tone.
pub fn approach(from: Palette, to: Palette, k: f32) -> Palette {
    Palette {
        deep: mixed(from.deep, to.deep, k),
        middle: mixed(from.middle, to.middle, k),
        bright: mixed(from.bright, to.bright, k),
    }
}

/// [`approach`] for the accent, in sRGB.
pub fn approach_colour(from: Color32, to: Color32, k: f32) -> Color32 {
    crate::ui::theme::mix(from, to, k)
}

/// How far two palettes are apart: the largest channel difference.
pub fn distance(a: &Palette, b: &Palette) -> f32 {
    let pairs = [(a.deep, b.deep), (a.middle, b.middle), (a.bright, b.bright)];
    pairs
        .iter()
        .flat_map(|(x, y)| (0..3).map(move |i| (x[i] - y[i]).abs()))
        .fold(0.0, f32::max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::contrast;
    use crate::ui::theme::{Theme, readable_ink};

    fn flat(r: u8, g: u8, b: u8) -> image::RgbaImage {
        image::RgbaImage::from_pixel(16, 16, image::Rgba([r, g, b, 255]))
    }

    fn lit(p: &Palette) -> f64 {
        luminance(mixed(p.middle, p.bright, LIT_MIX))
    }

    #[test]
    fn the_limit_is_what_aurora_has() {
        let aurora = Theme::Aurora.palette();
        assert!(lit(&aurora) <= LIT_MAX + 0.005, "{}", lit(&aurora));
    }

    #[test]
    fn a_red_picture_gives_a_red_look() {
        let look = look_of(&flat(220, 30, 30)).unwrap();
        assert!(look.palette.bright[0] > look.palette.bright[2]);
        assert!(look.accent.r() > look.accent.b());
    }

    #[test]
    fn two_halves_give_two_colours() {
        let mut image = flat(200, 30, 30);
        for y in 0..16 {
            for x in 8..16 {
                image.put_pixel(x, y, image::Rgba([30, 60, 220, 255]));
            }
        }
        let colours = dominant(&image);
        assert_eq!(colours.len(), 2);
    }

    #[test]
    fn grey_art_still_gives_something_and_nothing_gives_none() {
        assert_eq!(dominant(&flat(120, 120, 120)).len(), 1);
        assert!(look_of(&flat(120, 120, 120)).is_some());
        assert!(dominant(&image::RgbaImage::new(0, 0)).is_empty());
        assert!(look_from(&[]).is_none());
    }

    #[test]
    fn every_look_keeps_the_limits_of_the_contrast_table() {
        // The brightest, the most saturated and the dullest art a game can bring.
        let arts = [
            (255, 255, 255),
            (255, 255, 0),
            (0, 255, 0),
            (0, 255, 255),
            (255, 0, 255),
            (255, 128, 0),
            (255, 0, 0),
            (0, 0, 255),
            (10, 10, 10),
            (128, 128, 128),
            (200, 190, 40),
        ];
        for (r, g, b) in arts {
            let Some(look) = look_of(&flat(r, g, b)) else {
                continue;
            };
            assert!(lit(&look.palette) <= LIT_MAX + 0.002, "{r},{g},{b}");
            let deep = look.palette.deep.iter().copied().fold(0.0, f32::max);
            assert!(deep <= DEEP_MAX + 0.001, "{r},{g},{b}");
            let a =
                [look.accent.r(), look.accent.g(), look.accent.b()].map(|v| f32::from(v) / 255.0);
            assert!(luminance(a) >= ACCENT_MIN - 0.01, "{r},{g},{b}: {a:?}");
            // And the ink on the accent reads.
            let ink = readable_ink(look.accent);
            assert!(contrast::ratio(ink, look.accent) >= 4.49, "{r},{g},{b}");
        }
    }

    #[test]
    fn the_accent_is_never_darker_than_the_darkest_built_in() {
        let blue = crate::ui::theme::Accent::Blue.color();
        let c = [blue.r(), blue.g(), blue.b()].map(|v| f32::from(v) / 255.0);
        assert!(luminance(c) >= ACCENT_MIN - 0.01, "{}", luminance(c));
    }

    #[test]
    fn a_transition_arrives() {
        let a = Theme::Cosmic.palette();
        let b = Theme::Sunset.palette();
        assert!(distance(&a, &b) > 0.3);
        let mut now = a;
        for _ in 0..80 {
            now = approach(now, b, 0.15);
        }
        assert!(distance(&now, &b) < 0.002);
    }
}
