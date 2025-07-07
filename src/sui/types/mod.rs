// Types module for Sui API - organizes all API-specific type definitions
// Each API endpoint has its own dedicated type file for better encapsulation

pub mod api_response;
pub mod coin_detail;
pub mod coin_dex_pools;

// Re-export ApiResponse for convenience
pub use api_response::ApiResponse;
