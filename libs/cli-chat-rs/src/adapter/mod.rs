pub mod traits;
pub mod demo;

pub use traits::*;
pub use demo::DemoAdapter;

/// Result type for adapter operations
pub type AdapterResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
