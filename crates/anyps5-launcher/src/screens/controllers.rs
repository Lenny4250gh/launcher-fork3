//! The Controllers page (docs/DESIGN.md 6.10): a full-screen page of Settings with every
//! connected pad (name, connection, battery, backend), a live button test, a vibration test, the
//! stick dead zone, the DualSense light bar, the launcher's own controls with Reset to Default,
//! and a troubleshooter.
//!
//! The live state comes from `ui::pad_hub`, which the pad thread fills. While a binding waits
//! for a button or the button test runs, the pad sends the launcher no commands
//! (`nav_bindings::set_capturing`), so a press only binds or lights. The test is left with Esc,
//! the Back button of the page, or by holding the Back button of the pad for a second. Nothing
//! is written to a pad while a game runs: the vibration test and the light bar are refused then.

use anyps5_core::settings::Preferences;
use egui::{Align2, Color32, Context, Sense, Ui, pos2, vec2};

use crate::screens::ToastRequest;
use crate::screens::controllers_strings::Pads;
use crate::screens::rows::{self, Change, Row};
use crate::screens::side::{self, Output, SideState, SideView};
use crate::ui::Shell;
use crate::ui::chrome::Track;
use crate::ui::components::corner;
use crate::ui::focus::PanelNav;
use crate::ui::i18n::{Arg, Language};
use crate::ui::icons::Icon;
use crate::ui::input::InputEvent;
use crate::ui::lightbar;
use crate::ui::nav_bindings::{self, Bindings, MAX_ZONE, MIN_ZONE, Refused, Role};
use crate::ui::pad_hub::{self, Link, PadInfo, Snapshot};
use crate::ui::pad_info::{Battery, Family};
use crate::ui::sound::Cue;
use crate::ui::theme::{self, color, white};

/// What the host has to do after a frame.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ControllersAction {
    /// Back: the page is gone; Settings is next.
    Back,
    Toast(ToastRequest),
}

/// What the page needs from the host for one frame.
pub struct ControllersData<'a> {
    pub prefs: &'a mut Preferences,
    pub language: Language,
    pub accent: Color32,
    /// A game runs: the pad is not ours to write to.
    pub game_running: bool,
}

/// The right face button (Circle on a PlayStation pad) confirms and the bottom one goes back.
/// Settings and the first start both read it here.
pub fn confirm_is_circle(prefs: &Preferences) -> bool {
    Bindings::from_prefs(prefs).pad_buttons(Role::Select) == ["East"]
}

/// Chooses which face button confirms: the right one (`true`) or the bottom one. The two swap
/// jobs, as in the Controllers page.
pub fn set_confirm_circle(prefs: &mut Preferences, circle: bool) {
    let mut bindings = Bindings::from_prefs(prefs);
    let _ = bindings.assign_pad(Role::Select, if circle { "East" } else { "South" });
    bindings.store(prefs);
}

/// How long a binding waits for a button, in seconds.
const CAPTURE_WAIT: f64 = 10.0;
/// How long the Back button of the pad is held to leave the test, in seconds.
const HOLD_TO_LEAVE: f64 = 1.0;
/// Height of the pad drawing of the test.
const DIAGRAM_H: f32 = 270.0;
/// A battery at or below this many percent is low.
const LOW_BATTERY: u8 = 20;

#[derive(Default)]
pub struct ControllersPanel {
    side: SideState,
    test: bool,
    settled: bool,
    capturing: Option<Role>,
    capture_seq: u64,
    capture_until: f64,
    hold_since: Option<f64>,
    /// The page has taken the pad and the keyboard's capture flag and owes them back.
    engaged: bool,
}

impl ControllersPanel {
    pub fn open(&mut self) {
        self.side.open_at(0);
        self.test = false;
        self.settled = false;
        self.capturing = None;
        self.hold_since = None;
    }

    pub fn close(&mut self, shell: &mut Shell) {
        self.release(shell);
        self.side.close();
    }

    pub fn is_active(&self) -> bool {
        self.side.is_active()
    }

    /// Gives the keyboard and the pad back to the launcher.
    fn release(&mut self, shell: &mut Shell) {
        self.engaged = false;
        self.test = false;
        self.capturing = None;
        self.hold_since = None;
        nav_bindings::set_capturing(false);
        pad_hub::set_watched(false);
        shell.input.capture_keys = false;
    }

    /// Previews: opens with the focus on a row.
    pub fn open_row(&mut self, row: usize) {
        self.open();
        self.side.open_at(row);
        self.settled = true;
    }

    /// Previews: opens the live test.
    pub fn open_test(&mut self) {
        self.open();
        self.test = true;
    }

