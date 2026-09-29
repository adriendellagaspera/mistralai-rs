pub use crate::prelude::*;

/// Deprecated: use WorkflowRunListResponse instead. Will be removed in the next major version.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowExecutionListResponse {
    /// A list of workflow executions
    #[serde(default)]
    pub executions: Vec<WorkflowExecutionWithoutResultResponse>,
    /// Token to use for fetching the next page of results. Null if this is the last page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl WorkflowExecutionListResponse {
    pub fn builder() -> WorkflowExecutionListResponseBuilder {
        <WorkflowExecutionListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowExecutionListResponseBuilder {
    executions: Option<Vec<WorkflowExecutionWithoutResultResponse>>,
    next_page_token: Option<String>,
}

impl WorkflowExecutionListResponseBuilder {
    pub fn executions(mut self, value: Vec<WorkflowExecutionWithoutResultResponse>) -> Self {
        self.executions = Some(value);
        self
    }

    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowExecutionListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`executions`](WorkflowExecutionListResponseBuilder::executions)
    pub fn build(self) -> Result<WorkflowExecutionListResponse, BuildError> {
        Ok(WorkflowExecutionListResponse {
            executions: self
                .executions
                .ok_or_else(|| BuildError::missing_field("executions"))?,
            next_page_token: self.next_page_token,
        })
    }
}
