pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatClassificationRequest {
    #[serde(default)]
    pub model: String,
    pub input: ChatClassificationRequestInputs,
}

impl ChatClassificationRequest {
    pub fn builder() -> ChatClassificationRequestBuilder {
        <ChatClassificationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatClassificationRequestBuilder {
    model: Option<String>,
    input: Option<ChatClassificationRequestInputs>,
}

impl ChatClassificationRequestBuilder {
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn input(mut self, value: ChatClassificationRequestInputs) -> Self {
        self.input = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatClassificationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model`](ChatClassificationRequestBuilder::model)
    /// - [`input`](ChatClassificationRequestBuilder::input)
    pub fn build(self) -> Result<ChatClassificationRequest, BuildError> {
        Ok(ChatClassificationRequest {
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
        })
    }
}
