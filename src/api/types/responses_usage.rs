pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesUsage {
    #[serde(default)]
    pub input_tokens: i64,
    #[serde(default)]
    pub input_tokens_details: ResponsesInputTokensDetails,
    #[serde(default)]
    pub output_tokens: i64,
    #[serde(default)]
    pub output_tokens_details: ResponsesOutputTokensDetails,
    #[serde(default)]
    pub total_tokens: i64,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesUsage {
    pub fn builder() -> ResponsesUsageBuilder {
        <ResponsesUsageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesUsageBuilder {
    input_tokens: Option<i64>,
    input_tokens_details: Option<ResponsesInputTokensDetails>,
    output_tokens: Option<i64>,
    output_tokens_details: Option<ResponsesOutputTokensDetails>,
    total_tokens: Option<i64>,
}

impl ResponsesUsageBuilder {
    pub fn input_tokens(mut self, value: i64) -> Self {
        self.input_tokens = Some(value);
        self
    }

    pub fn input_tokens_details(mut self, value: ResponsesInputTokensDetails) -> Self {
        self.input_tokens_details = Some(value);
        self
    }

    pub fn output_tokens(mut self, value: i64) -> Self {
        self.output_tokens = Some(value);
        self
    }

    pub fn output_tokens_details(mut self, value: ResponsesOutputTokensDetails) -> Self {
        self.output_tokens_details = Some(value);
        self
    }

    pub fn total_tokens(mut self, value: i64) -> Self {
        self.total_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesUsage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input_tokens`](ResponsesUsageBuilder::input_tokens)
    /// - [`input_tokens_details`](ResponsesUsageBuilder::input_tokens_details)
    /// - [`output_tokens`](ResponsesUsageBuilder::output_tokens)
    /// - [`output_tokens_details`](ResponsesUsageBuilder::output_tokens_details)
    /// - [`total_tokens`](ResponsesUsageBuilder::total_tokens)
    pub fn build(self) -> Result<ResponsesUsage, BuildError> {
        Ok(ResponsesUsage {
            input_tokens: self
                .input_tokens
                .ok_or_else(|| BuildError::missing_field("input_tokens"))?,
            input_tokens_details: self
                .input_tokens_details
                .ok_or_else(|| BuildError::missing_field("input_tokens_details"))?,
            output_tokens: self
                .output_tokens
                .ok_or_else(|| BuildError::missing_field("output_tokens"))?,
            output_tokens_details: self
                .output_tokens_details
                .ok_or_else(|| BuildError::missing_field("output_tokens_details"))?,
            total_tokens: self
                .total_tokens
                .ok_or_else(|| BuildError::missing_field("total_tokens"))?,
            extra: Default::default(),
        })
    }
}
