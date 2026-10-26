//! Game Colors at run time (docs/DESIGN.md 3.13): the colours of the selected game's art, found on
//! a worker thread, remembered per game, and eased from one game to the next.
//!
//! The picture is read and reduced to a thumbnail off the UI thread; `ui::gamecolors` turns it
//! into a clamped palette and accent. The UI thread only moves the current colours toward the
//! wanted ones each frame (at once with reduced motion).

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::UNIX_EPOCH;

use egui::{Color32, Context};

use crate::ui::gamecolors::{self, Look};
use crate::ui::theme::Palette;

/// How long the colours take to cover most of the way to the next game's, in seconds.
const SETTLE: f32 = 0.30;
/// Pictures read at the same time.
const MAX_IN_FLIGHT: usize = 2;
/// The thumbnail the colours are taken from (longer side).
const THUMBNAIL: u32 = 64;

/// The version of the way colours are taken from a picture; a file written by another one is not
/// trusted (change it with `ui::gamecolors`, and every game is read once more).
const ALGORITHM: u64 = 1;
/// The most games the file remembers.
const MAX_REMEMBERED: usize = 400;

pub struct GameColors {
    /// The look of every game seen so far, by game id; the fallback for art without colour.
    cache: HashMap<String, Look>,
    /// What the art of each game was when its look was found or loaded (see [`stamp_of`]); the
    /// look is only valid for that picture.
    stamps: HashMap<String, u64>,
    /// The file the looks are kept in between runs; `None` for previews and tests.
    file: Option<PathBuf>,
    /// Looks found since the file was last written.
    dirty: bool,
    /// What the file holds, by game id: `(stamp, look)`.
    stored: HashMap<String, (u64, Look)>,
    /// Games seen in this run, so a full file drops the ones nobody opens first.
    used: HashSet<String>,
    pending: HashSet<String>,
    tx: Sender<(String, Option<Look>)>,
    rx: Receiver<(String, Option<Look>)>,
    palette: Palette,
    accent: Color32,
}

