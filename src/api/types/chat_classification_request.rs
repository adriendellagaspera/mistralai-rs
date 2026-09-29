pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatClassificationRequest {
    pub input: ChatClassificationRequestInputs,
    #[serde(default)]
    pub model: String,
}

impl ChatClassificationRequest {
    pub fn builder() -> ChatClassificationRequestBuilder {
        <ChatClassificationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatClassificationRequestBuilder {
    input: Option<ChatClassificationRequestInputs>,
    model: Option<String>,
}

impl ChatClassificationRequestBuilder {
    pub fn input(mut self, value: ChatClassificationRequestInputs) -> Self {
        self.input = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChatClassificationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](ChatClassificationRequestBuilder::input)
    /// - [`model`](ChatClassificationRequestBuilder::model)
    pub fn build(self) -> Result<ChatClassificationRequest, BuildError> {
        Ok(ChatClassificationRequest {
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
        })
    }
}
