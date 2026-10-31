pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatCompletionResponseTokenLogprobsTopK {
    #[serde(default)]
    pub bytes: Vec<i64>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub logprob: f64,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub token_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<Vec<ChatCompletionResponseTokenLogprobs>>,
}

impl ChatCompletionResponseTokenLogprobsTopK {
    pub fn builder() -> ChatCompletionResponseTokenLogprobsTopKBuilder {
        <ChatCompletionResponseTokenLogprobsTopKBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionResponseTokenLogprobsTopKBuilder {
    bytes: Option<Vec<i64>>,
    logprob: Option<f64>,
    token: Option<String>,
    token_id: Option<i64>,
    top_logprobs: Option<Vec<ChatCompletionResponseTokenLogprobs>>,
}

impl ChatCompletionResponseTokenLogprobsTopKBuilder {
    pub fn bytes(mut self, value: Vec<i64>) -> Self {
        self.bytes = Some(value);
        self
    }

    pub fn logprob(mut self, value: f64) -> Self {
        self.logprob = Some(value);
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn token_id(mut self, value: i64) -> Self {
        self.token_id = Some(value);
        self
    }

    pub fn top_logprobs(mut self, value: Vec<ChatCompletionResponseTokenLogprobs>) -> Self {
        self.top_logprobs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatCompletionResponseTokenLogprobsTopK`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bytes`](ChatCompletionResponseTokenLogprobsTopKBuilder::bytes)
    /// - [`logprob`](ChatCompletionResponseTokenLogprobsTopKBuilder::logprob)
    /// - [`token`](ChatCompletionResponseTokenLogprobsTopKBuilder::token)
    /// - [`token_id`](ChatCompletionResponseTokenLogprobsTopKBuilder::token_id)
    pub fn build(self) -> Result<ChatCompletionResponseTokenLogprobsTopK, BuildError> {
        Ok(ChatCompletionResponseTokenLogprobsTopK {
            bytes: self
                .bytes
                .ok_or_else(|| BuildError::missing_field("bytes"))?,
            logprob: self
                .logprob
                .ok_or_else(|| BuildError::missing_field("logprob"))?,
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
            token_id: self
                .token_id
                .ok_or_else(|| BuildError::missing_field("token_id"))?,
            top_logprobs: self.top_logprobs,
        })
    }
}
