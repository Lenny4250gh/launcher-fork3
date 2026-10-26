//! Broadcast: the running game's window, live on Twitch, YouTube or any RTMP server
//! (docs/DESIGN.md 9.13).
//!
//! The share worker does the capturing, encoding and sending (network spec/broadcast.md); this
//! module is the launcher's side of it: the services and their addresses, the quality presets,
//! where the stream key is kept, and the worker as a child process with a command and an event
//! channel.
//!
//! The stream key is a secret. It lives in the system's credential store (Windows) or in a file
//! only its owner can read (elsewhere), goes to the worker on its standard input, and is in no
//! preference, no command line and no log.

use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver};

use serde::Deserialize;

use crate::paths::AppPaths;

// --- services and presets ---------------------------------------------------------------------

/// Where a broadcast goes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Service {
    Twitch,
    YouTube,
    /// Any RTMP server, at the address the player types.
    Custom,
}

impl Service {
    pub const ALL: [Service; 3] = [Service::Twitch, Service::YouTube, Service::Custom];

    /// The name stored in the preferences.
    pub fn id(self) -> &'static str {
        match self {
            Service::Twitch => "twitch",
            Service::YouTube => "youtube",
            Service::Custom => "custom",
        }
    }

    /// The service for a stored name; Twitch for anything else.
    pub fn from_id(id: &str) -> Service {
        Service::ALL
            .into_iter()
            .find(|s| s.id() == id)
            .unwrap_or(Service::Twitch)
    }

    /// The server a service takes streams on; `None` for a custom one.
    pub fn preset_url(self) -> Option<&'static str> {
        match self {
            Service::Twitch => Some("rtmp://live.twitch.tv/app"),
            Service::YouTube => Some("rtmp://a.rtmp.youtube.com/live2"),
            Service::Custom => None,
        }
    }
}

/// The address to publish to for the stored service and custom address; `None` while a custom
/// address is missing or is not an `rtmp://` one.
pub fn server_url(service: &str, custom: &str) -> Option<String> {
    let service = Service::from_id(service);
    if let Some(url) = service.preset_url() {
        return Some(url.to_owned());
    }
    let custom = custom.trim();
    (custom.starts_with("rtmp://") && custom.len() > "rtmp://".len()).then(|| custom.to_owned())
}

/// A picture size, rate and bit rate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Quality {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub kbps: u32,
}

/// The presets the player chooses from, lowest first.
pub const QUALITIES: [Quality; 3] = [
    Quality {
        width: 1280,
        height: 720,
        fps: 30,
        kbps: 4_500,
    },
    Quality {
        width: 1280,
        height: 720,
        fps: 60,
        kbps: 6_000,
    },
    Quality {
        width: 1920,
        height: 1080,
        fps: 60,
        kbps: 8_000,
    },
];

/// The preset for a stored index; the first one for an index that does not exist.
pub fn quality(index: u32) -> Quality {
    QUALITIES
        .get(index as usize)
        .copied()
        .unwrap_or(QUALITIES[0])
}

// --- the stream key ---------------------------------------------------------------------------

/// What the key field shows in place of the key.
pub const MASK: &str = "\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}\u{2022}";

/// The key a text field now holds, given that it showed [`MASK`] or nothing before: what is left
/// when the mask dots and spaces are taken out. `None` is an empty field, which removes the key.
pub fn key_from_field(value: &str) -> Option<String> {
    let key: String = value
        .chars()
        .filter(|c| *c != '\u{2022}' && !c.is_whitespace())
        .collect();
    (!key.is_empty()).then_some(key)
}

#[cfg(windows)]
const CREDENTIAL_NAME: &str = "AnyPlay Launcher broadcast key";

/// The kept stream key, if any.
pub fn load_key(paths: &AppPaths) -> Option<String> {
    #[cfg(windows)]
    {
        let _ = paths;
        credential::read(CREDENTIAL_NAME)
    }
    #[cfg(not(windows))]
    {
        let text = std::fs::read_to_string(key_file(paths)).ok()?;
        let text = text.trim();
        (!text.is_empty()).then(|| text.to_owned())
    }
}

