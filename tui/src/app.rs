//! The application model and the Elm-style runtime: input → message → update → view.
//!
//! [`App`] is the model. Input is translated to a [`Msg`] by [`from_key`], and
//! [`update`] consumes it (mutating the model and running async effects). Each
//! screen and dialog owns its own slice of state plus its `Msg`/`update`/`view`
//! (see [`crate::components`] and [`crate::dialogs`]); this module just wires
//! them together and owns state shared across screens.

use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use domain::entities::Worklog;
use domain::value_objects::UserId;
use ratatui::widgets::TableState;
use sdk::{SdkError, WorkloggerClient};
use tokio::runtime::Handle;
use uuid::Uuid;

use crate::components::{search_bar, table};
use crate::dialogs::{add, delete, open};
use crate::export::{export_dir, write_export_file};
use crate::format::{format_duration, jalali_date_string};
use crate::message::{Msg, Outcome};
use crate::search_dsl::parse_search_input;
use crate::ui;

/// Default number of worklogs fetched per API page.
pub const DEFAULT_PAGE_SIZE: u32 = 40;

/// The active screen / route. New pages can be added here in future iterations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Search,
    AddModal,
    DeleteModal,
    OpenModal,
}

impl Mode {
    pub fn is_modal(self) -> bool {
        matches!(
            self,
            Mode::AddModal | Mode::DeleteModal | Mode::OpenModal
        )
    }
}

/// A worklog flattened into display strings for the table and detail views.
#[derive(Clone, Debug)]
pub struct WorklogRow {
    pub id: Uuid,
    pub date: String,
    pub duration: String,
    pub description: String,
    pub tags: String,
}

/// Placeholder user id for command DTOs. The API resolves the real user from the bearer token.
pub(crate) fn command_user_id() -> UserId {
    UserId::from_uuid(Uuid::nil())
}

/// The whole application model.
pub struct App {
    pub client: WorkloggerClient,

    // Shared state.
    pub mode: Mode,
    pub rows: Vec<WorklogRow>,
    pub total_entries: usize,
    pub current_page: u32,
    pub total_pages: u32,
    pub page_size: u32,
    pub table_state: TableState,
    pub search_input: String,
    pub cursor_visible: bool,
    pub last_tick: Instant,
    pub status_message: Option<String>,
    pub status_clear_at: Option<Instant>,

    // Per-dialog state.
    pub add: add::Model,
    pub delete: delete::Model,
    pub open: open::Model,
}

impl App {
    pub async fn new(client: WorkloggerClient) -> io::Result<Self> {
        let mut app = Self {
            client,
            mode: Mode::Normal,
            rows: Vec::new(),
            total_entries: 0,
            current_page: 1,
            total_pages: 1,
            page_size: DEFAULT_PAGE_SIZE,
            table_state: TableState::default(),
            search_input: String::new(),
            cursor_visible: true,
            last_tick: Instant::now(),
            status_message: None,
            status_clear_at: None,
            add: add::Model::fresh(),
            delete: delete::Model::default(),
            open: open::Model::default(),
        };
        app.reload_worklogs().await?;
        Ok(app)
    }

    /// Advances time-based state: cursor blink and status-message expiry.
    pub fn tick(&mut self) {
        if self.last_tick.elapsed() >= Duration::from_millis(530) {
            self.cursor_visible = !self.cursor_visible;
            self.last_tick = Instant::now();
        }
        if let Some(clear_at) = self.status_clear_at {
            if Instant::now() >= clear_at {
                self.status_message = None;
                self.status_clear_at = None;
            }
        }
    }

    /// Runs the current search query and refreshes the table rows. Shared effect
    /// used by the search bar as well as the add/delete dialogs.
    pub async fn apply_search(&mut self) -> io::Result<()> {
        let mut command = parse_search_input(&self.search_input);
        command.paging.page = self.current_page;
        command.paging.size = self.page_size;
        if let Err(errors) = command.validate() {
            self.set_status(format!("Filter: {}", errors.join("; ")), 4);
            return Ok(());
        }

        match self.client.filter_worklogs(command).await {
            Ok(page) => {
                self.total_entries = page.total_items as usize;
                self.total_pages = page.total_pages.max(1);
                self.current_page = page.current_page.max(1);
                self.rows = page.items.iter().map(worklog_to_row).collect();
                if self.rows.is_empty() {
                    self.table_state.select(None);
                } else if self.table_state.selected().is_none() {
                    self.table_state.select(Some(0));
                }
                let status = if self.total_pages > 1 {
                    format!(
                        "{} worklog(s) matched · page {}/{}",
                        page.total_items, self.current_page, self.total_pages
                    )
                } else {
                    format!("{} worklog(s) matched", page.total_items)
                };
                self.set_status(status, 2);
            }
            Err(err) => self.set_status(sdk_error_message(&err), 4),
        }
        Ok(())
    }

