use chrono::{Datelike, NaiveDate, TimeZone, Utc};
use chrono_tz::Asia::Tehran;
use domain::value_objects::WorklogDateTime;
use jalali_rs::{
    gregorian_to_jalali, jalali_to_gregorian, persian_or_arabic_digits_to_latin,
};

use crate::error::{UseCaseResult, ValidationError};

/// Jalali (year, month, day) as returned by `jalali-rs`.
pub type JalaliDate = (i32, u32, u32);

pub fn today_jalali_in_tehran() -> JalaliDate {
    let now = Utc::now().with_timezone(&Tehran);
    gregorian_to_jalali(now.year(), now.month() as usize, now.day() as i32)
}

pub fn parse_jalali_date(raw: &str) -> UseCaseResult<JalaliDate> {
    let raw = persian_or_arabic_digits_to_latin(raw.trim());
    let parts: Vec<&str> = raw.split(['/', '-']).collect();
    if parts.len() != 3 {
        return Err(ValidationError::InvalidJalaliDate(raw.to_string()).into());
    }

    let year: i32 = parts[0]
        .parse()
        .map_err(|_| ValidationError::InvalidJalaliDate(raw.to_string()))?;
    let month: usize = parts[1]
        .parse()
        .map_err(|_| ValidationError::InvalidJalaliDate(raw.to_string()))?;
    let day: i32 = parts[2]
        .parse()
        .map_err(|_| ValidationError::InvalidJalaliDate(raw.to_string()))?;

    if !(1..=12).contains(&month) || !(1..=31).contains(&(day as usize)) {
        return Err(ValidationError::InvalidJalaliDate(raw.to_string()).into());
    }

    let (gy, gm, gd) = jalali_to_gregorian(year, month, day);
    if NaiveDate::from_ymd_opt(gy, gm, gd).is_none() {
        return Err(ValidationError::InvalidJalaliDate(raw.to_string()).into());
    }

    Ok((year, month as u32, day as u32))
}

pub fn jalali_date_to_worklog_datetime(
    jy: i32,
    jm: u32,
    jd: u32,
) -> UseCaseResult<WorklogDateTime> {
    let (gy, gm, gd) = jalali_to_gregorian(jy, jm as usize, jd as i32);
    let naive_date = NaiveDate::from_ymd_opt(gy, gm, gd).ok_or_else(|| {
        ValidationError::InvalidJalaliDate(format!("{jy}/{jm}/{jd}"))
    })?;
    let naive_dt = naive_date
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| ValidationError::InvalidJalaliDate(format!("{jy}/{jm}/{jd}")))?;

    let local = Tehran
        .from_local_datetime(&naive_dt)
        .single()
        .ok_or_else(|| ValidationError::InvalidJalaliDate(format!("{jy}/{jm}/{jd}")))?;

    Ok(WorklogDateTime::new(local.with_timezone(&Utc)))
}
