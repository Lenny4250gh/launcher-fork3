//! The launcher's own navigation, rebindable (docs/DESIGN.md 6.10): which pad button and which
//! key is Select, Back, Options, and the two shoulder jobs (previous and next section). This is
//! the launcher only; a game keeps its own bindings (the Controls panel of a game).
//!
//! Fixed on purpose: the D-pad and the left stick are the directions, the arrow keys too, Home
//! (the PS button) switches between the game and the launcher, and Create saves a clip. They
//! cannot be bound, so a layout can never leave the player without a way to move.
//!
//! A binding is a name (`South`, `Enter`); a role that is not in the preferences keeps its
//! default. Giving a button to a job it already does elsewhere swaps: the other job takes what
//! the first one let go of, so no job is ever left without a button on the pad.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, RwLock};

use anyps5_core::settings::{DEFAULT_PAD_DEADZONE, Preferences};
use egui::Key;
use gilrs::Button;

use crate::ui::input::Command;
use crate::ui::pad_hub;

/// A job of the launcher's navigation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Role {
    Select,
    Back,
    Options,
    /// The shoulder jobs: the previous and the next section (the tab of the home screen, a row
    /// up or down in a panel). Keyboard: Shift+Tab and Tab stay as they are.
    Previous,
    Next,
}

impl Role {
    pub const ALL: [Role; 5] = [
        Role::Select,
        Role::Back,
        Role::Options,
        Role::Previous,
        Role::Next,
    ];

    /// The name the preferences use.
    pub const fn id(self) -> &'static str {
        match self {
            Role::Select => "select",
            Role::Back => "back",
            Role::Options => "options",
            Role::Previous => "previous",
            Role::Next => "next",
        }
    }

    pub fn from_id(id: &str) -> Option<Role> {
        Role::ALL.into_iter().find(|r| r.id() == id)
    }

    /// The command a pad button or key of this job sends; `None` for the shoulder jobs, which
    /// are tab steps.
    pub fn command(self) -> Option<Command> {
        match self {
            Role::Select => Some(Command::Select),
            Role::Back => Some(Command::Back),
            Role::Options => Some(Command::Options),
            Role::Previous | Role::Next => None,
        }
    }

    fn default_pad(self) -> &'static [&'static str] {
        match self {
            Role::Select => &["South"],
            Role::Back => &["East"],
            Role::Options => &["North", "Start"],
            Role::Previous => &["L1"],
            Role::Next => &["R1"],
        }
    }

    fn default_keys(self) -> &'static [&'static str] {
        match self {
            Role::Select => &["Enter", "Space"],
            Role::Back => &["Escape", "Backspace"],
            Role::Options => &["O"],
            Role::Previous | Role::Next => &[],
        }
    }
}

/// The pad buttons that can carry a job.
pub const BINDABLE_PAD: [&str; 11] = [
    "South", "East", "West", "North", "L1", "R1", "L2", "R2", "Start", "L3", "R3",
];

/// Why a button or key cannot be bound.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    /// It has a fixed job (the directions, Home, Create, Tab, F11).
    Fixed,
    /// It is not a button or key this launcher knows.
    Unknown,
}

/// What a new binding took from another job.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Swap {
    /// The job that had it.
    pub other: Role,
    /// The button or key that moved.
    pub took: String,
    /// What the other job got in return, when it was left with nothing.
    pub gave: Option<String>,
}

/// Every job's pad buttons and keys.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bindings {
    pad: BTreeMap<Role, Vec<String>>,
    keys: BTreeMap<Role, Vec<String>>,
}

fn owned(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| (*s).to_owned()).collect()
}

impl Default for Bindings {
    fn default() -> Self {
        Self::defaults()
    }
}

/// Keys that cannot be bound: the directions, Tab (focus order) and F11 (full screen).
fn key_fixed(key: Key) -> bool {
    matches!(
        key,
        Key::ArrowLeft | Key::ArrowRight | Key::ArrowUp | Key::ArrowDown | Key::Tab | Key::F11
    )
}

impl Bindings {
    pub fn defaults() -> Self {
        Self {
            pad: Role::ALL
                .into_iter()
                .map(|r| (r, owned(r.default_pad())))
                .collect(),
            keys: Role::ALL
                .into_iter()
                .map(|r| (r, owned(r.default_keys())))
                .collect(),
        }
    }