    pub fn reset_page(&mut self) {
        self.current_page = 1;
    }

    pub async fn reload_worklogs(&mut self) -> io::Result<()> {
        self.apply_search().await
    }

    /// Exports the current search results to an Excel file under [`export_dir`].
    pub async fn export_search_results(&mut self) -> io::Result<()> {
        let mut command = parse_search_input(&self.search_input);
        if let Err(errors) = command.validate() {
            self.set_status(format!("Filter: {}", errors.join("; ")), 4);
            return Ok(());
        }

        match self.client.export_worklogs(command).await {
            Ok(file) => {
                let path = write_export_file(&export_dir(), &file.filename, &file.bytes)?;
                self.set_status(
                    format!(
                        "Exported {} worklog(s) to {}",
                        file.row_count,
                        path.display()
                    ),
                    4,
                );
            }
            Err(err) => self.set_status(sdk_error_message(&err), 4),
        }
        Ok(())
    }

    /// Shows a transient status message that clears after `seconds`.
    pub fn set_status(&mut self, message: String, seconds: u64) {
        self.status_message = Some(message);
        self.status_clear_at = Some(Instant::now() + Duration::from_secs(seconds));
    }
}

fn sdk_error_message(err: &SdkError) -> String {
    err.to_string()
}

/// Translates a key press into a [`Msg`], routing to the active screen.
pub fn from_key(app: &App, key: KeyEvent) -> Option<Msg> {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Some(Msg::Quit);
    }

    match app.mode {
        Mode::Normal => table::from_key(key).map(Msg::Table),
        Mode::Search => search_bar::from_key(key).map(Msg::Search),
        Mode::AddModal => add::from_key(key).map(Msg::Add),
        Mode::DeleteModal => delete::from_key(key).map(Msg::Delete),
        Mode::OpenModal => open::from_key(key).map(Msg::Open),
    }
}

/// Applies a [`Msg`] to the model, routing to the owning screen's update.
pub async fn update(app: &mut App, msg: Msg) -> io::Result<Outcome> {
    match msg {
        Msg::Tick => {
            app.tick();
            Ok(Outcome::Continue)
        }
        Msg::Quit => Ok(Outcome::Quit),
        Msg::Table(m) => table::update(app, m).await,
        Msg::Search(m) => search_bar::update(app, m).await,
        Msg::Add(m) => add::update(app, m).await,
        Msg::Delete(m) => delete::update(app, m).await,
        Msg::Open(m) => open::update(app, m).await,
    }
}

/// Flattens a domain [`Worklog`] into a display [`WorklogRow`].
pub fn worklog_to_row(worklog: &Worklog) -> WorklogRow {
    WorklogRow {
        id: worklog.id().as_uuid(),
        date: jalali_date_string(worklog.datetime().as_datetime()),
        duration: format_duration(worklog.duration()),
        description: worklog
            .description()
            .map(|d| d.as_str().to_string())
            .unwrap_or_default(),
        tags: worklog
            .tags()
            .iter()
            .map(|t| t.as_str())
            .collect::<Vec<_>>()
            .join(", "),
    }
}

/// Sync crossterm loop; async work runs on the Tokio runtime via `handle`.
pub fn run_terminal(
    terminal: &mut ratatui::DefaultTerminal,
    handle: &Handle,
    mut app: App,
) -> io::Result<()> {
    loop {
        app.tick();
        terminal.draw(|frame| ui::view(frame, &mut app))?;

        if event::poll(Duration::from_millis(120))? {
            if let Event::Key(key) = event::read()? {
                if let Some(msg) = from_key(&app, key) {
                    if handle.block_on(update(&mut app, msg))? == Outcome::Quit {
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}
