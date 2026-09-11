impl<E: std::fmt::Debug> ApiOpError<E> {
    /// Returns the API envelope when this is an `Api` variant.
    pub fn api(&self) -> Option<&ApiError<E>> {
        match self {
            Self::Api(e) => Some(e),
            Self::Transport(_) => None,
        }
    }
    /// True when the underlying error came from the server (i.e.
    /// any `Api` variant) rather than the transport layer.
    pub fn is_api_error(&self) -> bool {
        matches!(self, Self::Api(_))
    }
}