/// Keeps `key`, or removes the kept one when it is `None`. An error is only a reason to tell
/// the player that the key was not kept.
pub fn save_key(paths: &AppPaths, key: Option<&str>) -> Result<(), String> {
    #[cfg(windows)]
    {
        let _ = paths;
        match key {
            Some(key) => credential::write(CREDENTIAL_NAME, key),
            None => credential::delete(CREDENTIAL_NAME),
        }
    }
    #[cfg(not(windows))]
    {
        let file = key_file(paths);
        let Some(key) = key else {
            return match std::fs::remove_file(&file) {
                Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e.to_string()),
                _ => Ok(()),
            };
        };
        write_private(&file, key).map_err(|e| e.to_string())
    }
}

#[cfg(not(windows))]
fn key_file(paths: &AppPaths) -> std::path::PathBuf {
    paths.data.join("broadcast.key")
}

/// Writes a file that only its owner may read.
#[cfg(not(windows))]
fn write_private(file: &Path, text: &str) -> std::io::Result<()> {
    if let Some(folder) = file.parent() {
        std::fs::create_dir_all(folder)?;
    }
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut out = options.open(file)?;
    out.write_all(text.as_bytes())?;
    out.flush()
}

/// Windows Credential Manager: one generic credential, kept for this user on this PC.
#[cfg(windows)]
mod credential {
    use windows::Win32::Security::Credentials::{
        CRED_PERSIST_LOCAL_MACHINE, CRED_TYPE_GENERIC, CREDENTIALW, CredDeleteW, CredFree,
        CredReadW, CredWriteW,
    };
    use windows::core::{PCWSTR, PWSTR};

    fn wide(text: &str) -> Vec<u16> {
        text.encode_utf16().chain(std::iter::once(0)).collect()
    }

    pub fn write(name: &str, secret: &str) -> Result<(), String> {
        let mut target = wide(name);
        let mut user = wide("AnyPlay");
        let mut blob = secret.as_bytes().to_vec();
        let credential = CREDENTIALW {
            Type: CRED_TYPE_GENERIC,
            TargetName: PWSTR(target.as_mut_ptr()),
            UserName: PWSTR(user.as_mut_ptr()),
            CredentialBlobSize: blob.len() as u32,
            CredentialBlob: blob.as_mut_ptr(),
            Persist: CRED_PERSIST_LOCAL_MACHINE,
            ..Default::default()
        };
        // SAFETY: `credential` points at buffers that live until the call returns; the call
        // copies what it keeps.
        unsafe { CredWriteW(&credential, 0) }.map_err(|e| e.to_string())
    }

    pub fn read(name: &str) -> Option<String> {
        let target = wide(name);
        let mut found: *mut CREDENTIALW = std::ptr::null_mut();
        // SAFETY: `target` is a null-terminated string that outlives the call; on success
        // `found` points at a record the system allocated, which is freed below.
        unsafe {
            CredReadW(PCWSTR(target.as_ptr()), CRED_TYPE_GENERIC, None, &mut found).ok()?;
            if found.is_null() {
                return None;
            }
            let record = &*found;
            let bytes = if record.CredentialBlob.is_null() {
                &[][..]
            } else {
                std::slice::from_raw_parts(
                    record.CredentialBlob,
                    record.CredentialBlobSize as usize,
                )
            };
            let text = String::from_utf8(bytes.to_vec()).ok();
            CredFree(found as *const _);
            text.filter(|t| !t.is_empty())
        }
    }

