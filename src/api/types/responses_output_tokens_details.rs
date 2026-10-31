pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesOutputTokensDetails {
    #[serde(default)]
    pub reasoning_tokens: i64,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesOutputTokensDetails {
    pub fn builder() -> ResponsesOutputTokensDetailsBuilder {
        <ResponsesOutputTokensDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesOutputTokensDetailsBuilder {
    reasoning_tokens: Option<i64>,
}

impl ResponsesOutputTokensDetailsBuilder {
    pub fn reasoning_tokens(mut self, value: i64) -> Self {
        self.reasoning_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesOutputTokensDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reasoning_tokens`](ResponsesOutputTokensDetailsBuilder::reasoning_tokens)
    pub fn build(self) -> Result<ResponsesOutputTokensDetails, BuildError> {
        Ok(ResponsesOutputTokensDetails {
            reasoning_tokens: self
                .reasoning_tokens
                .ok_or_else(|| BuildError::missing_field("reasoning_tokens"))?,
            extra: Default::default(),
        })
    }
}
