pub use crate::prelude::*;

/// Attributes for workflow execution continued-as-new events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowExecutionContinuedAsNewAttributesResponse {
    /// The input arguments passed to the new workflow execution.
    pub input: JsonPayloadResponse,
    /// The run ID of the new workflow execution that continues this workflow.
    #[serde(default)]
    pub new_execution_run_id: String,
    /// Unique identifier for the task within the workflow execution.
    #[serde(default)]
    pub task_id: String,
    /// The registered name of the continued workflow.
    #[serde(default)]
    pub workflow_name: String,
}

impl WorkflowExecutionContinuedAsNewAttributesResponse {
    pub fn builder() -> WorkflowExecutionContinuedAsNewAttributesResponseBuilder {
        <WorkflowExecutionContinuedAsNewAttributesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowExecutionContinuedAsNewAttributesResponseBuilder {
    input: Option<JsonPayloadResponse>,
    new_execution_run_id: Option<String>,
    task_id: Option<String>,
    workflow_name: Option<String>,
}

impl WorkflowExecutionContinuedAsNewAttributesResponseBuilder {
    pub fn input(mut self, value: JsonPayloadResponse) -> Self {
        self.input = Some(value);
        self
    }

    pub fn new_execution_run_id(mut self, value: impl Into<String>) -> Self {
        self.new_execution_run_id = Some(value.into());
        self
    }

    pub fn task_id(mut self, value: impl Into<String>) -> Self {
        self.task_id = Some(value.into());
        self
    }

    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowExecutionContinuedAsNewAttributesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](WorkflowExecutionContinuedAsNewAttributesResponseBuilder::input)
    /// - [`new_execution_run_id`](WorkflowExecutionContinuedAsNewAttributesResponseBuilder::new_execution_run_id)
    /// - [`task_id`](WorkflowExecutionContinuedAsNewAttributesResponseBuilder::task_id)
    /// - [`workflow_name`](WorkflowExecutionContinuedAsNewAttributesResponseBuilder::workflow_name)
    pub fn build(self) -> Result<WorkflowExecutionContinuedAsNewAttributesResponse, BuildError> {
        Ok(WorkflowExecutionContinuedAsNewAttributesResponse {
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
            new_execution_run_id: self
                .new_execution_run_id
                .ok_or_else(|| BuildError::missing_field("new_execution_run_id"))?,
            task_id: self
                .task_id
                .ok_or_else(|| BuildError::missing_field("task_id"))?,
            workflow_name: self
                .workflow_name
                .ok_or_else(|| BuildError::missing_field("workflow_name"))?,
        })
    }
}
