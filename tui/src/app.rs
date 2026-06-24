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
use domain::bootstrap::legacy_user_id;
use domain::traits::WorklogRepository;
use domain::value_objects::UserId;
use ratatui::widgets::TableState;
use tokio::runtime::Handle;
use use_cases::{
    CreateWorklogUseCase, DeleteWorklogUseCase, ExportWorklogsUsecase, FilterWorklogsUsecase,
    GetWorklogUseCase,
};
use uuid::Uuid;

use crate::components::{search_bar, table};
use crate::dialogs::{add, delete, open};
use crate::export::{export_dir, write_export_file};
use crate::format::{format_duration, jalali_date_string};
use crate::message::{Msg, Outcome};
use crate::search_dsl::parse_search_input;
use crate::ui;

/// The active screen / route. New pages can be added here in future iterations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Search,
    AddModal,
    DeleteModal,
    OpenModal,
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

/// The whole application model.
pub struct App<R: WorklogRepository> {
    // Effect runners (use cases).
    pub create_worklog_usecase: CreateWorklogUseCase<R>,
    pub filter_worklogs_usecase: FilterWorklogsUsecase<R>,
    pub export_worklogs_usecase: ExportWorklogsUsecase<R>,
    pub delete_worklog_usecase: DeleteWorklogUseCase<R>,
    pub get_worklog_usecase: GetWorklogUseCase<R>,

    pub user_id: UserId,

    // Shared state.
    pub mode: Mode,
    pub rows: Vec<WorklogRow>,
    pub total_entries: usize,
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

impl<R: WorklogRepository> App<R> {
    pub async fn new(
        create_worklog_usecase: CreateWorklogUseCase<R>,
        filter_worklogs_usecase: FilterWorklogsUsecase<R>,
        export_worklogs_usecase: ExportWorklogsUsecase<R>,
        delete_worklog_usecase: DeleteWorklogUseCase<R>,
        get_worklog_usecase: GetWorklogUseCase<R>,
    ) -> io::Result<Self> {
        let mut app = Self {
            create_worklog_usecase,
            filter_worklogs_usecase,
            export_worklogs_usecase,
            delete_worklog_usecase,
            get_worklog_usecase,
            user_id: legacy_user_id(),
            mode: Mode::Normal,
            rows: Vec::new(),
            total_entries: 0,
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
        let mut command = parse_search_input(&self.search_input, self.user_id);
        if let Err(errors) = command.validate() {
            self.set_status(format!("Filter: {}", errors.join("; ")), 4);
            return Ok(());
        }

        match self.filter_worklogs_usecase.execute(command).await {
            Ok(page) => {
                self.total_entries = page.total_items as usize;
                self.rows = page.items.iter().map(worklog_to_row).collect();
                if self.rows.is_empty() {
                    self.table_state.select(None);
                } else {
                    self.table_state.select(Some(0));
                }
                self.set_status(format!("{} worklog(s) matched", page.total_items), 2);
            }
            Err(err) => self.set_status(format!("{err}"), 4),
        }
        Ok(())
    }

    pub async fn reload_worklogs(&mut self) -> io::Result<()> {
        self.apply_search().await
    }

    /// Exports the current search results to an Excel file under [`export_dir`].
    pub async fn export_search_results(&mut self) -> io::Result<()> {
        let mut command = parse_search_input(&self.search_input, self.user_id);
        if let Err(errors) = command.validate() {
            self.set_status(format!("Filter: {}", errors.join("; ")), 4);
            return Ok(());
        }

        match self.export_worklogs_usecase.execute(command).await {
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
            Err(err) => self.set_status(format!("{err}"), 4),
        }
        Ok(())
    }

    /// Shows a transient status message that clears after `seconds`.
    pub fn set_status(&mut self, message: String, seconds: u64) {
        self.status_message = Some(message);
        self.status_clear_at = Some(Instant::now() + Duration::from_secs(seconds));
    }
}

/// Translates a key press into a [`Msg`], routing to the active screen.
pub fn from_key<R: WorklogRepository>(app: &App<R>, key: KeyEvent) -> Option<Msg> {
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
pub async fn update<R: WorklogRepository>(app: &mut App<R>, msg: Msg) -> io::Result<Outcome> {
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
pub fn run_terminal<R: WorklogRepository>(
    terminal: &mut ratatui::DefaultTerminal,
    handle: &Handle,
    mut app: App<R>,
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
