//! Push to talk while a game is in front (docs/DESIGN.md 17, "Push to talk and the game window").
//!
//! The launcher draws no frames while it is minimised behind a game, so the key and the pad
//! button cannot wait for the UI: a small thread of its own watches them and tells the net
//! service directly. It reads exactly one key, the one the player chose for talking, and only
//! while the game's window is in front (`GetAsyncKeyState`; no hook, nothing else is read). A
//! pad button comes from XInput (Windows) or the gamepad thread.
//!
//! The UI and this thread share the work by the window in front. A launcher in front is the
//! UI's: it reads the key from egui, the pad button from here, and says what it sent (`told`).
//! With something else in front the thread sends changes and the UI adopts its state.

use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU32, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use anyps5_core::online::LineWriter;
use egui::Key;

use super::home_button::HomeButton;
use super::pad_info::Family;
use super::sound::{Cue, CueSender};

/// The pad buttons that can be held to talk: the ones the launcher itself never reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PadButton {
    LeftBumper,
    RightBumper,
    LeftTrigger,
    RightTrigger,
    LeftStick,
    RightStick,
}

impl PadButton {
    pub const ALL: [PadButton; 6] = [
        PadButton::LeftBumper,
        PadButton::RightBumper,
        PadButton::LeftTrigger,
        PadButton::RightTrigger,
        PadButton::LeftStick,
        PadButton::RightStick,
    ];

    /// The name kept in the preferences.
    pub fn name(self) -> &'static str {
        match self {
            PadButton::LeftBumper => "lb",
            PadButton::RightBumper => "rb",
            PadButton::LeftTrigger => "lt",
            PadButton::RightTrigger => "rt",
            PadButton::LeftStick => "l3",
            PadButton::RightStick => "r3",
        }
    }

    pub fn from_name(name: &str) -> Option<PadButton> {
        Self::ALL.into_iter().find(|b| b.name() == name)
    }

    /// The button as the pad in use names it.
    pub fn label(self, family: Family) -> &'static str {
        let sony = family == Family::PlayStation;
        match (self, sony) {
            (PadButton::LeftBumper, true) => "L1",
            (PadButton::RightBumper, true) => "R1",
            (PadButton::LeftTrigger, true) => "L2",
            (PadButton::RightTrigger, true) => "R2",
            (PadButton::LeftBumper, false) => "LB",
            (PadButton::RightBumper, false) => "RB",
            (PadButton::LeftTrigger, false) => "LT",
            (PadButton::RightTrigger, false) => "RT",
            (PadButton::LeftStick, true) => "L3",
            (PadButton::RightStick, true) => "R3",
            (PadButton::LeftStick, false) => "LS",
            (PadButton::RightStick, false) => "RS",
        }
    }

    fn code(self) -> u8 {
        Self::ALL.iter().position(|b| *b == self).unwrap_or(0) as u8 + 1
    }

    fn from_code(code: u8) -> Option<PadButton> {
        usize::from(code)
            .checked_sub(1)
            .and_then(|i| Self::ALL.get(i).copied())
    }

    pub fn gilrs(self) -> gilrs::Button {
        match self {
            PadButton::LeftBumper => gilrs::Button::LeftTrigger,
            PadButton::RightBumper => gilrs::Button::RightTrigger,
            PadButton::LeftTrigger => gilrs::Button::LeftTrigger2,
            PadButton::RightTrigger => gilrs::Button::RightTrigger2,
            PadButton::LeftStick => gilrs::Button::LeftThumb,
            PadButton::RightStick => gilrs::Button::RightThumb,
        }
    }

    /// Whether `buttons` and `triggers` (an XInput state) show the button down.
    pub fn in_xinput(self, buttons: u16, triggers: [u8; 2]) -> bool {
        // The trigger threshold is 30 of 255, Microsoft's own advice for XInput.
        const TRIGGER: u8 = 30;
        match self {
            PadButton::LeftBumper => buttons & 0x0100 != 0,
            PadButton::RightBumper => buttons & 0x0200 != 0,
            PadButton::LeftStick => buttons & 0x0040 != 0,
            PadButton::RightStick => buttons & 0x0080 != 0,
            PadButton::LeftTrigger => triggers[0] > TRIGGER,
            PadButton::RightTrigger => triggers[1] > TRIGGER,
        }
    }
}

