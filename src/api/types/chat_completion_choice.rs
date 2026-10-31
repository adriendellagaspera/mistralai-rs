pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionChoice {
    pub finish_reason: ChatCompletionChoiceFinishReason,
    #[serde(default)]
    pub index: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<ChatCompletionResponseChoiceLogprobs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<AssistantMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<DeltaMessage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_logprobs: Option<ChatCompletionResponseChoiceLogprobs>,
}

impl ChatCompletionChoice {
    pub fn builder() -> ChatCompletionChoiceBuilder {
        <ChatCompletionChoiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionChoiceBuilder {
    finish_reason: Option<ChatCompletionChoiceFinishReason>,
    index: Option<i64>,
    logprobs: Option<ChatCompletionResponseChoiceLogprobs>,
    message: Option<AssistantMessage>,
    messages: Option<Vec<DeltaMessage>>,
    prompt_logprobs: Option<ChatCompletionResponseChoiceLogprobs>,
}

impl ChatCompletionChoiceBuilder {
    pub fn finish_reason(mut self, value: ChatCompletionChoiceFinishReason) -> Self {
        self.finish_reason = Some(value);
        self
    }

    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn logprobs(mut self, value: ChatCompletionResponseChoiceLogprobs) -> Self {
        self.logprobs = Some(value);
        self
    }

    pub fn message(mut self, value: AssistantMessage) -> Self {
        self.message = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<DeltaMessage>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn prompt_logprobs(mut self, value: ChatCompletionResponseChoiceLogprobs) -> Self {
        self.prompt_logprobs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatCompletionChoice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`finish_reason`](ChatCompletionChoiceBuilder::finish_reason)
    /// - [`index`](ChatCompletionChoiceBuilder::index)
    pub fn build(self) -> Result<ChatCompletionChoice, BuildError> {
        Ok(ChatCompletionChoice {
            finish_reason: self
                .finish_reason
                .ok_or_else(|| BuildError::missing_field("finish_reason"))?,
            index: self
                .index
                .ok_or_else(|| BuildError::missing_field("index"))?,
            logprobs: self.logprobs,
            message: self.message,
            messages: self.messages,
            prompt_logprobs: self.prompt_logprobs,
        })
    }
}
