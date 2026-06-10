use chrono::{DateTime, Datelike, Utc};
use chrono_tz::Asia::Tehran;
use common::filter::DurationFilter;
use domain::value_objects::WorklogDuration;
use jalali_rs::gregorian_to_jalali;
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};

use crate::theme;

pub fn jalali_date_string(dt: DateTime<Utc>) -> String {
    let local = dt.with_timezone(&Tehran);
    let (y, m, d) = gregorian_to_jalali(
        local.year(),
        local.month() as usize,
        local.day() as i32,
    );
    format!("{y:04}/{m:02}/{d:02}")
}

pub fn format_duration(duration: WorklogDuration) -> String {
    let secs = duration.as_std().as_secs();
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    let mut parts = Vec::new();
    if h > 0 {
        parts.push(format!("{h}h"));
    }
    if m > 0 {
        parts.push(format!("{m}m"));
    }
    if s > 0 || parts.is_empty() {
        parts.push(format!("{s}s"));
    }
    parts.join("")
}

pub fn styled_tags_line(tags: &str) -> Line<'static> {
    let parts = split_tags(tags);
    styled_tags_line_from_parts(&parts)
}

pub fn styled_tags_lines(tags: &str, width: usize) -> Vec<Line<'static>> {
    let width = width.max(1);
    let parts = split_tags(tags);
    if parts.is_empty() {
        return vec![Line::from(Span::raw(" "))];
    }

    let mut lines = Vec::new();
    let mut current = Vec::new();
    let mut current_width = 0usize;

    for tag in parts {
        let tag_width = tag.chars().count();
        if !current.is_empty() && current_width + 2 + tag_width > width {
            lines.push(styled_tags_line_from_parts(&current));
            current = vec![tag];
            current_width = tag_width;
        } else {
            current_width += if current.is_empty() {
                tag_width
            } else {
                2 + tag_width
            };
            current.push(tag);
        }
    }

    if !current.is_empty() {
        lines.push(styled_tags_line_from_parts(&current));
    }
    lines
}

pub fn wrap_text_lines(text: &str, width: usize) -> Vec<Line<'static>> {
    let width = width.max(1);
    let text = text.trim();
    if text.is_empty() {
        return vec![Line::from(Span::styled(" ", Style::default().fg(theme::TEXT)))];
    }

    let mut lines = Vec::new();
    let mut current = String::new();
    let mut current_len = 0usize;

    for word in text.split_whitespace() {
        let word_len = word.chars().count();
        if word_len > width {
            if !current.is_empty() {
                lines.push(text_line(&current));
                current.clear();
                current_len = 0;
            }
            let mut chunk = String::new();
            for ch in word.chars() {
                if chunk.chars().count() + 1 > width {
                    lines.push(text_line(&chunk));
                    chunk.clear();
                }
                chunk.push(ch);
            }
            if !chunk.is_empty() {
                current = chunk;
                current_len = current.chars().count();
            }
            continue;
        }

        if current_len == 0 {
            current = word.to_string();
            current_len = word_len;
        } else if current_len + 1 + word_len > width {
            lines.push(text_line(&current));
            current = word.to_string();
            current_len = word_len;
        } else {
            current.push(' ');
            current.push_str(word);
            current_len += 1 + word_len;
        }
    }

    if !current.is_empty() {
        lines.push(text_line(&current));
    }
    lines
}

fn split_tags(tags: &str) -> Vec<String> {
    tags.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

fn styled_tags_line_from_parts(parts: &[String]) -> Line<'static> {
    if parts.is_empty() {
        return Line::from(Span::raw(" "));
    }

    let mut spans = Vec::new();
    for (i, tag) in parts.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(", ", Style::default().fg(theme::MUTED)));
        }
        spans.push(Span::styled(
            tag.clone(),
            Style::default()
                .fg(theme::tag_color(tag))
                .add_modifier(Modifier::BOLD),
        ));
    }
    Line::from(spans)
}

fn text_line(text: &str) -> Line<'static> {
    Line::from(Span::styled(text.to_string(), Style::default().fg(theme::TEXT)))
}

pub fn parse_duration_input(raw: &str) -> Option<u64> {
    let raw = raw.trim();
    if raw.is_empty() {
        return None;
    }
    if raw.chars().all(|c| c.is_ascii_digit()) {
        return raw.parse().ok();
    }
    DurationFilter::parse_duration_str(raw)
}
