use chrono::{DateTime, Datelike, Utc};
use chrono_tz::Asia::Tehran;
use domain::entities::Worklog;
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

pub fn format_duration_secs(secs: u64) -> String {
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

pub fn format_description(worklog: &Worklog) -> String {
    worklog
        .description()
        .map(|d| d.as_str().to_string())
        .unwrap_or_default()
}
