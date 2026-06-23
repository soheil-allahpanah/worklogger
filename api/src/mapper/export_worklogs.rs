use axum::{
    http::{header, HeaderMap, HeaderValue},
    response::IntoResponse,
};
use use_cases::ExportWorklogsResponse;

pub fn export_response_to_http(response: ExportWorklogsResponse) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&response.content_type).unwrap_or(HeaderValue::from_static(
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        )),
    );
    let disposition = format!("attachment; filename=\"{}\"", response.filename);
    if let Ok(value) = HeaderValue::from_str(&disposition) {
        headers.insert(header::CONTENT_DISPOSITION, value);
    }
    if let Ok(value) = HeaderValue::from_str(&response.row_count.to_string()) {
        headers.insert("x-row-count", value);
    }
    (headers, response.bytes)
}
