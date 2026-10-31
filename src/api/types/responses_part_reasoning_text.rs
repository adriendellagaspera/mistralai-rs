pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesPartReasoningText {
    #[serde(default)]
    pub text: String,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesPartReasoningText {
    pub fn builder() -> ResponsesPartReasoningTextBuilder {
        <ResponsesPartReasoningTextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesPartReasoningTextBuilder {
    text: Option<String>,
}

impl ResponsesPartReasoningTextBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResponsesPartReasoningText`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](ResponsesPartReasoningTextBuilder::text)
    pub fn build(self) -> Result<ResponsesPartReasoningText, BuildError> {
        Ok(ResponsesPartReasoningText {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            extra: Default::default(),
        })
    }
}
