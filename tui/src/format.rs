use chrono::{DateTime, Datelike, Utc};
use chrono_tz::Asia::Tehran;
use common::filter::DurationFilter;
use domain::value_objects::WorklogDuration;
use jalali_rs::gregorian_to_jalali;

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
