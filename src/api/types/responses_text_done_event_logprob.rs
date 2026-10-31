pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesTextDoneEventLogprob {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub logprob: f64,
    #[serde(default)]
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<Vec<ResponsesTextDoneEventLogprobTopLogprob>>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesTextDoneEventLogprob {
    pub fn builder() -> ResponsesTextDoneEventLogprobBuilder {
        <ResponsesTextDoneEventLogprobBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesTextDoneEventLogprobBuilder {
    logprob: Option<f64>,
    token: Option<String>,
    top_logprobs: Option<Vec<ResponsesTextDoneEventLogprobTopLogprob>>,
}

impl ResponsesTextDoneEventLogprobBuilder {
    pub fn logprob(mut self, value: f64) -> Self {
        self.logprob = Some(value);
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn top_logprobs(mut self, value: Vec<ResponsesTextDoneEventLogprobTopLogprob>) -> Self {
        self.top_logprobs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesTextDoneEventLogprob`].
    /// This method will fail if any of the following fields are not set:
    /// - [`logprob`](ResponsesTextDoneEventLogprobBuilder::logprob)
    /// - [`token`](ResponsesTextDoneEventLogprobBuilder::token)
    pub fn build(self) -> Result<ResponsesTextDoneEventLogprob, BuildError> {
        Ok(ResponsesTextDoneEventLogprob {
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