    /// Previews: opens with a binding waiting.
    pub fn open_capture(&mut self, role: Role) {
        self.open();
        self.capturing = Some(role);
        self.capture_until = f64::MAX;
    }

    pub fn show(
        &mut self,
        shell: &mut Shell,
        ctx: &Context,
        events: &[InputEvent],
        data: &mut ControllersData<'_>,
    ) -> Output<ControllersAction> {
        let mut out = Output::default();
        if !self.side.is_active() {
            if self.engaged {
                self.release(shell);
            }
            return out;
        }
        let open = self.side.is_open();
        if !open && self.engaged {
            self.release(shell);
        }
        let snap = pad_hub::snapshot();
        let now = ctx.input(|i| i.time);
        let mut progress = 0.0;
        if open {
            self.engaged = true;
            pad_hub::set_watched(true);
            if self.test {
                progress = self.step_test(shell, ctx, events, &snap, now, data);
            } else {
                self.step_page(shell, ctx, events, &snap, now, data, &mut out);
            }
        }
        let rows_now = build_rows(self, data, &snap);
        if open && !self.settled {
            self.settled = true;
            self.side.nav.row = PanelNav::first_focusable(&rows::kinds(&rows_now));
        }
        let mut rows = rows_now;
        let title = if self.test {
            Pads::TestTitle.text(data.language)
        } else {
            Pads::Title.text(data.language)
        };
        let subtitle = (!self.test).then(|| Pads::Subtitle.text(data.language));
        let view = SideView {
            title,
            subtitle,
            question: false,
            list_id: "controllers-rows",
            nav: &self.side.nav,
            accent: data.accent,
            language: data.language,
            force: None,
        };
        let test = self.test;
        let back_label = Bindings::from_prefs(data.prefs)
            .pad_buttons(Role::Back)
            .first()
            .map_or("Circle", |n| button_label(pad_info_family(), n));
        let zone = nav_bindings::deadzone();
        let accent = data.accent;
        let language = data.language;
        match side::draw_with_header(ctx, open, &view, &mut rows, |ui| {
            if test {
                draw_test(ui, &snap, zone, accent, language, progress, back_label);
            }
        }) {
            Some(drawn) if open => {
                let (changes, _) =
                    rows::apply_mouse(shell, &mut self.side.nav, &rows, drawn.events);
                for change in &changes {
                    self.apply(shell, &snap, now, data, change, &mut out);
                }
                if drawn.back_clicked {
                    shell.play(Cue::Close);
                    if self.test {
                        self.leave_test(shell);
                    } else {
                        self.release(shell);
                        self.side.close();
                        out.actions.push(ControllersAction::Back);
                    }
                }
            }
            Some(_) => {}
            None => self.side.finished(),
        }
        out
    }

    fn leave_test(&mut self, shell: &mut Shell) {
        self.test = false;
        self.settled = false;
        self.hold_since = None;
        nav_bindings::set_capturing(false);
        shell.input.capture_keys = false;
    }

    /// The button test: the pad only lights the drawing. Returns how far the hold to leave is.
    fn step_test(
        &mut self,
        shell: &mut Shell,
        ctx: &Context,
        events: &[InputEvent],
        snap: &Snapshot,
        now: f64,
        data: &ControllersData<'_>,
    ) -> f32 {
        nav_bindings::set_capturing(true);
        shell.input.capture_keys = true;
        for event in events {
            if let InputEvent::CapturedKey {
                key: egui::Key::Escape,
                ..
            } = event
            {
                shell.play(Cue::Back);
                self.leave_test(shell);
                return 0.0;
            }
        }
        let bindings = Bindings::from_prefs(data.prefs);
        let held = bindings
            .pad_buttons(Role::Back)
            .first()
            .is_some_and(|name| snap.live.is_down(name));
        if held {
            let since = *self.hold_since.get_or_insert(now);
            ctx.request_repaint();
            let progress = ((now - since) / HOLD_TO_LEAVE) as f32;
            if progress >= 1.0 {
                shell.play(Cue::Back);
                self.leave_test(shell);
                return 0.0;
            }
            return progress;
        }
        self.hold_since = None;
        0.0
    }

