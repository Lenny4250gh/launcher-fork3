//! Find Players (docs/DESIGN.md 17, "Find Players"): the open listings of one game, a listing of
//! the player's own, and the few steps around them. A panel of the game frame like the saves or
//! the recommended settings: the Back row on top, the game's name as title, rows under it.
//!
//! The list is the home page: one primary row posts a listing (or opens the player's own, with
//! its requests), two choices narrow the list by mode and language, and every listing is a row
//! that opens the listing's page. A foreign listing's page has one primary action, Ask to Join,
//! and Report and Block under it; the player's own page lists who asked, with Accept and Decline
//! at the end of each row, and Close Listing. Posting is a page of its own: mode, language, how
//! many players, a note of at most 140 characters, and one button.
//!
//! The panel keeps only its page and the drafts; the listings are the model's
//! (`anyps5_core::online::lfg`) and every change leaves as an [`LfgAction`] the host carries out.

use std::collections::BTreeSet;

use anyps5_core::online::Online;
use anyps5_core::online::api::{
    self, Account, AccountId, LfgAsk, LfgMode, MAX_LFG_NOTE, MAX_LFG_SLOTS, ReportReason,
};
use anyps5_core::online::lfg::{self, Lfg};
use egui::{Color32, Context};

use crate::screens::lfg_strings::LfgText as T;
use crate::screens::rows::{self, Change, Row, RowId};
use crate::screens::side::{self, SideState, SideView};
use crate::ui::Shell;
use crate::ui::focus::PanelNav;
use crate::ui::i18n::{Arg, Language};
use crate::ui::icons::Icon;
use crate::ui::input::InputEvent;
use crate::ui::sound::Cue;

/// What the host carries out for the panel.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LfgAction {
    /// The panel closed (Back from the list, or the Back button).
    Close,
    /// Opens the sign-in (the Friends area).
    SignIn,
    /// Opens the Friends area on the invitations.
    OpenFriends,
    /// Asks the cloud for the listings again.
    Refresh,
    Post {
        mode: LfgMode,
        language: Option<String>,
        note: String,
        slots: u8,
    },
    CloseListing(String),
    Ask(String),
    Withdraw(String),
    Answer {
        listing: String,
        account: AccountId,
        accept: bool,
    },
    Report {
        listing: String,
        poster: AccountId,
        reason: ReportReason,
    },
    Block(AccountId),
}

/// The pages of the panel.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Page {
    List,
    Post,
    /// Someone else's listing, by ID.
    Listing(String),
    /// Why a listing is reported.
    Report(String),
    /// The player's own listing, by ID.
    Mine(String),
}

/// What the panel reads each frame.
pub struct LfgData<'a> {
    /// The game's name.
    pub game: &'a str,
    pub title_id: &'a str,
    pub online: &'a Online,
    /// Unix seconds.
    pub now: u64,
    pub language: Language,
    pub accent: Color32,
}

/// The panel and its drafts.
pub struct LfgPanel {
    side: SideState,
    page: Page,
    /// The filter choices of the list (0 is everything).
    mode: usize,
    language: usize,
    /// The draft of a new listing.
    draft_mode: usize,
    draft_language: usize,
    draft_slots: i32,
    draft_note: String,
    /// Listings the player reported or blocked the poster of: gone from the list at once.
    hidden: BTreeSet<String>,
}

impl Default for LfgPanel {
    fn default() -> Self {
        Self {
            side: SideState::default(),
            page: Page::List,
            mode: 0,
            language: 0,
            draft_mode: 0,
            draft_language: 0,
            draft_slots: 1,
            draft_note: String::new(),
            hidden: BTreeSet::new(),
        }
    }
}

/// The modes a listing can be posted for, in the order of the choice.
const MODES: [LfgMode; 3] = [LfgMode::Online, LfgMode::Lan, LfgMode::PlayTogether];

fn mode_text(mode: LfgMode) -> T {
    match mode {
        LfgMode::Lan => T::ModeLan,
        LfgMode::PlayTogether => T::ModePlayTogether,
        LfgMode::Online | LfgMode::Unknown => T::ModeOnline,
    }
}

