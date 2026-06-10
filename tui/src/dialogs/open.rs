//! The "open worklog details" read-only dialog: model, messages, update and view.

use std::io;

use crossterm::event::{KeyCode, KeyEvent};
use domain::traits::WorklogRepository;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{block::Position, Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::{App, Mode, WorklogRow};
use crate::format::{styled_tags_lines, wrap_text_lines};
use crate::message::Outcome;
use crate::theme;

const LABEL_WIDTH: u16 = 14;
const FRAME_PADDING: u16 = 4;

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

struct FieldRow {
    label: &'static str,
    value_lines: Vec<Line<'static>>,
}

fn dialog_width_from_percent(percent: u16, area: Rect) -> u16 {
    let raw_width = area.width * percent / 100;
    raw_width.max(LABEL_WIDTH + 8).min(area.width)
}

fn value_wrap_width(dialog_width: u16) -> usize {
    dialog_width
        .saturating_sub(LABEL_WIDTH)
        .saturating_sub(2)
        .max(1) as usize
}

fn row_height(line_count: usize) -> u16 {
    (line_count.max(1) as u16).saturating_add(2)
}

fn build_rows(worklog: &WorklogRow, wrap_width: usize) -> Vec<FieldRow> {
    vec![
        FieldRow {
            label: "Date",
            value_lines: vec![Line::from(Span::styled(
                worklog.date.clone(),
                Style::default().fg(theme::TEXT),
            ))],
        },
        FieldRow {
            label: "Duration",
            value_lines: vec![Line::from(Span::styled(
                worklog.duration.clone(),
                Style::default().fg(theme::TEXT),
            ))],
        },
        FieldRow {
            label: "Tags",
            value_lines: styled_tags_lines(&worklog.tags, wrap_width),
        },
        FieldRow {
            label: "Description",
            value_lines: wrap_text_lines(&worklog.description, wrap_width),
        },
    ]
}

fn dialog_height(rows: &[FieldRow]) -> u16 {
    let body: u16 = rows
        .iter()
        .map(|row| row_height(row.value_lines.len()))
        .sum();
    body.saturating_add(FRAME_PADDING)
}

fn cell_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(theme::BORDER))
        .style(Style::default().bg(theme::BG))
}

fn centered_label_lines(label: &str, inner_height: u16) -> Vec<Line<'static>> {
    let inner_lines = inner_height.max(1) as usize;
    let pad_top = inner_lines.saturating_sub(1) / 2;
    let mut lines = Vec::with_capacity(inner_lines);
    for _ in 0..pad_top {
        lines.push(Line::from(" "));
    }
    lines.push(Line::from(Span::styled(
        label.to_string(),
        Style::default()
            .fg(theme::MUTED)
            .add_modifier(Modifier::BOLD),
    )));
    while lines.len() < inner_lines {
        lines.push(Line::from(" "));
    }
    lines
}

fn render_label_cell(frame: &mut Frame, area: Rect, label: &str) {
    let block = cell_block();
    let inner_height = block.inner(area).height;
    frame.render_widget(
        Paragraph::new(centered_label_lines(label, inner_height))
            .alignment(Alignment::Center)
            .block(block),
        area,
    );
}

fn render_value_cell(frame: &mut Frame, area: Rect, lines: &[Line<'static>]) {
    frame.render_widget(
        Paragraph::new(lines.to_vec()).block(cell_block()),
        area,
    );
}

fn render_field_row(frame: &mut Frame, area: Rect, row: &FieldRow) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(LABEL_WIDTH), Constraint::Min(0)])
        .split(area);

    render_label_cell(frame, cols[0], row.label);
    render_value_cell(frame, cols[1], &row.value_lines);
}

/// Renders the details dialog over the current frame.
pub fn view<R: WorklogRepository>(frame: &mut Frame, app: &App<R>) {
    let Some(worklog) = app.open.worklog.as_ref() else {
        return;
    };
    let dialog_width =dialog_width_from_percent(45, frame.area()).min(frame.area().width).max(LABEL_WIDTH + 8);
    let wrap_width = value_wrap_width(dialog_width);
    let rows = build_rows(worklog, wrap_width);
    let height = dialog_height(&rows).min(frame.area().height).max(8);

    let area = theme::centered_rect_chars(dialog_width, height, frame.area());
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

    let inner = Block::default().inner(area);
    let constraints: Vec<Constraint> = rows
        .iter()
        .map(|row| Constraint::Length(row_height(row.value_lines.len())))
        .collect();
    let row_areas = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints)
        .split(inner);

    for (row_area, row) in row_areas.iter().zip(rows.iter()) {
        render_field_row(frame, *row_area, row);
    }
}