impl GameColors {
    pub fn new(file: Option<PathBuf>) -> Self {
        let (tx, rx) = channel();
        let start = Look::fallback();
        let stored = file.as_deref().map(load).unwrap_or_default();
        Self {
            cache: HashMap::new(),
            stamps: HashMap::new(),
            file,
            dirty: false,
            stored,
            used: HashSet::new(),
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
            self.dirty = true;
        }
        if self.dirty {
            self.save();
        }
        let wanted = match game {
            None => Some(Look::fallback()),
            Some((id, art)) => {
                self.recall(id, art);
                match self.cache.get(id) {
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
                }
            }
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

    /// The first time a game is asked for in this run: takes its look from the file when the art
    /// is still the picture the look was found in. Costs one `stat`, once per game.
    fn recall(&mut self, id: &str, art: Option<&Path>) {
        if self.stamps.contains_key(id) {
            return;
        }
        let stamp = art.map_or(0, stamp_of);
        self.stamps.insert(id.to_owned(), stamp);
        self.used.insert(id.to_owned());
        if let Some((kept, look)) = self.stored.get(id)
            && *kept == stamp
            && stamp != 0
        {
            self.cache.insert(id.to_owned(), *look);
        }
    }

    /// Writes the looks found so far, next to the ones the file already held. A failed write is
    /// not worth a message: the colours are found again next time.
    fn save(&mut self) {
        self.dirty = false;
        let Some(file) = self.file.clone() else {
            return;
        };
        for (id, look) in &self.cache {
            if let Some(&stamp) = self.stamps.get(id)
                && stamp != 0
            {
                self.stored.insert(id.clone(), (stamp, *look));
            }
        }
        if self.stored.len() > MAX_REMEMBERED {
            let surplus = self.stored.len() - MAX_REMEMBERED;
            let drop: Vec<String> = self
                .stored
                .keys()
                .filter(|id| !self.used.contains(*id))
                .take(surplus)
                .cloned()
                .collect();
            for id in drop {
                self.stored.remove(&id);
            }
        }
        let _ = write(&file, &self.stored);
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
        Self::new(None)
    }
}

/// A short fingerprint of the picture at `path`: its path, size and time of last change, so a
/// replaced picture is read again without reading the old one to find out. 0 when it cannot be
/// looked at (nothing is remembered for it).
fn stamp_of(path: &Path) -> u64 {
    let Ok(meta) = std::fs::metadata(path) else {
        return 0;
    };
    let changed = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map_or(0, |d| d.as_nanos());
    // FNV-1a over the three parts.
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    let mut feed = |bytes: &[u8]| {
        for b in bytes {
            hash ^= u64::from(*b);
            hash = hash.wrapping_mul(0x0100_0000_01b3);
        }
    };
    feed(path.to_string_lossy().as_bytes());
    feed(&meta.len().to_le_bytes());
    feed(&changed.to_le_bytes());
    hash.max(1)
}

fn rgb_of(v: &serde_json::Value) -> Option<[f32; 3]> {
    let a = v.as_array()?;
    if a.len() != 3 {
        return None;
    }
    let mut out = [0.0f32; 3];
    for (o, x) in out.iter_mut().zip(a) {
        let x = x.as_f64()? as f32;
        if !(0.0..=1.0).contains(&x) {
            return None;
        }
        *o = x;
    }
    Some(out)
}

/// The looks in the file; nothing when it is missing, damaged or from another algorithm.
fn load(file: &Path) -> HashMap<String, (u64, Look)> {
    let parse = || -> Option<HashMap<String, (u64, Look)>> {
        let root: serde_json::Value = serde_json::from_slice(&std::fs::read(file).ok()?).ok()?;
        if root.get("algorithm")?.as_u64()? != ALGORITHM {
            return None;
        }
        let mut out = HashMap::new();
        for (id, entry) in root.get("games")?.as_object()? {
            let one = || -> Option<(u64, Look)> {
                let stamp = u64::from_str_radix(entry.get("art")?.as_str()?, 16).ok()?;
                let accent = rgb_of(entry.get("accent")?)?.map(|v| (v * 255.0).round() as u8);
                Some((
                    stamp,
                    Look {
                        palette: crate::ui::theme::Palette {
                            deep: rgb_of(entry.get("deep")?)?,
                            middle: rgb_of(entry.get("middle")?)?,
                            bright: rgb_of(entry.get("bright")?)?,
                        },
                        accent: Color32::from_rgb(accent[0], accent[1], accent[2]),
                    },
                ))
            };
            if let Some(look) = one() {
                out.insert(id.clone(), look);
            }
        }
        Some(out)
    };
    parse().unwrap_or_default()
}

fn write(file: &Path, looks: &HashMap<String, (u64, Look)>) -> std::io::Result<()> {
    let mut games = serde_json::Map::new();
    for (id, (stamp, look)) in looks {
        let accent =
            [look.accent.r(), look.accent.g(), look.accent.b()].map(|v| f32::from(v) / 255.0);
        games.insert(
            id.clone(),
            serde_json::json!({
                "art": format!("{stamp:x}"),
                "deep": look.palette.deep,
                "middle": look.palette.middle,
                "bright": look.palette.bright,
                "accent": accent,
            }),
        );
    }
    let body = serde_json::json!({ "algorithm": ALGORITHM, "games": games });
    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut temp = file.as_os_str().to_owned();
    temp.push(".tmp");
    let temp = PathBuf::from(temp);
    std::fs::write(&temp, serde_json::to_vec(&body)?)?;
    std::fs::rename(&temp, file).inspect_err(|_| {
        let _ = std::fs::remove_file(&temp);
    })
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
    fn a_look_survives_a_restart_until_the_art_changes() {
        let dir = std::env::temp_dir().join(format!("anyps5-gc-keep-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let art = dir.join("art.png");
        image::RgbaImage::from_pixel(8, 8, image::Rgba([30, 200, 60, 255]))
            .save(&art)
            .unwrap();
        let file = dir.join("game-colors.json");
        let green = Look {
            palette: crate::ui::theme::Theme::Aurora.palette(),
            accent: Color32::from_rgb(90, 235, 190),
        };
        let mut first = GameColors::new(Some(file.clone()));
        first.recall("g", Some(&art));
        first.cache.insert("g".into(), green);
        first.dirty = true;
        first.save();
        // A new run finds it without reading the picture.
        let mut second = GameColors::new(Some(file.clone()));
        second.recall("g", Some(&art));
        let back = second.cache.get("g").copied().unwrap();
        assert!(gamecolors::distance(&back.palette, &green.palette) < 0.001);
        assert_eq!(back.accent, green.accent);
        // Another picture under the same id is read again.
        image::RgbaImage::from_pixel(9, 9, image::Rgba([200, 30, 60, 255]))
            .save(&art)
            .unwrap();
        let mut third = GameColors::new(Some(file.clone()));
        third.recall("g", Some(&art));
        assert!(!third.cache.contains_key("g"));
        // A damaged file is the same as none.
        std::fs::write(&file, b"{ not json").unwrap();
        assert!(GameColors::new(Some(file)).stored.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn colours_ease_to_the_look_and_reduced_motion_jumps() {
        let ctx = Context::default();
        let mut colors = GameColors::new(None);
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
