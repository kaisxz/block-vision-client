use reqwest::RequestBuilder;
use secrecy::{ExposeSecret, SecretString};

/// Extension trait for RequestBuilder to add default headers
///
/// This trait provides a clean, reusable way to add authentication and content type headers
/// to all requests in a consistent manner. It encapsulates the header logic and makes
/// it available to any RequestBuilder instance.
pub trait RequestBuilderExt {
    /// Adds default headers (Content-Type and x-api-key) to the request
    ///
    /// This method ensures all requests include the required authentication and content type headers
    /// following security best practices and API standards.
    ///
    /// # Arguments
    /// * `api_key` - The API key to include in the x-api-key header
    ///
    /// # Returns
    /// * `Self` - The request builder with default headers applied
    fn with_default_headers(self, api_key: &SecretString) -> Self;
}

impl RequestBuilderExt for RequestBuilder {
    fn with_default_headers(self, api_key: &SecretString) -> Self {
        self.header("Content-Type", "application/json")
            .header("x-api-key", api_key.expose_secret())
    }
}
