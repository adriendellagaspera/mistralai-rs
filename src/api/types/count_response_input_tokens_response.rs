pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CountResponseInputTokensResponse {
    #[serde(default)]
    pub input_tokens: i64,
    pub object: CountResponseInputTokensResponseObject,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl CountResponseInputTokensResponse {
    pub fn builder() -> CountResponseInputTokensResponseBuilder {
        <CountResponseInputTokensResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CountResponseInputTokensResponseBuilder {
    input_tokens: Option<i64>,
    object: Option<CountResponseInputTokensResponseObject>,
}

impl CountResponseInputTokensResponseBuilder {
    pub fn input_tokens(mut self, value: i64) -> Self {
        self.input_tokens = Some(value);
        self
    }

    pub fn object(mut self, value: CountResponseInputTokensResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CountResponseInputTokensResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input_tokens`](CountResponseInputTokensResponseBuilder::input_tokens)
    /// - [`object`](CountResponseInputTokensResponseBuilder::object)
    pub fn build(self) -> Result<CountResponseInputTokensResponse, BuildError> {
        Ok(CountResponseInputTokensResponse {
            input_tokens: self
                .input_tokens
                .ok_or_else(|| BuildError::missing_field("input_tokens"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            extra: Default::default(),
        })
    }
}
