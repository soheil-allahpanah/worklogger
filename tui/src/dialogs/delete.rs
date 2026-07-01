//! The "delete worklog" confirmation dialog: model, messages, update and view.

use std::io;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{block::Position, Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};
use sdk::DeleteWorklogCommand;
use uuid::Uuid;

use crate::app::{command_user_id, App, Mode};
use crate::message::Outcome;
use crate::theme;

/// Which confirmation button is focused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Choice {
    Yes,
    #[default]
    No,
}

impl Choice {
    fn next(self) -> Self {
        match self {
            Self::Yes => Self::No,
            Self::No => Self::Yes,
        }
    }

    fn previous(self) -> Self {
        self.next()
    }
}

/// The model for the delete dialog: the worklog awaiting confirmation.
#[derive(Default, Clone)]
pub struct Model {
    pub target: Option<Uuid>,
    pub choice: Choice,
}

impl Model {
    pub fn confirm(target: Uuid) -> Self {
        Self {
            target: Some(target),
            choice: Choice::No,
        }
    }
}

/// Messages the delete dialog understands.
#[derive(Debug, Clone)]
pub enum Msg {
    Confirm,
    Cancel,
    Submit,
    FocusNext,
    FocusPrev,
}

/// Translates a key press into a dialog message.
pub fn from_key(key: KeyEvent) -> Option<Msg> {
    match key.code {
        KeyCode::Esc => Some(Msg::Cancel),
        KeyCode::Enter => Some(Msg::Submit),
        KeyCode::Tab => Some(Msg::FocusNext),
        KeyCode::BackTab => Some(Msg::FocusPrev),
        KeyCode::Left => Some(Msg::FocusPrev),
        KeyCode::Right => Some(Msg::FocusNext),
        KeyCode::Char('y' | 'Y') => Some(Msg::Confirm),
        KeyCode::Char('n' | 'N') => Some(Msg::Cancel),
        _ => None,
    }
}

async fn delete_target(app: &mut App) -> io::Result<()> {
    if let Some(id) = app.delete.target {
        app.client
            .delete_worklog(DeleteWorklogCommand {
                user_id: command_user_id(),
                id,
            })
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        app.reload_worklogs().await?;
        app.set_status("Worklog deleted".into(), 1);
    }
    app.delete.target = None;
    app.mode = Mode::Normal;
    Ok(())
}

/// Applies a message to the model, running effects as needed.
pub async fn update(app: &mut App, msg: Msg) -> io::Result<Outcome> {
    match msg {
        Msg::FocusNext => app.delete.choice = app.delete.choice.next(),
        Msg::FocusPrev => app.delete.choice = app.delete.choice.previous(),
        Msg::Confirm => delete_target(app).await?,
        Msg::Submit if app.delete.choice == Choice::Yes => delete_target(app).await?,
        Msg::Cancel | Msg::Submit => {
            app.delete.target = None;
            app.mode = Mode::Normal;
        }
    }
    Ok(Outcome::Continue)
}


/// Renders the delete confirmation dialog over the current frame.
pub fn view(frame: &mut Frame, app: &App) {
    let area = theme::centered_rect(40, 22, frame.area());
    let block = Block::default()
        .title(" Deleting Worklog ")
        .title_alignment(Alignment::Center)
        .title_style(Style::default().fg(theme::BLUE).add_modifier(Modifier::BOLD))
        .title_position(Position::Top)
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(theme::BORDER))
        .style(Style::default().bg(theme::SURFACE));

    frame.render_widget(Clear, area);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(inner);

    let confirmation = Paragraph::new(Line::from(vec![Span::styled(
        "Are you sure you want to delete this worklog?",
        Style::default().fg(theme::TEXT).bg(theme::SURFACE),
    )]))
    .alignment(Alignment::Center)
    .style(Style::default().bg(theme::SURFACE));
    frame.render_widget(confirmation, chunks[0]);

    let buttons = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(20),
            Constraint::Length(3),
            Constraint::Length(20),
            Constraint::Fill(1),
        ])
        .split(chunks[1]);

    let yes_focused = app.delete.choice == Choice::Yes;
    let no_focused = app.delete.choice == Choice::No;

    render_button(frame, buttons[1], "Yes (y)", yes_focused);
    render_button(frame, buttons[3], "No (n)", no_focused);
}


fn render_button(frame: &mut Frame, area: Rect, label: &str, focused: bool) {
    let border_color = if focused { theme::EMERALD } else { theme::BORDER };
    let text_style = if focused {
        Style::default().fg(theme::EMERALD).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::MUTED)
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(theme::BG));
    let button = Paragraph::new(Span::styled(label, text_style))
        .alignment(Alignment::Center)
        .block(block);
    frame.render_widget(button, area);
}