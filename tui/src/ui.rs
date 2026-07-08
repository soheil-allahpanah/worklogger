//! The root view: composes the persistent components and overlays the active
//! dialog plus any transient status toast.

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::app::{format_total_duration_secs, App, Mode};
use crate::components::{help_bar, search_bar, table};
use crate::dialogs::{add, delete, edit, open};
use crate::theme;

pub fn view(frame: &mut Frame, app: &mut App) {
    if app.mode.is_modal() {
        draw_modal_screen(frame, app);
    } else {
        draw_main_screen(frame, app);
    }

    draw_status_toast(frame, app);
}

fn draw_main_screen(frame: &mut Frame, app: &mut App) {
    let root = root_layout(frame.area());

    draw_title(frame, app, root[0]);
    table::view(frame, app, root[1]);
    search_bar::view(frame, app, root[2]);
    help_bar::view(frame, app, root[3]);
}

fn draw_modal_screen(frame: &mut Frame, app: &mut App) {
    theme::fill_area(frame, frame.area(), theme::BG);

    let root = root_layout(frame.area());

    draw_title(frame, app, root[0]);
    theme::fill_area(frame, root[1], theme::BG);
    theme::fill_area(frame, root[2], theme::BG);
    help_bar::view(frame, app, root[3]);

    match app.mode {
        Mode::AddModal => add::view(frame, app),
        Mode::EditModal => edit::view(frame, app),
        Mode::DeleteModal => delete::view(frame, app),
        Mode::OpenModal => open::view(frame, app),
        Mode::Normal | Mode::Search => {}
    }
}

const SEARCH_BAR_HEIGHT: u16 = 3;
const HELP_BAR_HEIGHT: u16 = 1;
const TITLE_ROW_HEIGHT: u16 = 1;
const TOAST_HEIGHT: u16 = SEARCH_BAR_HEIGHT; // ~1.5× search bar

fn root_layout(area: ratatui::layout::Rect) -> [ratatui::layout::Rect; 4] {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(TITLE_ROW_HEIGHT),
            Constraint::Min(3),
            Constraint::Length(SEARCH_BAR_HEIGHT),
            Constraint::Length(HELP_BAR_HEIGHT),
        ])
        .split(area);
    [chunks[0], chunks[1], chunks[2], chunks[3]]
}

/// Full-width toast strip sitting directly above the search bar.
fn toast_rect(area: ratatui::layout::Rect) -> ratatui::layout::Rect {
    let bottom_reserved = HELP_BAR_HEIGHT + SEARCH_BAR_HEIGHT;
    let height = TOAST_HEIGHT.min(area.height.saturating_sub(bottom_reserved));
    let y = area
        .y
        .saturating_add(area.height.saturating_sub(bottom_reserved + height));
    ratatui::layout::Rect {
        x: area.x,
        y,
        width: area.width,
        height,
    }
}

fn draw_title(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let version = env!("CARGO_PKG_VERSION");
    let view_name = match app.mode {
        Mode::Normal | Mode::Search => "Main View",
        Mode::AddModal => "Add Entry",
        Mode::EditModal => "Edit Entry",
        Mode::DeleteModal => "Delete Entry",
        Mode::OpenModal => "Entry Detail",
    };

    let mut title_spans = vec![
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
        Span::styled(" entries", Style::default().fg(theme::MUTED)),
    ];

    let duration_label = format_total_duration_secs(app.total_duration_secs);
    let days_label = if app.days_worked == 1 {
        "1 day".to_string()
    } else {
        format!("{} days", app.days_worked)
    };
    title_spans.push(Span::styled(" · ", Style::default().fg(theme::MUTED)));
    title_spans.push(Span::styled(
        duration_label,
        Style::default().fg(theme::EMERALD).bold(),
    ));
    title_spans.push(Span::styled(" over ", Style::default().fg(theme::MUTED)));
    title_spans.push(Span::styled(
        days_label,
        Style::default().fg(theme::EMERALD).bold(),
    ));

    if app.total_pages > 1 && !app.mode.is_modal() {
        title_spans.push(Span::styled(" · page ", Style::default().fg(theme::MUTED)));
        title_spans.push(Span::styled(
            format!("{}/{}", app.current_page, app.total_pages),
            Style::default().fg(theme::AMBER).bold(),
        ));
    }

    title_spans.push(Span::styled("]", Style::default().fg(theme::MUTED)));

    let title = Line::from(title_spans);

    let block = Paragraph::new(title)
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().bg(theme::BG));
    frame.render_widget(block, area);
}

fn draw_status_toast(frame: &mut Frame, app: &App) {
    let Some(msg) = app.status_message.clone() else {
        return;
    };

    let area = toast_rect(frame.area());
    if area.height == 0 || area.width == 0 {
        return;
    }

    theme::fill_area(frame, area, theme::EMERALD);

    let text = Paragraph::new(msg)
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(theme::BG).bg(theme::EMERALD))
        .wrap(Wrap { trim: true });
    frame.render_widget(text, area);
}
