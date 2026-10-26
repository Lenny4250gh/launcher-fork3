//! "Storage": how much room the unlocked copies take, game by game and in all, how much is free
//! on their drive, and the way to remove one copy. Removing never touches the game files; the
//! question before it is the card every question uses.

use egui::{Color32, Context};

use crate::screens::library_strings::LibraryText as T;
use crate::screens::rows::{self, Row};
use crate::screens::side::{self, Frame, Leave, SideState, SideView};
use crate::ui::Shell;
use crate::ui::i18n::{Arg, Language, byte_text};
use crate::ui::icons::Icon;
use crate::ui::input::InputEvent;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StorageAction {
    /// The copy at this place of the list was selected: ask before removing it.
    Ask(usize),
    /// Back: return to the library.
    Close,
}

/// One unlocked copy.
#[derive(Clone, Debug)]
pub struct StorageEntry {
    pub name: String,
    /// Bytes once measured.
    pub bytes: Option<u64>,
    /// The game runs: its copy cannot be removed now.
    pub running: bool,
}

/// The copies and the drive, borrowed for one frame.
pub struct StorageData<'a> {
    /// Biggest first.
    pub entries: &'a [StorageEntry],
    /// Free bytes of the drive that holds the copies, when known.
    pub free: Option<u64>,
    pub language: Language,
    pub accent: Color32,
}

impl StorageData<'_> {
    /// The sum of what is measured; `None` while any copy is still being measured.
    pub fn total(&self) -> Option<u64> {
        self.entries.iter().map(|e| e.bytes).sum()
    }
}

fn size_text(bytes: Option<u64>, language: Language) -> String {
    match bytes {
        Some(bytes) => byte_text(i64::try_from(bytes).unwrap_or(i64::MAX), language),
        None => T::StorageMeasuring.text(language).to_owned(),
    }
}

/// The sentence under the title: the room the copies take and the room that is left.
pub fn summary(data: &StorageData<'_>) -> String {
    let language = data.language;
    if data.entries.is_empty() {
        return T::StorageEmptyText.text(language).to_owned();
    }
    let total = size_text(data.total(), language);
    match data.free {
        Some(free) => T::StorageSummary.format(
            language,
            &[Arg::Str(&total), Arg::Str(&size_text(Some(free), language))],
        ),
        None => T::StorageSummaryNoDrive.format(language, &[Arg::Str(&total)]),
    }
}

pub fn build_rows(data: &StorageData<'_>) -> Vec<Row> {
    let language = data.language;
    if data.entries.is_empty() {
        return vec![Row::info("empty", T::StorageEmpty.text(language), None)];
    }
    let mut rows = vec![Row::info(
        "total",
        T::StorageTotal.text(language),
        Some(size_text(data.total(), language)),
    )];
    for (i, entry) in data.entries.iter().enumerate() {
        let detail = if entry.running {
            T::RemoveCopyRunning.text(language).to_owned()
        } else {
            T::StorageCopy.format(language, &[Arg::Str(&size_text(entry.bytes, language))])
        };
        let row = Row::action("copy", entry.name.clone())
            .detail(detail)
            .icon(Icon::TRASH);
        rows.push(if i == 0 { row.group_start() } else { row });
    }
    rows
}

/// The side panel.
#[derive(Default)]
pub struct StoragePanel {
    side: SideState,
}

impl StoragePanel {
    /// Opens on the first copy, or on the note when there is none.
    pub fn open(&mut self, data: &StorageData<'_>) {
        self.side.open_at(usize::from(!data.entries.is_empty()));
    }

    /// Opens on the copy at `index` (coming back from the question about it).
    pub fn open_copy(&mut self, index: usize) {
        self.side.open_at(1 + index);
    }

    pub fn close(&mut self) {
        self.side.close();
    }

    pub fn dismiss(&mut self) {
        self.side.dismiss();
    }

    pub fn is_active(&self) -> bool {
        self.side.is_active()
    }

    pub fn show(
        &mut self,
        shell: &mut Shell,
        ctx: &Context,
        events: &[InputEvent],
        data: &StorageData<'_>,
    ) -> Vec<StorageAction> {
        let mut actions = Vec::new();
        let language = data.language;
        let subtitle = summary(data);
        let frame = Frame {
            title: T::StorageTitle.text(language),
            subtitle: Some(subtitle.as_str()),
            list_id: "library-storage-rows",
            accent: data.accent,
            language,
        };
        self.side.show_rows(
            shell,
            ctx,
            events,
            &mut { data },
            &frame,
            Leave::Slide,
            |data| build_rows(data),
            |side, _, _, change, actions| {
                if let rows::Change::Run { row: "copy" } = change {
                    // The first row is the total.
                    if let Some(index) = side.nav.row.checked_sub(1) {
                        actions.push(StorageAction::Ask(index));
                    }
                }
            },
            StorageAction::Close,
            &mut actions,
        );
        actions
    }
}