/// The name of a language tag in that language itself.
pub fn language_name(tag: &str) -> &'static str {
    match tag {
        "en" => "English",
        "tr" => "Türkçe",
        "de" => "Deutsch",
        "fr" => "Français",
        "es" => "Español",
        _ => "—",
    }
}

fn language_label(tag: Option<&str>, language: Language) -> String {
    match tag {
        Some(tag) => language_name(tag).to_owned(),
        None => T::LanguageAny.text(language).to_owned(),
    }
}

/// The row a list row stands for: how many rows with this ID stand before the focused one.
pub fn item_at(rows: &[Row], id: RowId, focus: usize) -> Option<usize> {
    (rows.get(focus)?.id == id).then(|| rows[..focus].iter().filter(|r| r.id == id).count())
}

/// The facts of a listing in one line: places and time.
fn facts(listing: &api::Listing, data: &LfgData<'_>) -> String {
    let language = data.language;
    let places = T::PlacesFree.format(
        language,
        &[
            Arg::Int(i64::from(lfg::places_left(listing))),
            Arg::Int(i64::from(listing.slots)),
        ],
    );
    let minutes = T::MinutesLeft.format(
        language,
        &[Arg::Int(lfg::minutes_left(listing, data.now) as i64)],
    );
    let tongue = listing.language.as_deref().map(language_name);
    match tongue {
        Some(tongue) => format!("{tongue} · {places} · {minutes}"),
        None => format!("{places} · {minutes}"),
    }
}

