use crate::dto::FilterWorklogsRequest;
use crate::error::{ApiError, ApiResult};

pub fn validate_filter(request: &mut FilterWorklogsRequest) -> ApiResult<()> {
    request.validate().map_err(|errors| {
        ApiError::bad_request_with_details("invalid filter parameters", errors)
    })
}
