//! The panels around the library page: search, storage and the question before an unlocked copy is
//! removed. They hold their own small state (the measured sizes, the copy the question is about)
//! and work on their own thread, so the big panel host only hands them the frame.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender, channel};

use anyps5_core::events::CancelToken;
use anyps5_core::paths::{self, GameId};
use anyps5_core::saves::{Reason, SaveData, SavesError};
use anyps5_core::{fsx, library, unlock};
use egui::Context;

use crate::app::library_view::{Compat, Filter, Sort};
use crate::app::model::Model;
use crate::app::overlay::{Overlay, OverlayCx};
use crate::screens::library::{BarData, StateText};
use crate::screens::library_search::{SearchAction, SearchData, SearchPanel};
use crate::screens::library_storage::{
    RemoveAction, RemovePanel, StorageAction, StorageData, StorageEntry, StoragePanel,
};
use crate::screens::library_strings::LibraryText;
use crate::screens::strings::Extra;
use crate::ui::i18n::{Arg, Key, Language};
use crate::ui::icons::Icon;
use crate::ui::input::InputEvent;
use crate::ui::theme::color;

/// The line under a tile for what players found and whether the game can be played together.
pub fn compat_line(tags: &[Compat], language: Language) -> Option<StateText> {
    let (icon, tint) = match tags.first()? {
        Compat::RunsWell => (Icon::CHECK_CIRCLE, color::state_neutral()),
        Compat::RunsWithIssues => (Icon::WARNING, color::warning_text()),
        Compat::NoStart => (Icon::X_CIRCLE, color::error_state()),
        Compat::Lan => (Icon::USERS, color::state_neutral()),
    };
    let words: Vec<&str> = tags
        .iter()
        .enumerate()
        .map(|(i, tag)| {
            match tag {
                Compat::RunsWell => LibraryText::RunsWell,
                Compat::RunsWithIssues => LibraryText::RunsWithIssues,
                Compat::NoStart => LibraryText::NoStart,
                // After another tag the line has less room, so the shorter word stands in.
                Compat::Lan if i > 0 => LibraryText::LanShort,
                Compat::Lan => LibraryText::Lan,
            }
            .text(language)
        })
        .collect();
    Some(StateText {
        icon,
        text: words.join(" \u{b7} "),
        tint,
        quiet: false,
    })
}

pub fn sort_name(sort: Sort) -> LibraryText {
    match sort {
        Sort::Recent => LibraryText::SortRecent,
        Sort::Name => LibraryText::SortName,
        Sort::Size => LibraryText::SortSize,
        Sort::Status => LibraryText::SortStatus,
    }
}

pub fn filter_name(filter: Filter) -> LibraryText {
    match filter {
        Filter::All => LibraryText::FilterAll,
        Filter::Ready => LibraryText::FilterReady,
        Filter::NeedsUnlock => LibraryText::FilterNeedsUnlock,
        Filter::Together => LibraryText::FilterTogether,
    }
}

/// What the bar of the library page says.
pub fn bar_data(model: &Model) -> BarData {
    let language = model.language();
    let (sort, filter) = (model.library_sort(), model.library_filter());
    BarData {
        sort: LibraryText::SortBy.format(language, &[Arg::Str(sort_name(sort).text(language))]),
        filter: LibraryText::ShowFilter
            .format(language, &[Arg::Str(filter_name(filter).text(language))]),
        query: model.library_query.clone(),
        search: LibraryText::Search.text(language).to_owned(),
        storage: LibraryText::Storage.text(language).to_owned(),
        narrowed: !model.games.is_empty()
            && (filter != Filter::All || !model.library_query.trim().is_empty()),
        nothing_title: LibraryText::NothingHere.text(language).to_owned(),
        nothing_text: LibraryText::NothingHereText.text(language).to_owned(),
        clear: LibraryText::ClearFilters.text(language).to_owned(),
    }
}

enum Msg {
    /// The bytes an unlocked copy takes.
    Size(GameId, u64),
    /// The free bytes of the drive that holds the copies.
    Free(Option<u64>),
    Removed {
        game: GameId,
        result: Result<(), String>,
    },
}

