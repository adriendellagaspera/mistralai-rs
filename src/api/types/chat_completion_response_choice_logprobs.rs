pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatCompletionResponseChoiceLogprobs {
    #[serde(default)]
    pub content: Vec<ChatCompletionResponseTokenLogprobsTopK>,
}

impl ChatCompletionResponseChoiceLogprobs {
    pub fn builder() -> ChatCompletionResponseChoiceLogprobsBuilder {
        <ChatCompletionResponseChoiceLogprobsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionResponseChoiceLogprobsBuilder {
    content: Option<Vec<ChatCompletionResponseTokenLogprobsTopK>>,
}

impl ChatCompletionResponseChoiceLogprobsBuilder {
    pub fn content(mut self, value: Vec<ChatCompletionResponseTokenLogprobsTopK>) -> Self {
        self.content = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatCompletionResponseChoiceLogprobs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](ChatCompletionResponseChoiceLogprobsBuilder::content)
    pub fn build(self) -> Result<ChatCompletionResponseChoiceLogprobs, BuildError> {
        Ok(ChatCompletionResponseChoiceLogprobs {
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
