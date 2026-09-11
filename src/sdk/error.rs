/// Errors surfaced by the idiomatic SDK facade.
#[derive(Debug, thiserror::Error)]
pub enum SdkError {
    #[error("failed to translate SDK request to the generated OpenAPI model: {0}")]
    Request(#[from] serde_json::Error),

    #[error("Mistral API request failed: {0}")]
    Api(String),
}

impl SdkError {
    pub(crate) fn api(error: impl std::fmt::Display) -> Self {
        Self::Api(error.to_string())
    }
}
