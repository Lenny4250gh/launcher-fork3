//! Start with the player's session: the launcher opens by itself when the player signs in
//! (docs/DESIGN.md 9.10). One entry per user, written by the launcher and removed by it, in the
//! place each system documents for exactly this:
//!
//! ```text
//! Windows  HKCU\Software\Microsoft\Windows\CurrentVersion\Run   AnyPlayLauncher = "<exe>" --autostart
//! Linux    ~/.config/autostart/anyplay-launcher.desktop         (XDG autostart; Flatpak and AppImage below)
//! macOS    ~/Library/LaunchAgents/net.anyplay.launcher.plist    (a per-user launchd agent, RunAtLoad)
//! ```
//!
//! No administrator rights, nothing outside the user's own folders and key, no shell replacement.
//! The interface is one [`Entry`] that knows where it lives and what it starts: [`Entry::state`],
//! [`Entry::enable`], [`Entry::disable`] and [`Entry::stale`] (the entry points at a program that
//! is no longer this one, so the launcher rewrites it at start). Everything that differs per
//! system sits behind them, and every system's text is a pure function, so the tests of all three
//! run on any host in a temp folder.
//!
//! * **AppImage:** the running binary lives in a temporary mount, so the entry starts the
//!   AppImage file itself (`$APPIMAGE`). Moving that file breaks the entry until the player
//!   switches it off and on again; the launcher rewrites a stale entry at its next start.
//! * **Flatpak:** the entry starts `flatpak run <id>`. The sandbox has the home folder, so the
//!   file lands in the real `~/.config/autostart`, not in the sandbox's own config folder.
//! * **macOS:** a launchd agent, not `SMAppService`: that API needs an app bundle with the
//!   launcher as its main app and a signed build, and the agent needs neither. The system lists
//!   it under Login Items "Allow in the Background".

use std::io;
use std::path::{Path, PathBuf};

use crate::platform::{Os, Vars};

/// The argument the entry starts the launcher with. It changes nothing by itself; the log says
/// the launcher was started by the session.
pub const ARG: &str = "--autostart";

/// The name of the Windows value and of the Linux file (without the extension).
const NAME: &str = "AnyPlayLauncher";
const FILE_STEM: &str = "anyplay-launcher";
const LABEL: &str = "net.anyplay.launcher";
/// Where the Flatpak build says its id.
const FLATPAK_ID: &str = "com.anyplaynet.Launcher";

#[cfg(windows)]
const RUN_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
/// Where Task Manager and Settings keep "this startup app is switched off" for the Run key.
#[cfg(windows)]
const APPROVED_KEY: &str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\StartupApproved\\Run";

/// Whether the launcher starts with the session.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Off,
    On,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Place {
    /// A value of the Windows Run key.
    RunValue(String),
    /// An XDG autostart file.
    Desktop(PathBuf),
    /// A launchd agent file.
    Agent(PathBuf),
}

/// One session entry: where it lives and what it starts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    place: Place,
    /// The program and its arguments.
    command: Vec<String>,
}

impl Entry {
    /// The entry of this system for the running launcher. `None` when there is no honest one:
    /// no home folder, or a binary that will not be there at the next sign-in.
    pub fn current(var: Vars) -> Option<Entry> {
        let exe = std::env::current_exe().ok()?;
        Entry::for_os(Os::current(), var, &exe)
    }

    /// [`Entry::current`] for any system, from the environment and the running program.
    pub fn for_os(os: Os, var: Vars, exe: &Path) -> Option<Entry> {
        let home = || var("HOME").map(PathBuf::from);
        match os {
            Os::Windows => Some(Entry {
                place: Place::RunValue(NAME.into()),
                command: vec![exe.display().to_string(), ARG.into()],
            }),
            Os::Macos => Some(Entry {
                place: Place::Agent(
                    home()?
                        .join("Library")
                        .join("LaunchAgents")
                        .join(format!("{LABEL}.plist")),
                ),
                command: vec![exe.display().to_string(), ARG.into()],
            }),
            Os::Linux => {
                if let Some(id) = var("FLATPAK_ID") {
                    let id = id.to_string_lossy().into_owned();
                    // The sandbox's XDG_CONFIG_HOME is its own folder; the session reads the real one.
                    let file = home()?
                        .join(".config")
                        .join("autostart")
                        .join(format!("{id}.desktop"));
                    let id = if id.is_empty() { FLATPAK_ID.into() } else { id };
                    return Some(Entry {
                        place: Place::Desktop(file),
                        command: vec!["flatpak".into(), "run".into(), id, ARG.into()],
                    });
                }
                let config = match var("XDG_CONFIG_HOME") {
                    Some(dir) if Path::new(&dir).is_absolute() => PathBuf::from(dir),
                    _ => home()?.join(".config"),
                };
                let program = match var("APPIMAGE") {
                    Some(image) => PathBuf::from(image),
                    // Run from an AppImage's mount without knowing the file: nothing to point at.
                    None if exe.to_string_lossy().starts_with("/tmp/.mount_") => return None,
                    None => exe.to_path_buf(),
                };
                Some(Entry {
                    place: Place::Desktop(
                        config
                            .join("autostart")
                            .join(format!("{FILE_STEM}.desktop")),
                    ),
                    command: vec![program.display().to_string(), ARG.into()],
                })
            }
        }
    }