    pub fn delete(name: &str) -> Result<(), String> {
        let target = wide(name);
        // SAFETY: `target` is a null-terminated string that outlives the call.
        match unsafe { CredDeleteW(PCWSTR(target.as_ptr()), CRED_TYPE_GENERIC, None) } {
            Ok(()) => Ok(()),
            // 0x80070490: there was no credential to remove.
            Err(e) if e.code().0 as u32 == 0x8007_0490 => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

// --- the worker -------------------------------------------------------------------------------

/// Where a broadcast stands, for the panels.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Phase {
    #[default]
    Off,
    /// The worker is reaching the server.
    Connecting,
    /// The server accepted the stream.
    Live,
}

/// What the worker reports, one JSON object per line (network spec/broadcast.md).
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum WorkerEvent {
    Connecting,
    Live {
        width: u32,
        height: u32,
        fps: u32,
        kbps: u32,
    },
    Stats {
        seconds: u32,
        kbps: u32,
        dropped: u32,
    },
    Ended {
        reason: String,
    },
}

impl WorkerEvent {
    pub fn parse(line: &str) -> Option<WorkerEvent> {
        serde_json::from_str(line).ok()
    }
}

/// Why a broadcast ended, as the launcher tells it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndKind {
    /// The player stopped it, or the game ended.
    Normal,
    /// The service did not accept the key.
    BadKey,
    /// The address is wrong or the server did not answer.
    NoServer,
    /// The connection broke while live.
    Lost,
    NoEncoder,
    NoWindow,
    Other,
}

/// Sorts the worker's `ended` reason.
pub fn end_kind(reason: &str) -> EndKind {
    if reason == "quit" || reason == "window_closed" {
        EndKind::Normal
    } else if reason == "no_encoder" {
        EndKind::NoEncoder
    } else if reason == "no_window" {
        EndKind::NoWindow
    } else if reason.contains("stream key") {
        EndKind::BadKey
    } else if reason.starts_with("connect_failed") {
        EndKind::NoServer
    } else if reason.starts_with("disconnected") {
        EndKind::Lost
    } else {
        EndKind::Other
    }
}

/// The command line of the worker. The key is not on it.
pub fn worker_args(
    quality: Quality,
    window: Option<i64>,
    pids: &[u32],
    title: &str,
) -> Vec<String> {
    let mut args = vec![
        "--broadcast".to_owned(),
        "--width".into(),
        quality.width.to_string(),
        "--height".into(),
        quality.height.to_string(),
        "--fps".into(),
        quality.fps.to_string(),
        "--kbps".into(),
        quality.kbps.to_string(),
    ];
    if let Some(window) = window {
        args.push("--window".into());
        args.push(window.to_string());
    }
    for pid in pids {
        args.push("--pid".into());
        args.push(pid.to_string());
    }
    if !title.is_empty() {
        args.push("--title".into());
        args.push(title.to_owned());
    }
    args
}

/// The first line the worker reads: where to publish and under which key.
fn publish_line(url: &str, key: &str) -> String {
    serde_json::json!({"do": "publish", "url": url, "key": key}).to_string()
}

/// The worker running for one broadcast. Dropping it ends the worker.
pub struct Broadcaster {
    child: Child,
    input: Option<ChildStdin>,
    pub events: Receiver<WorkerEvent>,
}

impl Broadcaster {
    #[allow(clippy::too_many_arguments)]
    pub fn start(
        worker: &Path,
        url: &str,
        key: &str,
        quality: Quality,
        window: Option<i64>,
        pids: &[u32],
        title: &str,
        wake: impl Fn() + Send + 'static,
    ) -> std::io::Result<Broadcaster> {
        let mut command = Command::new(worker);
        command
            .args(worker_args(quality, window, pids, title))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            command.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
        }
        let mut child = command.spawn()?;
        let mut input = child.stdin.take();
        let stdout = child.stdout.take().ok_or(std::io::ErrorKind::BrokenPipe)?;
        if let Some(input) = input.as_mut() {
            writeln!(input, "{}", publish_line(url, key))?;
            input.flush()?;
        }
        let (tx, events) = mpsc::channel();
        std::thread::Builder::new()
            .name("broadcast-events".into())
            .spawn(move || {
                for line in BufReader::new(stdout).lines().map_while(Result::ok) {
                    if let Some(event) = WorkerEvent::parse(&line) {
                        if tx.send(event).is_err() {
                            break;
                        }
                        wake();
                    }
                }
            })?;
        Ok(Broadcaster {
            child,
            input,
            events,
        })
    }

    /// The worker is still running.
    pub fn alive(&mut self) -> bool {
        matches!(self.child.try_wait(), Ok(None))
    }
}

