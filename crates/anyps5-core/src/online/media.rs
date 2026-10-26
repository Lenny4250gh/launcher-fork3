//! Screenshots and clips sent to a friend: the limits the cloud holds, the names of the files the
//! launcher keeps of what it received, and how a size or a length reads.
//!
//! The cloud checks everything again; these rules only spare the player an upload that would be
//! turned away, and name the reason in words.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use super::api::{self, MediaKind};

/// What the launcher knows of the files of received media, by the media's ID.
#[derive(Debug, Default)]
pub struct Cache {
    /// Downloads on their way or done: asked for once per run.
    pub asked: BTreeSet<String>,
    /// The file is in the cache folder.
    pub ready: BTreeSet<String>,
    /// The download failed (the cloud no longer has the file); not asked again until the player
    /// opens it.
    pub failed: BTreeSet<String>,
    /// Files the player wants opened once they are here, and where they will be.
    pub open: BTreeMap<String, PathBuf>,
    /// The cache folder was swept of stale files this run.
    pub swept: bool,
}

/// Why a file cannot be sent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// A kind of file the cloud does not take.
    Unsupported,
    /// An empty file.
    Empty,
    /// A screenshot above 10 MB.
    ImageTooLarge,
    /// A clip above 100 MB.
    ClipTooLarge,
    /// A clip above 60 seconds.
    ClipTooLong,
}

/// The kind of a file by its extension: PNG and JPEG are screenshots, MP4 is a clip.
pub fn kind_of(path: &Path) -> Option<MediaKind> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    match ext.as_str() {
        "png" | "jpg" | "jpeg" => Some(MediaKind::Image),
        "mp4" => Some(MediaKind::Clip),
        _ => None,
    }
}

/// Checks a file against the limits. `duration_ms` is a clip's length when the launcher knows it.
pub fn check(kind: MediaKind, bytes: u64, duration_ms: Option<u64>) -> Result<(), Refusal> {
    if bytes == 0 {
        return Err(Refusal::Empty);
    }
    match kind {
        MediaKind::Image if bytes > api::MAX_IMAGE_BYTES => Err(Refusal::ImageTooLarge),
        MediaKind::Clip if bytes > api::MAX_CLIP_BYTES => Err(Refusal::ClipTooLarge),
        MediaKind::Clip
            if duration_ms.is_some_and(|d| d > u64::from(api::MAX_CLIP_SECS) * 1000) =>
        {
            Err(Refusal::ClipTooLong)
        }
        MediaKind::Image | MediaKind::Clip => Ok(()),
        MediaKind::Unknown => Err(Refusal::Unsupported),
    }
}

/// The extension a received file is kept under.
pub fn extension(content_type: &str) -> &'static str {
    match content_type.split(';').next().unwrap_or("").trim() {
        "image/png" => "png",
        "image/jpeg" => "jpg",
        _ => "mp4",
    }
}

/// True for an ID that is safe to use as a file name: what the cloud makes, 1 to 64 letters,
/// digits, hyphens and underscores.
pub fn safe_id(id: &str) -> bool {
    api::is_title_id(id)
}

/// The file a received media is kept in, inside the cache folder; `None` for an ID that is no file
/// name.
pub fn cache_file(dir: &Path, id: &str, content_type: &str) -> Option<PathBuf> {
    safe_id(id).then(|| dir.join(format!("{id}.{}", extension(content_type))))
}

/// A cached file older than the cloud keeps the original (30 days) is of no use.
pub fn stale(modified: u64, now: u64) -> bool {
    now.saturating_sub(modified) > api::MEDIA_KEEP_SECS
}

/// Deletes the files of the cache folder that are stale; returns how many went. Files it cannot
/// read are left.
pub fn sweep(dir: &Path, now: u64) -> usize {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    let mut removed = 0;
    for entry in entries.flatten() {
        let modified = entry
            .metadata()
            .ok()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs());
        if modified.is_some_and(|m| stale(m, now)) && std::fs::remove_file(entry.path()).is_ok() {
            removed += 1;
        }
    }
    removed
}

/// A size as `3.4 MB`, `820 KB`.
pub fn size_text(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    let b = bytes as f64;
    if b >= KB * KB {
        format!("{:.1} MB", b / (KB * KB))
    } else {
        format!("{} KB", (b / KB).ceil().max(1.0) as u64)
    }
}

/// A length as `0:23`, `1:05`.
pub fn duration_text(ms: u64) -> String {
    let secs = ms.div_ceil(1000);
    format!("{}:{:02}", secs / 60, secs % 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_limits_name_the_reason() {
        assert_eq!(check(MediaKind::Image, 0, None), Err(Refusal::Empty));
        assert_eq!(check(MediaKind::Image, 10 * 1024 * 1024, None), Ok(()));
        assert_eq!(
            check(MediaKind::Image, 10 * 1024 * 1024 + 1, None),
            Err(Refusal::ImageTooLarge)
        );
        assert_eq!(
            check(MediaKind::Clip, 50 * 1024 * 1024, Some(60_000)),
            Ok(())
        );
        assert_eq!(
            check(MediaKind::Clip, 101 * 1024 * 1024, None),
            Err(Refusal::ClipTooLarge)
        );
        assert_eq!(
            check(MediaKind::Clip, 1024, Some(60_001)),
            Err(Refusal::ClipTooLong)
        );
        // Without a known length the cloud has the last word.
        assert_eq!(check(MediaKind::Clip, 1024, None), Ok(()));
    }

    #[test]
    fn files_are_told_apart_by_their_extension() {
        assert_eq!(kind_of(Path::new("a.PNG")), Some(MediaKind::Image));
        assert_eq!(kind_of(Path::new("a.jpeg")), Some(MediaKind::Image));
        assert_eq!(kind_of(Path::new("a.mp4")), Some(MediaKind::Clip));
        assert_eq!(kind_of(Path::new("a.exe")), None);
        assert_eq!(kind_of(Path::new("a")), None);
    }

    #[test]
    fn a_cache_file_is_named_by_its_id_and_never_escapes_the_folder() {
        let dir = Path::new("cache");
        assert_eq!(
            cache_file(dir, "ab-12_x", "image/jpeg"),
            Some(dir.join("ab-12_x.jpg"))
        );
        assert_eq!(cache_file(dir, "ab", "video/mp4"), Some(dir.join("ab.mp4")));
        assert_eq!(cache_file(dir, "../evil", "image/png"), None);
        assert_eq!(cache_file(dir, "a/b", "image/png"), None);
        assert_eq!(cache_file(dir, "", "image/png"), None);
    }

    #[test]
    fn a_cached_file_goes_stale_after_thirty_days() {
        let day = 24 * 60 * 60;
        assert!(!stale(1_000_000, 1_000_000 + 30 * day));
        assert!(stale(1_000_000, 1_000_000 + 30 * day + 1));
        assert!(!stale(2_000_000, 1_000_000));
    }

    #[test]
    fn sizes_and_lengths_read_short() {
        assert_eq!(size_text(1), "1 KB");
        assert_eq!(size_text(820 * 1024), "820 KB");
        assert_eq!(size_text(3 * 1024 * 1024 + 400 * 1024), "3.4 MB");
        assert_eq!(duration_text(23_000), "0:23");
        assert_eq!(duration_text(65_000), "1:05");
        assert_eq!(duration_text(22_100), "0:23");
    }
}
