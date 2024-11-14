// Type alias for Result
pub type Result<T> = std::result::Result<T, Error>;

// Using `Box<dyn std::error::Error>` for flexibility in error handling
pub type Error = Box<dyn std::error::Error + Send + Sync>;
