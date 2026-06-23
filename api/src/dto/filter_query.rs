use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FilterQuery {
    pub tags: Option<String>,
    pub exclude_tags: Option<String>,
    pub ids: Option<String>,
    pub exclude_ids: Option<String>,
    pub description: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub duration_from: Option<String>,
    pub duration_to: Option<String>,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub size: u32,
}

fn default_page() -> u32 {
    1
}

pub const DEFAULT_PAGE_SIZE: u32 = 20;

fn default_page_size() -> u32 {
    DEFAULT_PAGE_SIZE
}
