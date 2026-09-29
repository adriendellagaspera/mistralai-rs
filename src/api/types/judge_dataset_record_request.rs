pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JudgeDatasetRecordRequest {
    pub judge_definition: CreateJudgeRequest,
}

impl JudgeDatasetRecordRequest {
    pub fn builder() -> JudgeDatasetRecordRequestBuilder {
        <JudgeDatasetRecordRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JudgeDatasetRecordRequestBuilder {
    judge_definition: Option<CreateJudgeRequest>,
}

impl JudgeDatasetRecordRequestBuilder {
    pub fn judge_definition(mut self, value: CreateJudgeRequest) -> Self {
        self.judge_definition = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JudgeDatasetRecordRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`judge_definition`](JudgeDatasetRecordRequestBuilder::judge_definition)
    pub fn build(self) -> Result<JudgeDatasetRecordRequest, BuildError> {
        Ok(JudgeDatasetRecordRequest {
            judge_definition: self
                .judge_definition
                .ok_or_else(|| BuildError::missing_field("judge_definition"))?,
        })
    }
}
