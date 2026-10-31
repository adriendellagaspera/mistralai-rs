pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesOutputText {
    #[serde(default)]
    pub annotations: Vec<ResponsesOutputTextAnnotationsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Vec<ResponsesOutputTextLogprob>>,
    #[serde(default)]
    pub text: String,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesOutputText {
    pub fn builder() -> ResponsesOutputTextBuilder {
        <ResponsesOutputTextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesOutputTextBuilder {
    annotations: Option<Vec<ResponsesOutputTextAnnotationsItem>>,
    logprobs: Option<Vec<ResponsesOutputTextLogprob>>,
    text: Option<String>,
}

impl ResponsesOutputTextBuilder {
    pub fn annotations(mut self, value: Vec<ResponsesOutputTextAnnotationsItem>) -> Self {
        self.annotations = Some(value);
        self
    }

    pub fn logprobs(mut self, value: Vec<ResponsesOutputTextLogprob>) -> Self {
        self.logprobs = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResponsesOutputText`].
    /// This method will fail if any of the following fields are not set:
    /// - [`annotations`](ResponsesOutputTextBuilder::annotations)
    /// - [`text`](ResponsesOutputTextBuilder::text)
    pub fn build(self) -> Result<ResponsesOutputText, BuildError> {
        Ok(ResponsesOutputText {
            annotations: self
                .annotations
                .ok_or_else(|| BuildError::missing_field("annotations"))?,
            logprobs: self.logprobs,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            extra: Default::default(),
        })
    }
}
