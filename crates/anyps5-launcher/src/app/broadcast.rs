//! Broadcast while a game runs (docs/DESIGN.md 9.13): starts and stops the worker that publishes
//! the game's window to Twitch, YouTube or another RTMP server, keeps the model's phase, turns the
//! worker's answers into toasts and draws the small LIVE mark.
//!
//! Nothing runs until the player starts a broadcast from the Home button panel. The worker is
//! the share worker in its broadcast mode (network spec/broadcast.md); the stream key is read
//! from the credential store when the broadcast starts and goes to the worker on its input.

use std::sync::Arc;

use anyps5_core::broadcast::{self, Broadcaster, EndKind, Phase, WorkerEvent};
use anyps5_core::paths::GameId;
use egui::{Align2, Color32, Context, FontId, Sense, vec2};

use super::model::Model;
use super::pipeline::Pipeline;
use crate::screens::broadcast_strings::BroadcastText;
use crate::ui::icons::Icon;

pub struct BroadcastService {
    wake: Arc<dyn Fn() + Send + Sync>,
    /// Previews and tests never start a process or read the credential store.
    simulated: bool,
    active: Option<Broadcaster>,
    game: Option<GameId>,
    /// The credential store was asked whether a key is kept.
    key_checked: bool,
    /// How long the stream has been live, as the worker last said.
    seconds: u32,
}

impl BroadcastService {
    pub fn new(wake: Arc<dyn Fn() + Send + Sync>, simulated: bool) -> Self {
        BroadcastService {
            wake,
            simulated,
            active: None,
            game: None,
            key_checked: false,
            seconds: 0,
        }
    }

    /// Once per frame: ends the broadcast with its game and takes in what the worker reports.
    pub fn frame(&mut self, model: &mut Model) {
        if !self.simulated && !self.key_checked {
            self.key_checked = true;
            model.broadcast_key_set = broadcast::load_key(&model.paths).is_some();
        }
        if self.active.is_some() && model.running != self.game {
            self.end(model);
            return;
        }
        let mut ended = None;
        if let Some(active) = &mut self.active {
            while let Ok(event) = active.events.try_recv() {
                match event {
                    WorkerEvent::Connecting => model.broadcast = Phase::Connecting,
                    WorkerEvent::Live { .. } => {
                        model.broadcast = Phase::Live;
                        self.seconds = 0;
                        model.toast(
                            BroadcastText::Started.text(model.language()),
                            Icon::WIFI_HIGH,
                        );
                    }
                    WorkerEvent::Stats { seconds, .. } => self.seconds = seconds,
                    WorkerEvent::Ended { reason } => ended = Some(reason),
                }
            }
            if ended.is_none() && !active.alive() {
                ended = Some("exited".to_owned());
            }
        }
        if let Some(reason) = ended {
            tracing::info!("broadcast: the worker ended: {reason}");
            self.active = None;
            self.game = None;
            model.broadcast = Phase::Off;
            let text = match broadcast::end_kind(&reason) {
                EndKind::Normal => return,
                EndKind::BadKey => BroadcastText::BadKey,
                EndKind::NoServer => BroadcastText::NoServer,
                EndKind::Lost => BroadcastText::Lost,
                EndKind::NoEncoder => BroadcastText::NoEncoder,
                EndKind::NoWindow => BroadcastText::NoWindow,
                EndKind::Other => BroadcastText::Failed,
            };
            model.toast_failure(text.text(model.language()));
        }
    }

    /// "Start Broadcast" / "Stop Broadcast".
    pub fn toggle(&mut self, model: &mut Model, pipeline: &dyn Pipeline) {
        if self.active.is_some() {
            self.end(model);
            model.toast(BroadcastText::Ended.text(model.language()), Icon::WIFI_HIGH);
        } else {
            self.start(model, pipeline);
        }
    }

    fn end(&mut self, model: &mut Model) {
        self.active = None;
        self.game = None;
        model.broadcast = Phase::Off;
    }

