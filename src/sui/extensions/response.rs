use reqwest::Response;
use serde::de::DeserializeOwned;

use crate::error::ClientError;

/// Extension trait for RequestBuilder to add default headers
///
/// This trait provides a clean, reusable way to add authentication and content type headers
/// to all requests in a consistent manner. It encapsulates the header logic and makes
/// it available to any RequestBuilder instance.
#[async_trait::async_trait]
pub trait ResponseExt {
    async fn json_or_err<T: DeserializeOwned>(self) -> Result<T, ClientError>;
}

#[async_trait::async_trait]
impl ResponseExt for Response {
    async fn json_or_err<T: DeserializeOwned>(self) -> Result<T, ClientError> {
        let full = self.bytes().await?;

        serde_json::from_slice(&full).map_err(|e| {
            // Convert bytes to string for error reporting, with fallback for invalid UTF-8
            let response_text = String::from_utf8_lossy(&full);
            ClientError::Other(format!("JSON parse error: {}. Response: {}", e, response_text))
        })
    }
}