    /// A Windows entry under another value name (the tests use their own, the real one stays).
    #[cfg(all(windows, test))]
    fn run_value(name: &str, exe: &Path) -> Entry {
        Entry {
            place: Place::RunValue(name.into()),
            command: vec![exe.display().to_string(), ARG.into()],
        }
    }

    /// The text the entry has when it is current.
    fn rendered(&self) -> String {
        match &self.place {
            Place::RunValue(_) => windows_command(&self.command),
            Place::Desktop(_) => desktop_entry(&self.command),
            Place::Agent(_) => launch_agent(&self.command),
        }
    }

    /// What the entry holds now; `None` when there is none (or it is switched off by hand).
    fn current_text(&self) -> Option<String> {
        match &self.place {
            Place::RunValue(name) => run_value::read(name),
            Place::Desktop(path) => {
                let text = std::fs::read_to_string(path).ok()?;
                let off = text.lines().any(|line| {
                    let line = line.trim().replace(' ', "");
                    line.eq_ignore_ascii_case("Hidden=true")
                        || line.eq_ignore_ascii_case("X-GNOME-Autostart-enabled=false")
                });
                (!off).then_some(text)
            }
            Place::Agent(path) => std::fs::read_to_string(path).ok(),
        }
    }

    pub fn state(&self) -> State {
        if self.current_text().is_some() {
            State::On
        } else {
            State::Off
        }
    }

    /// The entry exists but is not what [`Entry::enable`] would write now: the launcher moved, or
    /// was updated to another way of starting.
    pub fn stale(&self) -> bool {
        self.current_text()
            .is_some_and(|text| text != self.rendered())
    }

    /// Writes the entry (and, on Windows, lifts a "disabled" mark Task Manager may have set).
    pub fn enable(&self) -> io::Result<()> {
        match &self.place {
            Place::RunValue(name) => {
                run_value::write(name, &self.rendered())?;
                run_value::approve(name)
            }
            Place::Desktop(path) | Place::Agent(path) => {
                if let Some(folder) = path.parent() {
                    std::fs::create_dir_all(folder)?;
                }
                // Next to the file first, so a crash never leaves half an entry for the session.
                let partial = path.with_extension("part");
                std::fs::write(&partial, self.rendered())?;
                std::fs::rename(&partial, path)
            }
        }
    }

    /// Removes the entry. One that is not there is fine.
    pub fn disable(&self) -> io::Result<()> {
        match &self.place {
            Place::RunValue(name) => run_value::delete(name),
            Place::Desktop(path) | Place::Agent(path) => match std::fs::remove_file(path) {
                Err(error) if error.kind() != io::ErrorKind::NotFound => Err(error),
                _ => Ok(()),
            },
        }
    }

    /// The entry starts an AppImage file (the row says moving it breaks the entry).
    pub fn is_appimage_file(&self) -> bool {
        self.command
            .first()
            .is_some_and(|p| p.to_ascii_lowercase().ends_with(".appimage"))
    }
}

/// The Run value: the program quoted, its arguments after.
fn windows_command(command: &[String]) -> String {
    let mut parts = command.iter();
    let mut text = String::new();
    if let Some(program) = parts.next() {
        text = format!("\"{program}\"");
    }
    for arg in parts {
        text.push(' ');
        text.push_str(arg);
    }
    text
}

/// One argument of a desktop entry's `Exec` line (Desktop Entry Specification, "The Exec key"):
/// quoted when it holds a reserved character, `"`, `` ` ``, `$` and `\` escaped, `%` doubled; the
/// file then doubles every backslash once more (the key-file string escape).
fn exec_argument(arg: &str) -> String {
    const RESERVED: &str = " \t\n\"'\\><~|&;$*?#()`";
    let percent = arg.replace('%', "%%");
    let quoted = if percent.chars().any(|c| RESERVED.contains(c)) {
        let mut out = String::from("\"");
        for c in percent.chars() {
            if matches!(c, '"' | '`' | '$' | '\\') {
                out.push('\\');
            }
            out.push(c);
        }
        out.push('"');
        out
    } else {
        percent
    };
    quoted.replace('\\', "\\\\")
}