    #[allow(clippy::too_many_arguments)]
    fn step_page(
        &mut self,
        shell: &mut Shell,
        ctx: &Context,
        events: &[InputEvent],
        snap: &Snapshot,
        now: f64,
        data: &mut ControllersData<'_>,
        out: &mut Output<ControllersAction>,
    ) {
        if let Some(role) = self.capturing {
            nav_bindings::set_capturing(true);
            shell.input.capture_keys = true;
            ctx.request_repaint();
            let mut bindings = Bindings::from_prefs(data.prefs);
            let family = pad_info_family();
            let mut result = None;
            if snap.press_seq != self.capture_seq
                && let Some(name) = snap.last_press
            {
                result = Some(
                    bindings
                        .assign_pad(role, name)
                        .map(|swap| (swap, name.to_owned(), true)),
                );
            }
            for event in events {
                if let InputEvent::CapturedKey {
                    key, physical_key, ..
                } = event
                {
                    let key = physical_key.unwrap_or(*key);
                    if key == egui::Key::Escape {
                        shell.play(Cue::Back);
                        self.end_capture(shell);
                        return;
                    }
                    result = Some(
                        bindings
                            .assign_key(role, key)
                            .map(|swap| (swap, key.name().to_owned(), false)),
                    );
                    break;
                }
            }
            if result.is_none() && now >= self.capture_until {
                self.end_capture(shell);
                return;
            }
            let Some(result) = result else {
                return;
            };
            match result {
                Ok((swap, name, pad)) => {
                    bindings.store(data.prefs);
                    nav_bindings::apply(data.prefs);
                    out.changed = true;
                    shell.play(Cue::Select);
                    if let Some(swap) = swap {
                        let label = |n: &str| {
                            if pad {
                                button_label(family, n).to_owned()
                            } else {
                                n.to_owned()
                            }
                        };
                        let other = role_title(swap.other, data.language);
                        let text = match &swap.gave {
                            Some(gave) => Pads::Swapped.format(
                                data.language,
                                &[
                                    Arg::Str(&label(&swap.took)),
                                    Arg::Str(other),
                                    Arg::Str(other),
                                    Arg::Str(&label(gave)),
                                ],
                            ),
                            None => Pads::SwappedNoReturn
                                .format(data.language, &[Arg::Str(&label(&name)), Arg::Str(other)]),
                        };
                        out.actions.push(ControllersAction::Toast(ToastRequest::new(
                            Icon::GAME_CONTROLLER,
                            text,
                        )));
                    }
                }
                Err(Refused::Fixed | Refused::Unknown) => {
                    shell.play(Cue::Error);
                    out.actions
                        .push(ControllersAction::Toast(ToastRequest::warning(
                            Pads::FixedButton.text(data.language),
                        )));
                }
            }
            self.end_capture(shell);
            return;
        }
        nav_bindings::set_capturing(false);
        shell.input.capture_keys = false;
        for event in events {
            let InputEvent::Command(command) = event else {
                continue;
            };
            let rows = build_rows(self, data, snap);
            let driven = rows::drive(shell, &mut self.side.nav, &rows, &[*command]);
            for change in &driven.changes {
                self.apply(shell, snap, now, data, change, out);
            }
            if driven.close {
                self.release(shell);
                self.side.close();
                out.actions.push(ControllersAction::Back);
                return;
            }
        }
    }

    fn end_capture(&mut self, shell: &mut Shell) {
        self.capturing = None;
        nav_bindings::set_capturing(false);
        shell.input.capture_keys = false;
    }