    /// The defaults with the preferences' changes laid over them. Names this version does not
    /// know are dropped; a job whose names were all dropped keeps its default.
    pub fn from_prefs(prefs: &Preferences) -> Self {
        let mut bindings = Self::defaults();
        for (id, names) in &prefs.nav_pad {
            let Some(role) = Role::from_id(id) else {
                continue;
            };
            let kept: Vec<String> = names
                .iter()
                .filter(|n| BINDABLE_PAD.contains(&n.as_str()))
                .cloned()
                .collect();
            if !kept.is_empty() {
                bindings.pad.insert(role, kept);
            }
        }
        for (id, names) in &prefs.nav_keys {
            let Some(role) = Role::from_id(id) else {
                continue;
            };
            let kept: Vec<String> = names
                .iter()
                .filter(|n| Key::from_name(n).is_some_and(|k| !key_fixed(k)))
                .cloned()
                .collect();
            // An empty list is a choice for the keys (a job with no key), not for the pad.
            if kept.len() == names.len() {
                bindings.keys.insert(role, kept);
            }
        }
        bindings
    }

    /// Writes only what differs from the defaults into the preferences.
    pub fn store(&self, prefs: &mut Preferences) {
        let defaults = Self::defaults();
        prefs.nav_pad = Role::ALL
            .into_iter()
            .filter(|r| self.pad[r] != defaults.pad[r])
            .map(|r| (r.id().to_owned(), self.pad[&r].clone()))
            .collect();
        prefs.nav_keys = Role::ALL
            .into_iter()
            .filter(|r| self.keys[r] != defaults.keys[r])
            .map(|r| (r.id().to_owned(), self.keys[&r].clone()))
            .collect();
    }

    pub fn is_default(&self) -> bool {
        *self == Self::defaults()
    }

    pub fn reset(&mut self) {
        *self = Self::defaults();
    }

    pub fn pad_buttons(&self, role: Role) -> &[String] {
        &self.pad[&role]
    }

    pub fn keys(&self, role: Role) -> &[String] {
        &self.keys[&role]
    }

    /// The job a pad button does.
    pub fn pad_role(&self, button: Button) -> Option<Role> {
        let name = pad_hub::button_name(button)?;
        Role::ALL
            .into_iter()
            .find(|r| self.pad[r].iter().any(|n| n == name))
    }

    /// The job a key does.
    pub fn key_role(&self, key: Key) -> Option<Role> {
        let name = key.name();
        Role::ALL
            .into_iter()
            .find(|r| self.keys[r].iter().any(|n| n == name))
    }

    /// Makes `button` the only pad button of `role`. If another job had it, that job lets go of
    /// it and, left with nothing, takes what `role` had.
    pub fn assign_pad(&mut self, role: Role, button: &str) -> Result<Option<Swap>, Refused> {
        if !BINDABLE_PAD.contains(&button) {
            return Err(if pad_hub::index_of(button).is_some() {
                Refused::Fixed
            } else {
                Refused::Unknown
            });
        }
        Ok(assign(&mut self.pad, role, button))
    }

    /// The same for a key.
    pub fn assign_key(&mut self, role: Role, key: Key) -> Result<Option<Swap>, Refused> {
        if key_fixed(key) {
            return Err(Refused::Fixed);
        }
        Ok(assign(&mut self.keys, role, key.name()))
    }
}

fn assign(map: &mut BTreeMap<Role, Vec<String>>, role: Role, name: &str) -> Option<Swap> {
    let old = map[&role].first().cloned();
    let mut swap = None;
    for other in Role::ALL {
        if other == role || !map[&other].iter().any(|n| n == name) {
            continue;
        }
        let list = map.get_mut(&other).expect("every role has a list");
        list.retain(|n| n != name);
        let gave = if list.is_empty() {
            // Not for a job that was already without (the keys of the shoulders).
            old.clone().inspect(|o| list.push(o.clone()))
        } else {
            None
        };
        swap = Some(Swap {
            other,
            took: name.to_owned(),
            gave,
        });
    }
    map.insert(role, vec![name.to_owned()]);
    swap
}

// ---------------------------------------------------------------------------------------------
// What the input pipeline reads
// ---------------------------------------------------------------------------------------------