impl Drop for Broadcaster {
    fn drop(&mut self) {
        // Closing the input is a quit; a worker that stays is killed.
        if let Some(mut input) = self.input.take() {
            let _ = writeln!(input, r#"{{"do":"quit"}}"#);
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
        if self.alive() {
            let _ = self.child.kill();
        }
        let _ = self.child.wait();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_services_have_their_servers_and_a_custom_one_must_be_rtmp() {
        assert_eq!(
            server_url("twitch", ""),
            Some("rtmp://live.twitch.tv/app".into())
        );
        assert_eq!(
            server_url("youtube", "ignored"),
            Some("rtmp://a.rtmp.youtube.com/live2".into())
        );
        assert_eq!(
            server_url("custom", " rtmp://example.com/live "),
            Some("rtmp://example.com/live".into())
        );
        assert_eq!(server_url("custom", ""), None);
        assert_eq!(server_url("custom", "https://example.com"), None);
        assert_eq!(server_url("custom", "rtmp://"), None);
        // An unknown stored name is Twitch.
        assert_eq!(Service::from_id("nonsense"), Service::Twitch);
        assert_eq!(Service::from_id(""), Service::Twitch);
        for service in Service::ALL {
            assert_eq!(Service::from_id(service.id()), service);
        }
    }

    #[test]
    fn a_quality_index_that_does_not_exist_falls_back() {
        assert_eq!(quality(0), QUALITIES[0]);
        assert_eq!(quality(2).height, 1080);
        assert_eq!(quality(99), QUALITIES[0]);
    }

    #[test]
    fn the_key_field_gives_back_the_pasted_key_and_clears_on_empty() {
        assert_eq!(key_from_field("live_123_abc"), Some("live_123_abc".into()));
        assert_eq!(
            key_from_field(&format!("{MASK}live_123_abc")),
            Some("live_123_abc".into())
        );
        assert_eq!(key_from_field("  live_1 \n"), Some("live_1".into()));
        assert_eq!(key_from_field(MASK), None);
        assert_eq!(key_from_field(""), None);
    }

    #[test]
    fn the_worker_gets_the_picture_and_the_game_but_never_the_key() {
        let args = worker_args(QUALITIES[1], Some(77), &[10, 11], "My Game");
        assert_eq!(
            args,
            [
                "--broadcast",
                "--width",
                "1280",
                "--height",
                "720",
                "--fps",
                "60",
                "--kbps",
                "6000",
                "--window",
                "77",
                "--pid",
                "10",
                "--pid",
                "11",
                "--title",
                "My Game"
            ]
        );
        let line = publish_line("rtmp://h/app", "live_secret");
        assert!(line.contains("live_secret"));
        assert!(!args.iter().any(|a| a.contains("secret")));
    }

    #[test]
    fn the_worker_events_parse_and_garbage_is_ignored() {
        assert_eq!(
            WorkerEvent::parse(r#"{"event":"connecting"}"#),
            Some(WorkerEvent::Connecting)
        );
        assert_eq!(
            WorkerEvent::parse(r#"{"event":"stats","seconds":4,"kbps":4400,"dropped":1}"#),
            Some(WorkerEvent::Stats {
                seconds: 4,
                kbps: 4400,
                dropped: 1
            })
        );
        assert!(WorkerEvent::parse("not json").is_none());
        assert!(WorkerEvent::parse(r#"{"event":"future"}"#).is_none());
    }

    #[test]
    fn the_reasons_are_sorted() {
        assert_eq!(end_kind("quit"), EndKind::Normal);
        assert_eq!(end_kind("window_closed"), EndKind::Normal);
        assert_eq!(
            end_kind("connect_failed: the server does not accept this stream key"),
            EndKind::BadKey
        );
        assert_eq!(
            end_kind("connect_failed: could not reach x: the server closed the connection"),
            EndKind::NoServer
        );
        assert_eq!(
            end_kind("disconnected: the server closed the connection"),
            EndKind::Lost
        );
        assert_eq!(end_kind("no_encoder"), EndKind::NoEncoder);
        assert_eq!(end_kind("no_window"), EndKind::NoWindow);
        assert_eq!(end_kind("encoder_failed: x"), EndKind::Other);
    }

    #[cfg(not(windows))]
    #[test]
    fn the_key_is_kept_in_a_private_file_and_removed() {
        let folder = tempfile::tempdir().unwrap();
        let paths = AppPaths::under(folder.path());
        assert_eq!(load_key(&paths), None);
        save_key(&paths, Some("live_1")).unwrap();
        assert_eq!(load_key(&paths), Some("live_1".into()));
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(key_file(&paths))
                .unwrap()
                .permissions()
                .mode();
            assert_eq!(mode & 0o077, 0, "{mode:o}");
        }
        save_key(&paths, Some("live_2")).unwrap();
        assert_eq!(load_key(&paths), Some("live_2".into()));
        save_key(&paths, None).unwrap();
        assert_eq!(load_key(&paths), None);
        // Removing what is not there is fine.
        save_key(&paths, None).unwrap();
    }
}
