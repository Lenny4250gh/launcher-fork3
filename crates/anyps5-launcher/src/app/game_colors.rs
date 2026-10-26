//! Game Colors at run time (docs/DESIGN.md 3.13): the colours of the selected game's art, found on
//! a worker thread, remembered per game, and eased from one game to the next.
//!
//! The picture is read and reduced to a thumbnail off the UI thread; `ui::gamecolors` turns it
//! into a clamped palette and accent. The UI thread only moves the current colours toward the
//! wanted ones each frame (at once with reduced motion).

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender, channel};

use egui::{Color32, Context};

use crate::ui::gamecolors::{self, Look};
use crate::ui::theme::Palette;

/// How long the colours take to cover most of the way to the next game's, in seconds.
const SETTLE: f32 = 0.30;
/// Pictures read at the same time.
const MAX_IN_FLIGHT: usize = 2;
/// The thumbnail the colours are taken from (longer side).
const THUMBNAIL: u32 = 64;

pub struct GameColors {
    /// The look of every game seen so far, by game id; the fallback for art without colour.
    cache: HashMap<String, Look>,
    pending: HashSet<String>,
    tx: Sender<(String, Option<Look>)>,
    rx: Receiver<(String, Option<Look>)>,
    palette: Palette,
    accent: Color32,
}

impl GameColors {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let start = Look::fallback();
        Self {
            cache: HashMap::new(),
            pending: HashSet::new(),
            tx,
            rx,
            palette: start.palette,
            accent: start.accent,
        }
    }

    /// One frame: takes what the workers found, starts the picture of `game` (its id and its art)
    /// when it is new, and moves the colours a step toward its look. Returns the palette and the
    /// accent to paint with now. No game, or art without colour, eases to the fallback.
    pub fn update(
        &mut self,
        ctx: &Context,
        game: Option<(&str, Option<&Path>)>,
        reduced: bool,
    ) -> (Palette, Color32) {
        while let Ok((id, look)) = self.rx.try_recv() {
            self.pending.remove(&id);
            self.cache.insert(id, look.unwrap_or_else(Look::fallback));
        }
        let wanted = match game {
            None => Some(Look::fallback()),
            Some((id, art)) => match self.cache.get(id) {
                Some(look) => Some(*look),
                None => {
                    match art {
                        Some(path) => self.start(ctx, id, path),
                        None => {
                            self.cache.insert(id.to_owned(), Look::fallback());
                        }
                    }
                    // Until the picture is read the colours stay where they are.
                    None
                }
            },
        };
        let Some(wanted) = wanted else {
            return (self.palette, self.accent);
        };
        let dt = ctx.input(|i| i.stable_dt).clamp(0.0, 0.1);
        let k = if reduced {
            1.0
        } else {
            1.0 - (-dt / SETTLE).exp()
        };
        self.palette = gamecolors::approach(self.palette, wanted.palette, k);
        self.accent = gamecolors::approach_colour(self.accent, wanted.accent, k);
        if gamecolors::distance(&self.palette, &wanted.palette) < 0.002
            && self.accent == wanted.accent
        {
            self.palette = wanted.palette;
        } else if gamecolors::distance(&self.palette, &wanted.palette) < 0.002 {
            self.accent = wanted.accent;
        } else {
            ctx.request_repaint();
        }
        (self.palette, self.accent)
    }

    fn start(&mut self, ctx: &Context, id: &str, path: &Path) {
        if self.pending.contains(id) || self.pending.len() >= MAX_IN_FLIGHT {
            return;
        }
        self.pending.insert(id.to_owned());
        let (tx, ctx, id, path): (_, _, String, PathBuf) = (
            self.tx.clone(),
            ctx.clone(),
            id.to_owned(),
            path.to_path_buf(),
        );
        let spawned = std::thread::Builder::new()
            .name("game-colors".into())
            .spawn(move || {
                let look = look_of_file(&path);
                let _ = tx.send((id, look));
                ctx.request_repaint();
            });
        if spawned.is_err() {
            self.pending.clear();
        }
    }
}

impl Default for GameColors {
    fn default() -> Self {
        Self::new()
    }
}

/// The look of the picture at `path`, `None` when it cannot be read or has no colour.
fn look_of_file(path: &Path) -> Option<Look> {
    let image = image::open(path).ok()?;
    gamecolors::look_of(&image.thumbnail(THUMBNAIL, THUMBNAIL).to_rgba8())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_picture_on_disk_gives_its_colours_and_a_missing_one_gives_none() {
        let dir = std::env::temp_dir().join(format!("anyps5-gc-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("art.png");
        image::RgbaImage::from_pixel(200, 100, image::Rgba([30, 200, 60, 255]))
            .save(&file)
            .unwrap();
        let look = look_of_file(&file).unwrap();
        assert!(look.palette.bright[1] > look.palette.bright[0]);
        assert!(look_of_file(&dir.join("missing.png")).is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn colours_ease_to_the_look_and_reduced_motion_jumps() {
        let ctx = Context::default();
        let mut colors = GameColors::new();
        let green = Look {
            palette: crate::ui::theme::Theme::Aurora.palette(),
            accent: Color32::from_rgb(90, 235, 190),
        };
        colors.cache.insert("g".into(), green);
        let (palette, _) = colors.update(&ctx, Some(("g", None)), true);
        assert_eq!(palette, green.palette);
        let (palette, accent) = colors.update(&ctx, None, true);
        assert_eq!(palette, Look::fallback().palette);
        assert_eq!(accent, Look::fallback().accent);
    }
}
