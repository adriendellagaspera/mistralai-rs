pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowBulkError {
    /// Error message describing why the operation failed
    #[serde(default)]
    pub message: String,
    /// The workflow, if found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<Workflow>,
    /// The requested workflow ID
    #[serde(default)]
    pub workflow_id: String,
}

impl WorkflowBulkError {
    pub fn builder() -> WorkflowBulkErrorBuilder {
        <WorkflowBulkErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowBulkErrorBuilder {
    message: Option<String>,
    workflow: Option<Workflow>,
    workflow_id: Option<String>,
}

impl WorkflowBulkErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn workflow(mut self, value: Workflow) -> Self {
        self.workflow = Some(value);
        self
    }

    pub fn workflow_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowBulkError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](WorkflowBulkErrorBuilder::message)
    /// - [`workflow_id`](WorkflowBulkErrorBuilder::workflow_id)
    pub fn build(self) -> Result<WorkflowBulkError, BuildError> {
        Ok(WorkflowBulkError {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            workflow: self.workflow,
            workflow_id: self
                .workflow_id
                .ok_or_else(|| BuildError::missing_field("workflow_id"))?,
        })
    }
}
