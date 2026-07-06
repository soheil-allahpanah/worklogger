//! The main worklog list screen (the "Normal" mode): navigation, the keys that
//! open dialogs, and the table view.

use std::io;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
    Frame,
};
use sdk::GetWorklogCommand;

use crate::app::{command_user_id, worklog_to_row, App, Mode};
use crate::dialogs::{add, delete, edit};
use crate::format::styled_tags_line;
use crate::message::Outcome;
use crate::theme;

const COL_DATE: u16 = 12;
const COL_DURATION: u16 = 10;
const COL_TAGS_BASE: u16 = 24;
const COL_SPACING: u16 = 5; // left + 3 separators + right
const MIN_DESC: u16 = 8;
const MIN_TAGS: u16 = 12;

fn column_widths(area_width: u16) -> [u16; 4] {
    let old_desc = area_width
        .saturating_sub(COL_DATE + COL_DURATION + COL_TAGS_BASE + COL_SPACING)
        .max(MIN_DESC);
    let shift = old_desc / 3;
    let col_desc = old_desc.saturating_sub(shift).max(MIN_DESC);
    let col_tags = COL_TAGS_BASE.saturating_add(shift).max(MIN_TAGS);
    [COL_DATE, COL_DURATION, col_desc, col_tags]
}

/// Messages the list screen understands.
#[derive(Debug, Clone)]
pub enum Msg {
    Quit,
    SelectNext,
    SelectPrev,
    PageNext,
    PagePrev,
    ApplySearch,
    Export,
    OpenSearch,
    OpenAdd,
    OpenEdit,
    OpenDelete,
    OpenDetail,
    Refresh,
}

/// Translates a key press into a list-screen message.
pub fn from_key(key: KeyEvent) -> Option<Msg> {
    match key.code {
        KeyCode::Char('q') => Some(Msg::Quit),
        KeyCode::Char('/') => Some(Msg::OpenSearch),
        KeyCode::Char('d') => Some(Msg::OpenDelete),
        KeyCode::Char('o') => Some(Msg::OpenDetail),
        KeyCode::Char('n') | KeyCode::Char('a') => Some(Msg::OpenAdd),
        KeyCode::Char('e') => Some(Msg::OpenEdit),
        KeyCode::Char('x') => Some(Msg::Export),
        KeyCode::Char('r') | KeyCode::F(5) => Some(Msg::Refresh),
        KeyCode::Down | KeyCode::Char('j') => Some(Msg::SelectNext),
        KeyCode::Up | KeyCode::Char('k') => Some(Msg::SelectPrev),
        KeyCode::Char(']') | KeyCode::PageDown => Some(Msg::PageNext),
        KeyCode::Char('[') | KeyCode::PageUp => Some(Msg::PagePrev),
        KeyCode::Enter => Some(Msg::ApplySearch),
        _ => None,
    }
}

/// Applies a message to the model, running effects as needed.
pub async fn update(app: &mut App, msg: Msg) -> io::Result<Outcome> {
    match msg {
        Msg::Quit => return Ok(Outcome::Quit),
        Msg::SelectNext => select_next(app),
        Msg::SelectPrev => select_prev(app),
        Msg::PageNext => {
            if app.current_page < app.total_pages {
                app.current_page += 1;
                app.apply_search().await?;
                app.table_state.select(Some(0));
            }
        }
        Msg::PagePrev => {
            if app.current_page > 1 {
                app.current_page -= 1;
                app.apply_search().await?;
                app.table_state.select(Some(0));
            }
        }
        Msg::ApplySearch => {
            app.reset_page();
            app.apply_search().await?;
        }
        Msg::Export => app.export_search_results().await?,
        Msg::Refresh => {
            app.reload_worklogs().await?;
            app.set_status("Refreshed".into(), 2);
        }
        Msg::OpenSearch => {
            app.mode = Mode::Search;
            app.cursor_visible = true;
        }
        Msg::OpenAdd => {
            app.add = add::Model::fresh();
            app.mode = Mode::AddModal;
            app.cursor_visible = true;
        }
        Msg::OpenEdit => {
            if let Some(id) = selected_id(app) {
                match app
                    .client
                    .get_worklog(GetWorklogCommand {
                        user_id: command_user_id(),
                        id,
                    })
                    .await
                {
                    Ok(worklog) => {
                        app.edit = edit::Model::from_worklog(&worklog);
                        app.mode = Mode::EditModal;
                        app.cursor_visible = true;
                    }
                    Err(err) => {
                        app.set_status(err.to_string(), 4);
                    }
                }
            }
        }
        Msg::OpenDelete => {
            if let Some(id) = selected_id(app) {
                app.delete = delete::Model::confirm(id);
                app.mode = Mode::DeleteModal;
                app.cursor_visible = true;
            }
        }
        Msg::OpenDetail => {
            if let Some(id) = selected_id(app) {
                let worklog = app
                    .client
                    .get_worklog(GetWorklogCommand {
                        user_id: command_user_id(),
                        id,
                    })
                    .await
                    .map(|worklog| worklog_to_row(&worklog))
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                app.open.worklog = Some(worklog);
                app.mode = Mode::OpenModal;
                app.cursor_visible = true;
            }
        }
    }
    Ok(Outcome::Continue)
}

