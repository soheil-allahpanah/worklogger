//! The root view: composes the persistent components and overlays the active
//! dialog plus any transient status toast.

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Mode};
use crate::components::{help_bar, search_bar, table};
use crate::dialogs::{add, delete, open};
use crate::theme;

pub fn view(frame: &mut Frame, app: &mut App) {
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(3),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(frame.area());

    draw_title(frame, app, root[0]);
    table::view(frame, app, root[1]);
    search_bar::view(frame, app, root[2]);
    help_bar::view(frame, app, root[3]);

    match app.mode {
        Mode::AddModal => add::view(frame, app),
        Mode::DeleteModal => delete::view(frame, app),
        Mode::OpenModal => open::view(frame, app),
        Mode::Normal | Mode::Search => {}
    }

    draw_status_toast(frame, app);
}

fn draw_title(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let version = env!("CARGO_PKG_VERSION");
    let view_name = match app.mode {
        Mode::Normal | Mode::Search => "Main View",
        Mode::AddModal => "Add Entry",
        Mode::DeleteModal => "Delete Entry",
        Mode::OpenModal => "Entry Detail",
    };

    let title = Line::from(vec![
        Span::styled(
            format!("WORK LOGGER v{version} "),
            Style::default()
                .fg(theme::TEXT)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("[", Style::default().fg(theme::MUTED)),
        Span::styled(view_name, Style::default().fg(theme::BLUE)),
        Span::styled(" | Logged: ", Style::default().fg(theme::MUTED)),
        Span::styled(
            format!("{}", app.total_entries),
            Style::default().fg(theme::EMERALD).bold(),
        ),
        Span::styled(" entries]", Style::default().fg(theme::MUTED)),
    ]);

    let block = Paragraph::new(title)
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().bg(theme::BG));
    frame.render_widget(block, area);
}

fn draw_status_toast(frame: &mut Frame, app: &App) {
    let Some(msg) = app.status_message.clone() else {
        return;
    };

    let area = theme::centered_rect(60, 20, frame.area());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(theme::BORDER))
        .style(Style::default().bg(theme::SURFACE));
    let text = Paragraph::new(msg)
        .style(Style::default().fg(theme::TEXT))
        .block(block)
        .wrap(Wrap { trim: true });
    frame.render_widget(text, area);
}
