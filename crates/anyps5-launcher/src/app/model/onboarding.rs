//! The first start (docs/DESIGN.md 5.15): which steps there are, where the focus sits on the
//! current one, and what a press does.
//!
//! The screen (`screens::onboarding`) only paints this state and reports taps. Every choice the
//! steps make goes through the same functions as the Settings pages (`settings::apply_value`,
//! `controllers::set_confirm_circle`), so there is one place that knows how a preference is
//! written.
//!
//! Focus is a grid: [`Model::onboarding_layout`] lists the rows of the step from top to bottom
//! and the targets in each row from left to right. Left and right move along a row, up and down
//! between rows, Tab walks the grid in reading order. Select does what the target says.

use super::*;
use crate::screens::onboarding::THEMES_PER_ROW;
use crate::screens::rows::Change;
use crate::ui::pad_info::Family;

/// The most rows a step has.
const MAX_ROWS: usize = 5;

/// A step of the first start. Which of them a player meets is [`Model::onboarding_flow`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Step {
    /// Welcome and language.
    #[default]
    Language,
    /// Theme, and the accessibility choices.
    Look,
    /// Only when a controller was found: the button check and the confirm button.
    Controller,
    /// The games folder, what was found in it, and preparing new games.
    Games,
    /// Only with a cloud: signing in, creating an account, or not now.
    Online,
    /// What to know before Home.
    Done,
}

/// Something on a step the focus can sit on and a press can act on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Language(usize),
    Theme(usize),
    LargerText,
    HighContrast,
    ReducedMotion,
    /// 0: the bottom face button confirms (Cross on a PlayStation pad), 1: the right one (Circle).
    Confirm(usize),
    HomeButton,
    Prepare,
    /// The step's one primary action: Continue, Choose Folder, Start.
    Primary,
    /// The games step's second button: Not Now without a folder, Change Folder with one.
    Secondary,
    SignIn,
    CreateAccount,
    NotNow,
    /// The two links of the sentence under the online buttons: they open the page in the browser.
    Terms,
    Privacy,
}

/// The controller the first start found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PadSeen {
    pub name: String,
    pub family: Family,
}

/// Where the first-run setup stands.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Onboarding {
    pub step: Step,
    /// `None`: the step's own starting point.
    row: Option<usize>,
    cols: [Option<usize>; MAX_ROWS],
    /// The first controller seen. Once there is one the controller step stays in the flow, so the
    /// progress does not lose a dot when the pad goes to sleep.
    pad: Option<PadSeen>,
    pad_connected: bool,
    /// The online step's choice, carried out when the first start ends: sign in (`false`) or
    /// create an account (`true`).
    wants_sign_in: Option<bool>,
}

impl Model {
    // --- the flow ----------------------------------------------------------------------------

    /// The steps this player meets, in order. The controller step only exists once a controller
    /// was found, the online step only when there is a cloud to sign in to (release builds name
    /// AnyPlay's; a build without one stays offline until set up).
    pub fn onboarding_flow(&self) -> Vec<Step> {
        let mut flow = vec![Step::Language, Step::Look];
        // Gaming Mode (Steam Deck, gamescope) leaves the buttons and the Home button to Steam.
        if self.onboarding.pad.is_some() && !crate::ui::osk::gaming_mode() {
            flow.push(Step::Controller);
        }
        flow.push(Step::Games);
        if anyps5_core::online::cloud(self.prefs.online_cloud.as_deref()).is_some() {
            flow.push(Step::Online);
        }
        flow.push(Step::Done);
        flow
    }

    pub fn onboarding_steps(&self) -> usize {
        self.onboarding_flow().len()
    }

    /// The place of the current step in [`Model::onboarding_flow`].
    pub fn onboarding_position(&self) -> usize {
        self.onboarding_flow()
            .iter()
            .position(|s| *s == self.onboarding.step)
            .unwrap_or(0)
    }

    /// The launcher runs in Gaming Mode: the Home button and the face buttons are Steam's.
    pub fn onboarding_gaming_mode(&self) -> bool {
        crate::ui::osk::gaming_mode()
    }

