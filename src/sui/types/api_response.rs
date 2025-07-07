use http::StatusCode;
use serde::{Deserialize, Serialize};

/// Generic API response wrapper for BlockVision API
///
/// All BlockVision API responses follow this standard format with
/// code, message, and result fields. This wrapper can be used
/// to deserialize any API response.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// HTTP status code returned by the API
    #[serde(with = "http_serde::status_code")]
    pub code: StatusCode,

    /// Status message from the API
    pub message: String,

    /// The actual response data
    pub result: Option<T>,
}
