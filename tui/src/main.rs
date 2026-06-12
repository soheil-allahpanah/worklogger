mod app;
mod components;
mod dialogs;
mod export;
mod format;
mod message;
mod search_dsl;
mod theme;
mod ui;

use std::io::{self, stdout, Stdout};
use std::sync::Arc;

use app::{run_terminal, App};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use infrastructure::postgres::{connect, PostgresWorklogRepository};
use ratatui::prelude::*;
use use_cases::{
    CreateWorklogUseCase, DeleteWorklogUseCase, ExportWorklogsUsecase, FilterWorklogsUsecase,
    GetWorklogUseCase,
};

type TuiApp = App<Arc<PostgresWorklogRepository>>;

fn main() -> io::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let handle = rt.handle().clone();

    let mut terminal = setup_terminal()?;
    let app = rt.block_on(wiringup())?;
    let result = run_terminal(&mut terminal, &handle, app);
    restore_terminal(&mut terminal)?;
    result
}

async fn wiringup() -> io::Result<TuiApp> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "DATABASE_URL must be set"))?;
    let pool = connect(&database_url)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to connect: {e}")))?;

    let repo = Arc::new(PostgresWorklogRepository::new(pool));
    let create_worklog = CreateWorklogUseCase::new(Arc::clone(&repo));
    let filter_worklogs = FilterWorklogsUsecase::new(Arc::clone(&repo));
    let export_worklogs = ExportWorklogsUsecase::new(Arc::clone(&repo));
    let delete_worklog = DeleteWorklogUseCase::new(Arc::clone(&repo));
    let get_worklog = GetWorklogUseCase::new(Arc::clone(&repo));
    App::new(
        create_worklog,
        filter_worklogs,
        export_worklogs,
        delete_worklog,
        get_worklog,
    )
    .await
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(out);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