    /// The games page leads with choosing a folder (Choose Folder, or Change Folder when the one
    /// chosen holds no games) and then offers going on; otherwise it leads with Continue.
    pub fn onboarding_games_pick_first(&self) -> bool {
        self.prefs.games_folder.is_none() || (self.games.is_empty() && !self.scanning)
    }

    /// The controller the first start found, and whether it is still connected.
    pub fn onboarding_pad(&self) -> Option<(&PadSeen, bool)> {
        self.onboarding
            .pad
            .as_ref()
            .map(|pad| (pad, self.onboarding.pad_connected))
    }

    /// The host tells once per frame which controller is connected (`None`: none).
    pub fn note_pad(&mut self, pad: Option<PadSeen>) {
        self.onboarding.pad_connected = pad.is_some();
        if pad.is_some() {
            self.onboarding.pad = pad;
        }
    }

    /// What the player chose on the online step: `Some(create)` for Sign In or Create Account.
    pub fn onboarding_sign_in(&self) -> Option<bool> {
        self.onboarding.wants_sign_in
    }

    /// The online step's choice, set without walking there (the gallery).
    pub fn onboarding_choose_sign_in(&mut self, create: Option<bool>) {
        self.onboarding.wants_sign_in = create;
    }

    // --- the grid ----------------------------------------------------------------------------

    /// The rows of the current step, top to bottom, each a row of targets left to right.
    pub fn onboarding_layout(&self) -> Vec<Vec<Target>> {
        let primary = vec![Target::Primary];
        match self.onboarding.step {
            Step::Language => vec![
                (0..Language::ALL.len()).map(Target::Language).collect(),
                primary,
            ],
            Step::Look => vec![
                (0..THEMES_PER_ROW).map(Target::Theme).collect(),
                (THEMES_PER_ROW..Theme::ALL.len())
                    .map(Target::Theme)
                    .collect(),
                vec![
                    Target::LargerText,
                    Target::HighContrast,
                    Target::ReducedMotion,
                ],
                primary,
            ],
            Step::Controller => vec![
                vec![Target::Confirm(0), Target::Confirm(1)],
                vec![Target::HomeButton],
                primary,
            ],
            Step::Games if self.prefs.games_folder.is_some() => {
                vec![
                    vec![Target::Prepare],
                    vec![Target::Primary, Target::Secondary],
                ]
            }
            Step::Games => vec![vec![Target::Primary, Target::Secondary]],
            Step::Online => vec![
                vec![Target::Terms, Target::Privacy],
                vec![Target::SignIn, Target::CreateAccount, Target::NotNow],
            ],
            Step::Done => vec![primary],
        }
    }

    /// The row and the place in it where the focus sits.
    pub fn onboarding_focus(&self) -> (usize, usize) {
        let layout = self.onboarding_layout();
        let last = layout.len() - 1;
        let row = self
            .onboarding
            .row
            .unwrap_or(match self.onboarding.step {
                // The choices come first here, so a pad can try them at once; the look cards start
                // on the row of the look that is on now.
                Step::Look => self.current_theme_position() / THEMES_PER_ROW,
                Step::Language | Step::Controller => 0,
                // The step's button is what a player came for.
                _ => last,
            })
            .min(last);
        let col = self.onboarding.cols[row.min(MAX_ROWS - 1)]
            .unwrap_or_else(|| self.starting_col(row))
            .min(layout[row].len() - 1);
        (row, col)
    }

    /// Where the focus starts in a row of choices: on the one that is on now.
    fn starting_col(&self, row: usize) -> usize {
        match (self.onboarding.step, row) {
            (Step::Language, 0) => Language::ALL
                .iter()
                .position(|l| *l == self.language())
                .unwrap_or(0),
            (Step::Look, 0 | 1) => self.current_theme_position() % THEMES_PER_ROW,
            (Step::Controller, 0) => {
                usize::from(crate::screens::controllers::confirm_is_circle(&self.prefs))
            }
            _ => 0,
        }
    }

    /// Where the look that is on now sits among the cards (Game Colors is not one of them).
    fn current_theme_position(&self) -> usize {
        Theme::ALL
            .iter()
            .position(|t| *t == self.theme())
            .unwrap_or(0)
    }

