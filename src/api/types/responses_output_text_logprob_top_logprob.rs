pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesOutputTextLogprobTopLogprob {
    #[serde(default)]
    pub bytes: Vec<i64>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub logprob: f64,
    #[serde(default)]
    pub token: String,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesOutputTextLogprobTopLogprob {
    pub fn builder() -> ResponsesOutputTextLogprobTopLogprobBuilder {
        <ResponsesOutputTextLogprobTopLogprobBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesOutputTextLogprobTopLogprobBuilder {
    bytes: Option<Vec<i64>>,
    logprob: Option<f64>,
    token: Option<String>,
}

impl ResponsesOutputTextLogprobTopLogprobBuilder {
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

    /// Consumes the builder and constructs a [`ResponsesOutputTextLogprobTopLogprob`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bytes`](ResponsesOutputTextLogprobTopLogprobBuilder::bytes)
    /// - [`logprob`](ResponsesOutputTextLogprobTopLogprobBuilder::logprob)
    /// - [`token`](ResponsesOutputTextLogprobTopLogprobBuilder::token)
    pub fn build(self) -> Result<ResponsesOutputTextLogprobTopLogprob, BuildError> {
        Ok(ResponsesOutputTextLogprobTopLogprob {
            bytes: self
                .bytes
                .ok_or_else(|| BuildError::missing_field("bytes"))?,
            logprob: self
                .logprob
                .ok_or_else(|| BuildError::missing_field("logprob"))?,
            token: self
                .token
                .ok_or_else(|| BuildError::missing_field("token"))?,
            extra: Default::default(),
        })
    }
}