fn selected_id(app: &App) -> Option<uuid::Uuid> {
    app.table_state
        .selected()
        .and_then(|i| app.rows.get(i))
        .map(|row| row.id)
}

fn select_next(app: &mut App) {
    if app.rows.is_empty() {
        return;
    }
    let i = match app.table_state.selected() {
        Some(i) => (i + 1).min(app.rows.len().saturating_sub(1)),
        None => 0,
    };
    app.table_state.select(Some(i));
}

fn select_prev(app: &mut App) {
    if app.rows.is_empty() {
        return;
    }
    let i = match app.table_state.selected() {
        Some(i) => i.saturating_sub(1),
        None => 0,
    };
    app.table_state.select(Some(i));
}

fn truncate(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        text.to_string()
    } else if max <= 1 {
        text.chars().take(max).collect()
    } else {
        format!("{}…", text.chars().take(max - 1).collect::<String>())
    }
}

fn pad_cell(text: &str, width: u16) -> String {
    let max = width.saturating_sub(2) as usize;
    let clipped = truncate(text, max);
    let pad = width as usize - 1 - clipped.chars().count();
    format!(" {}{}", clipped, " ".repeat(pad))
}

struct BorderedTable<'a> {
    rows: &'a [crate::app::WorklogRow],
    selected: Option<usize>,
    scroll_offset: usize,
    cols: [u16; 4],
}

impl Widget for BorderedTable<'_> {

    fn render(self, area: Rect, buf: &mut Buffer) {
        let min_w = self.cols.iter().sum::<u16>() + COL_SPACING;
        if area.width < min_w || area.height < 3 {
            return;
        }

        let border = Style::default().fg(theme::BORDER);
        let x = area.x;
        let y = area.y;
        let w = area.width;
        let cols = self.cols;

        // Header — fully bordered (top, bottom, left, right).
        draw_hline(buf, x, y, &cols, '┌', '─', '┐', '┬', border);
        draw_header_row(buf, x, y + 1, w, &cols, border);
        draw_hline(buf, x, y + 2, &cols, '└', '─', '┘', '┴', border);

        let body_y = y + 3;
        let body_h = area.height.saturating_sub(3);
        if body_h == 0 {
            return;
        }

        let visible = body_h as usize;
        let start = self.scroll_offset;
        let end = (start + visible).min(self.rows.len());

        for (i, row) in self.rows[start..end].iter().enumerate() {
            let row_y = body_y + i as u16;
            let row_idx = start + i;
            let selected = self.selected == Some(row_idx);
            let base = if selected {
                Style::default().bg(theme::SURFACE).fg(theme::TEXT)
            } else {
                Style::default().bg(theme::BG).fg(theme::TEXT)
            };

            if selected {
                let row_bg = Style::default().bg(theme::SURFACE);
                for col_x in x..x + w {
                    buf[(col_x, row_y)].set_style(row_bg);
                }
            }

            draw_vline(buf, x, row_y, border);
            draw_vline(buf, x + w - 1, row_y, border);

            let mut cx = x + 1;
            let date = pad_cell(&row.date, cols[0]);
            write_spans(buf, cx, row_y, cols[0], &[Span::styled(date, base.fg(if selected { theme::EMERALD } else { theme::TEXT }))]);
            cx += cols[0] + 1;
            draw_vline(buf, cx - 1, row_y, border);

            let duration = pad_cell(&row.duration, cols[1]);
            write_spans(buf, cx, row_y, cols[1], &[Span::styled(duration, base.fg(theme::BLUE))]);
            cx += cols[1] + 1;
            draw_vline(buf, cx - 1, row_y, border);

            let desc = pad_cell(&row.description, cols[2]);
            write_spans(buf, cx, row_y, cols[2], &[Span::styled(desc, base)]);
            cx += cols[2] + 1;
            draw_vline(buf, cx - 1, row_y, border);

            write_tag_line(buf, cx, row_y, cols[3], &styled_tags_line(&row.tags), base.bg);
        }
    }
}