    /// What the focus sits on.
    pub fn onboarding_target(&self) -> Target {
        let (row, col) = self.onboarding_focus();
        self.onboarding_layout()[row][col]
    }

    fn set_onboarding_focus(&mut self, row: usize, col: usize) {
        self.onboarding.row = Some(row);
        self.onboarding.cols[row.min(MAX_ROWS - 1)] = Some(col);
    }

    fn enter_step(&mut self, step: Step) {
        self.onboarding.step = step;
        self.onboarding.row = None;
        self.onboarding.cols = [None; MAX_ROWS];
    }

    // --- input -------------------------------------------------------------------------------

    pub fn onboarding_command(&mut self, command: Command) {
        let (row, col) = self.onboarding_focus();
        let layout = self.onboarding_layout();
        match command {
            Command::Left if col > 0 => self.move_onboarding_focus(row, col - 1),
            Command::Right if col + 1 < layout[row].len() => {
                self.move_onboarding_focus(row, col + 1);
            }
            Command::Up if row > 0 => {
                let col = col.min(layout[row - 1].len() - 1);
                self.move_onboarding_focus(row - 1, col);
            }
            Command::Down if row + 1 < layout.len() => {
                let col = col.min(layout[row + 1].len() - 1);
                self.move_onboarding_focus(row + 1, col);
            }
            Command::Select => self.onboarding_confirm(),
            Command::Back => self.onboarding_back(),
            _ => {}
        }
    }

    /// Tab and Shift+Tab: the next target in reading order, along each row and then the next.
    pub fn onboarding_tab(&mut self, backwards: bool) {
        let layout = self.onboarding_layout();
        let (row, col) = self.onboarding_focus();
        let flat: Vec<(usize, usize)> = layout
            .iter()
            .enumerate()
            .flat_map(|(r, cols)| (0..cols.len()).map(move |c| (r, c)))
            .collect();
        let at = flat.iter().position(|p| *p == (row, col)).unwrap_or(0);
        let next = if backwards {
            at.checked_sub(1)
        } else {
            Some(at + 1).filter(|n| *n < flat.len())
        };
        if let Some((row, col)) = next.map(|n| flat[n]) {
            self.move_onboarding_focus(row, col);
        }
    }

    fn move_onboarding_focus(&mut self, row: usize, col: usize) {
        self.set_onboarding_focus(row, col);
        self.preview(self.onboarding_target());
        self.sound(Cue::Move);
    }

    /// The language and theme choices show at once, as the focus reaches them.
    fn preview(&mut self, target: Target) {
        match target {
            Target::Language(i) => {
                let language = core_language(Language::ALL[i.min(Language::ALL.len() - 1)]);
                self.edit_prefs(|p| p.language = language);
            }
            Target::Theme(i) => {
                let theme = core_theme(Theme::ALL[i.min(Theme::ALL.len() - 1)]);
                self.edit_prefs(|p| p.theme = theme);
            }
            _ => {}
        }
    }

    /// Enter, Space or A on the focus.
    pub fn onboarding_confirm(&mut self) {
        let target = self.onboarding_target();
        self.run_target(target, false);
    }

    /// A tap on something. Language and theme cards only select (the page changes at once);
    /// everything else does what Select does.
    pub fn tap_onboarding(&mut self, target: Target) {
        let found = self
            .onboarding_layout()
            .iter()
            .enumerate()
            .find_map(|(r, cols)| cols.iter().position(|t| *t == target).map(|c| (r, c)));
        if let Some((row, col)) = found {
            self.set_onboarding_focus(row, col);
        }
        self.run_target(target, true);
    }