    fn apply(
        &mut self,
        shell: &mut Shell,
        snap: &Snapshot,
        now: f64,
        data: &mut ControllersData<'_>,
        change: &Change,
        out: &mut Output<ControllersAction>,
    ) {
        let language = data.language;
        let warn = |out: &mut Output<ControllersAction>, text: &str| {
            out.actions
                .push(ControllersAction::Toast(ToastRequest::warning(text)));
        };
        match change {
            Change::Run { row: "testButtons" } => {
                self.test = true;
                self.settled = false;
                self.hold_since = None;
            }
            Change::Run {
                row: "testVibration",
            } => {
                if data.game_running {
                    warn(out, Pads::NotWhileGame.text(language));
                } else if !snap.pads.iter().any(|p| p.rumble) {
                    warn(out, Pads::NoRumble.text(language));
                } else {
                    shell.haptics.test();
                    out.actions.push(ControllersAction::Toast(ToastRequest::new(
                        Icon::WAVE_SINE,
                        Pads::VibrationSent.text(language),
                    )));
                }
            }
            Change::Stepper {
                row: "deadzone",
                value,
            } => {
                data.prefs.pad_deadzone =
                    (*value).clamp(i32::from(MIN_ZONE), i32::from(MAX_ZONE)) as u8;
                nav_bindings::apply(data.prefs);
                out.changed = true;
            }
            Change::Choice {
                row: "lightbar",
                index,
            } => {
                if data.game_running {
                    warn(out, Pads::NotWhileGame.text(language));
                } else {
                    if let Some((_, rgb)) = lightbar::COLOURS.get(*index) {
                        set_lightbar(data, *rgb, out);
                    } else if lightbar::is_preset(data.prefs.lightbar.as_deref()) {
                        // Custom: it starts from the colour the bar has now, as pure hue and
                        // brightness. A custom colour already chosen stays as it is.
                        let (hue, brightness) = lightbar::custom_of(light_now(data.prefs));
                        set_lightbar(data, lightbar::custom(hue, brightness), out);
                    }
                    out.actions.push(ControllersAction::Toast(ToastRequest::new(
                        Icon::GAME_CONTROLLER,
                        Pads::LightBarSet.text(language),
                    )));
                }
            }
            Change::Stepper {
                row: row @ ("lightbarHue" | "lightbarBrightness"),
                value,
            } => {
                if data.game_running {
                    warn(out, Pads::NotWhileGame.text(language));
                } else {
                    let (mut hue, mut brightness) = lightbar::custom_of(light_now(data.prefs));
                    if *row == "lightbarHue" {
                        hue = (*value).clamp(0, i32::from(lightbar::HUE_MAX)) as u16;
                    } else {
                        brightness = (*value).clamp(10, 100) as u8;
                    }
                    // The bar is the answer: no toast for every step.
                    set_lightbar(data, lightbar::custom(hue, brightness), out);
                }
            }
            Change::Toggle {
                row: "confirmCircle",
                on,
            } => {
                set_confirm_circle(data.prefs, *on);
                nav_bindings::apply(data.prefs);
                out.changed = true;
            }
            Change::Run {
                row: "resetControls",
            } => {
                let mut bindings = Bindings::from_prefs(data.prefs);
                if !bindings.is_default() {
                    bindings.reset();
                    bindings.store(data.prefs);
                    nav_bindings::apply(data.prefs);
                    out.changed = true;
                }
            }
            Change::Capture { row } => {
                if let Some(role) = role_of_row(row) {
                    self.capturing = Some(role);
                    self.capture_seq = snap.press_seq;
                    self.capture_until = now + CAPTURE_WAIT;
                }
            }
            _ => {}
        }
    }
}

fn pad_info_family() -> Family {
    crate::ui::pad_info::family()
}

// ---------------------------------------------------------------------------------------------
// Names
// ---------------------------------------------------------------------------------------------

fn role_title(role: Role, language: Language) -> &'static str {
    match role {
        Role::Select => Pads::Select,
        Role::Back => Pads::Back,
        Role::Options => Pads::Options,
        Role::Previous => Pads::Previous,
        Role::Next => Pads::Next,
    }
    .text(language)
}

const ROLE_ROWS: [(&str, Role); 5] = [
    ("navSelect", Role::Select),
    ("navBack", Role::Back),
    ("navOptions", Role::Options),
    ("navPrevious", Role::Previous),
    ("navNext", Role::Next),
];

fn role_of_row(row: &str) -> Option<Role> {
    ROLE_ROWS.iter().find(|(id, _)| *id == row).map(|(_, r)| *r)
}

/// What a button is called on this family of pad.
pub fn button_label(family: Family, name: &str) -> &'static str {
    let (ps, xbox, nintendo): (&'static str, &'static str, &'static str) = match name {
        "South" => ("Cross", "A", "B"),
        "East" => ("Circle", "B", "A"),
        "West" => ("Square", "X", "Y"),
        "North" => ("Triangle", "Y", "X"),
        "L1" => ("L1", "LB", "L"),
        "R1" => ("R1", "RB", "R"),
        "L2" => ("L2", "LT", "ZL"),
        "R2" => ("R2", "RT", "ZR"),
        "Start" => ("Options", "Menu", "+"),
        "Create" => ("Create", "View", "-"),
        "Home" => ("PS", "Guide", "Home"),
        "L3" => ("L3", "L3", "L3"),
        "R3" => ("R3", "R3", "R3"),
        "Up" => ("↑", "↑", "↑"),
        "Down" => ("↓", "↓", "↓"),
        "Left" => ("←", "←", "←"),
        "Right" => ("→", "→", "→"),
        _ => ("?", "?", "?"),
    };
    match family {
        Family::PlayStation => ps,
        Family::Nintendo => nintendo,
        Family::Xbox | Family::Other => xbox,
    }
}

fn link_text(link: Link, language: Language) -> &'static str {
    match link {
        Link::Usb => Pads::Usb,
        Link::Bluetooth => Pads::Bluetooth,
        Link::Unknown => Pads::LinkUnknown,
    }
    .text(language)
}

fn battery_text(battery: Battery, language: Language) -> String {
    match battery {
        Battery::Unknown => Pads::BatteryNone.text(language).to_owned(),
        Battery::Level {
            percent,
            charging: true,
        } => Pads::BatteryCharging.format(language, &[Arg::Int(i64::from(percent))]),
        Battery::Level { percent, .. } => {
            Pads::Battery.format(language, &[Arg::Int(i64::from(percent))])
        }
    }
}

