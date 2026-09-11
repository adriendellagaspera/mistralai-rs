/// Transport-level errors: failures where no safely inspectable
/// HTTP response is available to the caller.
///
/// HTTP responses with non-2xx status codes are surfaced as
/// [`ApiError`] inside [`ApiOpError::Api`], not here, so callers can
/// always inspect status, headers, and the raw body when the server
/// actually responded.
#[derive(Error, Debug)]
pub enum HttpError {
    /// Network or connection error (from reqwest)
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    /// Middleware error (from reqwest-middleware)
    #[error("Middleware error: {0}")]
    Middleware(#[from] reqwest_middleware::Error),
    /// Request serialization error
    #[error("Failed to serialize request: {0}")]
    Serialization(String),
    /// Authentication error
    #[error("Authentication error: {0}")]
    Auth(String),
    /// Request timeout
    #[error("Request timeout")]
    Timeout,
    /// A response body exceeded the configured in-memory limit
    #[error("Response body exceeded configured limit of {limit} bytes")]
    ResponseTooLarge { limit: usize },
    /// Invalid configuration
    #[error("Configuration error: {0}")]
    Config(String),
    /// Generic error
    #[error("{0}")]
    Other(String),
}