    fn run_target(&mut self, target: Target, tapped: bool) {
        match target {
            Target::Language(_) | Target::Theme(_) => {
                self.preview(target);
                if tapped {
                    self.sound(Cue::Move);
                } else {
                    self.onboarding_advance();
                }
            }
            Target::LargerText => self.toggle_setting("largerText", self.prefs.larger_text),
            Target::HighContrast => self.toggle_setting("highContrast", self.prefs.high_contrast),
            Target::ReducedMotion => {
                let reduced = !self.prefs.reduced_motion;
                self.edit_prefs(|p| {
                    crate::screens::settings::apply_value(
                        p,
                        &Change::Choice {
                            row: "motion",
                            index: usize::from(reduced),
                        },
                    );
                });
                self.sound(if reduced { Cue::On } else { Cue::Off });
            }
            Target::Confirm(i) => {
                self.edit_prefs(|p| crate::screens::controllers::set_confirm_circle(p, i == 1));
                self.sound(Cue::Select);
            }
            Target::HomeButton => self.toggle_setting("homeButton", self.prefs.ps_button),
            Target::Prepare => self.toggle_setting("prepare", self.prefs.prepare_games),
            Target::Primary => match self.onboarding.step {
                Step::Games if self.onboarding_games_pick_first() => self.request_folder_dialog(),
                Step::Done => self.onboarding_finish(),
                _ => self.onboarding_advance(),
            },
            Target::Secondary => {
                if self.onboarding_games_pick_first() {
                    // Not Now, or Continue past a folder with no games: Home explains how to add some.
                    self.onboarding_advance();
                } else {
                    self.request_folder_dialog();
                }
            }
            Target::Terms => {
                self.effects
                    .push(Effect::OpenPage(crate::screens::legal::Legal::Terms.url()));
            }
            Target::Privacy => {
                self.effects.push(Effect::OpenPage(
                    crate::screens::legal::Legal::Privacy.url(),
                ));
            }
            Target::SignIn | Target::CreateAccount | Target::NotNow => {
                self.onboarding.wants_sign_in = match target {
                    Target::SignIn => Some(false),
                    Target::CreateAccount => Some(true),
                    _ => None,
                };
                self.onboarding_advance();
            }
        }
    }

    /// Flips a switch the Settings pages have too, through their own writer.
    fn toggle_setting(&mut self, row: &'static str, now: bool) {
        self.edit_prefs(|p| {
            crate::screens::settings::apply_value(p, &Change::Toggle { row, on: !now });
        });
        self.sound(if now { Cue::Off } else { Cue::On });
    }

    fn onboarding_advance(&mut self) {
        let flow = self.onboarding_flow();
        let at = self.onboarding_position();
        if let Some(next) = flow.get(at + 1) {
            self.sound(Cue::Select);
            self.enter_step(*next);
        }
    }

    pub fn onboarding_back(&mut self) {
        let at = self.onboarding_position();
        if at == 0 {
            return;
        }
        let step = self.onboarding_flow()[at - 1];
        self.sound(Cue::Back);
        self.enter_step(step);
    }

    /// The games folder was chosen while the games step is open: the focus goes to Continue.
    pub(super) fn onboarding_folder_chosen(&mut self) {
        if !self.onboarded() && self.onboarding.step == Step::Games {
            self.onboarding.row = None;
            self.onboarding.cols = [None; MAX_ROWS];
        }
    }

