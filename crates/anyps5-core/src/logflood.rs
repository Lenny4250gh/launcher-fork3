//! Keeps a game that repeats one line over and over from filling its log (and the launcher's
//! memory of it). A line that occurs more than [`LIMIT`] times within [`WINDOW`] is written
//! [`LIMIT`] times; the copies after that are dropped, and one notice says how many were dropped
//! once the window has passed or the game has ended.
//!
//! The filter sees the pipe's raw bytes in arbitrary chunks and hands back what to write. Lines
//! are compared as bytes without their line end; a line that never ends (more than
//! [`MAX_PENDING`] bytes) is passed on unfiltered.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Copies of one line written per window; the next ones are dropped.
pub const LIMIT: u32 = 50;
/// The window the copies are counted in.
pub const WINDOW: Duration = Duration::from_secs(10);
/// Longest unfinished line held back before it is passed on as it is.
const MAX_PENDING: usize = 64 * 1024;
/// Distinct lines remembered; older windows are closed (and their notices written) beyond this.
const MAX_TRACKED: usize = 2048;
/// A line longer than this is shortened in the notice.
const NOTICE_WIDTH: usize = 160;

struct Seen {
    window_start: Instant,
    written: u32,
    dropped: u32,
}

/// See the module text.
#[derive(Default)]
pub struct FloodFilter {
    pending: Vec<u8>,
    seen: HashMap<Vec<u8>, Seen>,
    last_sweep: Option<Instant>,
}

impl FloodFilter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Takes the next bytes of the pipe and returns what to write.
    pub fn push(&mut self, chunk: &[u8], now: Instant) -> Vec<u8> {
        let mut out = Vec::with_capacity(chunk.len());
        self.pending.extend_from_slice(chunk);
        let mut start = 0;
        while let Some(at) = self.pending[start..].iter().position(|&b| b == b'\n') {
            let end = start + at + 1;
            let line = self.pending[start..end].to_vec();
            self.line(&line, now, &mut out);
            start = end;
        }
        self.pending.drain(..start);
        if self.pending.len() > MAX_PENDING {
            out.append(&mut self.pending);
        }
        self.sweep(now, &mut out);
        out
    }

    /// The game has ended: the unfinished line and every open notice.
    pub fn finish(&mut self) -> Vec<u8> {
        let mut out = std::mem::take(&mut self.pending);
        let mut keys: Vec<Vec<u8>> = self.seen.keys().cloned().collect();
        keys.sort();
        for key in keys {
            if let Some(seen) = self.seen.remove(&key) {
                notice(&key, &seen, &mut out);
            }
        }
        out
    }

    fn line(&mut self, line: &[u8], now: Instant, out: &mut Vec<u8>) {
        let key = trim_end(line);
        // Blank lines and short ones carry no flood worth a notice.
        if key.len() < 4 {
            out.extend_from_slice(line);
            return;
        }
        let seen = self.seen.entry(key.to_vec()).or_insert(Seen {
            window_start: now,
            written: 0,
            dropped: 0,
        });
        if now.duration_since(seen.window_start) >= WINDOW {
            notice(key, seen, out);
            *seen = Seen {
                window_start: now,
                written: 0,
                dropped: 0,
            };
        }
        if seen.written < LIMIT {
            seen.written += 1;
            out.extend_from_slice(line);
        } else {
            seen.dropped += 1;
        }
    }

    /// Closes the windows that have passed (at most once a second) and keeps the table small.
    fn sweep(&mut self, now: Instant, out: &mut Vec<u8>) {
        let due = self
            .last_sweep
            .is_none_or(|t| now.duration_since(t) >= Duration::from_secs(1));
        if !due && self.seen.len() <= MAX_TRACKED {
            return;
        }
        self.last_sweep = Some(now);
        let force = self.seen.len() > MAX_TRACKED;
        let mut done: Vec<Vec<u8>> = self
            .seen
            .iter()
            .filter(|(_, s)| force || now.duration_since(s.window_start) >= WINDOW)
            .map(|(k, _)| k.clone())
            .collect();
        done.sort();
        for key in done {
            if let Some(seen) = self.seen.remove(&key) {
                notice(&key, &seen, out);
            }
        }
    }
}

fn trim_end(line: &[u8]) -> &[u8] {
    let mut end = line.len();
    while end > 0 && matches!(line[end - 1], b'\n' | b'\r') {
        end -= 1;
    }
    &line[..end]
}

/// The one line that says how many copies were dropped (nothing when none were).
fn notice(key: &[u8], seen: &Seen, out: &mut Vec<u8>) {
    if seen.dropped == 0 {
        return;
    }
    let text = String::from_utf8_lossy(key);
    let shown: String = text.chars().take(NOTICE_WIDTH).collect();
    let cut = if text.chars().count() > NOTICE_WIDTH {
        "..."
    } else {
        ""
    };
    out.extend_from_slice(
        format!(
            "[anyps5-launcher] a line repeated {} more times within {} s and the copies were dropped: {shown}{cut}\n",
            seen.dropped,
            WINDOW.as_secs()
        )
        .as_bytes(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(bytes: Vec<u8>) -> String {
        String::from_utf8(bytes).unwrap()
    }

    #[test]
    fn a_line_over_the_limit_is_cut_and_counted_once() {
        let mut filter = FloodFilter::new();
        let t0 = Instant::now();
        let mut written = String::new();
        for _ in 0..200 {
            written += &text(filter.push(b"spam spam\n", t0));
        }
        assert_eq!(written.matches("spam spam\n").count(), LIMIT as usize);
        // The window passes: one notice with the count, then the line is written again.
        let later = t0 + WINDOW + Duration::from_secs(1);
        let after = text(filter.push(b"other line\n", later));
        assert_eq!(after.matches("repeated 150 more times").count(), 1, "{after}");
        assert!(after.starts_with("other line
"));
    }

    #[test]
    fn the_end_of_the_game_writes_the_open_notice() {
        let mut filter = FloodFilter::new();
        let t0 = Instant::now();
        for _ in 0..60 {
            filter.push(b"same\r\n", t0);
        }
        let last = text(filter.finish());
        assert!(last.contains("repeated 10 more times"), "{last}");
        assert!(last.contains(": same\n"), "{last}");
    }

    #[test]
    fn varied_lines_and_split_chunks_pass_untouched() {
        let mut filter = FloodFilter::new();
        let t0 = Instant::now();
        let mut written = Vec::new();
        for i in 0..500 {
            let line = format!("frame {i} ok\n");
            let (a, b) = line.as_bytes().split_at(3);
            written.extend(filter.push(a, t0));
            written.extend(filter.push(b, t0));
        }
        written.extend(filter.finish());
        let text = text(written);
        assert_eq!(text.lines().count(), 500);
        assert!(!text.contains("dropped"));
    }

    #[test]
    fn an_endless_line_is_not_held_back() {
        let mut filter = FloodFilter::new();
        let out = filter.push(&vec![b'x'; MAX_PENDING + 1], Instant::now());
        assert_eq!(out.len(), MAX_PENDING + 1);
    }
}
