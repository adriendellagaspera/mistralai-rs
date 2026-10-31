pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesInputTokensDetails {
    #[serde(default)]
    pub cache_write_tokens: i64,
    #[serde(default)]
    pub cached_tokens: i64,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesInputTokensDetails {
    pub fn builder() -> ResponsesInputTokensDetailsBuilder {
        <ResponsesInputTokensDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesInputTokensDetailsBuilder {
    cache_write_tokens: Option<i64>,
    cached_tokens: Option<i64>,
}

impl ResponsesInputTokensDetailsBuilder {
    pub fn cache_write_tokens(mut self, value: i64) -> Self {
        self.cache_write_tokens = Some(value);
        self
    }

    pub fn cached_tokens(mut self, value: i64) -> Self {
        self.cached_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesInputTokensDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cache_write_tokens`](ResponsesInputTokensDetailsBuilder::cache_write_tokens)
    /// - [`cached_tokens`](ResponsesInputTokensDetailsBuilder::cached_tokens)
    pub fn build(self) -> Result<ResponsesInputTokensDetails, BuildError> {
        Ok(ResponsesInputTokensDetails {
            cache_write_tokens: self
                .cache_write_tokens
                .ok_or_else(|| BuildError::missing_field("cache_write_tokens"))?,
            cached_tokens: self
                .cached_tokens
                .ok_or_else(|| BuildError::missing_field("cached_tokens"))?,
            extra: Default::default(),
        })
    }
}