static CURRENT: RwLock<Option<Arc<Bindings>>> = RwLock::new(None);
static DEADZONE: AtomicU8 = AtomicU8::new(DEFAULT_PAD_DEADZONE);
static CAPTURING: AtomicBool = AtomicBool::new(false);

/// Makes the preferences' bindings and dead zone the ones the pad thread and the keyboard use.
pub fn apply(prefs: &Preferences) {
    let bindings = Bindings::from_prefs(prefs);
    let mut slot = CURRENT.write().unwrap_or_else(|e| e.into_inner());
    if slot.as_deref() != Some(&bindings) {
        *slot = Some(Arc::new(bindings));
    }
    DEADZONE.store(
        prefs.pad_deadzone.clamp(MIN_ZONE, MAX_ZONE),
        Ordering::Relaxed,
    );
}

/// The bindings in force.
pub fn current() -> Arc<Bindings> {
    let slot = CURRENT.read().unwrap_or_else(|e| e.into_inner());
    slot.clone()
        .unwrap_or_else(|| Arc::new(Bindings::defaults()))
}

/// The smallest and largest stick dead zone the page offers, in percent.
pub const MIN_ZONE: u8 = 5;
pub const MAX_ZONE: u8 = 50;

/// The stick dead zone in force, 0.05 to 0.5.
pub fn deadzone() -> f32 {
    f32::from(DEADZONE.load(Ordering::Relaxed)) / 100.0
}

/// While a binding waits for a button, or the button test runs, the pad sends the launcher no
/// commands (Home and Create still work), so a press only binds or lights.
pub fn set_capturing(on: bool) {
    CAPTURING.store(on, Ordering::Relaxed);
}

pub fn capturing() -> bool {
    CAPTURING.load(Ordering::Relaxed)
}

/// The step a key stands for when it is bound to a shoulder job: `Some(backwards)`.
pub fn tab_for_key(key: Key) -> Option<bool> {
    match current().key_role(key) {
        Some(Role::Previous) => Some(true),
        Some(Role::Next) => Some(false),
        _ => None,
    }
}