    fn start(&mut self, model: &mut Model, pipeline: &dyn Pipeline) {
        let language = model.language();
        if self.simulated || !cfg!(windows) {
            return;
        }
        let Some(game) = model.running.clone() else {
            return;
        };
        let Some(url) = broadcast::server_url(
            &model.prefs.broadcast_service,
            &model.prefs.broadcast_server,
        ) else {
            model.toast(BroadcastText::NeedServer.text(language), Icon::WARNING);
            return;
        };
        let Some(key) = broadcast::load_key(&model.paths) else {
            model.broadcast_key_set = false;
            model.toast(BroadcastText::NeedKey.text(language), Icon::WARNING);
            return;
        };
        let Some(target) = pipeline.game_target(&game).filter(|t| !t.pids.is_empty()) else {
            model.toast(BroadcastText::NoWindow.text(language), Icon::WARNING);
            return;
        };
        let Some(worker) = super::online::share_worker_path(model) else {
            model.toast_failure(BroadcastText::NoWorker.text(language));
            return;
        };
        let title = model
            .game(&game)
            .map(|g| g.name.clone())
            .unwrap_or_default();
        let wake = self.wake.clone();
        match Broadcaster::start(
            &worker,
            &url,
            &key,
            broadcast::quality(model.prefs.broadcast_quality),
            target.window,
            &target.pids,
            &title,
            move || wake(),
        ) {
            Ok(active) => {
                tracing::info!("broadcast: starting for {title}");
                self.active = Some(active);
                self.game = Some(game);
                self.seconds = 0;
                model.broadcast = Phase::Connecting;
            }
            Err(error) => {
                tracing::warn!("broadcast: the worker could not start: {error}");
                model.toast_failure(BroadcastText::NoWorker.text(language));
            }
        }
    }

    /// The small LIVE mark in the corner of the launcher while a broadcast runs.
    pub fn paint(&self, ctx: &Context, model: &Model) {
        if model.broadcast != Phase::Live {
            return;
        }
        let language = model.language();
        let label = format!(
            "{}  {}",
            BroadcastText::Badge.text(language),
            clock(self.seconds)
        );
        egui::Area::new(egui::Id::unique("broadcast-live-mark"))
            .order(egui::Order::Foreground)
            .anchor(Align2::RIGHT_TOP, vec2(-20.0, 16.0))
            .interactable(false)
            .show(ctx, |ui| {
                let (rect, _) = ui.allocate_exact_size(vec2(104.0, 26.0), Sense::hover());
                let painter = ui.painter();
                painter.rect_filled(rect, 13.0, Color32::from_rgba_unmultiplied(0, 0, 0, 170));
                painter.circle_filled(
                    egui::pos2(rect.min.x + 14.0, rect.center().y),
                    4.0,
                    Color32::from_rgb(235, 64, 64),
                );
                painter.text(
                    egui::pos2(rect.min.x + 26.0, rect.center().y),
                    Align2::LEFT_CENTER,
                    label,
                    FontId::proportional(13.0),
                    Color32::WHITE,
                );
            });
        // The clock moves while the launcher is in front.
        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}

/// Keeps the stream key (or removes it) and tells the player when that did not work.
pub fn set_key(model: &mut Model, key: Option<&str>) {
    match broadcast::save_key(&model.paths, key) {
        Ok(()) => model.broadcast_key_set = key.is_some(),
        Err(error) => {
            tracing::warn!("broadcast: the stream key was not kept: {error}");
            model.toast_failure(BroadcastText::KeyNotKept.text(model.language()));
        }
    }
}

/// `4:07` or `1:02:09`.
fn clock(seconds: u32) -> String {
    let (h, m, s) = (seconds / 3600, seconds / 60 % 60, seconds % 60);
    if h > 0 {
        format!("{h}:{m:02}:{s:02}")
    } else {
        format!("{m}:{s:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_clock_reads_like_a_stopwatch() {
        assert_eq!(clock(0), "0:00");
        assert_eq!(clock(247), "4:07");
        assert_eq!(clock(3729), "1:02:09");
    }
}