impl LfgPanel {
    /// Opens on the list of a game.
    pub fn open(&mut self) {
        self.page = Page::List;
        self.side.open_at(0);
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

    pub fn page(&self) -> &Page {
        &self.page
    }

    fn go(&mut self, page: Page, row: usize) {
        self.page = page;
        self.side.nav = PanelNav::single(row);
    }

    /// Starts a draft: the launcher's language is the language the player most likely plays in.
    fn start_draft(&mut self, tag: Option<&str>) {
        self.draft_mode = 0;
        self.draft_language = lfg::language_index(tag);
        self.draft_slots = 1;
        self.draft_note.clear();
    }

    /// A page whose listing is gone leads back to the list.
    fn settle_page(&mut self, lfg: &Lfg) {
        let gone = match &self.page {
            Page::Listing(id) | Page::Report(id) | Page::Mine(id) => lfg.find(id).is_none(),
            Page::List | Page::Post => false,
        };
        if gone {
            self.go(Page::List, 0);
        }
    }

    /// The title, the subtitle and the rows of the page being shown.
    fn build(&self, data: &LfgData<'_>) -> (String, Option<String>, Vec<Row>) {
        let language = data.language;
        let t = |text: T| text.text(language).to_owned();
        let online = data.online;
        let me = online.account.as_ref().map(|a| a.id);
        let mut rows = Vec::new();
        if !online.is_signed_in() {
            rows.push(Row::info("signedOut", t(T::SignedOut), None));
            rows.push(Row::command("signIn", t(T::SignIn)).icon(Icon::SIGN_IN));
            return (data.game.to_owned(), Some(t(T::FindPlayers)), rows);
        }
        match &self.page {
            Page::List => {
                self.list_rows(data, me, &mut rows);
                (data.game.to_owned(), Some(t(T::FindPlayers)), rows)
            }
            Page::Post => {
                self.post_rows(data, &mut rows);
                (t(T::PostTitle), Some(data.game.to_owned()), rows)
            }
            Page::Listing(id) => {
                let title = online
                    .lfg
                    .find(id)
                    .map(|l| l.poster.handle.clone())
                    .unwrap_or_default();
                if let Some(listing) = online.lfg.find(id) {
                    self.listing_rows(data, listing, &mut rows);
                }
                (title, Some(data.game.to_owned()), rows)
            }
            Page::Report(_) => {
                rows.push(Row::command("reportSpam", t(T::ReportSpam)).icon(Icon::FLAG));
                rows.push(
                    Row::command("reportHarassment", t(T::ReportHarassment)).icon(Icon::FLAG),
                );
                rows.push(Row::command("reportOther", t(T::ReportOther)).icon(Icon::FLAG));
                (t(T::ReportListing), Some(t(T::ReportTitle)), rows)
            }
            Page::Mine(id) => {
                if let Some(listing) = online.lfg.find(id) {
                    Self::mine_rows(data, listing, &mut rows);
                }
                (t(T::YourListing), Some(data.game.to_owned()), rows)
            }
        }
    }

    fn list_rows(&self, data: &LfgData<'_>, me: Option<AccountId>, rows: &mut Vec<Row>) {
        let language = data.language;
        let t = |text: T| text.text(language).to_owned();
        let lfg = &data.online.lfg;
        match lfg.own_in(data.title_id) {
            Some(own) => {
                let detail = T::YourListingDetail.format(
                    language,
                    &[
                        Arg::Int(own.requests.len() as i64),
                        Arg::Int(lfg::minutes_left(own, data.now) as i64),
                    ],
                );
                rows.push(
                    Row::action("mine", t(T::YourListing))
                        .icon(Icon::USERS)
                        .detail(detail),
                );
            }
            None => rows.push(
                Row::command("post", t(T::PostListing))
                    .icon(Icon::PLUS)
                    .detail(t(T::PostListingDetail)),
            ),
        }
        rows.push(Row::caption("openListings", t(T::OpenListings)).group_start());
        let modes = [T::ModeAll, T::ModeOnline, T::ModeLan, T::ModePlayTogether]
            .map(t)
            .to_vec();
        rows.push(Row::choice(
            "filterMode",
            t(T::FilterMode),
            modes,
            self.mode.min(3),
        ));
        let mut languages = vec![t(T::LanguageAny)];
        languages.extend(
            lfg::LANGUAGES
                .iter()
                .map(|tag| language_name(tag).to_owned()),
        );
        rows.push(Row::choice(
            "filterLanguage",
            t(T::FilterLanguage),
            languages,
            self.language.min(lfg::LANGUAGES.len()),
        ));
        match &lfg.listings {
            None if lfg.failed => {
                rows.push(Row::info("failed", t(T::LoadFailed), None).group_start());
                rows.push(Row::command("retry", t(T::TryAgain)).icon(Icon::ARROWS_CLOCKWISE));
            }
            None => rows.push(Row::info("loading", t(T::Loading), None).group_start()),
            Some(all) => {
                let shown: Vec<&api::Listing> = lfg::filter(
                    all,
                    me,
                    lfg::mode_of(self.mode),
                    lfg::language_of(self.language),
                )
                .into_iter()
                .filter(|l| !self.hidden.contains(&l.id))
                .collect();
                if shown.is_empty() {
                    rows.push(Row::info("empty", t(T::NobodyTitle), None).group_start());
                    rows.push(Row::info("emptyNote", t(T::NobodyBody), None).attached());
                }
                for (n, listing) in shown.iter().enumerate() {
                    let title =
                        format!("{} · {}", listing.poster.handle, t(mode_text(listing.mode)));
                    let row = Row::action("listing", title)
                        .icon(Icon::USERS)
                        .detail(facts(listing, data));
                    rows.push(if n == 0 { row.group_start() } else { row });
                    if !listing.note.is_empty() {
                        rows.push(Row::info("listingNote", listing.note.clone(), None).attached());
                    }
                }
            }
        }
    }

    fn post_rows(&self, data: &LfgData<'_>, rows: &mut Vec<Row>) {
        let language = data.language;
        let t = |text: T| text.text(language).to_owned();
        let modes = MODES.map(|m| t(mode_text(m))).to_vec();
        rows.push(Row::choice(
            "postMode",
            t(T::Mode),
            modes,
            self.draft_mode.min(MODES.len() - 1),
        ));
        let mut languages = vec![t(T::LanguageAny)];
        languages.extend(
            lfg::LANGUAGES
                .iter()
                .map(|tag| language_name(tag).to_owned()),
        );
        rows.push(Row::choice(
            "postLanguage",
            t(T::Language),
            languages,
            self.draft_language.min(lfg::LANGUAGES.len()),
        ));
        let players = if self.draft_slots == 1 {
            t(T::PlayersOne)
        } else {
            T::PlayersMany.format(language, &[Arg::Int(i64::from(self.draft_slots))])
        };
        rows.push(Row::stepper(
            "postSlots",
            t(T::Players),
            self.draft_slots,
            (1, i32::from(MAX_LFG_SLOTS)),
            1,
            players,
        ));
        let count = self.draft_note.chars().count();
        rows.push(
            Row::text("postNote", t(T::Note), self.draft_note.clone())
                .icon(Icon::PENCIL_SIMPLE)
                .wide()
                .detail(T::NoteCount.format(
                    language,
                    &[Arg::Int(count as i64), Arg::Int(MAX_LFG_NOTE as i64)],
                )),
        );
        let mut post = Row::command("submit", t(T::PostNow))
            .icon(Icon::CHECK)
            .group_start();
        if let Some(code) = &data.online.lfg.post_error {
            post = post.detail(post_error_text(code, language));
        }
        rows.push(post);
    }

    fn listing_facts(data: &LfgData<'_>, listing: &api::Listing, rows: &mut Vec<Row>) {
        let language = data.language;
        let t = |text: T| text.text(language).to_owned();
        rows.push(Row::info(
            "mode",
            t(T::Mode),
            Some(t(mode_text(listing.mode))),
        ));
        rows.push(Row::info(
            "language",
            t(T::Language),
            Some(language_label(listing.language.as_deref(), language)),
        ));
        rows.push(Row::info(
            "places",
            t(T::Places),
            Some(T::PlacesFree.format(
                language,
                &[
                    Arg::Int(i64::from(lfg::places_left(listing))),
                    Arg::Int(i64::from(listing.slots)),
                ],
            )),
        ));
        rows.push(Row::info(
            "closes",
            t(T::ClosesIn),
            Some(T::MinutesLeft.format(
                language,
                &[Arg::Int(lfg::minutes_left(listing, data.now) as i64)],
            )),
        ));
        if listing.note.is_empty() {
            rows.push(Row::info("note", t(T::NoNote), None).group_start());
        } else {
            rows.push(Row::body("note", listing.note.clone()).group_start());
        }
    }

    fn listing_rows(&self, data: &LfgData<'_>, listing: &api::Listing, rows: &mut Vec<Row>) {
        let language = data.language;
        let t = |text: T| text.text(language).to_owned();
        Self::listing_facts(data, listing, rows);
        match listing.ask {
            None | Some(LfgAsk::Unknown) => {
                if lfg::places_left(listing) == 0 {
                    rows.push(Row::info("full", t(T::Full), None).group_start());
                } else {
                    rows.push(
                        Row::command("ask", t(T::AskToJoin))
                            .icon(Icon::USER_PLUS)
                            .detail(t(T::AskToJoinDetail))
                            .group_start(),
                    );
                }
            }
            Some(LfgAsk::Pending) => {
                rows.push(Row::info("asked", t(T::RequestSent), None).group_start());
                rows.push(Row::info("askedNote", t(T::RequestSentNote), None).attached());
                rows.push(Row::command("withdraw", t(T::Withdraw)).icon(Icon::X));
            }
            Some(LfgAsk::Accepted) => {
                rows.push(Row::info("accepted", t(T::Accepted), None).group_start());
                rows.push(Row::info("acceptedNote", t(T::AcceptedNote), None).attached());
                rows.push(Row::command("openFriends", t(T::OpenFriends)).icon(Icon::USERS));
            }
        }
        rows.push(
            Row::action("report", t(T::ReportListing))
                .icon(Icon::FLAG)
                .group_start(),
        );
        rows.push(
            Row::command(
                "block",
                T::BlockPoster.format(language, &[Arg::Str(&listing.poster.handle)]),
            )
            .icon(Icon::PROHIBIT)
            .destructive(),
        );
    }

    fn mine_rows(data: &LfgData<'_>, listing: &api::Listing, rows: &mut Vec<Row>) {
        let language = data.language;
        let t = |text: T| text.text(language).to_owned();
        Self::listing_facts(data, listing, rows);
        rows.push(Row::caption("requests", t(T::Requests)).group_start());
        if listing.requests.is_empty() {
            rows.push(Row::info("noRequests", t(T::NoRequests), None));
        }
        for request in &listing.requests {
            if request.accepted {
                rows.push(Row::info(
                    "accepted",
                    request.from.handle.clone(),
                    Some(t(T::Accepted)),
                ));
            } else {
                rows.push(
                    Row::buttons(
                        "request",
                        request.from.handle.clone(),
                        vec![t(T::Accept), t(T::Decline)],
                        0,
                    )
                    .icon(Icon::USER_PLUS),
                );
            }
        }
        rows.push(
            Row::command("close", t(T::CloseListing))
                .icon(Icon::X)
                .destructive()
                .group_start(),
        );
    }

    /// The accounts with an open request on the listing, in the order of the request rows.
    fn pending(listing: &api::Listing) -> Vec<&Account> {
        listing
            .requests
            .iter()
            .filter(|r| !r.accepted)
            .map(|r| &r.from)
            .collect()
    }

    /// One frame: the commands, the drawing, the pointer. Returns what the host must do.
    pub fn show(
        &mut self,
        shell: &mut Shell,
        ctx: &Context,
        events: &[InputEvent],
        data: &LfgData<'_>,
    ) -> Vec<LfgAction> {
        let mut actions = Vec::new();
        if !self.side.is_active() {
            return actions;
        }
        if self.side.is_open() {
            self.settle_page(&data.online.lfg);
            let (_, _, rows) = self.build(data);
            self.side.nav.row = side::settle(&rows::kinds(&rows), self.side.nav.row);
            for command in side::commands(events) {
                let driven = rows::drive(shell, &mut self.side.nav, &rows, &[command]);
                for change in &driven.changes {
                    self.handle(ctx, change, data, &rows, &mut actions);
                }
                if driven.close {
                    self.back(shell, false, &mut actions);
                    break;
                }
            }
            if !self.side.is_active() {
                return actions;
            }
        }
        self.settle_page(&data.online.lfg);
        let (title, subtitle, mut rows) = self.build(data);
        let list_id = match self.page {
            Page::List => "lfg-list",
            Page::Post => "lfg-post",
            Page::Listing(_) | Page::Mine(_) => "lfg-listing",
            Page::Report(_) => "lfg-report",
        };
        let view = SideView {
            title: &title,
            subtitle: subtitle.as_deref(),
            question: false,
            list_id,
            nav: &self.side.nav,
            accent: data.accent,
            language: data.language,
            force: None,
        };
        match side::draw(ctx, self.side.is_open(), &view, &mut rows) {
            Some(drawn) if self.side.is_open() => {
                let (changes, texts) =
                    rows::apply_mouse(shell, &mut self.side.nav, &rows, drawn.events);
                for (row, value) in texts {
                    if row == "postNote" {
                        self.draft_note = value.chars().take(MAX_LFG_NOTE + 20).collect();
                    }
                }
                for change in &changes {
                    self.handle(ctx, change, data, &rows, &mut actions);
                }
                if drawn.back_clicked {
                    self.back(shell, true, &mut actions);
                }
            }
            Some(_) => {}
            None => self.side.finished(),
        }
        actions
    }

    /// One step back: up to the list, or out of the panel from the list.
    fn back(&mut self, shell: &mut Shell, clicked: bool, actions: &mut Vec<LfgAction>) {
        if self.page == Page::List {
            if clicked {
                shell.play(Cue::Close);
            }
            self.side.close();
            actions.push(LfgAction::Close);
        } else {
            if clicked {
                shell.play(Cue::Back);
            }
            self.go(Page::List, 0);
            actions.push(LfgAction::Refresh);
        }
    }

    fn handle(
        &mut self,
        ctx: &Context,
        change: &Change,
        data: &LfgData<'_>,
        rows: &[Row],
        actions: &mut Vec<LfgAction>,
    ) {
        let lfg = &data.online.lfg;
        let focus = self.side.nav.row;
        if matches!(change, Change::Run { row: "signIn" }) {
            actions.push(LfgAction::SignIn);
            return;
        }
        match self.page.clone() {
            Page::List => match change {
                Change::Run { row: "post" } => {
                    self.start_draft(Some(data.language.code()));
                    self.go(Page::Post, 0);
                }
                Change::Run { row: "mine" } => {
                    if let Some(own) = lfg.own_in(data.title_id) {
                        self.go(Page::Mine(own.id.clone()), 0);
                    }
                }
                Change::Run { row: "retry" } => actions.push(LfgAction::Refresh),
                Change::Choice {
                    row: "filterMode",
                    index,
                } => self.mode = *index,
                Change::Choice {
                    row: "filterLanguage",
                    index,
                } => self.language = *index,
                Change::Run { row: "listing" } => {
                    let me = data.online.account.as_ref().map(|a| a.id);
                    let shown = self.shown(data, me);
                    if let Some(n) = item_at(rows, "listing", focus)
                        && let Some(listing) = shown.get(n)
                    {
                        self.go(Page::Listing(listing.id.clone()), 0);
                    }
                }
                _ => {}
            },
            Page::Post => match change {
                Change::Choice {
                    row: "postMode",
                    index,
                } => self.draft_mode = *index,
                Change::Choice {
                    row: "postLanguage",
                    index,
                } => self.draft_language = *index,
                Change::Stepper {
                    row: "postSlots",
                    value,
                } => self.draft_slots = (*value).clamp(1, i32::from(MAX_LFG_SLOTS)),
                // The caret goes to the field; typing arrives as text events.
                Change::Edit { .. } => {
                    ctx.data_mut(|d| d.insert_temp(egui::Id::unique("panel-text-focus"), true));
                }
                Change::Run { row: "submit" } | Change::Submit { row: "postNote" } => {
                    let note = self.draft_note.trim().to_owned();
                    if lfg::note_ok(&note) {
                        actions.push(LfgAction::Post {
                            mode: MODES[self.draft_mode.min(MODES.len() - 1)],
                            language: lfg::language_of(self.draft_language).map(str::to_owned),
                            note,
                            slots: u8::try_from(self.draft_slots)
                                .unwrap_or(1)
                                .clamp(1, MAX_LFG_SLOTS),
                        });
                    }
                }
                _ => {}
            },
            Page::Listing(id) => match change {
                Change::Run { row: "ask" } => actions.push(LfgAction::Ask(id)),
                Change::Run { row: "withdraw" } => actions.push(LfgAction::Withdraw(id)),
                Change::Run { row: "openFriends" } => actions.push(LfgAction::OpenFriends),
                Change::Run { row: "report" } => self.go(Page::Report(id), 0),
                Change::Run { row: "block" } => {
                    if let Some(listing) = lfg.find(&id) {
                        actions.push(LfgAction::Block(listing.poster.id));
                        self.hidden.insert(id);
                        self.go(Page::List, 1);
                    }
                }
                _ => {}
            },
            Page::Report(id) => {
                let reason = match change {
                    Change::Run { row: "reportSpam" } => Some(ReportReason::Spam),
                    Change::Run {
                        row: "reportHarassment",
                    } => Some(ReportReason::Harassment),
                    Change::Run { row: "reportOther" } => Some(ReportReason::Other),
                    _ => None,
                };
                if let Some(reason) = reason
                    && let Some(listing) = lfg.find(&id)
                {
                    actions.push(LfgAction::Report {
                        listing: id.clone(),
                        poster: listing.poster.id,
                        reason,
                    });
                    self.hidden.insert(id);
                    self.go(Page::List, 1);
                }
            }
            Page::Mine(id) => match change {
                Change::Press {
                    row: "request",
                    index,
                } if *index < 2 => {
                    if let Some(listing) = lfg.find(&id)
                        && let Some(n) = item_at(rows, "request", focus)
                        && let Some(account) = Self::pending(listing).get(n)
                    {
                        actions.push(LfgAction::Answer {
                            listing: id,
                            account: account.id,
                            accept: *index == 0,
                        });
                    }
                }
                Change::Run { row: "close" } => {
                    actions.push(LfgAction::CloseListing(id));
                    self.go(Page::List, 0);
                }
                _ => {}
            },
        }
    }

    /// The listings the list shows, after the filters and the ones the player hid.
    fn shown<'a>(&self, data: &'a LfgData<'_>, me: Option<AccountId>) -> Vec<&'a api::Listing> {
        let Some(all) = &data.online.lfg.listings else {
            return Vec::new();
        };
        lfg::filter(
            all,
            me,
            lfg::mode_of(self.mode),
            lfg::language_of(self.language),
        )
        .into_iter()
        .filter(|l| !self.hidden.contains(&l.id))
        .collect()
    }

    /// A listing the player posted was accepted by the cloud: back to the list, which now shows
    /// "Your Listing".
    pub fn posted(&mut self) {
        if self.page == Page::Post {
            self.go(Page::List, 0);
        }
    }
}

/// Why the cloud turned a post away, in words.
pub fn post_error_text(code: &str, language: Language) -> String {
    match code {
        "limit" | "full" => T::PostLimit,
        "rate_limited" => T::PostRate,
        "restricted" | "banned" | "forbidden" => T::PostRestricted,
        "bad_request" => T::PostBadNote,
        _ => T::PostFailed,
    }
    .text(language)
    .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyps5_core::online::api::{LfgRequest, Listing};

    fn account(id: u64, handle: &str) -> Account {
        Account {
            id: AccountId(id),
            handle: handle.into(),
        }
    }

    fn listing(id: &str, poster: u64) -> Listing {
        Listing {
            id: id.into(),
            poster: account(poster, &format!("p{poster}")),
            title_id: "GAME00001".into(),
            mode: LfgMode::Online,
            language: Some("en".into()),
            note: "Chill run".into(),
            slots: 3,
            filled: 1,
            created: 1000,
            expires: 4600,
            ask: None,
            requests: Vec::new(),
        }
    }

    fn data<'a>(online: &'a Online) -> LfgData<'a> {
        LfgData {
            game: "A Game",
            title_id: "GAME00001",
            online,
            now: 1000,
            language: Language::English,
            accent: Color32::WHITE,
        }
    }

    fn signed_in() -> Online {
        let mut online = Online::default();
        let account = anyps5_core::online::SignedIn {
            id: AccountId(1),
            handle: "me".into(),
            email: String::new(),
            appear_offline: false,
        };
        online.account = Some(account);
        online
    }

    #[test]
    fn a_list_row_knows_its_place_among_the_listings() {
        let rows = vec![
            Row::command("post", "Post"),
            Row::action("listing", "a"),
            Row::info("listingNote", "n", None).attached(),
            Row::action("listing", "b"),
        ];
        assert_eq!(item_at(&rows, "listing", 1), Some(0));
        assert_eq!(item_at(&rows, "listing", 3), Some(1));
        assert_eq!(item_at(&rows, "listing", 0), None);
        assert_eq!(item_at(&rows, "listing", 9), None);
    }

    #[test]
    fn signed_out_the_panel_says_so_and_offers_sign_in() {
        let online = Online::default();
        let panel = LfgPanel::default();
        let (_, _, rows) = panel.build(&data(&online));
        let ids: Vec<_> = rows.iter().map(|r| r.id).collect();
        assert_eq!(ids, ["signedOut", "signIn"]);
    }

    #[test]
    fn the_list_offers_posting_until_the_player_has_a_listing_in_the_game() {
        let mut online = signed_in();
        online.lfg.open("GAME00001");
        online.lfg.listings = Some(vec![listing("a", 2)]);
        let panel = LfgPanel::default();
        let (_, _, rows) = panel.build(&data(&online));
        let ids: Vec<_> = rows.iter().map(|r| r.id).collect();
        assert_eq!(
            ids,
            [
                "post",
                "openListings",
                "filterMode",
                "filterLanguage",
                "listing",
                "listingNote"
            ]
        );
        online.lfg.mine = Some(api::LfgMine {
            posted: vec![listing("mine", 1)],
            asked: vec![],
        });
        let (_, _, rows) = panel.build(&data(&online));
        assert_eq!(rows[0].id, "mine");
    }

    #[test]
    fn a_listing_with_nobody_around_says_so() {
        let mut online = signed_in();
        online.lfg.open("GAME00001");
        online.lfg.listings = Some(vec![]);
        let panel = LfgPanel::default();
        let (_, _, rows) = panel.build(&data(&online));
        assert!(rows.iter().any(|r| r.id == "empty"));
        assert!(rows.iter().all(|r| r.id != "listing"));
    }

    #[test]
    fn a_reported_listing_leaves_the_list_at_once() {
        let mut online = signed_in();
        online.lfg.open("GAME00001");
        online.lfg.listings = Some(vec![listing("a", 2), listing("b", 3)]);
        let mut panel = LfgPanel::default();
        panel.hidden.insert("a".into());
        let (_, _, rows) = panel.build(&data(&online));
        assert_eq!(rows.iter().filter(|r| r.id == "listing").count(), 1);
    }

    #[test]
    fn the_foreign_page_offers_the_one_action_that_fits_the_state_of_the_request() {
        let mut online = signed_in();
        online.lfg.open("GAME00001");
        let mut l = listing("a", 2);
        online.lfg.listings = Some(vec![l.clone()]);
        let panel = LfgPanel {
            page: Page::Listing("a".into()),
            ..LfgPanel::default()
        };
        let has = |online: &Online, id: &str| {
            let (_, _, rows) = panel.build(&data(online));
            rows.iter().any(|r| r.id == id)
        };
        assert!(has(&online, "ask"));
        l.ask = Some(LfgAsk::Pending);
        online.lfg.listings = Some(vec![l.clone()]);
        assert!(has(&online, "withdraw") && !has(&online, "ask"));
        l.ask = Some(LfgAsk::Accepted);
        online.lfg.listings = Some(vec![l.clone()]);
        assert!(has(&online, "openFriends"));
        l.ask = None;
        l.filled = 3;
        online.lfg.listings = Some(vec![l]);
        assert!(has(&online, "full") && !has(&online, "ask"));
    }

    #[test]
    fn your_own_page_lists_the_requests_with_accept_and_decline() {
        let mut online = signed_in();
        online.lfg.open("GAME00001");
        let mut own = listing("mine", 1);
        own.requests = vec![
            LfgRequest {
                from: account(5, "Ask"),
                created: 1,
                accepted: false,
            },
            LfgRequest {
                from: account(6, "In"),
                created: 2,
                accepted: true,
            },
        ];
        online.lfg.mine = Some(api::LfgMine {
            posted: vec![own],
            asked: vec![],
        });
        let panel = LfgPanel {
            page: Page::Mine("mine".into()),
            ..LfgPanel::default()
        };
        let (_, _, rows) = panel.build(&data(&online));
        let request = rows.iter().find(|r| r.id == "request").expect("a request");
        assert_eq!(
            request.control,
            rows::Control::Buttons {
                labels: vec!["Accept".into(), "Decline".into()],
                selected: 0
            }
        );
        assert_eq!(rows.iter().filter(|r| r.id == "accepted").count(), 1);
        assert_eq!(rows.last().map(|r| r.id), Some("close"));
    }

    #[test]
    fn a_page_whose_listing_is_gone_leads_back_to_the_list() {
        let mut panel = LfgPanel {
            page: Page::Listing("zzz".into()),
            ..LfgPanel::default()
        };
        panel.settle_page(&Lfg::default());
        assert_eq!(panel.page, Page::List);
    }

    #[test]
    fn the_post_page_counts_the_note_and_says_why_a_post_was_turned_away() {
        let mut online = signed_in();
        online.lfg.open("GAME00001");
        let panel = LfgPanel {
            page: Page::Post,
            draft_note: "hello".into(),
            ..LfgPanel::default()
        };
        let (_, _, rows) = panel.build(&data(&online));
        let note = rows.iter().find(|r| r.id == "postNote").expect("note");
        assert_eq!(note.detail.as_deref(), Some("5 of 140 characters"));
        online.lfg.post_error = Some("limit".into());
        let (_, _, rows) = panel.build(&data(&online));
        let submit = rows.iter().find(|r| r.id == "submit").expect("submit");
        assert!(submit.detail.as_deref().unwrap_or("").contains("Close one"));
        assert_eq!(
            post_error_text("rate_limited", Language::English),
            "You posted a lot of listings. Try again in a few minutes."
        );
    }

    #[test]
    fn the_draft_starts_in_the_launcher_language() {
        let mut panel = LfgPanel::default();
        panel.start_draft(Some("tr"));
        assert_eq!(lfg::language_of(panel.draft_language), Some("tr"));
        panel.start_draft(Some("xx"));
        assert_eq!(lfg::language_of(panel.draft_language), None);
    }
}
