use crate::generated::client::{ApiOpError, HttpError};

/// Stable classification of failures that happen before a response is available.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportErrorKind {
    Network,
    Middleware,
    Serialization,
    Authentication,
    Timeout,
    ResponseTooLarge,
    Configuration,
    Other,
}

/// Transport failure exposed without leaking a generated error type.
#[derive(Debug)]
pub struct TransportError {
    kind: TransportErrorKind,
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl TransportError {
    pub fn kind(&self) -> TransportErrorKind {
        self.kind
    }

    pub fn is_retryable(&self) -> bool {
        matches!(
            self.kind,
            TransportErrorKind::Network
                | TransportErrorKind::Middleware
                | TransportErrorKind::Timeout
        )
    }
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for TransportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn std::error::Error + 'static))
    }
}

impl From<HttpError> for TransportError {
    fn from(error: HttpError) -> Self {
        match error {
            HttpError::Network(source) => Self {
                kind: TransportErrorKind::Network,
                message: source.to_string(),
                source: Some(Box::new(source)),
            },
            HttpError::Middleware(source) => Self {
                kind: TransportErrorKind::Middleware,
                message: source.to_string(),
                source: Some(Box::new(source)),
            },
            HttpError::Serialization(message) => Self {
                kind: TransportErrorKind::Serialization,
                message,
                source: None,
            },
            HttpError::Auth(message) => Self {
                kind: TransportErrorKind::Authentication,
                message,
                source: None,
            },
            HttpError::Timeout => Self {
                kind: TransportErrorKind::Timeout,
                message: "request timed out".into(),
                source: None,
            },
            HttpError::ResponseTooLarge { limit } => Self {
                kind: TransportErrorKind::ResponseTooLarge,
                message: format!("response body exceeded configured limit of {limit} bytes"),
                source: None,
            },
            HttpError::Config(message) => Self {
                kind: TransportErrorKind::Configuration,
                message,
                source: None,
            },
            HttpError::Other(message) => Self {
                kind: TransportErrorKind::Other,
                message,
                source: None,
            },
        }
    }
}

/// An HTTP response rejected by the Mistral API.
///
/// The stable facade deliberately does not expose operation-specific generated
/// error enums. It does preserve the complete response so callers can inspect
/// or deserialize an error body without losing information.
#[derive(Debug, Clone)]
pub struct ApiError {
    status: u16,
    headers: reqwest::header::HeaderMap,
    body: String,
    raw_body: Vec<u8>,
    parse_error: Option<String>,
}

impl ApiError {
    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn headers(&self) -> &reqwest::header::HeaderMap {
        &self.headers
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn raw_body(&self) -> &[u8] {
        &self.raw_body
    }

    pub fn parse_error(&self) -> Option<&str> {
        self.parse_error.as_deref()
    }

    pub fn deserialize_body<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.raw_body)
    }

    pub fn is_client_error(&self) -> bool {
        (400..500).contains(&self.status)
    }

    pub fn is_server_error(&self) -> bool {
        (500..600).contains(&self.status)
    }

    pub fn is_retryable(&self) -> bool {
        matches!(self.status, 429 | 500 | 502 | 503 | 504)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const LIMIT: usize = 500;
        let end = self
            .body
            .char_indices()
            .nth(LIMIT)
            .map_or(self.body.len(), |(index, _)| index);
        write!(
            f,
            "Mistral API returned HTTP {}: {}",
            self.status,
            &self.body[..end]
        )?;
        if end < self.body.len() {
            f.write_str("... [truncated]")?;
        }
        Ok(())
    }
}

impl std::error::Error for ApiError {}

/// Errors surfaced by the idiomatic SDK facade.
#[derive(Debug, thiserror::Error)]
pub enum SdkError {
    #[error(transparent)]
    Transport(#[from] TransportError),

    #[error(transparent)]
    Api(#[from] ApiError),

    #[error(transparent)]
    Stream(#[from] crate::streaming::Error),
}

impl<E: std::fmt::Debug> From<ApiOpError<E>> for SdkError {
    fn from(error: ApiOpError<E>) -> Self {
        match error {
            ApiOpError::Transport(error) => Self::Transport(error.into()),
            ApiOpError::Api(error) => Self::Api(ApiError {
                status: error.status,
                headers: error.headers,
                body: error.body,
                raw_body: error.raw_body,
                parse_error: error.parse_error,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::client;

    #[test]
    fn api_errors_preserve_the_complete_response() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-request-id", "request-7".parse().unwrap());
        let source: ApiOpError<()> = ApiOpError::Api(client::ApiError {
            status: 429,
            headers,
            body: "rate limited".into(),
            raw_body: b"rate limited".to_vec(),
            typed: None,
            parse_error: Some("not json".into()),
        });

        let SdkError::Api(error) = SdkError::from(source) else {
            panic!("expected an API error");
        };
        assert_eq!(error.status(), 429);
        assert_eq!(error.headers()["x-request-id"], "request-7");
        assert_eq!(error.body(), "rate limited");
        assert_eq!(error.raw_body(), b"rate limited");
        assert_eq!(error.parse_error(), Some("not json"));
        assert!(error.is_client_error());
        assert!(error.is_retryable());
    }
}