/// The Windows virtual key of a key the player can pick, when the thread can read it.
pub fn virtual_key(key: Key) -> Option<u32> {
    let name = key.name();
    if let [letter @ b'A'..=b'Z'] = name.as_bytes() {
        return Some(u32::from(*letter));
    }
    if let [digit @ b'0'..=b'9'] = name.as_bytes() {
        return Some(u32::from(*digit));
    }
    if let Some(n) = name.strip_prefix('F').and_then(|d| d.parse::<u32>().ok())
        && (1..=12).contains(&n)
    {
        return Some(0x70 + n - 1);
    }
    Some(match name {
        "Insert" => 0x2D,
        "Delete" => 0x2E,
        "Home" => 0x24,
        "End" => 0x23,
        "PageUp" => 0x21,
        "PageDown" => 0x22,
        _ => return None,
    })
}

/// Shared by the UI, the pad threads and the watcher.
#[derive(Default)]
pub struct Talk {
    /// Voice runs here in push-to-talk mode.
    active: AtomicBool,
    /// The Windows virtual key of the talk key; 0 for none.
    key: AtomicU32,
    button: AtomicU8,
    /// The button is down, as the gamepad thread and as XInput see it.
    gilrs_down: AtomicBool,
    xinput_down: AtomicBool,
    /// What the net service was last told, by the UI or by the thread.
    told: AtomicBool,
    cues_on: AtomicBool,
    writer: Mutex<Option<LineWriter>>,
    cues: Mutex<Option<CueSender>>,
    ctx: OnceLock<egui::Context>,
    home: OnceLock<Arc<HomeButton>>,
    started: OnceLock<()>,
}

static TALK: OnceLock<Talk> = OnceLock::new();

pub fn shared() -> &'static Talk {
    TALK.get_or_init(Talk::default)
}

/// What the UI says every frame.
pub struct Setup<'a> {
    pub ctx: &'a egui::Context,
    pub home: &'a Arc<HomeButton>,
    /// Voice runs here and the mode in use is push to talk.
    pub active: bool,
    pub key: Option<u32>,
    pub button: Option<PadButton>,
    /// "Interface Sounds" is on.
    pub cues_on: bool,
}

impl Talk {
    pub fn configure(&self, setup: Setup<'_>) {
        self.active.store(setup.active, Ordering::Relaxed);
        self.key.store(setup.key.unwrap_or(0), Ordering::Relaxed);
        self.button
            .store(setup.button.map_or(0, PadButton::code), Ordering::Relaxed);
        self.cues_on.store(setup.cues_on, Ordering::Relaxed);
        if setup.active && self.started.set(()).is_ok() {
            let _ = self.ctx.set(setup.ctx.clone());
            let _ = self.home.set(setup.home.clone());
            let spawned = std::thread::Builder::new()
                .name("push-to-talk".into())
                .spawn(|| shared().watch());
            if let Err(error) = spawned {
                tracing::warn!("push-to-talk thread not started: {error}");
            }
        }
    }

    pub fn set_writer(&self, writer: Option<LineWriter>) {
        *self.writer.lock().unwrap_or_else(|e| e.into_inner()) = writer;
    }

    pub fn has_cues(&self) -> bool {
        self.cues
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .is_some()
    }

    pub fn set_cues(&self, cues: CueSender) {
        *self.cues.lock().unwrap_or_else(|e| e.into_inner()) = Some(cues);
    }

    /// What the service was last told.
    pub fn told(&self) -> bool {
        self.told.load(Ordering::Relaxed)
    }

    pub fn set_told(&self, talking: bool) {
        self.told.store(talking, Ordering::Relaxed);
    }

    /// The chosen pad button is down.
    pub fn pad_down(&self) -> bool {
        self.gilrs_down.load(Ordering::Relaxed) || self.xinput_down.load(Ordering::Relaxed)
    }

    pub fn button(&self) -> Option<PadButton> {
        PadButton::from_code(self.button.load(Ordering::Relaxed))
    }

    /// The gamepad thread saw a button change.
    pub fn gilrs_button(&self, button: gilrs::Button, down: bool) {
        if self.button().is_some_and(|b| b.gilrs() == button)
            && self.gilrs_down.swap(down, Ordering::Relaxed) != down
        {
            self.wake();
        }
    }

