pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JudgeChatCompletionEventRequest {
    pub judge_definition: CreateJudgeRequest,
}

impl JudgeChatCompletionEventRequest {
    pub fn builder() -> JudgeChatCompletionEventRequestBuilder {
        <JudgeChatCompletionEventRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JudgeChatCompletionEventRequestBuilder {
    judge_definition: Option<CreateJudgeRequest>,
}

impl JudgeChatCompletionEventRequestBuilder {
    pub fn judge_definition(mut self, value: CreateJudgeRequest) -> Self {
        self.judge_definition = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JudgeChatCompletionEventRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`judge_definition`](JudgeChatCompletionEventRequestBuilder::judge_definition)
    pub fn build(self) -> Result<JudgeChatCompletionEventRequest, BuildError> {
        Ok(JudgeChatCompletionEventRequest {
            judge_definition: self
                .judge_definition
                .ok_or_else(|| BuildError::missing_field("judge_definition"))?,
        })
    }
}
