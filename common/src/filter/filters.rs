use regex::Regex;
use std::sync::OnceLock;
use serde::Deserialize;
use chrono::NaiveDate;
use crate::util::convert_jalali_string_to_naive_date;


#[derive(Debug, Clone, Deserialize, Default)]
pub struct ListFilter<T> {
    pub in_list: Option<Vec<T>>,
    pub not_in: Option<Vec<T>>,
}

impl<T: PartialEq> ListFilter<T> {

    pub fn new(in_list: Option<Vec<T>>, not_in: Option<Vec<T>>) -> Self {
        let mut filter = Self { in_list, not_in };
        filter.remove_shared_items();
        filter
    }

    /// Removes items that exist in both `in_list` and `not_in`.
    fn remove_shared_items(&mut self) {

        if let (Some(in_vec), Some(not_in_vec)) = (&mut self.in_list, &mut self.not_in) {
            let mut i = 0;
            
            while i < in_vec.len() {
                // Check if the item in `in_list` also exists in `not_in`
                if not_in_vec.contains(&in_vec[i]) {
                    // Remove the shared item from `in_list`
                    let shared_item = in_vec.remove(i);
                    
                    // Remove all occurrences of this item from `not_in`
                    not_in_vec.retain(|x| x != &shared_item);
                } else {
                    // Move to the next item only if we didn't remove the current one
                    i += 1;
                }
            }

            // Optional: If the lists become empty after cleanup, set them back to None
            if in_vec.is_empty() {
                self.in_list = None;
            }
            if not_in_vec.is_empty() {
                self.not_in = None;
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TextFilter {
    pub contains: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JalaliDateFilter {
    pub from: Option<String>, // Expected format: YYYY/MM/DD (Jalali)
    pub to: Option<String>,
}


impl JalaliDateFilter {

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let mut from_valid = true;
        let mut to_valid = true;

        if let Some(from) = &self.from {
            if !Self::is_valid_jalali_format(from) {
                errors.push("Invalid 'date.from' format. Expected YYYY/MM/DD".to_string());
                from_valid = false;
            }
        }
        
        if let Some(to) = &self.to {
            if !Self::is_valid_jalali_format(to) {
                errors.push("Invalid 'date.to' format. Expected YYYY/MM/DD".to_string());
                to_valid = false;
            }
        }
          // String comparison works perfectly for zero-padded YYYY/MM/DD
        if from_valid && to_valid {
            if let (Some(from), Some(to)) = (&self.from, &self.to) {
                if from > to {
                    errors.push("'date.to' must be greater than or equal to 'date.from'".to_string());
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn is_valid_jalali_format(date: &str) -> bool {
        static JALALI_RE: OnceLock<Regex> = OnceLock::new();
        let re = JALALI_RE.get_or_init(|| {
            Regex::new(r"^(1[34]\d{2})/(0[1-9]|1[0-2])/(0[1-9]|[12]\d|3[01])$").unwrap()
        });
        re.is_match(date)
    }
}


#[derive(Debug, Clone)]
pub struct DateFilter {
    pub from: Option<NaiveDate>,
    pub to: Option<NaiveDate>,
}

impl DateFilter {
    pub fn validate(&self) -> Result<(), String> {
        // If both 'from' and 'to' are provided, ensure 'from' is not after 'to'
        if let (Some(from), Some(to)) = (self.from, self.to) {
            if from > to {
                return Err("The 'from' date cannot be after the 'to' date.".to_string());
            }
        }
        Ok(())
    }
}

impl TryFrom<JalaliDateFilter> for DateFilter {
    type Error = String;

    fn try_from(jalali_filter: JalaliDateFilter) -> Result<Self, Self::Error> {
        let from_date = jalali_filter.from
            .map(|date_str| convert_jalali_string_to_naive_date(&date_str))
            .transpose()?;

        let to_date = jalali_filter.to
            .map(|date_str| convert_jalali_string_to_naive_date(&date_str))
            .transpose()?;

        Ok(DateFilter {
            from: from_date,
            to: to_date,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DurationFilter {
    pub from: Option<String>, // Expected format: xhymzs
    pub to: Option<String>,
}

impl DurationFilter {
    /// Parses a duration token such as `2h30m` into total seconds.
    pub fn parse_duration_str(duration: &str) -> Option<u64> {
        if Self::is_valid_duration_format(duration) {
            Some(Self::parse_duration_to_seconds(duration))
        } else {
            None
        }
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let mut from_valid = true;
        let mut to_valid = true;

        if let Some(from) = &self.from {
            if !Self::is_valid_duration_format(from) {
                errors.push("Invalid 'duration.from'. Expected format: xhymzs (e.g., 2h30m)".to_string());
                from_valid = false;
            }
        }
        
        if let Some(to) = &self.to {
            if !Self::is_valid_duration_format(to) {
                errors.push("Invalid 'duration.to'. Expected format: xhymzs (e.g., 45m10s)".to_string());
                to_valid = false;
            }
        }

        if from_valid && to_valid {
            if let (Some(from), Some(to)) = (&self.from, &self.to) {
                let from_secs = Self::parse_duration_to_seconds(from);
                let to_secs = Self::parse_duration_to_seconds(to);
                
                if from_secs > to_secs {
                    errors.push("'duration.to' must be greater than or equal to 'duration.from'".to_string());
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn is_valid_duration_format(duration: &str) -> bool {
        static DURATION_RE: OnceLock<Regex> = OnceLock::new();
        let re = DURATION_RE.get_or_init(|| {
            Regex::new(r"^(?:(\d+)h)?(?:(\d+)m)?(?:(\d+)s)?$").unwrap()
        });
        re.is_match(duration) && !duration.is_empty()
    }

    pub fn from_secs(&self) -> Option<u64> {
        self.from.as_deref().map(Self::parse_duration_to_seconds)
    }

    pub fn to_secs(&self) -> Option<u64> {
        self.to.as_deref().map(Self::parse_duration_to_seconds)
    }

    fn parse_duration_to_seconds(duration: &str) -> u64 {
        static DURATION_RE: OnceLock<Regex> = OnceLock::new();
        let re = DURATION_RE.get_or_init(|| {
            Regex::new(r"^(?:(\d+)h)?(?:(\d+)m)?(?:(\d+)s)?$").unwrap()
        });

        if let Some(caps) = re.captures(duration) {
            let h = caps.get(1).map_or(0, |m| m.as_str().parse::<u64>().unwrap_or(0));
            let m = caps.get(2).map_or(0, |m| m.as_str().parse::<u64>().unwrap_or(0));
            let s = caps.get(3).map_or(0, |m| m.as_str().parse::<u64>().unwrap_or(0));
            
            // Return total seconds: $h * 3600 + m * 60 + s$
            h * 3600 + m * 60 + s
        } else {
            0
        }
    }
}