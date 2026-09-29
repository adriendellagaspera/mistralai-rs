pub use crate::prelude::*;

/// Attributes for workflow execution started events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowExecutionStartedAttributesResponse {
    /// Unique identifier for the task within the workflow execution.
    #[serde(default)]
    pub task_id: String,
    /// The registered name of the workflow being executed.
    #[serde(default)]
    pub workflow_name: String,
    /// The user-friendly display name of the workflow, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The input arguments passed to the workflow.
    pub input: JsonPayloadResponse,
    /// Workflow retry attempt number. 1 on first run and CAN; >1 on workflow-level retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt: Option<i64>,
}

impl WorkflowExecutionStartedAttributesResponse {
    pub fn builder() -> WorkflowExecutionStartedAttributesResponseBuilder {
        <WorkflowExecutionStartedAttributesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowExecutionStartedAttributesResponseBuilder {
    task_id: Option<String>,
    workflow_name: Option<String>,
    display_name: Option<String>,
    input: Option<JsonPayloadResponse>,
    attempt: Option<i64>,
}

impl WorkflowExecutionStartedAttributesResponseBuilder {
    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn input(mut self, value: JsonPayloadResponse) -> Self {
        self.input = Some(value);
        self
    }

    pub fn attempt(mut self, value: i64) -> Self {
        self.attempt = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowExecutionStartedAttributesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`task_id`](WorkflowExecutionStartedAttributesResponseBuilder::task_id)
    /// - [`workflow_name`](WorkflowExecutionStartedAttributesResponseBuilder::workflow_name)
    /// - [`input`](WorkflowExecutionStartedAttributesResponseBuilder::input)
    pub fn build(self) -> Result<WorkflowExecutionStartedAttributesResponse, BuildError> {
        Ok(WorkflowExecutionStartedAttributesResponse {
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
            workflow_name: self
                .workflow_name
                .ok_or_else(|| BuildError::missing_field("workflow_name"))?,
            display_name: self.display_name,
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
            attempt: self.attempt,
        })
    }
}