fn desktop_entry(command: &[String]) -> String {
    let exec: Vec<String> = command.iter().map(|a| exec_argument(a)).collect();
    format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=AnyPlay Launcher\n\
         Comment=Opens AnyPlay when you log in\n\
         Exec={}\n\
         Icon={FILE_STEM}\n\
         Terminal=false\n\
         X-GNOME-Autostart-enabled=true\n",
        exec.join(" ")
    )
}

fn xml_text(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn launch_agent(command: &[String]) -> String {
    let arguments: String = command
        .iter()
        .map(|a| format!("        <string>{}</string>\n", xml_text(a)))
        .collect();
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
         <plist version=\"1.0\">\n\
         <dict>\n\
         \x20   <key>Label</key>\n\
         \x20   <string>{LABEL}</string>\n\
         \x20   <key>ProgramArguments</key>\n\
         \x20   <array>\n\
         {arguments}\
         \x20   </array>\n\
         \x20   <key>RunAtLoad</key>\n\
         \x20   <true/>\n\
         \x20   <key>ProcessType</key>\n\
         \x20   <string>Interactive</string>\n\
         \x20   <key>LimitLoadToSessionType</key>\n\
         \x20   <string>Aqua</string>\n\
         </dict>\n\
         </plist>\n"
    )
}

#[cfg(windows)]
mod run_value {
    use std::io;

    use windows::Win32::Foundation::{ERROR_FILE_NOT_FOUND, ERROR_SUCCESS, WIN32_ERROR};
    use windows::Win32::System::Registry::{
        HKEY, HKEY_CURRENT_USER, KEY_WRITE, REG_OPTION_NON_VOLATILE, REG_SZ, RRF_RT_REG_SZ,
        RegCloseKey, RegCreateKeyExW, RegDeleteKeyValueW, RegGetValueW, RegSetValueExW,
    };
    use windows::core::PCWSTR;

    use super::{APPROVED_KEY, RUN_KEY};

    fn wide(text: &str) -> Vec<u16> {
        text.encode_utf16().chain(std::iter::once(0)).collect()
    }

    fn check(status: WIN32_ERROR) -> io::Result<()> {
        if status == ERROR_SUCCESS {
            Ok(())
        } else {
            Err(io::Error::from_raw_os_error(status.0 as i32))
        }
    }

    pub fn read(name: &str) -> Option<String> {
        let (key, name) = (wide(RUN_KEY), wide(name));
        let mut buffer = [0u16; 2048];
        let mut size = std::mem::size_of_val(&buffer) as u32;
        // SAFETY: NUL-terminated strings; the buffer and its size describe the same memory.
        let status = unsafe {
            RegGetValueW(
                HKEY_CURRENT_USER,
                PCWSTR(key.as_ptr()),
                PCWSTR(name.as_ptr()),
                RRF_RT_REG_SZ,
                None,
                Some(buffer.as_mut_ptr().cast()),
                Some(&mut size),
            )
        };
        if status != ERROR_SUCCESS {
            return None;
        }
        let len = buffer.iter().position(|c| *c == 0).unwrap_or(buffer.len());
        Some(String::from_utf16_lossy(&buffer[..len]))
    }

    pub fn write(name: &str, command: &str) -> io::Result<()> {
        let (key, name, data) = (wide(RUN_KEY), wide(name), wide(command));
        let mut handle = HKEY::default();
        // SAFETY: NUL-terminated strings that outlive the calls; the handle is closed.
        unsafe {
            check(RegCreateKeyExW(
                HKEY_CURRENT_USER,
                PCWSTR(key.as_ptr()),
                None,
                PCWSTR::null(),
                REG_OPTION_NON_VOLATILE,
                KEY_WRITE,
                None,
                &mut handle,
                None,
            ))?;
            let bytes = std::slice::from_raw_parts(data.as_ptr().cast::<u8>(), data.len() * 2);
            let result = check(RegSetValueExW(
                handle,
                PCWSTR(name.as_ptr()),
                None,
                REG_SZ,
                Some(bytes),
            ));
            let _ = RegCloseKey(handle);
            result
        }
    }

