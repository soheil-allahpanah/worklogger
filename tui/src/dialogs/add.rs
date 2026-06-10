//! The "add worklog" dialog: its model, messages, update and view.

use std::io;

use chrono::Utc;
use crossterm::event::{KeyCode, KeyEvent};
use domain::traits::WorklogRepository;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{block::Position, Block, BorderType, Borders, Paragraph},
    Frame,
};
use use_cases::CreateWorklogCommand;

use crate::app::{App, Mode};
use crate::format::{jalali_date_string, parse_duration_input};
use crate::message::Outcome;
use crate::theme;

/// The currently focused input within the add form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Field {
    #[default]
    Date,
    Duration,
    Description,
    Tags,
}

impl Field {
    fn next(self) -> Self {
        match self {
            Self::Date => Self::Duration,
            Self::Duration => Self::Tags,
            Self::Tags => Self::Description,
            Self::Description => Self::Date,
        }
    }

    fn previous(self) -> Self {
        match self {
            Self::Date => Self::Description,
            Self::Duration => Self::Date,
            Self::Tags => Self::Duration,
            Self::Description => Self::Tags,
        }
    }
}

/// The model for the add dialog.
#[derive(Default, Clone)]
pub struct Model {
    pub date: String,
    pub duration: String,
    pub description: String,
    pub tags: String,
    pub focused: Field,
}

impl Model {
    /// A fresh form with the first field focused and today's Jalali date pre-filled.
    pub fn fresh() -> Self {
        Self {
            date: jalali_date_string(Utc::now()),
            focused: Field::Duration,
            ..Default::default()
        }
    }

    fn field_mut(&mut self) -> &mut String {
        match self.focused {
            Field::Date => &mut self.date,
            Field::Duration => &mut self.duration,
            Field::Description => &mut self.description,
            Field::Tags => &mut self.tags,
        }
    }
}

/// Messages the add dialog understands.
#[derive(Debug, Clone)]
pub enum Msg {
    FocusNext,
    FocusPrev,
    Input(char),
    Backspace,
    Submit,
    Cancel,
}

/// Translates a key press into a dialog message.
pub fn from_key(key: KeyEvent) -> Option<Msg> {
    match key.code {
        KeyCode::Esc => Some(Msg::Cancel),
        KeyCode::Tab => Some(Msg::FocusNext),
        KeyCode::BackTab => Some(Msg::FocusPrev),
        KeyCode::Enter => Some(Msg::Submit),
        KeyCode::Backspace => Some(Msg::Backspace),
        KeyCode::Char(c) => Some(Msg::Input(c)),
        _ => None,
    }
}

/// Applies a message to the model, running effects as needed.
pub async fn update<R: WorklogRepository>(
    app: &mut App<R>,
    msg: Msg,
) -> io::Result<Outcome> {
    match msg {
        Msg::Cancel => app.mode = Mode::Normal,
        Msg::FocusNext => app.add.focused = app.add.focused.next(),
        Msg::FocusPrev => app.add.focused = app.add.focused.previous(),
        Msg::Backspace => {
            app.add.field_mut().pop();
        }
        Msg::Input(c) => app.add.field_mut().push(c),
        Msg::Submit => {
            if submit(app).await? {
                app.mode = Mode::Normal;
            }
        }
    }
    Ok(Outcome::Continue)
}

/// Validates and persists the form. Returns `true` on success.
async fn submit<R: WorklogRepository>(app: &mut App<R>) -> io::Result<bool> {
    let Some(duration_secs) = parse_duration_input(&app.add.duration) else {
        app.set_status("Invalid duration (e.g. 2h30m or seconds)".into(), 4);
        return Ok(false);
    };

    let jalali_date = {
        let d = app.add.date.trim();
        if d.is_empty() {
            None
        } else {
            Some(d.to_string())
        }
    };

    let tags: Vec<String> = app
        .add
        .tags
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();

    let command = CreateWorklogCommand {
        jalali_date,
        duration_secs,
        tags,
        description: app.add.description.clone(),
    };

    match app.create_worklog_usecase.execute(command).await {
        Ok(resp) => {
            app.set_status(format!("Created {}", resp.id), 2);
            app.reload_worklogs().await?;
            Ok(true)
        }
        Err(err) => {
            app.set_status(format!("{err}"), 5);
            Ok(false)
        }
    }
}

/// Renders the add dialog over the current frame.
pub fn view<R: WorklogRepository>(frame: &mut Frame, app: &App<R>) {
    let area = theme::centered_rect(45, 37, frame.area());
    let block = Block::default()
        .title(" Adding New Worklog ")
        .title_alignment(Alignment::Center)
        .title_style(Style::default().fg(theme::BLUE).add_modifier(Modifier::BOLD))
        .title_position(Position::Top)
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(theme::BORDER))
        .style(Style::default().bg(theme::SURFACE));

    frame.render_widget(block, area);

    let inner = theme::centered_rect(43, 37, frame.area());
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(inner);

    let fields = [
        (Field::Date, "Date (Jalali YYYY/MM/DD)", &app.add.date),
        (Field::Duration, "Duration (e.g. 2h30m)", &app.add.duration),
        (Field::Tags, "Tags (comma-separated)", &app.add.tags),
        (Field::Description, "Description", &app.add.description),
    ];

    for (i, (field, label, value)) in fields.iter().enumerate() {
        let focused = app.add.focused == *field;
        let label_style = if focused {
            Style::default().fg(theme::EMERALD).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::MUTED)
        };
        let border_color = if focused { theme::EMERALD } else { theme::BORDER };

        let field_block = Block::default()
            .title(*label)
            .title_style(label_style)
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(border_color))
            .style(Style::default().bg(theme::BG));

        let display = if value.is_empty() {
            Span::styled(" ", Style::default().fg(theme::TEXT))
        } else {
            Span::styled(value.as_str(), Style::default().fg(theme::TEXT))
        };
        let cursor = if focused && app.cursor_visible {
            Span::styled("█", Style::default().fg(theme::EMERALD))
        } else {
            Span::raw("")
        };

        frame.render_widget(
            Paragraph::new(Line::from(vec![display, cursor])).block(field_block),
            chunks[i],
        );
    }

}