fn pad_detail(pad: &PadInfo, language: Language) -> String {
    Pads::PadDetail.format(
        language,
        &[
            Arg::Str(link_text(pad.link, language)),
            Arg::Str(&battery_text(pad.battery, language)),
            Arg::Str(&pad.backend),
        ],
    )
}

// ---------------------------------------------------------------------------------------------
// The rows
// ---------------------------------------------------------------------------------------------

/// The lowest battery of a pad that is not charging.
fn low_battery(snap: &Snapshot) -> Option<u8> {
    snap.pads
        .iter()
        .filter_map(|p| match p.battery {
            Battery::Level {
                percent,
                charging: false,
            } if percent <= LOW_BATTERY => Some(percent),
            _ => None,
        })
        .min()
}

/// Two pads at once, one of them the virtual Xbox pad (045E:028E) tools like Steam Input,
/// DS4Windows and the like present, or a Sony pad next to any second pad.
fn doubled(snap: &Snapshot) -> bool {
    let virtual_xbox = snap
        .pads
        .iter()
        .any(|p| p.vendor == Some(0x045E) && p.product == Some(0x028E));
    let sony = snap.pads.iter().any(|p| p.vendor == Some(0x054C));
    snap.pads.len() >= 2 && (virtual_xbox || sony)
}

/// The colour the light bar is meant to have: the one the preferences hold, or the pad's own blue.
fn light_now(prefs: &Preferences) -> [u8; 3] {
    prefs
        .lightbar
        .as_deref()
        .and_then(lightbar::parse)
        .unwrap_or(lightbar::COLOURS[0].1)
}

/// Keeps `rgb` as the light bar colour and puts it on the pad. The write is a few milliseconds of
/// HID I/O, and a drag asks for many: `lightbar::set_soon` does it off the UI thread and keeps only
/// the latest. The caller has checked that no game runs.
fn set_lightbar(data: &mut ControllersData<'_>, rgb: [u8; 3], out: &mut Output<ControllersAction>) {
    data.prefs.lightbar = Some(lightbar::hex(rgb));
    out.changed = true;
    lightbar::set_soon(rgb);
}

