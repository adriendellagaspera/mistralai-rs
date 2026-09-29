pub use crate::prelude::*;

/// Attributes for workflow execution completed events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowExecutionCompletedAttributesResponse {
    /// Workflow retry attempt number. 1 on first run and CAN; >1 on workflow-level retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i64>,
    /// The final result returned by the workflow.
    pub result: JsonPayloadResponse,
    /// Unique identifier for the task within the workflow execution.
    #[serde(default)]
    pub task_id: String,
}

impl WorkflowExecutionCompletedAttributesResponse {
    pub fn builder() -> WorkflowExecutionCompletedAttributesResponseBuilder {
        <WorkflowExecutionCompletedAttributesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowExecutionCompletedAttributesResponseBuilder {
    attempt: Option<i64>,
    result: Option<JsonPayloadResponse>,
    task_id: Option<String>,
}

impl WorkflowExecutionCompletedAttributesResponseBuilder {
    pub fn attempt(mut self, value: i64) -> Self {
        self.attempt = Some(value);
        self
    }

    pub fn result(mut self, value: JsonPayloadResponse) -> Self {
        self.result = Some(value);
        self
    }

    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowExecutionCompletedAttributesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`result`](WorkflowExecutionCompletedAttributesResponseBuilder::result)
    /// - [`task_id`](WorkflowExecutionCompletedAttributesResponseBuilder::task_id)
    pub fn build(self) -> Result<WorkflowExecutionCompletedAttributesResponse, BuildError> {
        Ok(WorkflowExecutionCompletedAttributesResponse {
            attempt: self.attempt,
            result: self
                .result
                .ok_or_else(|| BuildError::missing_field("result"))?,
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
        })
    }
}
