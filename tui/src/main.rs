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

use app::{run_terminal, App};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use sdk::WorkloggerClient;

fn main() -> io::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let handle = rt.handle().clone();

    let mut terminal = setup_terminal()?;
    let app = rt.block_on(wiringup())?;
    let result = run_terminal(&mut terminal, &handle, app);
    restore_terminal(&mut terminal)?;
    result
}

async fn wiringup() -> io::Result<App> {
    let base_url = std::env::var("WORKLOGGER_BASE_URL").map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "WORKLOGGER_BASE_URL must be set (e.g. http://127.0.0.1:3000)",
        )
    })?;
    let token = std::env::var("WORKLOGGER_TOKEN").map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "WORKLOGGER_TOKEN must be set to a device API token",
        )
    })?;

    let client = WorkloggerClient::builder()
        .base_url(base_url)
        .token(token)
        .build()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;

    client
        .health()
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("API health check failed: {e}")))?;

    App::new(client).await
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