fn build_rows(panel: &ControllersPanel, data: &ControllersData<'_>, snap: &Snapshot) -> Vec<Row> {
    let language = data.language;
    if panel.test {
        let text = if snap.pads.is_empty() {
            Pads::TestNoPad
        } else {
            Pads::TestWaiting
        };
        return vec![Row::info("testHint", text.text(language), None)];
    }
    const PAD_ROWS: [&str; 4] = ["pad0", "pad1", "pad2", "pad3"];
    let bindings = Bindings::from_prefs(data.prefs);
    let family = pad_info_family();
    let mut rows = vec![Row::caption(
        "padsCaption",
        Pads::CaptionConnected.text(language),
    )];
    if snap.pads.is_empty() {
        rows.push(
            Row::info("noPad", Pads::NoPad.text(language), None)
                .detail(Pads::NoPadDetail.text(language))
                .icon(Icon::GAME_CONTROLLER),
        );
    }
    for (pad, id) in snap.pads.iter().zip(PAD_ROWS) {
        rows.push(
            Row::info(
                id,
                pad.name.clone(),
                pad.active.then(|| Pads::InUse.text(language).to_owned()),
            )
            .detail(pad_detail(pad, language))
            .icon(Icon::GAME_CONTROLLER),
        );
    }

    rows.push(Row::caption("testCaption", Pads::CaptionTest.text(language)).group_start());
    rows.push(
        Row::action("testButtons", Pads::TestButtons.text(language))
            .detail(Pads::TestButtonsDetail.text(language))
            .icon(Icon::GAME_CONTROLLER),
    );
    let vibration_detail = if data.game_running {
        Pads::NotWhileGame.text(language)
    } else {
        Pads::TestVibrationDetail.text(language)
    };
    rows.push(
        Row::action("testVibration", Pads::TestVibration.text(language))
            .detail(vibration_detail)
            .icon(Icon::WAVE_SINE),
    );
    let zone = i32::from(
        data.prefs
            .pad_deadzone
            .clamp(nav_bindings::MIN_ZONE, nav_bindings::MAX_ZONE),
    );
    rows.push(
        Row::stepper(
            "deadzone",
            Pads::DeadZone.text(language),
            zone,
            (i32::from(MIN_ZONE), i32::from(MAX_ZONE)),
            5,
            format!("{zone}%"),
        )
        .detail(Pads::DeadZoneDetail.text(language))
        .icon(Icon::SLIDERS_HORIZONTAL),
    );
    if snap.pads.iter().any(|p| p.dualsense) {
        let names: Vec<String> = [
            Pads::ColourBlue,
            Pads::ColourRed,
            Pads::ColourGreen,
            Pads::ColourPurple,
            Pads::ColourOrange,
            Pads::ColourWhite,
            Pads::ColourOff,
            Pads::ColourCustom,
        ]
        .iter()
        .map(|p| p.text(language).to_owned())
        .collect();
        let detail = if data.game_running {
            Pads::NotWhileGame.text(language)
        } else {
            Pads::LightBarDetail.text(language)
        };
        rows.push(
            Row::choice(
                "lightbar",
                Pads::LightBar.text(language),
                names,
                lightbar::index_of(data.prefs.lightbar.as_deref()),
            )
            .detail(detail)
            .icon(Icon::WAVE_SINE),
        );
        if !lightbar::is_preset(data.prefs.lightbar.as_deref()) {
            let (hue, brightness) = lightbar::custom_of(light_now(data.prefs));
            let percent = match language {
                Language::Turkish => format!("%{brightness}"),
                Language::German | Language::Spanish | Language::French => {
                    format!("{brightness} %")
                }
                Language::Portuguese => format!("{brightness}%"),
                Language::English => format!("{brightness}%"),
            };
            rows.push(
                Row::slider(
                    "lightbarHue",
                    Pads::LightBarHue.text(language),
                    i32::from(hue),
                    (0, i32::from(lightbar::HUE_MAX)),
                    10,
                    format!("{hue}\u{b0}"),
                )
                .track(Track::Hue(lightbar::HUE_MAX))
                .icon(Icon::PALETTE),
            );
            rows.push(
                Row::slider(
                    "lightbarBrightness",
                    Pads::LightBarBrightness.text(language),
                    i32::from(brightness),
                    (10, 100),
                    10,
                    percent,
                )
                .track(Track::Ramp(lightbar::custom(hue, 100)))
                .icon(Icon::CIRCLE_HALF),
            );
        }
    }

    rows.push(Row::caption("controlsCaption", Pads::CaptionControls.text(language)).group_start());
    let circle = confirm_is_circle(data.prefs);
    let confirm = button_label(family, "East");
    rows.push(
        Row::toggle(
            "confirmCircle",
            Pads::ConfirmWith.format(language, &[Arg::Str(confirm)]),
            circle,
        )
        .detail(Pads::ConfirmWithDetail.text(language))
        .icon(Icon::GAME_CONTROLLER),
    );
    rows.push(Row::info("controlsNote", Pads::ControlsNote.text(language), None).attached());
    for (id, role) in ROLE_ROWS {
        let mut sources: Vec<String> = bindings
            .keys(role)
            .iter()
            .map(|k| format!("KEY:{k}"))
            .collect();
        sources.extend(
            bindings
                .pad_buttons(role)
                .iter()
                .map(|b| format!("PAD:{}", button_label(family, b))),
        );
        rows.push(Row::keys(
            id,
            role_title(role, language),
            sources,
            panel.capturing == Some(role),
        ));
    }
    rows.push(
        Row::command("resetControls", Pads::Reset.text(language))
            .detail(Pads::ResetDetail.text(language))
            .icon(Icon::ARROWS_CLOCKWISE),
    );

    rows.push(Row::caption("helpCaption", Pads::CaptionHelp.text(language)).group_start());
    let status = |bad: bool| {
        if bad {
            Pads::StatusCheck.text(language).to_owned()
        } else {
            Pads::StatusOk.text(language).to_owned()
        }
    };
    let none = snap.pads.is_empty();
    rows.push(
        Row::info(
            "helpMissing",
            Pads::HelpMissingTitle.text(language),
            Some(status(none)),
        )
        .detail(
            if none {
                Pads::HelpMissingNone
            } else {
                Pads::HelpMissingFound
            }
            .text(language),
        ),
    );
    rows.push(
        Row::info(
            "helpDouble",
            Pads::HelpDoubleTitle.text(language),
            Some(status(doubled(snap))),
        )
        .detail(Pads::HelpDouble.text(language)),
    );
    rows.push(
        Row::info("helpSteam", Pads::HelpSteamTitle.text(language), None)
            .detail(Pads::HelpSteam.text(language)),
    );
    rows.push(
        Row::info("helpHide", Pads::HelpHideTitle.text(language), None)
            .detail(Pads::HelpHide.text(language)),
    );
    rows.push(
        Row::info("helpHome", Pads::HelpHomeTitle.text(language), None).detail(
            if cfg!(target_os = "linux") {
                Pads::HelpHomeLinux
            } else {
                Pads::HelpHome
            }
            .text(language),
        ),
    );
    if cfg!(target_os = "linux") {
        rows.push(
            Row::info("helpLinux", Pads::HelpLinuxTitle.text(language), None)
                .detail(Pads::HelpLinux.text(language)),
        );
    }
    let low = low_battery(snap);
    rows.push(
        Row::info(
            "helpBattery",
            Pads::HelpBatteryTitle.text(language),
            Some(match low {
                Some(p) => Pads::LowBattery.format(language, &[Arg::Int(i64::from(p))]),
                None => Pads::StatusOk.text(language).to_owned(),
            }),
        )
        .detail(Pads::HelpBattery.text(language)),
    );
    rows
}

