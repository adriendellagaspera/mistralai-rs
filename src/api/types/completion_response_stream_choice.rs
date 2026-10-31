pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CompletionResponseStreamChoice {
    #[serde(default)]
    pub delta: DeltaMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<CompletionResponseStreamChoiceFinishReason>,
    #[serde(default)]
    pub index: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<ChatCompletionResponseChoiceLogprobs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_logprobs: Option<ChatCompletionResponseChoiceLogprobs>,
}

impl CompletionResponseStreamChoice {
    pub fn builder() -> CompletionResponseStreamChoiceBuilder {
        <CompletionResponseStreamChoiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompletionResponseStreamChoiceBuilder {
    delta: Option<DeltaMessage>,
    finish_reason: Option<CompletionResponseStreamChoiceFinishReason>,
    index: Option<i64>,
    logprobs: Option<ChatCompletionResponseChoiceLogprobs>,
    prompt_logprobs: Option<ChatCompletionResponseChoiceLogprobs>,
}

impl CompletionResponseStreamChoiceBuilder {
    pub fn delta(mut self, value: DeltaMessage) -> Self {
        self.delta = Some(value);
        self
    }

    pub fn finish_reason(mut self, value: CompletionResponseStreamChoiceFinishReason) -> Self {
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

    pub fn prompt_logprobs(mut self, value: ChatCompletionResponseChoiceLogprobs) -> Self {
        self.prompt_logprobs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompletionResponseStreamChoice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`delta`](CompletionResponseStreamChoiceBuilder::delta)
    /// - [`index`](CompletionResponseStreamChoiceBuilder::index)
    pub fn build(self) -> Result<CompletionResponseStreamChoice, BuildError> {
        Ok(CompletionResponseStreamChoice {
            delta: self
                .delta
                .ok_or_else(|| BuildError::missing_field("delta"))?,
            finish_reason: self.finish_reason,
            index: self
                .index
                .ok_or_else(|| BuildError::missing_field("index"))?,
            logprobs: self.logprobs,
            prompt_logprobs: self.prompt_logprobs,
        })
    }
}
