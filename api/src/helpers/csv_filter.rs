use common::filter::ListFilter;
use uuid::Uuid;

pub fn list_filter_from_csv(
    include: Option<String>,
    exclude: Option<String>,
) -> Option<ListFilter<String>> {
    let in_list = include.map(|csv| split_csv(&csv));
    let not_in = exclude.map(|csv| split_csv(&csv));
    if in_list.is_none() && not_in.is_none() {
        None
    } else {
        Some(ListFilter::new(in_list, not_in))
    }
}

pub fn uuid_list_filter_from_csv(
    include: Option<String>,
    exclude: Option<String>,
) -> Option<ListFilter<Uuid>> {
    let in_list = include.map(|csv| parse_uuid_csv(&csv));
    let not_in = exclude.map(|csv| parse_uuid_csv(&csv));
    if in_list.is_none() && not_in.is_none() {
        None
    } else {
        Some(ListFilter::new(in_list, not_in))
    }
}

pub fn split_csv(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

pub fn parse_uuid_csv(raw: &str) -> Vec<Uuid> {
    raw.split(',')
        .filter_map(|s| Uuid::parse_str(s.trim()).ok())
        .collect()
}