    fn delete_in(key: &str, name: &str) -> io::Result<()> {
        let (key, name) = (wide(key), wide(name));
        // SAFETY: NUL-terminated strings.
        let status = unsafe {
            RegDeleteKeyValueW(
                HKEY_CURRENT_USER,
                PCWSTR(key.as_ptr()),
                PCWSTR(name.as_ptr()),
            )
        };
        if status == ERROR_FILE_NOT_FOUND {
            Ok(())
        } else {
            check(status)
        }
    }

    pub fn delete(name: &str) -> io::Result<()> {
        delete_in(RUN_KEY, name)?;
        delete_in(APPROVED_KEY, name)
    }

    /// Takes away the mark that switched the entry off in Task Manager.
    pub fn approve(name: &str) -> io::Result<()> {
        delete_in(APPROVED_KEY, name)
    }
}

#[cfg(not(windows))]
mod run_value {
    use std::io;

    pub fn read(_name: &str) -> Option<String> {
        None
    }

    pub fn write(_name: &str, _command: &str) -> io::Result<()> {
        Err(io::ErrorKind::Unsupported.into())
    }

    pub fn delete(_name: &str) -> io::Result<()> {
        Err(io::ErrorKind::Unsupported.into())
    }

    pub fn approve(_name: &str) -> io::Result<()> {
        Err(io::ErrorKind::Unsupported.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;

    fn vars(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<OsString> {
        let pairs: Vec<(String, String)> = pairs
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect();
        move |name| {
            pairs
                .iter()
                .find(|(k, _)| k == name)
                .map(|(_, v)| OsString::from(v))
        }
    }

    fn entry(os: Os, pairs: &[(&str, &str)], exe: &str) -> Option<Entry> {
        let var = vars(pairs);
        Entry::for_os(os, &var, Path::new(exe))
    }

    fn file_of(entry: &Entry) -> &Path {
        match &entry.place {
            Place::Desktop(p) | Place::Agent(p) => p,
            Place::RunValue(_) => panic!("not a file"),
        }
    }

    #[test]
    fn linux_writes_an_xdg_autostart_file_and_removes_it() {
        let home = tempfile::tempdir().unwrap();
        let home_text = home.path().to_str().unwrap();
        let e = entry(
            Os::Linux,
            &[("HOME", home_text)],
            "/opt/anyplay/anyplay-launcher",
        )
        .unwrap();
        assert_eq!(e.state(), State::Off);
        e.enable().unwrap();
        let file = file_of(&e);
        assert_eq!(
            file,
            home.path()
                .join(".config/autostart/anyplay-launcher.desktop")
        );
        let text = std::fs::read_to_string(file).unwrap();
        assert!(text.contains("Exec=/opt/anyplay/anyplay-launcher --autostart\n"));
        assert!(text.starts_with("[Desktop Entry]\nType=Application\n"));
        assert_eq!(e.state(), State::On);
        assert!(!e.stale());
        e.disable().unwrap();
        assert_eq!(e.state(), State::Off);
        // Twice is fine, and no half-written file stays behind.
        e.disable().unwrap();
        let left: Vec<_> = std::fs::read_dir(file.parent().unwrap()).unwrap().collect();
        assert!(left.is_empty());
    }

    #[test]
    fn linux_follows_xdg_config_home_when_it_is_absolute() {
        let dir = tempfile::tempdir().unwrap();
        let config = dir.path().to_str().unwrap();
        let e = entry(
            Os::Linux,
            &[("HOME", "/home/x"), ("XDG_CONFIG_HOME", config)],
            "/bin/l",
        )
        .unwrap();
        assert!(file_of(&e).starts_with(dir.path().join("autostart")));
        let relative = entry(
            Os::Linux,
            &[("HOME", "/home/x"), ("XDG_CONFIG_HOME", "rel")],
            "/bin/l",
        )
        .unwrap();
        assert_eq!(
            file_of(&relative),
            Path::new("/home/x/.config/autostart/anyplay-launcher.desktop")
        );
    }

    #[test]
    fn an_appimage_starts_the_file_not_its_temporary_mount() {
        let e = entry(
            Os::Linux,
            &[
                ("HOME", "/home/x"),
                ("APPIMAGE", "/home/x/Apps/AnyPlay Launcher.AppImage"),
            ],
            "/tmp/.mount_AnyPlaXYZ/usr/bin/anyplay-launcher",
        )
        .unwrap();
        assert!(e.is_appimage_file());
        let text = e.rendered();
        assert!(
            text.contains("Exec=\"/home/x/Apps/AnyPlay Launcher.AppImage\" --autostart\n"),
            "{text}"
        );
        // A mount without the file to point at: no entry rather than a broken one.
        assert!(
            entry(
                Os::Linux,
                &[("HOME", "/home/x")],
                "/tmp/.mount_AnyPlaXYZ/usr/bin/anyplay-launcher"
            )
            .is_none()
        );
    }

    #[test]
    fn flatpak_starts_flatpak_run_and_writes_to_the_real_config_folder() {
        let e = entry(
            Os::Linux,
            &[
                ("HOME", "/home/x"),
                ("FLATPAK_ID", "com.anyplaynet.Launcher"),
                (
                    "XDG_CONFIG_HOME",
                    "/home/x/.var/app/com.anyplaynet.Launcher/config",
                ),
            ],
            "/app/bin/anyplay-launcher",
        )
        .unwrap();
        assert_eq!(
            file_of(&e),
            Path::new("/home/x/.config/autostart/com.anyplaynet.Launcher.desktop")
        );
        assert!(
            e.rendered()
                .contains("Exec=flatpak run com.anyplaynet.Launcher --autostart\n")
        );
    }

    #[test]
    fn exec_arguments_are_quoted_and_escaped_by_the_specification() {
        assert_eq!(exec_argument("plain"), "plain");
        assert_eq!(exec_argument("a b"), "\"a b\"");
        assert_eq!(exec_argument("100%"), "100%%");
        assert_eq!(exec_argument("a\"b"), "\"a\\\\\"b\"");
        assert_eq!(exec_argument("$HOME x"), "\"\\\\$HOME x\"");
    }

    #[test]
    fn a_hidden_entry_counts_as_off_and_a_moved_program_as_stale() {
        let home = tempfile::tempdir().unwrap();
        let home_text = home.path().to_str().unwrap();
        let old = entry(Os::Linux, &[("HOME", home_text)], "/old/launcher").unwrap();
        old.enable().unwrap();
        let new = entry(Os::Linux, &[("HOME", home_text)], "/new/launcher").unwrap();
        assert_eq!(new.state(), State::On);
        assert!(new.stale());
        new.enable().unwrap();
        assert!(!new.stale());
        // The desktop's own "switch off" is respected.
        let file = file_of(&new);
        let text = std::fs::read_to_string(file).unwrap();
        std::fs::write(file, format!("{text}Hidden=true\n")).unwrap();
        assert_eq!(new.state(), State::Off);
        assert!(!new.stale());
    }

    #[test]
    fn macos_writes_a_launch_agent() {
        let home = tempfile::tempdir().unwrap();
        let home_text = home.path().to_str().unwrap();
        let e = entry(
            Os::Macos,
            &[("HOME", home_text)],
            "/Applications/AnyPlay & Co.app/Contents/MacOS/anyplay-launcher",
        )
        .unwrap();
        e.enable().unwrap();
        let file = file_of(&e);
        assert_eq!(
            file,
            home.path()
                .join("Library/LaunchAgents/net.anyplay.launcher.plist")
        );
        let text = std::fs::read_to_string(file).unwrap();
        assert!(text.contains("<key>RunAtLoad</key>\n    <true/>"));
        assert!(text.contains("<string>net.anyplay.launcher</string>"));
        assert!(text.contains("AnyPlay &amp; Co.app/Contents/MacOS/anyplay-launcher</string>"));
        assert!(text.contains("<string>--autostart</string>"));
        e.disable().unwrap();
        assert_eq!(e.state(), State::Off);
    }

    #[test]
    fn no_home_folder_means_no_entry() {
        assert!(entry(Os::Linux, &[], "/bin/l").is_none());
        assert!(entry(Os::Macos, &[], "/bin/l").is_none());
    }

    #[test]
    fn windows_command_quotes_the_program() {
        let e = entry(Os::Windows, &[], "C:\\Program Files\\AnyPlay\\launcher.exe").unwrap();
        assert_eq!(
            e.rendered(),
            "\"C:\\Program Files\\AnyPlay\\launcher.exe\" --autostart"
        );
    }

    /// The real Run key, under a value name of its own that the test removes again.
    #[cfg(windows)]
    #[test]
    fn windows_run_key_round_trip() {
        let name = format!("AnyPlayLauncherTest{}", std::process::id());
        let e = Entry::run_value(&name, Path::new("C:\\Test Dir\\launcher.exe"));
        assert_eq!(e.state(), State::Off);
        e.enable().unwrap();
        assert_eq!(e.state(), State::On);
        assert!(!e.stale());
        let moved = Entry::run_value(&name, Path::new("C:\\Other\\launcher.exe"));
        assert!(moved.stale());
        moved.enable().unwrap();
        assert!(!moved.stale());
        e.disable().unwrap();
        assert_eq!(e.state(), State::Off);
        e.disable().unwrap();
    }
}
