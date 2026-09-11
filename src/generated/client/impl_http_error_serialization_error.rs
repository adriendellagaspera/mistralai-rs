impl HttpError {
    /// Create a serialization error
    pub fn serialization_error(error: impl std::fmt::Display) -> Self {
        Self::Serialization(error.to_string())
    }
    /// Check if this transport error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(self, Self::Network(_) | Self::Middleware(_) | Self::Timeout)
    }
}
