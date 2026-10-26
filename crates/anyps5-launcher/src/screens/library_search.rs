//! Search of the library page: a text row for the keyboard and rows of keys for a pad, on the
//! frame the game panels share. The games behind it are filtered as the query changes, so
//! "Show Games" only goes back to them.

use egui::{Color32, Context};

use crate::screens::library_strings::LibraryText as T;
use crate::screens::rows::{self, Change, Row};
use crate::screens::side::{self, SideState, SideView};
use crate::ui::Shell;
use crate::ui::i18n::{Arg, Language};
use crate::ui::icons::Icon;
use crate::ui::input::InputEvent;
use crate::ui::sound::Cue;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SearchAction {
    /// The query changed to this.
    Query(String),
    /// Back to the games.
    Close,
}

/// What the panel needs to know, borrowed for one frame.
pub struct SearchData<'a> {
    pub query: &'a str,
    /// How many games match the query now.
    pub found: usize,
    pub language: Language,
    pub accent: Color32,
}

const KEY_ROWS: [&str; 4] = ["keyA", "keyB", "keyC", "keyCtl"];

#[derive(Default)]
pub struct SearchPanel {
    side: SideState,
    shift: bool,
    symbols: bool,
    /// The key each row of keys rests on.
    keys: [usize; 4],
}

impl SearchPanel {
    /// Opens on the text row: a keyboard types at once, a pad moves down to the keys.
    pub fn open(&mut self) {
        self.side.open_at(1);
        self.shift = false;
        self.symbols = false;
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

    fn labels(&self, row: &str, language: Language) -> Vec<String> {
        let letters = |chars: &str| -> Vec<String> {
            chars
                .chars()
                .map(|c| {
                    if self.shift && !self.symbols {
                        c.to_uppercase().collect()
                    } else {
                        c.to_string()
                    }
                })
                .collect()
        };
        match (row, self.symbols) {
            ("keyA", false) => letters("qwertyuiop"),
            ("keyB", false) => letters("asdfghjkl"),
            ("keyC", false) => letters("zxcvbnm-:."),
            ("keyA", true) => letters("1234567890"),
            ("keyB", true) => letters("-/:;()&@\"!"),
            ("keyC", true) => letters("'+=#%*_<>~"),
            _ => vec![
                T::KeyShift.text(language).to_owned(),
                if self.symbols {
                    T::KeyLetters.text(language).to_owned()
                } else {
                    T::KeySymbols.text(language).to_owned()
                },
                T::KeySpace.text(language).to_owned(),
                T::KeyDelete.text(language).to_owned(),
            ],
        }
    }

    fn rows(&self, data: &SearchData<'_>) -> Vec<Row> {
        let language = data.language;
        let mut rows = vec![Row::info(
            "found",
            if data.found == 0 {
                T::SearchNone.text(language).to_owned()
            } else {
                T::SearchFound.format(language, &[Arg::Int(data.found as i64)])
            },
            None,
        )];
        rows.push(
            Row::text("query", T::SearchField.text(language), data.query)
                .icon(Icon::MAGNIFYING_GLASS),
        );
        for (slot, row) in KEY_ROWS.into_iter().enumerate() {
            rows.push(Row::buttons(
                row,
                "",
                self.labels(row, language),
                self.keys[slot],
            ));
        }
        rows.push(
            Row::command("show", T::SearchShow.text(language))
                .icon(Icon::CHECK)
                .group_start(),
        );
        if !data.query.is_empty() {
            rows.push(Row::command("clear", T::SearchClear.text(language)).icon(Icon::X));
        }
        rows
    }

    fn change(&mut self, change: &Change, data: &SearchData<'_>, actions: &mut Vec<SearchAction>) {
        match change {
            Change::Choice { row, index } => {
                if let Some(slot) = KEY_ROWS.iter().position(|r| r == row) {
                    self.keys[slot] = *index;
                }
            }
            Change::Press { row, index } if KEY_ROWS.contains(row) => {
                let mut query = data.query.to_owned();
                if *row == "keyCtl" {
                    match *index {
                        0 => self.shift = !self.shift,
                        1 => {
                            self.symbols = !self.symbols;
                            self.shift = false;
                        }
                        2 => query.push(' '),
                        _ => {
                            query.pop();
                        }
                    }
                } else if let Some(key) = self.labels(row, data.language).get(*index) {
                    query.push_str(key);
                    self.shift = false;
                }
                if query != data.query {
                    actions.push(SearchAction::Query(query));
                }
            }
            Change::Run { row: "show" } => {
                self.side.close();
                actions.push(SearchAction::Close);
            }
            Change::Run { row: "clear" } => {
                actions.push(SearchAction::Query(String::new()));
                self.side.nav.row = 1;
            }
            Change::Submit { row: "query" } => {
                self.side.close();
                actions.push(SearchAction::Close);
            }
            _ => {}
        }
    }

    pub fn show(
        &mut self,
        shell: &mut Shell,
        ctx: &Context,
        events: &[InputEvent],
        data: &SearchData<'_>,
    ) -> Vec<SearchAction> {
        let mut actions = Vec::new();
        if !self.side.is_active() {
            return actions;
        }
        let language = data.language;
        if self.side.is_open() {
            let rows = self.rows(data);
            for command in side::commands(events) {
                let driven = rows::drive(shell, &mut self.side.nav, &rows, &[command]);
                for change in &driven.changes {
                    self.change(change, data, &mut actions);
                }
                if driven.close {
                    self.side.close();
                    actions.push(SearchAction::Close);
                    break;
                }
            }
            if !self.side.is_active() {
                return actions;
            }
        }
        // A change above may have changed the query: the rows show the new one.
        let query_now = actions
            .iter()
            .rev()
            .find_map(|a| match a {
                SearchAction::Query(q) => Some(q.clone()),
                SearchAction::Close => None,
            })
            .unwrap_or_else(|| data.query.to_owned());
        let shown = SearchData {
            query: &query_now,
            found: data.found,
            language,
            accent: data.accent,
        };
        let mut rows = self.rows(&shown);
        let view = SideView {
            title: T::SearchTitle.text(language),
            subtitle: None,
            question: false,
            list_id: "library-search-rows",
            nav: &self.side.nav,
            accent: data.accent,
            language,
            force: None,
        };
        match side::draw(ctx, self.side.is_open(), &view, &mut rows) {
            Some(drawn) if self.side.is_open() => {
                let (changes, texts) =
                    rows::apply_mouse(shell, &mut self.side.nav, &rows, drawn.events);
                for (row, value) in texts {
                    if row == "query" {
                        actions.push(SearchAction::Query(value));
                    }
                }
                for change in &changes {
                    self.change(change, &shown, &mut actions);
                }
                if drawn.back_clicked {
                    shell.play(Cue::Close);
                    self.side.close();
                    actions.push(SearchAction::Close);
                }
            }
            Some(_) => {}
            None => self.side.finished(),
        }
        actions
    }
}
