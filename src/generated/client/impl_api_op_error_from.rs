impl<E: std::fmt::Debug> From<reqwest::Error> for ApiOpError<E> {
    fn from(e: reqwest::Error) -> Self {
        Self::Transport(HttpError::Network(e))
    }
}
