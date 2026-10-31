pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesTextDoneEventLogprobTopLogprob {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprob: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesTextDoneEventLogprobTopLogprob {
    pub fn builder() -> ResponsesTextDoneEventLogprobTopLogprobBuilder {
        <ResponsesTextDoneEventLogprobTopLogprobBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesTextDoneEventLogprobTopLogprobBuilder {
    logprob: Option<f64>,
    token: Option<String>,
}

impl ResponsesTextDoneEventLogprobTopLogprobBuilder {
    pub fn logprob(mut self, value: f64) -> Self {
        self.logprob = Some(value);
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResponsesTextDoneEventLogprobTopLogprob`].
    pub fn build(self) -> Result<ResponsesTextDoneEventLogprobTopLogprob, BuildError> {
        Ok(ResponsesTextDoneEventLogprobTopLogprob {
            logprob: self.logprob,
            token: self.token,
            extra: Default::default(),
        })
    }
}
