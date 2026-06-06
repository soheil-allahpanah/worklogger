//! Search-bar DSL: maps user text to [`FilterWorklogsCommand`] for the use case layer.
//!
//! Parsing stays in the TUI so `FilterWorklogsCommand` remains a plain application DTO
//! (HTTP/API can build it from JSON without going through this syntax).

use std::str::FromStr;

use common::filter::{DurationFilter, JalaliDateFilter, ListFilter, TextFilter};
use common::pagination::PagingParams;
use use_cases::FilterWorklogsCommand;
use uuid::Uuid;

/// Parses the bottom search bar input into a filter command.
pub fn parse_search_input(input: &str) -> FilterWorklogsCommand {
    let mut cmd = FilterWorklogsCommand {
        tags: None,
        ids: None,
        description: None,
        date: None,
        duration: None,
        paging: PagingParams::default(),
    };

    let tokens = shlex::split(input).unwrap_or_default();

    for token in tokens {
        let Some((key, value)) = token.split_once(':') else {
            continue;
        };
        match key {
            "tag" => add_tags(&mut cmd, value, false),
            "-tag" => add_tags(&mut cmd, value, true),
            "id" => add_ids(&mut cmd, value, false),
            "-id" => add_ids(&mut cmd, value, true),
            "desc" => {
                cmd.description = Some(TextFilter {
                    contains: Some(value.to_string()),
                });
            }
            "date" => cmd.date = Some(parse_range(value, jalali_range)),
            "duration" => cmd.duration = Some(parse_range(value, duration_range)),
            _ => {}
        }
    }

    cmd
}

fn add_tags(cmd: &mut FilterWorklogsCommand, value: &str, exclude: bool) {
    let list = value.split(',').map(String::from).collect();
    let filter = cmd.tags.get_or_insert_with(ListFilter::default);
    if exclude {
        filter.not_in = Some(list);
    } else {
        filter.in_list = Some(list);
    }
}

fn add_ids(cmd: &mut FilterWorklogsCommand, value: &str, exclude: bool) {
    let list: Vec<Uuid> = value
        .split(',')
        .filter_map(|s| Uuid::from_str(s.trim()).ok())
        .collect();
    if list.is_empty() {
        return;
    }
    let filter = cmd.ids.get_or_insert_with(ListFilter::default);
    if exclude {
        filter.not_in = Some(list);
    } else {
        filter.in_list = Some(list);
    }
}

fn jalali_range(from: Option<String>, to: Option<String>) -> JalaliDateFilter {
    JalaliDateFilter { from, to }
}

fn duration_range(from: Option<String>, to: Option<String>) -> DurationFilter {
    DurationFilter { from, to }
}

fn parse_range<T, F>(value: &str, constructor: F) -> T
where
    F: Fn(Option<String>, Option<String>) -> T,
{
    if let Some((from, to)) = value.split_once("..") {
        let from_opt = if from.is_empty() {
            None
        } else {
            Some(from.to_string())
        };
        let to_opt = if to.is_empty() {
            None
        } else {
            Some(to.to_string())
        };
        constructor(from_opt, to_opt)
    } else if let Some(from) = value.strip_prefix(">=") {
        constructor(Some(from.to_string()), None)
    } else if let Some(to) = value.strip_prefix("<=") {
        constructor(None, Some(to.to_string()))
    } else {
        constructor(Some(value.to_string()), Some(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_quoted_description() {
        let cmd = parse_search_input(r#"tag:rust desc:"fix bug""#);
        assert_eq!(
            cmd.tags.as_ref().unwrap().in_list.as_ref().unwrap(),
            &["rust".to_string()]
        );
        assert_eq!(
            cmd.description.as_ref().unwrap().contains.as_deref(),
            Some("fix bug")
        );
    }
}
