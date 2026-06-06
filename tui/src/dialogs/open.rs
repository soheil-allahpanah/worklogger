//! The "open worklog details" read-only dialog: model, messages, update and view.

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

use crate::app::{App, Mode, WorklogRow};
use crate::message::Outcome;
use crate::theme;

/// The model for the open dialog: the worklog being inspected.
#[derive(Default, Clone)]
pub struct Model {
    pub worklog: Option<WorklogRow>,
}

/// Messages the open dialog understands.
#[derive(Debug, Clone)]
pub enum Msg {
    Close,
}

/// Translates a key press into a dialog message.
pub fn from_key(key: KeyEvent) -> Option<Msg> {
    match key.code {
        KeyCode::Esc => Some(Msg::Close),
        _ => None,
    }
}

/// Applies a message to the model.
pub async fn update<R: WorklogRepository>(
    app: &mut App<R>,
    msg: Msg,
) -> io::Result<Outcome> {
    match msg {
        Msg::Close => {
            app.open.worklog = None;
            app.mode = Mode::Normal;
        }
    }
    Ok(Outcome::Continue)
}

/// Renders the details dialog over the current frame.
pub fn view<R: WorklogRepository>(frame: &mut Frame, app: &App<R>) {
    let Some(worklog) = app.open.worklog.as_ref() else {
        return;
    };

    let area = theme::centered_rect(40, 40, frame.area());
    let block = Block::default()
        .title(" Worklog Details ")
        .title_alignment(Alignment::Center)
        .title_style(Style::default().fg(theme::BLUE).add_modifier(Modifier::BOLD))
        .title_position(Position::Top)
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(theme::BORDER))
        .style(Style::default().bg(theme::SURFACE));
    frame.render_widget(block, area);

    let inner = theme::centered_rect(36, 36, frame.area());
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(2),
        ])
        .split(inner);

    let fields_values = [
        ("Date: ", worklog.date.clone()),
        ("Duration: ", worklog.duration.clone()),
        ("Description: ", worklog.description.clone()),
        ("Tags: ", worklog.tags.clone()),
    ];

    for (index, (label, value)) in fields_values.iter().enumerate() {
        let field = Paragraph::new(Line::from(vec![
            Span::styled(label.to_string(), Style::default().fg(theme::TEXT)),
            Span::styled(value.to_string(), Style::default().fg(theme::TEXT)),
        ]))
        .alignment(Alignment::Left);
        frame.render_widget(field, chunks[index]);
    }

    let help = Paragraph::new(Line::from(vec![
        Span::styled("Esc", Style::default().fg(theme::BLUE)),
        Span::styled(" close", Style::default().fg(theme::MUTED)),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(help, chunks[4]);
}
