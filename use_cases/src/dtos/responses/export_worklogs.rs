/// Binary file payload returned by export use cases (Excel, CSV, etc.).
#[derive(Debug, Clone)]
pub struct ExportWorklogsResponse {
    pub bytes: Vec<u8>,
    pub filename: String,
    pub content_type: String,
    pub row_count: usize,
}

impl ExportWorklogsResponse {
    pub fn new(
        bytes: Vec<u8>,
        filename: String,
        content_type: impl Into<String>,
        row_count: usize,
    ) -> Self {
        Self {
            bytes,
            filename,
            content_type: content_type.into(),
            row_count,
        }
    }
}