// ---------------------------------------------------------------------------------------------
// The question before a copy is removed
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RemoveAction {
    Confirm,
    Cancel,
}

/// "Remove Unlocked Copy" / "Cancel" on the card of every question.
#[derive(Default)]
pub struct RemovePanel {
    side: SideState,
}

impl RemovePanel {
    /// Opens on Cancel: Enter must never remove a copy by accident.
    pub fn open(&mut self) {
        self.side.open_at(1);
    }

    pub fn close(&mut self) {
        self.side.close();
    }

    pub fn dismiss(&mut self) {
        self.side.dismiss();
    }

    pub fn is_active(&self) -> bool {
        self.side.is_active()
    }

    fn handle(&mut self, change: &rows::Change, actions: &mut Vec<RemoveAction>) {
        if let rows::Change::Run { row } = change {
            self.side.close();
            actions.push(if *row == "remove" {
                RemoveAction::Confirm
            } else {
                RemoveAction::Cancel
            });
        }
    }

    /// `name` is the game, `bytes` what the removal frees.
    pub fn show(
        &mut self,
        shell: &mut Shell,
        ctx: &Context,
        events: &[InputEvent],
        (name, bytes): (&str, Option<u64>),
        language: Language,
        accent: Color32,
    ) -> Vec<RemoveAction> {
        let mut actions = Vec::new();
        if !self.side.is_active() {
            return actions;
        }
        let build = || {
            vec![
                Row::command("remove", T::RemoveCopy.text(language))
                    .icon(Icon::TRASH)
                    .destructive(),
                Row::command("cancel", T::Cancel.text(language))
                    .icon(Icon::X)
                    .refusal(),
            ]
        };
        if self.side.is_open() {
            let rows = build();
            let driven = crate::screens::dialog::drive(
                shell,
                &mut self.side.nav,
                &rows,
                &side::commands(events),
            );
            for change in &driven.changes {
                self.handle(change, &mut actions);
            }
            if driven.close {
                self.side.close();
                actions.push(RemoveAction::Cancel);
            }
        }
        let mut rows = build();
        let title = T::RemoveCopyTitle.format(language, &[Arg::Str(name)]);
        let ask = T::RemoveCopyBody.format(language, &[Arg::Str(&size_text(bytes, language))]);
        let view = SideView {
            title: &title,
            subtitle: Some(&ask),
            question: true,
            list_id: "library-remove-rows",
            nav: &self.side.nav,
            accent,
            language,
            force: None,
        };
        match side::draw(ctx, self.side.is_open(), &view, &mut rows) {
            Some(drawn) if self.side.is_open() => {
                let (changes, _) =
                    rows::apply_mouse(shell, &mut self.side.nav, &rows, drawn.events);
                for change in &changes {
                    self.handle(change, &mut actions);
                }
                if drawn.back_clicked {
                    self.side.close();
                    actions.push(RemoveAction::Cancel);
                }
            }
            Some(_) => {}
            None => self.side.finished(),
        }
        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(name: &str, bytes: Option<u64>) -> StorageEntry {
        StorageEntry {
            name: name.into(),
            bytes,
            running: false,
        }
    }

    #[test]
    fn the_total_waits_for_every_copy() {
        let entries = [entry("A", Some(5)), entry("B", None)];
        let data = StorageData {
            entries: &entries,
            free: None,
            language: Language::English,
            accent: Color32::WHITE,
        };
        assert_eq!(data.total(), None);
        let entries = [entry("A", Some(5)), entry("B", Some(7))];
        let data = StorageData {
            entries: &entries,
            ..data
        };
        assert_eq!(data.total(), Some(12));
    }

    #[test]
    fn one_row_per_copy_after_the_total() {
        let entries = [entry("A", Some(5)), entry("B", Some(7))];
        let data = StorageData {
            entries: &entries,
            free: Some(1_000_000_000),
            language: Language::English,
            accent: Color32::WHITE,
        };
        let rows = build_rows(&data);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].id, "total");
        assert_eq!(rows[1].id, "copy");
        assert!(summary(&data).contains("free"));
        let none = StorageData {
            entries: &[],
            ..data
        };
        assert_eq!(build_rows(&none).len(), 1);
    }
}
