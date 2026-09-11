impl<E: std::fmt::Debug> From<reqwest_middleware::Error> for ApiOpError<E> {
    fn from(e: reqwest_middleware::Error) -> Self {
        Self::Transport(HttpError::Middleware(e))
    }
}
