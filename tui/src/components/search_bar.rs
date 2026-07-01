//! The bottom search bar: its messages, update and view. The search query lives
//! on the shared model ([`App::search_input`]) since the table view depends on it.

use std::io;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::{App, Mode};
use crate::message::Outcome;
use crate::theme;

/// Messages the search bar understands.
#[derive(Debug, Clone)]
pub enum Msg {
    Input(char),
    Backspace,
    Submit,
    Cancel,
}

/// Translates a key press into a search-bar message.
pub fn from_key(key: KeyEvent) -> Option<Msg> {
    match key.code {
        KeyCode::Esc => Some(Msg::Cancel),
        KeyCode::Enter => Some(Msg::Submit),
        KeyCode::Backspace => Some(Msg::Backspace),
        KeyCode::Char(c) => Some(Msg::Input(c)),
        _ => None,
    }
}

/// Applies a message to the model, running effects as needed.
pub async fn update(app: &mut App, msg: Msg) -> io::Result<Outcome> {
    match msg {
        Msg::Cancel => app.mode = Mode::Normal,
        Msg::Submit => {
            app.reset_page();
            app.apply_search().await?;
            app.mode = Mode::Normal;
        }
        Msg::Backspace => {
            app.search_input.pop();
        }
        Msg::Input(c) => app.search_input.push(c),
    }
    Ok(Outcome::Continue)
}

/// Renders the search bar into `area`.
pub fn view(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(theme::BORDER))
        .style(Style::default().bg(theme::SURFACE));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let query = if app.search_input.is_empty() && app.mode != Mode::Search {
        Span::styled(
            "tag:rust desc:\"meeting\" date:1403/01/01..",
            Style::default().fg(theme::MUTED),
        )
    } else {
        Span::styled(app.search_input.as_str(), Style::default().fg(theme::TEXT))
    };

    let cursor = if app.mode == Mode::Search && app.cursor_visible {
        Span::styled("█", Style::default().fg(theme::EMERALD).bg(theme::SURFACE))
    } else {
        Span::raw("")
    };

    let bar = Paragraph::new(Line::from(vec![
        Span::styled("/", Style::default().fg(theme::BLUE).bold()),
        query,
        cursor,
    ]))
    .style(Style::default().bg(theme::SURFACE));

    frame.render_widget(bar, inner);
}
