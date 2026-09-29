pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatModerationRequest {
    /// Chat to classify
    pub input: ChatModerationRequestInput,
    #[serde(default)]
    pub model: String,
}

impl ChatModerationRequest {
    pub fn builder() -> ChatModerationRequestBuilder {
        <ChatModerationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatModerationRequestBuilder {
    input: Option<ChatModerationRequestInput>,
    model: Option<String>,
}

impl ChatModerationRequestBuilder {
    pub fn input(mut self, value: ChatModerationRequestInput) -> Self {
        self.input = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChatModerationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](ChatModerationRequestBuilder::input)
    /// - [`model`](ChatModerationRequestBuilder::model)
    pub fn build(self) -> Result<ChatModerationRequest, BuildError> {
        Ok(ChatModerationRequest {
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
            model: self
                .model
                .ok_or_else(|| BuildError::missing_field("model"))?,
        })
    }
}
