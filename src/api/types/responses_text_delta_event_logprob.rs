pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesTextDeltaEventLogprob {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub logprob: f64,
    #[serde(default)]
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<Vec<ResponsesTextDeltaEventLogprobTopLogprob>>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesTextDeltaEventLogprob {
    pub fn builder() -> ResponsesTextDeltaEventLogprobBuilder {
        <ResponsesTextDeltaEventLogprobBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesTextDeltaEventLogprobBuilder {
    logprob: Option<f64>,
    token: Option<String>,
    top_logprobs: Option<Vec<ResponsesTextDeltaEventLogprobTopLogprob>>,
}

impl ResponsesTextDeltaEventLogprobBuilder {
    pub fn logprob(mut self, value: f64) -> Self {
        self.logprob = Some(value);
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn top_logprobs(mut self, value: Vec<ResponsesTextDeltaEventLogprobTopLogprob>) -> Self {
        self.top_logprobs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesTextDeltaEventLogprob`].
    /// This method will fail if any of the following fields are not set:
    /// - [`logprob`](ResponsesTextDeltaEventLogprobBuilder::logprob)
    /// - [`token`](ResponsesTextDeltaEventLogprobBuilder::token)
    pub fn build(self) -> Result<ResponsesTextDeltaEventLogprob, BuildError> {
        Ok(ResponsesTextDeltaEventLogprob {
            logprob: self
                .logprob
                .ok_or_else(|| BuildError::missing_field("logprob"))?,
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
            top_logprobs: self.top_logprobs,
            extra: Default::default(),
        })
    }
}
