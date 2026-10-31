pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesOutputTextLogprob {
    #[serde(default)]
    pub bytes: Vec<i64>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub logprob: f64,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub top_logprobs: Vec<ResponsesOutputTextLogprobTopLogprob>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesOutputTextLogprob {
    pub fn builder() -> ResponsesOutputTextLogprobBuilder {
        <ResponsesOutputTextLogprobBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesOutputTextLogprobBuilder {
    bytes: Option<Vec<i64>>,
    logprob: Option<f64>,
    token: Option<String>,
    top_logprobs: Option<Vec<ResponsesOutputTextLogprobTopLogprob>>,
}

impl ResponsesOutputTextLogprobBuilder {
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

    pub fn top_logprobs(mut self, value: Vec<ResponsesOutputTextLogprobTopLogprob>) -> Self {
        self.top_logprobs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesOutputTextLogprob`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bytes`](ResponsesOutputTextLogprobBuilder::bytes)
    /// - [`logprob`](ResponsesOutputTextLogprobBuilder::logprob)
    /// - [`token`](ResponsesOutputTextLogprobBuilder::token)
    /// - [`top_logprobs`](ResponsesOutputTextLogprobBuilder::top_logprobs)
    pub fn build(self) -> Result<ResponsesOutputTextLogprob, BuildError> {
        Ok(ResponsesOutputTextLogprob {
            bytes: self
                .bytes
                .ok_or_else(|| BuildError::missing_field("bytes"))?,
            logprob: self
                .logprob
                .ok_or_else(|| BuildError::missing_field("logprob"))?,
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
            top_logprobs: self
                .top_logprobs
                .ok_or_else(|| BuildError::missing_field("top_logprobs"))?,
            extra: Default::default(),
        })
    }
}