// ---------------------------------------------------------------------------------------------
// The drawing
// ---------------------------------------------------------------------------------------------

/// The pad drawing of the test, with the numbers under it.
fn draw_test(
    ui: &mut Ui,
    snap: &Snapshot,
    zone: f32,
    accent: Color32,
    language: Language,
    progress: f32,
    back: &str,
) {
    let width = ui.available_width().min(640.0);
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), DIAGRAM_H), Sense::hover());
    let painter = ui.painter().clone();
    let c = pos2(rect.center().x, rect.min.y + 130.0);
    let scale = (width / 560.0).min(1.0);
    let at = |x: f32, y: f32| pos2(c.x + x * scale, c.y + y * scale);
    let live = &snap.live;
    let idle = white(0.16);
    let lit = |name: &str| if live.is_down(name) { accent } else { idle };
    let ink = color::text_detail();
    let label = |pos: egui::Pos2, text: &str| {
        painter.text(
            pos,
            Align2::CENTER_CENTER,
            text,
            theme::font_mono(11.0 * scale.max(0.8)),
            ink,
        );
    };
    let r = 15.0 * scale;
    // Face buttons: the diamond on the right.
    for (name, x, y, glyph) in [
        ("South", 150.0, 8.0, "✕"),
        ("East", 180.0, -22.0, "○"),
        ("West", 120.0, -22.0, "□"),
        ("North", 150.0, -52.0, "△"),
    ] {
        painter.circle_filled(at(x, y), r, lit(name));
        label(at(x, y), glyph);
    }
    // The D-pad on the left.
    for (name, x, y) in [
        ("Up", -150.0, -52.0),
        ("Down", -150.0, 8.0),
        ("Left", -180.0, -22.0),
        ("Right", -120.0, -22.0),
    ] {
        painter.circle_filled(at(x, y), r * 0.8, lit(name));
    }
    // Create, Options and Home.
    for (name, x, y, text) in [
        ("Create", -48.0, -48.0, "C"),
        ("Start", 48.0, -48.0, "O"),
        ("Home", 0.0, 18.0, "PS"),
    ] {
        painter.circle_filled(at(x, y), r * 0.6, lit(name));
        label(at(x, y), text);
    }
    // Sticks, with the dead zone as a ring and the lean as a dot.
    for (name, x, y, value) in [
        ("L3", -70.0, 62.0, live.left),
        ("R3", 70.0, 62.0, live.right),
    ] {
        let centre = at(x, y);
        let radius = 38.0 * scale;
        painter.circle_filled(
            centre,
            radius,
            if live.is_down(name) {
                accent
            } else {
                white(0.07)
            },
        );
        painter.circle_stroke(
            centre,
            radius * zone * 2.0,
            egui::Stroke::new(1.0, white(0.35)),
        );
        painter.circle_stroke(centre, radius, egui::Stroke::new(1.0, white(0.25)));
        let dot = pos2(
            centre.x + value.0 * radius * 0.8,
            centre.y - value.1 * radius * 0.8,
        );
        painter.circle_filled(dot, 7.0 * scale, color::text());
    }
    // Shoulders and triggers.
    for (side, sign) in [(0, -1.0_f32), (1, 1.0)] {
        let (bumper, trigger, pull) = if side == 0 {
            ("L1", "L2", live.l2)
        } else {
            ("R1", "R2", live.r2)
        };
        let x = sign * 130.0;
        let bar = egui::Rect::from_center_size(at(x, -92.0), vec2(96.0 * scale, 16.0 * scale));
        painter.rect_filled(bar, corner(6.0), lit(bumper));
        label(bar.center(), bumper);
        let well = egui::Rect::from_center_size(at(x, -118.0), vec2(96.0 * scale, 16.0 * scale));
        painter.rect_filled(well, corner(6.0), white(0.07));
        let mut fill = well;
        fill.set_width(
            well.width()
                * if live.is_down(trigger) {
                    pull.max(1.0)
                } else {
                    pull
                },
        );
        painter.rect_filled(fill, corner(6.0), accent);
        label(well.center(), trigger);
    }
    let numbers = format!(
        "L2 {:.2}  R2 {:.2}   LS {:+.2} {:+.2}   RS {:+.2} {:+.2}",
        live.l2, live.r2, live.left.0, live.left.1, live.right.0, live.right.1
    );
    painter.text(
        pos2(rect.center().x, rect.max.y - 40.0),
        Align2::CENTER_CENTER,
        numbers,
        theme::font_mono(13.0),
        color::text_detail(),
    );
    // The hold to leave.
    painter.text(
        pos2(rect.center().x, rect.max.y - 16.0),
        Align2::CENTER_CENTER,
        Pads::TestLeave.format(language, &[Arg::Str(back)]),
        theme::font_mono(12.0),
        color::text_hint(),
    );
    if progress > 0.0 {
        let line = egui::Rect::from_min_size(
            pos2(rect.center().x - 80.0, rect.max.y - 4.0),
            vec2(160.0 * progress.min(1.0), 2.0),
        );
        painter.rect_filled(line, corner(1.0), accent);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::pad_hub::Live;

    #[test]
    fn the_confirm_button_swaps_and_comes_back_without_leaving_a_trace() {
        let mut prefs = Preferences::default();
        assert!(!confirm_is_circle(&prefs));
        set_confirm_circle(&mut prefs, true);
        assert!(confirm_is_circle(&prefs));
        // The two buttons swapped jobs: Cross goes back now.
        assert_eq!(
            Bindings::from_prefs(&prefs).pad_buttons(Role::Back),
            ["South"]
        );
        set_confirm_circle(&mut prefs, false);
        assert!(!confirm_is_circle(&prefs));
        assert!(
            prefs.nav_pad.is_empty(),
            "the defaults are stored as nothing"
        );
    }

    fn pad(name: &str, vendor: u16, product: u16, battery: Battery) -> PadInfo {
        PadInfo {
            id: 0,
            name: name.into(),
            vendor: Some(vendor),
            product: Some(product),
            family: Family::PlayStation,
            link: Link::Usb,
            battery,
            backend: "gilrs".into(),
            active: true,
            dualsense: false,
            rumble: true,
        }
    }

    fn snap(pads: Vec<PadInfo>) -> Snapshot {
        Snapshot {
            pads,
            live: Live::default(),
            press_seq: 0,
            last_press: None,
        }
    }

    #[test]
    fn a_sony_pad_beside_a_virtual_xbox_pad_is_a_double() {
        let real = pad("DualSense", 0x054C, 0x0CE6, Battery::Unknown);
        let fake = pad("Xbox 360", 0x045E, 0x028E, Battery::Unknown);
        assert!(doubled(&snap(vec![real.clone(), fake])));
        assert!(!doubled(&snap(vec![real])));
        assert!(!doubled(&snap(vec![])));
    }

    #[test]
    fn only_a_pad_that_is_not_charging_and_low_counts_as_low() {
        let level = |percent, charging| Battery::Level { percent, charging };
        let low = snap(vec![pad("a", 1, 1, level(15, false))]);
        assert_eq!(low_battery(&low), Some(15));
        assert_eq!(
            low_battery(&snap(vec![pad("a", 1, 1, level(15, true))])),
            None
        );
        assert_eq!(
            low_battery(&snap(vec![pad("a", 1, 1, level(80, false))])),
            None
        );
        assert_eq!(
            low_battery(&snap(vec![pad("a", 1, 1, Battery::Unknown)])),
            None
        );
    }

    #[test]
    fn buttons_are_named_for_the_family() {
        assert_eq!(button_label(Family::PlayStation, "South"), "Cross");
        assert_eq!(button_label(Family::Xbox, "South"), "A");
        assert_eq!(button_label(Family::Nintendo, "South"), "B");
        assert_eq!(button_label(Family::PlayStation, "Start"), "Options");
        for (name, _) in pad_hub::BUTTONS {
            assert_ne!(button_label(Family::PlayStation, name), "?", "{name}");
        }
    }

    #[test]
    fn the_page_has_a_row_for_every_job_and_ids_are_unique() {
        let mut prefs = Preferences::default();
        let data = ControllersData {
            prefs: &mut prefs,
            language: Language::English,
            accent: Color32::WHITE,
            game_running: false,
        };
        let rows = build_rows(&ControllersPanel::default(), &data, &snap(vec![]));
        for (id, _) in ROLE_ROWS {
            assert!(rows.iter().any(|r| r.id == id), "{id}");
        }
        let mut ids: Vec<_> = rows.iter().map(|r| r.id).collect();
        ids.sort_unstable();
        let before = ids.len();
        ids.dedup();
        assert_eq!(before, ids.len(), "row ids must be unique");
    }
}