    /// Start: the first start is over. A sign-in the player asked for on the online step begins
    /// in the online area, where the browser opens as soon as the net service has said hello.
    pub fn onboarding_finish(&mut self) {
        self.sound(Cue::Unlocked);
        self.edit_prefs(|p| p.onboarded = true);
        self.focus = HomeFocus::Tiles;
        if self.games.is_empty() {
            self.rescan();
        }
        if let Some(create) = self.onboarding.wants_sign_in.take() {
            self.sign_in_after_onboarding = Some(create);
            self.open(Overlay::Friends);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::app::model::Effect;

    fn model(cloud: bool) -> Model {
        let prefs = Preferences {
            language: settings::Language::English,
            online_cloud: Some(if cloud {
                "https://cloud.example.org".to_owned()
            } else {
                String::new()
            }),
            ..Preferences::default()
        };
        Model::new(
            prefs,
            AppPaths::under(std::path::Path::new("C:/none")),
            false,
        )
    }

    fn pad() -> PadSeen {
        PadSeen {
            name: "DualSense Wireless Controller".to_owned(),
            family: Family::PlayStation,
        }
    }

    /// Select on every step until the games step.
    fn to_games(m: &mut Model) {
        m.handle(Command::Select);
        m.handle(Command::Select);
        if m.onboarding.step == Step::Controller {
            m.handle(Command::Down);
            m.handle(Command::Down);
            m.handle(Command::Select);
        }
        assert_eq!(m.onboarding.step, Step::Games);
    }

    #[test]
    fn the_flow_has_the_steps_a_player_needs() {
        let mut m = model(false);
        assert_eq!(
            m.onboarding_flow(),
            [Step::Language, Step::Look, Step::Games, Step::Done]
        );
        m.note_pad(Some(pad()));
        assert_eq!(
            m.onboarding_flow(),
            [
                Step::Language,
                Step::Look,
                Step::Controller,
                Step::Games,
                Step::Done
            ]
        );
        // The pad goes to sleep: the step stays, so the progress keeps its dots.
        m.note_pad(None);
        assert_eq!(m.onboarding_steps(), 5);
        assert!(!m.onboarding_pad().unwrap().1);
        let m = model(true);
        assert_eq!(
            m.onboarding_flow(),
            [
                Step::Language,
                Step::Look,
                Step::Games,
                Step::Online,
                Step::Done
            ]
        );
    }

    #[test]
    fn language_and_theme_apply_while_the_focus_moves_and_select_goes_on() {
        let mut m = model(false);
        // The cards are German, English, Turkish or whatever `Language::ALL` says: the focus
        // starts on the language in use.
        assert_eq!(m.onboarding_target(), Target::Language(1));
        m.handle(Command::Left);
        assert_eq!(m.language(), Language::ALL[0]);
        m.handle(Command::Select);
        assert_eq!(m.onboarding.step, Step::Look);
        let before = m.theme();
        m.handle(Command::Right);
        assert_ne!(m.theme(), before);
        m.handle(Command::Select);
        assert_eq!(m.onboarding.step, Step::Games);
    }

    #[test]
    fn the_six_looks_make_two_rows_and_each_card_shows_its_own_look() {
        let mut m = model(false);
        m.handle(Command::Select);
        assert_eq!(m.onboarding.step, Step::Look);
        let layout = m.onboarding_layout();
        assert_eq!(layout[0].len(), THEMES_PER_ROW);
        assert_eq!(layout[1].len(), Theme::ALL.len() - THEMES_PER_ROW);
        for (index, look) in Theme::ALL.iter().enumerate() {
            m.tap_onboarding(Target::Theme(index));
            assert_eq!(m.theme(), *look, "card {index}");
        }
        // The focus starts on the row of the look that is on now.
        m.onboarding.row = None;
        assert_eq!(m.onboarding_focus().0, 1, "Daylight is in the second row");
    }

    #[test]
    fn a_tap_on_a_card_selects_and_the_button_goes_on() {
        let mut m = model(false);
        m.tap_onboarding(Target::Language(0));
        assert_eq!(m.language(), Language::ALL[0], "the page changes at once");
        assert_eq!(m.onboarding.step, Step::Language, "a tap does not advance");
        m.tap_onboarding(Target::Primary);
        assert_eq!(m.onboarding.step, Step::Look);
    }

    #[test]
    fn the_look_step_writes_what_the_settings_pages_write() {
        let mut m = model(false);
        m.handle(Command::Select);
        assert_eq!(m.onboarding.step, Step::Look);
        // Down from the first theme lands on the second row of looks, and once more on the first
        // switch.
        m.handle(Command::Down);
        assert_eq!(m.onboarding_target(), Target::Theme(THEMES_PER_ROW));
        m.handle(Command::Down);
        assert_eq!(m.onboarding_target(), Target::LargerText);
        m.handle(Command::Select);
        assert!(m.prefs.larger_text);
        m.handle(Command::Right);
        m.handle(Command::Select);
        assert!(m.prefs.high_contrast);
        m.handle(Command::Right);
        assert_eq!(m.onboarding_target(), Target::ReducedMotion);
        m.handle(Command::Select);
        assert!(m.prefs.reduced_motion);
        m.handle(Command::Select);
        assert!(!m.prefs.reduced_motion, "a second press turns it off");
        // Down once more reaches Continue.
        m.handle(Command::Down);
        assert_eq!(m.onboarding_target(), Target::Primary);
    }

    #[test]
    fn the_controller_step_sets_the_confirm_button_and_the_home_button() {
        let mut m = model(false);
        m.note_pad(Some(pad()));
        m.handle(Command::Select);
        m.handle(Command::Select);
        assert_eq!(m.onboarding.step, Step::Controller);
        assert_eq!(
            m.onboarding_target(),
            Target::Confirm(0),
            "Cross is the default"
        );
        m.handle(Command::Right);
        m.handle(Command::Select);
        assert!(crate::screens::controllers::confirm_is_circle(&m.prefs));
        m.handle(Command::Left);
        m.handle(Command::Select);
        assert!(!crate::screens::controllers::confirm_is_circle(&m.prefs));
        m.handle(Command::Down);
        assert_eq!(m.onboarding_target(), Target::HomeButton);
        assert!(m.prefs.ps_button);
        m.handle(Command::Select);
        assert!(!m.prefs.ps_button);
        m.handle(Command::Down);
        m.handle(Command::Select);
        assert_eq!(m.onboarding.step, Step::Games);
        m.handle(Command::Back);
        assert_eq!(m.onboarding.step, Step::Controller, "Back returns to it");
    }

    #[test]
    fn the_games_step_picks_a_folder_or_goes_on_without() {
        let mut m = model(false);
        to_games(&mut m);
        // No folder: the first button picks one, the second (Not Now) goes on.
        assert_eq!(m.onboarding_target(), Target::Primary);
        m.effects.clear();
        m.handle(Command::Select);
        assert!(m.effects.contains(&Effect::PickGamesFolder));
        m.choose_folder(Some(PathBuf::from("C:/games")));
        assert!(m.effects.contains(&Effect::Rescan));
        // With a folder the preparing switch joins, and the focus is on Continue.
        assert_eq!(m.onboarding_layout().len(), 2);
        assert_eq!(m.onboarding_target(), Target::Primary);
        m.handle(Command::Up);
        assert_eq!(m.onboarding_target(), Target::Prepare);
        assert!(m.prefs.prepare_games);
        m.handle(Command::Select);
        assert!(!m.prefs.prepare_games);
        m.handle(Command::Down);
        m.handle(Command::Select);
        assert_eq!(m.onboarding.step, Step::Done);

        let mut m = model(false);
        to_games(&mut m);
        m.handle(Command::Right);
        assert_eq!(m.onboarding_target(), Target::Secondary);
        m.effects.clear();
        m.handle(Command::Select);
        assert!(!m.effects.contains(&Effect::PickGamesFolder));
        assert_eq!(
            m.onboarding.step,
            Step::Done,
            "Not Now goes on without a folder"
        );
        m.handle(Command::Select);
        assert!(m.onboarded());
    }

    #[test]
    fn a_folder_without_games_leads_with_choosing_another() {
        let mut m = model(false);
        to_games(&mut m);
        m.choose_folder(Some(PathBuf::from("C:/games")));
        // While the scan runs, Continue leads.
        assert!(m.scanning);
        assert!(!m.onboarding_games_pick_first());
        // The scan came back empty.
        m.scanning = false;
        assert!(m.games.is_empty());
        assert!(m.onboarding_games_pick_first());
        assert_eq!(m.onboarding_target(), Target::Primary);
        m.effects.clear();
        m.handle(Command::Select);
        assert!(
            m.effects.contains(&Effect::PickGamesFolder),
            "Primary is Change Folder"
        );
        assert_eq!(m.onboarding.step, Step::Games);
        m.handle(Command::Right);
        m.handle(Command::Select);
        assert_eq!(m.onboarding.step, Step::Done, "Secondary is Continue");
    }

    #[test]
    fn the_two_links_open_from_the_pad_and_do_not_go_on() {
        use crate::screens::legal::Legal;
        let mut m = on_online_step();
        assert_eq!(m.onboarding_target(), Target::SignIn);
        m.handle(Command::Up);
        assert_eq!(m.onboarding_target(), Target::Terms);
        m.effects.clear();
        m.handle(Command::Select);
        assert!(m.effects.contains(&Effect::OpenPage(Legal::Terms.url())));
        assert_eq!(m.onboarding.step, Step::Online, "a link does not go on");
        m.handle(Command::Right);
        m.handle(Command::Select);
        assert!(m.effects.contains(&Effect::OpenPage(Legal::Privacy.url())));
        m.handle(Command::Down);
        assert_eq!(m.onboarding_target(), Target::CreateAccount);
    }

    /// A model on the online step.
    fn on_online_step() -> Model {
        let mut m = model(true);
        to_games(&mut m);
        m.choose_folder(Some(PathBuf::from("C:/games")));
        m.handle(Command::Select);
        assert_eq!(m.onboarding.step, Step::Online);
        assert!(!m.onboarded());
        m
    }

    #[test]
    fn signing_in_waits_for_start_on_the_last_page() {
        let mut m = on_online_step();
        assert_eq!(m.onboarding_target(), Target::SignIn);
        m.handle(Command::Select);
        assert_eq!(m.onboarding.step, Step::Done);
        assert_eq!(m.onboarding_sign_in(), Some(false));
        assert!(!m.onboarded());
        assert_eq!(
            m.sign_in_after_onboarding, None,
            "nothing starts before Start"
        );
        m.handle(Command::Select);
        assert!(m.onboarded());
        assert_eq!(m.sign_in_after_onboarding, Some(false));
        assert_eq!(m.overlay, Some(Overlay::Friends));

        // Create Account asks for a new account.
        let mut m = on_online_step();
        m.handle(Command::Right);
        m.handle(Command::Select);
        m.handle(Command::Select);
        assert_eq!(m.sign_in_after_onboarding, Some(true));

        // Not Now ends it with nothing pending and nothing open.
        let mut m = on_online_step();
        m.handle(Command::Right);
        m.handle(Command::Right);
        m.handle(Command::Right);
        assert_eq!(
            m.onboarding_target(),
            Target::NotNow,
            "the row ends at Not Now"
        );
        m.handle(Command::Select);
        m.handle(Command::Select);
        assert!(m.onboarded());
        assert_eq!(m.sign_in_after_onboarding, None);
        assert_eq!(m.overlay, None);

        // Back from the online step returns to the folder.
        let mut m = on_online_step();
        m.handle(Command::Back);
        assert_eq!(m.onboarding.step, Step::Games);
    }

    #[test]
    fn the_sign_in_waits_for_the_net_service() {
        use anyps5_core::online::Status;
        let mut m = on_online_step();
        m.handle(Command::Select);
        m.handle(Command::Select);
        // Not running yet: the wish stays.
        crate::app::online::poll_onboarding_sign_in(&mut m);
        assert_eq!(m.sign_in_after_onboarding, Some(false));
        // A missing service drops it.
        m.online.status = Status::Missing;
        crate::app::online::poll_onboarding_sign_in(&mut m);
        assert_eq!(m.sign_in_after_onboarding, None);
    }

    #[test]
    fn tab_walks_the_grid_in_reading_order() {
        let mut m = model(false);
        m.handle(Command::Select);
        assert_eq!(m.onboarding.step, Step::Look);
        let mut seen = vec![m.onboarding_target()];
        for _ in 0..Theme::ALL.len() + 3 {
            m.onboarding_tab(false);
            seen.push(m.onboarding_target());
        }
        assert_eq!(
            seen.last(),
            Some(&Target::Primary),
            "the walk ends on Continue"
        );
        assert_eq!(seen[Theme::ALL.len()], Target::LargerText);
        m.onboarding_tab(true);
        assert_eq!(m.onboarding_target(), Target::ReducedMotion);
    }

    #[test]
    fn back_is_not_there_on_the_first_step() {
        let mut m = model(false);
        m.handle(Command::Back);
        assert_eq!(m.onboarding.step, Step::Language);
    }

    #[test]
    fn a_pad_found_late_adds_its_step_without_moving_the_player() {
        let mut m = model(false);
        to_games(&mut m);
        m.note_pad(Some(pad()));
        assert_eq!(m.onboarding.step, Step::Games);
        assert_eq!(m.onboarding_position(), 3);
        m.handle(Command::Back);
        assert_eq!(m.onboarding.step, Step::Controller);
    }
}
