//! Shared visual constants and layout helpers used across every view.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
};

pub const BG: Color = Color::Rgb(28, 28, 30);
pub const SURFACE: Color = Color::Rgb(38, 38, 42);
pub const BLUE: Color = Color::Rgb(120, 175, 235);
pub const EMERALD: Color = Color::Rgb(88, 196, 140);
pub const AMBER: Color = Color::Rgb(235, 180, 100);
pub const MUTED: Color = Color::Rgb(120, 124, 132);
pub const BORDER: Color = Color::Rgb(72, 76, 84);
pub const TEXT: Color = Color::Rgb(220, 222, 228);

/// Harmonious palette for tag labels — hues spaced for readability on dark BG.
const TAG_PALETTE: [Color; 10] = [
    Color::Rgb(120, 175, 235), // blue
    Color::Rgb(88, 196, 140),  // emerald
    Color::Rgb(235, 163, 120), // coral
    Color::Rgb(180, 140, 235), // lavender
    Color::Rgb(235, 196, 88),  // gold
    Color::Rgb(120, 210, 210), // teal
    Color::Rgb(235, 130, 170), // rose
    Color::Rgb(160, 210, 120), // lime
    Color::Rgb(210, 150, 235), // orchid
    Color::Rgb(235, 210, 130), // sand
];

/// Maps a tag string to a stable color from [`TAG_PALETTE`].
pub fn tag_color(tag: &str) -> Color {
    let hash = tag
        .bytes()
        .fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(u64::from(b)));
    TAG_PALETTE[(hash as usize) % TAG_PALETTE.len()]
}

/// Returns a `Rect` centered within `area` with an exact character size (clamped to `area`).
pub fn centered_rect_chars(width: u16, height: u16, area: Rect) -> Rect {
    let width = width.min(area.width).max(1);
    let height = height.min(area.height).max(1);
    Rect {
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
        width,
        height,
    }
}

/// Returns a `Rect` centered within `area`, sized as a percentage of it.
pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