/// The command a key stands for in the launcher's own navigation.
pub fn command_for_key(key: Key) -> Option<Command> {
    current().key_role(key).and_then(Role::command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_defaults_are_the_old_fixed_keys() {
        let b = Bindings::defaults();
        assert_eq!(b.pad_role(Button::South), Some(Role::Select));
        assert_eq!(b.pad_role(Button::East), Some(Role::Back));
        assert_eq!(b.pad_role(Button::North), Some(Role::Options));
        assert_eq!(b.pad_role(Button::Start), Some(Role::Options));
        assert_eq!(b.pad_role(Button::LeftTrigger), Some(Role::Previous));
        assert_eq!(b.pad_role(Button::RightTrigger), Some(Role::Next));
        assert_eq!(b.pad_role(Button::West), None);
        assert_eq!(b.key_role(Key::Enter), Some(Role::Select));
        assert_eq!(b.key_role(Key::Space), Some(Role::Select));
        assert_eq!(b.key_role(Key::Escape), Some(Role::Back));
        assert_eq!(b.key_role(Key::Backspace), Some(Role::Back));
        assert_eq!(b.key_role(Key::O), Some(Role::Options));
        assert_eq!(b.key_role(Key::Q), None);
        assert!(b.is_default());
    }

    #[test]
    fn a_free_button_just_moves_the_job() {
        let mut b = Bindings::defaults();
        assert_eq!(b.assign_pad(Role::Select, "West"), Ok(None));
        assert_eq!(b.pad_buttons(Role::Select), ["West"]);
        assert_eq!(b.pad_role(Button::South), None);
        assert!(!b.is_default());
    }

    #[test]
    fn a_taken_button_swaps_so_no_job_is_left_empty() {
        let mut b = Bindings::defaults();
        // Cross goes to Back: Select, left with nothing, takes Circle.
        let swap = b.assign_pad(Role::Back, "South").unwrap().unwrap();
        assert_eq!(swap.other, Role::Select);
        assert_eq!(swap.took, "South");
        assert_eq!(swap.gave.as_deref(), Some("East"));
        assert_eq!(b.pad_buttons(Role::Back), ["South"]);
        assert_eq!(b.pad_buttons(Role::Select), ["East"]);
        // Every job still has a button.
        for role in Role::ALL {
            assert!(!b.pad_buttons(role).is_empty(), "{role:?}");
        }
    }

    #[test]
    fn a_job_with_a_second_button_keeps_it_without_a_gift() {
        let mut b = Bindings::defaults();
        let swap = b.assign_pad(Role::Select, "Start").unwrap().unwrap();
        assert_eq!(swap.other, Role::Options);
        assert_eq!(swap.gave, None);
        assert_eq!(b.pad_buttons(Role::Options), ["North"]);
    }

    #[test]
    fn the_directions_home_and_create_cannot_be_bound() {
        let mut b = Bindings::defaults();
        for fixed in ["Up", "Down", "Left", "Right", "Home", "Create"] {
            assert_eq!(
                b.assign_pad(Role::Select, fixed),
                Err(Refused::Fixed),
                "{fixed}"
            );
        }
        assert_eq!(b.assign_pad(Role::Select, "Nope"), Err(Refused::Unknown));
        assert_eq!(
            b.assign_key(Role::Select, Key::ArrowUp),
            Err(Refused::Fixed)
        );
        assert_eq!(b.assign_key(Role::Select, Key::Tab), Err(Refused::Fixed));
        assert_eq!(b.assign_key(Role::Select, Key::F11), Err(Refused::Fixed));
        assert!(b.is_default());
    }

    #[test]
    fn keys_swap_the_same_way() {
        let mut b = Bindings::defaults();
        let swap = b.assign_key(Role::Back, Key::O).unwrap().unwrap();
        assert_eq!(swap.other, Role::Options);
        assert_eq!(swap.gave.as_deref(), Some("Escape"));
        assert_eq!(b.key_role(Key::O), Some(Role::Back));
        assert_eq!(b.key_role(Key::Escape), Some(Role::Options));
        // Backspace belonged to Back and went with the new single key.
        assert_eq!(b.key_role(Key::Backspace), None);
    }

    #[test]
    fn a_shoulder_job_can_get_a_key_and_loses_nothing_it_never_had() {
        let mut b = Bindings::defaults();
        assert_eq!(b.assign_key(Role::Next, Key::E), Ok(None));
        assert_eq!(b.key_role(Key::E), Some(Role::Next));
        // Taking a key from a job that had none of its own gives it nothing to fall back on.
        let swap = b.assign_key(Role::Previous, Key::E).unwrap().unwrap();
        assert_eq!(swap.other, Role::Next);
        assert_eq!(swap.gave, None);
        assert!(b.keys(Role::Next).is_empty());
    }

    #[test]
    fn only_the_differences_are_stored_and_they_come_back() {
        let mut b = Bindings::defaults();
        let mut prefs = Preferences::default();
        b.store(&mut prefs);
        assert!(prefs.nav_pad.is_empty() && prefs.nav_keys.is_empty());
        b.assign_pad(Role::Select, "West").unwrap();
        b.assign_key(Role::Options, Key::P).unwrap();
        b.store(&mut prefs);
        assert_eq!(prefs.nav_pad.len(), 1, "{:?}", prefs.nav_pad);
        assert_eq!(prefs.nav_keys.len(), 1, "{:?}", prefs.nav_keys);
        assert_eq!(Bindings::from_prefs(&prefs), b);
        b.reset();
        b.store(&mut prefs);
        assert!(prefs.nav_pad.is_empty() && prefs.nav_keys.is_empty());
    }

    #[test]
    fn names_from_a_newer_version_are_ignored() {
        let mut prefs = Preferences::default();
        prefs
            .nav_pad
            .insert("select".into(), vec!["Mystery".into(), "Up".into()]);
        prefs.nav_pad.insert("teleport".into(), vec!["West".into()]);
        prefs
            .nav_keys
            .insert("back".into(), vec!["NoSuchKey".into()]);
        prefs.nav_keys.insert("select".into(), vec!["Tab".into()]);
        assert!(Bindings::from_prefs(&prefs).is_default());
    }

    #[test]
    fn the_dead_zone_is_kept_between_its_limits() {
        let with = |pad_deadzone| Preferences {
            pad_deadzone,
            ..Preferences::default()
        };
        apply(&with(200));
        assert!((deadzone() - 0.5).abs() < 1e-6);
        apply(&with(0));
        assert!((deadzone() - 0.05).abs() < 1e-6);
        apply(&with(DEFAULT_PAD_DEADZONE));
        assert!((deadzone() - 0.25).abs() < 1e-6);
    }
}
