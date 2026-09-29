pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatCompletionResponse {
    #[serde(flatten)]
    pub chat_completion_response_base_fields: ChatCompletionResponseBase,
    #[serde(default)]
    pub choices: Vec<ChatCompletionChoice>,
}

impl ChatCompletionResponse {
    pub fn builder() -> ChatCompletionResponseBuilder {
        <ChatCompletionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionResponseBuilder {
    chat_completion_response_base_fields: Option<ChatCompletionResponseBase>,
    choices: Option<Vec<ChatCompletionChoice>>,
}

impl ChatCompletionResponseBuilder {
    pub fn chat_completion_response_base_fields(
        mut self,
        value: ChatCompletionResponseBase,
    ) -> Self {
        self.chat_completion_response_base_fields = Some(value);
        self
    }

    pub fn choices(mut self, value: Vec<ChatCompletionChoice>) -> Self {
        self.choices = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatCompletionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chat_completion_response_base_fields`](ChatCompletionResponseBuilder::chat_completion_response_base_fields)
    /// - [`choices`](ChatCompletionResponseBuilder::choices)
    pub fn build(self) -> Result<ChatCompletionResponse, BuildError> {
        Ok(ChatCompletionResponse {
            chat_completion_response_base_fields: self
                .chat_completion_response_base_fields
                .ok_or_else(|| BuildError::missing_field("chat_completion_response_base_fields"))?,
            choices: self
                .choices
                .ok_or_else(|| BuildError::missing_field("choices"))?,
        })
    }
}
