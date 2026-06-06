//! The "delete worklog" confirmation dialog: model, messages, update and view.

use std::io;

use crossterm::event::{KeyCode, KeyEvent};
use domain::traits::WorklogRepository;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{block::Position, Block, BorderType, Borders, Paragraph},
    Frame,
};
use use_cases::DeleteWorklogCommand;
use uuid::Uuid;

use crate::app::{App, Mode};
use crate::message::Outcome;
use crate::theme;

/// The model for the delete dialog: the worklog awaiting confirmation.
#[derive(Default, Clone)]
pub struct Model {
    pub target: Option<Uuid>,
}

/// Messages the delete dialog understands.
#[derive(Debug, Clone)]
pub enum Msg {
    Confirm,
    Cancel,
}

/// Translates a key press into a dialog message.
pub fn from_key(key: KeyEvent) -> Option<Msg> {
    match key.code {
        KeyCode::Esc => Some(Msg::Cancel),
        KeyCode::Enter => Some(Msg::Confirm),
        _ => None,
    }
}

/// Applies a message to the model, running effects as needed.
pub async fn update<R: WorklogRepository>(
    app: &mut App<R>,
    msg: Msg,
) -> io::Result<Outcome> {
    match msg {
        Msg::Cancel => {
            app.delete.target = None;
            app.mode = Mode::Normal;
        }
        Msg::Confirm => {
            if let Some(id) = app.delete.target {
                app.delete_worklog_usecase
                    .execute(DeleteWorklogCommand { id })
                    .await
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                app.reload_worklogs().await?;
                app.set_status("Worklog deleted".into(), 1);
            }
            app.delete.target = None;
            app.mode = Mode::Normal;
        }
    }
    Ok(Outcome::Continue)
}

/// Renders the delete confirmation dialog over the current frame.
pub fn view<R: WorklogRepository>(frame: &mut Frame, _app: &App<R>) {
    let area = theme::centered_rect(40, 20, frame.area());
    let block = Block::default()
        .title(" Deleting Worklog ")
        .title_alignment(Alignment::Center)
        .title_style(Style::default().fg(theme::BLUE).add_modifier(Modifier::BOLD))
        .title_position(Position::Top)
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(theme::BORDER))
        .style(Style::default().bg(theme::SURFACE));
    frame.render_widget(block, area);

    let inner = theme::centered_rect(36, 18, frame.area());
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Length(2)])
        .split(inner);

    let confirmation = Paragraph::new(Line::from(vec![Span::styled(
        "Are you sure you want to delete this worklog?",
        Style::default().fg(theme::TEXT),
    )]))
    .alignment(Alignment::Center);
    frame.render_widget(confirmation, chunks[0]);

    let help = Paragraph::new(Line::from(vec![
        Span::styled("Enter", Style::default().fg(theme::EMERALD)),
        Span::styled(" delete · ", Style::default().fg(theme::MUTED)),
        Span::styled("Esc", Style::default().fg(theme::BLUE)),
        Span::styled(" cancel", Style::default().fg(theme::MUTED)),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(help, chunks[1]);
}