fn draw_hline(
    buf: &mut Buffer,
    x: u16,
    y: u16,
    cols: &[u16; 4],
    left: char,
    mid: char,
    right: char,
    junction: char,
    style: Style,
) {
    let mut cx = x;
    buf.set_string(cx, y, left.to_string(), style);
    cx += 1;

    for (i, col_w) in cols.iter().enumerate() {
        for _ in 0..*col_w {
            buf.set_string(cx, y, mid.to_string(), style);
            cx += 1;
        }
        if i < cols.len() - 1 {
            buf.set_string(cx, y, junction.to_string(), style);
            cx += 1;
        }
    }
    buf.set_string(cx, y, right.to_string(), style);
}

fn draw_header_row(buf: &mut Buffer, x: u16, y: u16, w: u16, cols: &[u16; 4], border: Style) {
    let headers = ["Date", "Duration", "Description", "Tags"];
    let header_style = Style::default()
        .fg(theme::BLUE)
        .bg(theme::SURFACE)
        .add_modifier(Modifier::BOLD);

    draw_vline(buf, x, y, border);
    let mut cx = x + 1;
    for (i, (label, col_w)) in headers.iter().zip(cols.iter()).enumerate() {
        let cell = pad_cell(label, *col_w);
        buf.set_string(cx, y, cell, header_style);
        cx += *col_w;
        if i < headers.len() - 1 {
            draw_vline(buf, cx, y, border);
            cx += 1;
        }
    }
    draw_vline(buf, x + w - 1, y, border);
}

fn draw_vline(buf: &mut Buffer, x: u16, y: u16, style: Style) {
    buf.set_string(x, y, "│".to_string(), style);
}

fn write_spans(buf: &mut Buffer, x: u16, y: u16, width: u16, spans: &[Span]) {
    let line = Line::from(spans.to_vec());
    buf.set_line(x, y, &line, width);
}

fn write_tag_line(buf: &mut Buffer, x: u16, y: u16, width: u16, line: &Line, bg: Option<ratatui::style::Color>) {
    let mut styled = line.clone();
    if let Some(bg) = bg {
        for span in &mut styled.spans {
            span.style = span.style.bg(bg);
        }
    }
    buf.set_line(x, y, &styled, width);
}

fn scroll_offset(selected: Option<usize>, total: usize, visible: usize) -> usize {
    let Some(sel) = selected else {
        return 0;
    };
    if visible == 0 || total == 0 {
        return 0;
    }
    if sel >= visible {
        sel - visible + 1
    } else {
        0
    }
    .min(total.saturating_sub(visible))
}

/// Renders the worklog table into `area`.
pub fn view(frame: &mut Frame, app: &mut App, area: Rect) {
    let cols = column_widths(area.width);
    let body_h = area.height.saturating_sub(3).max(1) as usize;
    let scroll = scroll_offset(app.table_state.selected(), app.rows.len(), body_h);

    let table = BorderedTable {
        rows: &app.rows,
        selected: app.table_state.selected(),
        scroll_offset: scroll,
        cols,
    };
    frame.render_widget(table, area);

    if app.rows.is_empty() {
        let hint = Paragraph::new(Line::from(vec![
            Span::styled("No worklogs yet. ", Style::default().fg(theme::MUTED)),
            Span::styled("n", Style::default().fg(theme::EMERALD).bold()),
            Span::styled(" to add · ", Style::default().fg(theme::MUTED)),
            Span::styled("/", Style::default().fg(theme::BLUE).bold()),
            Span::styled(" to search", Style::default().fg(theme::MUTED)),
        ]))
        .alignment(Alignment::Center);
        frame.render_widget(hint, theme::centered_rect(50, 3, area));
    }
}
