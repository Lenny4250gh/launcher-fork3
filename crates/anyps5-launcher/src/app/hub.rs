//! Where the Game Hub and the trophy overview lead back to (docs/DESIGN.md 5.21). Both are
//! full-screen areas that can be reached from more than one place: the hub from Home, the library
//! and the overview; the overview from the hub and the player's profile. Back goes to where the
//! player came from, and to Home when they came from Home.

use crate::app::model::Model;
use crate::app::overlay::Overlay;
use crate::ui::focus::HomeFocus;
use crate::ui::sound::Cue;

impl Model {
    /// Opens the hub of the selected game; Back leads home.
    pub fn open_hub(&mut self) {
        if self.selected_game().is_none() {
            return;
        }
        self.hub_back = None;
        self.open(Overlay::GameHub);
    }

    /// Opens the hub of a game of the library (a title ID), Back leading to `back`.
    pub fn open_hub_of(&mut self, title_id: &str, back: Option<Overlay>) {
        let Some(index) = self.games.iter().position(|g| g.title_id == title_id) else {
            return;
        };
        self.selected = index;
        self.focus = HomeFocus::Tiles;
        self.measure_selected();
        self.hub_back = back;
        self.switch_overlay(Overlay::GameHub);
        self.sound(Cue::Open);
    }

    /// Back from the hub.
    pub fn close_hub(&mut self) {
        match self.hub_back.take() {
            Some(overlay) => {
                self.sound(Cue::Back);
                self.switch_overlay(overlay);
            }
            None => self.close_overlay(),
        }
    }

    /// Opens the overview of all games' trophies, Back leading to `back`.
    pub fn open_overview(&mut self, back: Option<Overlay>) {
        let back = back.filter(|o| *o != Overlay::Profile || self.profile.is_some());
        self.overview_back = back;
        self.switch_overlay(Overlay::TrophyOverview);
        self.sound(Cue::Open);
    }

    /// Back from the overview.
    pub fn close_overview(&mut self) {
        match self.overview_back.take() {
            Some(overlay) => {
                self.sound(Cue::Back);
                self.switch_overlay(overlay);
            }
            None => self.close_overlay(),
        }
    }
}