    pub fn gilrs_gone(&self) {
        self.gilrs_down.store(false, Ordering::Relaxed);
    }

    /// The XInput thread read a state (`None`: no pad answered).
    pub fn xinput(&self, state: Option<(u16, [u8; 2])>) {
        let down = state
            .zip(self.button())
            .is_some_and(|((buttons, triggers), b)| b.in_xinput(buttons, triggers));
        if self.xinput_down.swap(down, Ordering::Relaxed) != down {
            self.wake();
        }
    }

    fn wake(&self) {
        if let Some(ctx) = self.ctx.get() {
            ctx.request_repaint();
        }
    }

    pub fn play(&self, cue: Cue) {
        if self.cues_on.load(Ordering::Relaxed)
            && let Some(cues) = &*self.cues.lock().unwrap_or_else(|e| e.into_inner())
        {
            cues.play(cue);
        }
    }

    /// The thread: while anything but the launcher is in front, tells the service when the key
    /// or the button changes.
    fn watch(&self) {
        let Some(home) = self.home.get() else { return };
        loop {
            let active = self.active.load(Ordering::Relaxed);
            if home.launcher_in_front() {
                // The UI's frames carry it.
                std::thread::sleep(Duration::from_millis(30));
                continue;
            }
            let down = active && (self.pad_down() || self.global_key(home));
            if down != self.told.load(Ordering::Relaxed) && self.send(down) {
                self.told.store(down, Ordering::Relaxed);
                self.play(if down { Cue::TalkOn } else { Cue::TalkOff });
                self.wake();
            }
            std::thread::sleep(Duration::from_millis(if active { 8 } else { 100 }));
        }
    }

    /// The talk key is down while the game's window is in front.
    #[cfg(windows)]
    fn global_key(&self, home: &HomeButton) -> bool {
        let key = self.key.load(Ordering::Relaxed);
        if key == 0 || !home.game_running() {
            return false;
        }
        let window = home.game_window();
        if window != 0 && anyps5_core::win::foreground().0 != window {
            return false;
        }
        // Shift, Ctrl, Alt and the Windows keys make it a shortcut (Ctrl+V pastes), not talking.
        let modifier = [0x10, 0x11, 0x12, 0x5B, 0x5C]
            .into_iter()
            .any(anyps5_core::win::key_down);
        !modifier && anyps5_core::win::key_down(key)
    }

    /// Other systems give no way to read a key from a program that is not in front.
    #[cfg(not(windows))]
    fn global_key(&self, _home: &HomeButton) -> bool {
        false
    }

    fn send(&self, talking: bool) -> bool {
        let writer = self
            .writer
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone();
        writer.is_some_and(|w| w.push_to_talk(talking).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_button_name_comes_back_and_has_a_label_for_each_make() {
        for button in PadButton::ALL {
            assert_eq!(PadButton::from_name(button.name()), Some(button));
            assert_eq!(PadButton::from_code(button.code()), Some(button));
            assert!(!button.label(Family::PlayStation).is_empty());
            assert!(!button.label(Family::Xbox).is_empty());
        }
        assert_eq!(PadButton::from_name("south"), None);
        assert_eq!(PadButton::from_code(0), None);
        assert_eq!(PadButton::LeftBumper.label(Family::PlayStation), "L1");
        assert_eq!(PadButton::LeftBumper.label(Family::Xbox), "LB");
    }

    #[test]
    fn xinput_buttons_and_triggers() {
        assert!(PadButton::LeftBumper.in_xinput(0x0100, [0, 0]));
        assert!(!PadButton::LeftBumper.in_xinput(0x0200, [0, 0]));
        assert!(PadButton::RightTrigger.in_xinput(0, [0, 200]));
        assert!(!PadButton::RightTrigger.in_xinput(0, [200, 10]));
        assert!(PadButton::RightStick.in_xinput(0x0080, [0, 0]));
    }

    #[test]
    fn keys_map_to_virtual_keys() {
        assert_eq!(virtual_key(Key::V), Some(0x56));
        assert_eq!(virtual_key(Key::Num3), Some(0x33));
        assert_eq!(virtual_key(Key::F5), Some(0x74));
        assert_eq!(virtual_key(Key::F13), None);
        assert_eq!(virtual_key(Key::Space), None);
    }
}