pub struct LibraryExtras {
    search: SearchPanel,
    storage: StoragePanel,
    remove: RemovePanel,
    sizes: HashMap<GameId, u64>,
    free: Option<u64>,
    /// The copy the question is about.
    target: Option<GameId>,
    /// The place in the list to come back to after the question.
    return_to: Option<usize>,
    tx: Sender<Msg>,
    rx: Receiver<Msg>,
    ctx: Context,
}

fn spawn(name: &str, work: impl FnOnce() + Send + 'static) {
    if let Err(error) = std::thread::Builder::new().name(name.into()).spawn(work) {
        tracing::warn!("cannot start the {name} thread: {error}");
    }
}

impl LibraryExtras {
    pub fn new(ctx: &Context) -> Self {
        let (tx, rx) = channel();
        Self {
            search: SearchPanel::default(),
            storage: StoragePanel::default(),
            remove: RemovePanel::default(),
            sizes: HashMap::new(),
            free: None,
            target: None,
            return_to: None,
            tx,
            rx,
            ctx: ctx.clone(),
        }
    }

    /// Takes in what the worker threads found, once per frame.
    pub fn poll(&mut self, model: &mut Model) {
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                Msg::Size(game, bytes) => {
                    self.sizes.insert(game, bytes);
                }
                Msg::Free(free) => self.free = free,
                Msg::Removed { game, result } => match result {
                    Ok(()) => {
                        if let Some(g) = model.games.iter_mut().find(|g| g.id == game) {
                            g.unlocked_with = None;
                        }
                        model.failures.remove(&game);
                        model.trophies.remove(&game);
                        self.sizes.remove(&game);
                        let text = model.language().text(Key::UninstallDone);
                        model.toast(text, Icon::TRASH);
                    }
                    Err(error) => {
                        tracing::warn!("cannot remove the unlocked copy: {error}");
                        model.toast_failure(error);
                    }
                },
            }
        }
    }

    /// Measures the unlocked copies and the drive on a worker thread.
    fn measure(&self, model: &Model) {
        let Some(root) = model.store_root() else {
            return;
        };
        let copies: Vec<(GameId, String)> = model
            .games
            .iter()
            .filter(|g| g.is_unlocked())
            .map(|g| (g.id.clone(), g.title_id.clone()))
            .collect();
        let (tx, ctx) = (self.tx.clone(), self.ctx.clone());
        spawn("storage", move || {
            let _ = tx.send(Msg::Free(fsx::free_space(&root)));
            ctx.request_repaint();
            let cancel = CancelToken::new();
            for (id, title_id) in copies {
                let copy = paths::unlocked_copy_dir(&root, &title_id, &id);
                let bytes = library::folder_size(&copy, &cancel);
                let _ = tx.send(Msg::Size(id, bytes));
                ctx.request_repaint();
            }
        });
    }

    /// The unlocked copies, the biggest first (those not measured yet last).
    fn copies(&self, model: &Model) -> Vec<(GameId, StorageEntry)> {
        let mut list: Vec<(GameId, StorageEntry)> = model
            .games
            .iter()
            .filter(|g| g.is_unlocked())
            .map(|g| {
                (
                    g.id.clone(),
                    StorageEntry {
                        name: g.name.clone(),
                        bytes: self.sizes.get(&g.id).copied(),
                        running: model.is_active(&g.id),
                    },
                )
            })
            .collect();
        list.sort_by(|a, b| {
            b.1.bytes
                .cmp(&a.1.bytes)
                .then_with(|| a.1.name.to_lowercase().cmp(&b.1.name.to_lowercase()))
        });
        list
    }

    pub fn search_ui(&mut self, cx: &mut OverlayCx<'_>, wanted: bool, events: &[InputEvent]) {
        let side_next = cx.model.overlay.is_some_and(Overlay::is_side_panel);
        if wanted && !self.search.is_active() {
            self.search.open();
        } else if !wanted && self.search.is_active() {
            if side_next {
                self.search.dismiss();
            } else {
                self.search.close();
            }
        }
        let found = cx.model.library_order().len();
        let query = cx.model.library_query.clone();
        let data = SearchData {
            query: &query,
            found,
            language: cx.model.language(),
            accent: cx.shell.accent_color(),
        };
        for action in self.search.show(cx.shell, cx.ctx, events, &data) {
            match action {
                SearchAction::Query(text) => cx.model.library_query = text,
                SearchAction::Close => cx.model.switch_overlay(Overlay::Library),
            }
        }
    }

    pub fn storage_ui(&mut self, cx: &mut OverlayCx<'_>, wanted: bool, events: &[InputEvent]) {
        let side_next = cx.model.overlay.is_some_and(Overlay::is_side_panel);
        if wanted && !self.storage.is_active() {
            self.measure(cx.model);
        }
        let list = self.copies(cx.model);
        let entries: Vec<StorageEntry> = list.iter().map(|(_, e)| e.clone()).collect();
        let data = StorageData {
            entries: &entries,
            free: self.free,
            language: cx.model.language(),
            accent: cx.shell.accent_color(),
        };
        if wanted && !self.storage.is_active() {
            match self.return_to.take() {
                Some(index) => self
                    .storage
                    .open_copy(index.min(entries.len().saturating_sub(1))),
                None => self.storage.open(&data),
            }
        } else if !wanted && self.storage.is_active() {
            if side_next {
                self.storage.dismiss();
            } else {
                self.storage.close();
            }
        }
        for action in self.storage.show(cx.shell, cx.ctx, events, &data) {
            match action {
                StorageAction::Ask(index) => {
                    let Some((id, entry)) = list.get(index) else {
                        continue;
                    };
                    if entry.running {
                        let text = LibraryText::RemoveCopyRunning.text(data.language);
                        cx.model.toast(text, Icon::WARNING);
                    } else {
                        self.target = Some(id.clone());
                        self.return_to = Some(index);
                        cx.model.switch_overlay(Overlay::ConfirmRemoveCopy);
                    }
                }
                StorageAction::Close => cx.model.switch_overlay(Overlay::Library),
            }
        }
    }

    pub fn remove_ui(&mut self, cx: &mut OverlayCx<'_>, wanted: bool, events: &[InputEvent]) {
        let side_next = cx.model.overlay.is_some_and(Overlay::is_side_panel);
        if wanted && !self.remove.is_active() {
            self.remove.open();
        } else if !wanted && self.remove.is_active() {
            if side_next {
                self.remove.dismiss();
            } else {
                self.remove.close();
            }
        }
        let (language, accent) = (cx.model.language(), cx.shell.accent_color());
        let (name, bytes) = self
            .target
            .as_ref()
            .and_then(|id| {
                cx.model
                    .game(id)
                    .map(|g| (g.name.clone(), self.sizes.get(id).copied()))
            })
            .unwrap_or_default();
        for action in self
            .remove
            .show(cx.shell, cx.ctx, events, (&name, bytes), language, accent)
        {
            if action == RemoveAction::Confirm {
                self.start_remove(cx.model, language);
            }
            // Either answer goes back to the list, where the result shows.
            cx.model.switch_overlay(Overlay::Storage);
        }
    }

    /// Deletes the unlocked copy on a worker thread. Its saves go with it, so they are backed up
    /// first; when that fails nothing is removed. The game files are never touched.
    fn start_remove(&mut self, model: &mut Model, language: Language) {
        let Some(id) = self.target.take() else {
            return;
        };
        let Some(game) = model.game(&id).cloned() else {
            return;
        };
        if model.is_active(&game.id) {
            model.toast(LibraryText::RemoveCopyRunning.text(language), Icon::WARNING);
            return;
        }
        let Some(root) = model.store_root() else {
            return;
        };
        let backups: PathBuf = model.paths.game_backups_dir(&game.title_id);
        let (tx, ctx) = (self.tx.clone(), self.ctx.clone());
        spawn("remove-copy", move || {
            let copy = paths::unlocked_copy_dir(&root, &game.title_id, &game.id);
            let backed_up = match SaveData::new(copy, backups).backup(Reason::BeforeUninstall) {
                Ok(_) | Err(SavesError::NothingToSave) => Ok(()),
                Err(error) => {
                    tracing::warn!("cannot back up the saves before removing: {error}");
                    Err(Extra::UninstallNeedsBackup.text(language).to_owned())
                }
            };
            let result = backed_up.and_then(|()| {
                unlock::remove_unlocked(&game.title_id, &game.id, &root).map_err(|e| e.to_string())
            });
            let _ = tx.send(Msg::Removed {
                game: game.id.clone(),
                result,
            });
            ctx.request_repaint();
        });
    }
}
