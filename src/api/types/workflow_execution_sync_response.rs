pub use crate::prelude::*;

/// Response model for synchronous workflow execution
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkflowExecutionSyncResponse {
    /// ID of the workflow execution
    #[serde(default)]
    pub execution_id: String,
    pub result: serde_json::Value,
    /// Name of the workflow that was executed
    #[serde(default)]
    pub workflow_name: String,
}

impl WorkflowExecutionSyncResponse {
    pub fn builder() -> WorkflowExecutionSyncResponseBuilder {
        <WorkflowExecutionSyncResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowExecutionSyncResponseBuilder {
    execution_id: Option<String>,
    result: Option<serde_json::Value>,
    workflow_name: Option<String>,
}

impl WorkflowExecutionSyncResponseBuilder {
    pub fn execution_id(mut self, value: impl Into<String>) -> Self {
        self.execution_id = Some(value.into());
        self
    }

    pub fn result(mut self, value: serde_json::Value) -> Self {
        self.result = Some(value);
        self
    }

    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowExecutionSyncResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`execution_id`](WorkflowExecutionSyncResponseBuilder::execution_id)
    /// - [`result`](WorkflowExecutionSyncResponseBuilder::result)
    /// - [`workflow_name`](WorkflowExecutionSyncResponseBuilder::workflow_name)
    pub fn build(self) -> Result<WorkflowExecutionSyncResponse, BuildError> {
        Ok(WorkflowExecutionSyncResponse {
            execution_id: self
                .execution_id
                .ok_or_else(|| BuildError::missing_field("execution_id"))?,
            result: self
                .result
                .ok_or_else(|| BuildError::missing_field("result"))?,
            workflow_name: self
                .workflow_name
                .ok_or_else(|| BuildError::missing_field("workflow_name"))?,
        })
    }
}
