use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize)]
pub struct PagingParams {
    pub page: u32,
    pub size: u32,
}

impl Default for PagingParams {
    fn default() -> Self {
        Self {
            page: 1,
            size: 20,
        }
    }
}

impl PagingParams {
    pub fn offset(&self) -> u32 {
        (self.page.saturating_sub(1)) * self.size
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct PageResult<T> {
    pub items: Vec<T>,
    pub total_items: u64,
    pub total_pages: u32,
    pub current_page: u32,
    pub page_size: u32,
}

impl <T> PageResult<T> {
    
    pub fn new(items: Vec<T>, total_items: u64, current_page: u32, page_size: u32) -> Self {
        let total_pages = (total_items as f64 / page_size as f64).ceil() as u32;
        Self {
            items,
            total_items,
            total_pages,
            current_page,
            page_size,
        }
    }
}