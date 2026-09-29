pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowBulkError {
    /// The requested workflow ID
    #[serde(default)]
    pub workflow_id: String,
    /// The workflow, if found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<Workflow>,
    /// Error message describing why the operation failed
    #[serde(default)]
    pub message: String,
}

impl WorkflowBulkError {
    pub fn builder() -> WorkflowBulkErrorBuilder {
        <WorkflowBulkErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowBulkErrorBuilder {
    workflow_id: Option<String>,
    workflow: Option<Workflow>,
    message: Option<String>,
}

impl WorkflowBulkErrorBuilder {
    pub fn workflow_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_id = Some(value.into());
        self
    }

    pub fn workflow(mut self, value: Workflow) -> Self {
        self.workflow = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowBulkError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workflow_id`](WorkflowBulkErrorBuilder::workflow_id)
    /// - [`message`](WorkflowBulkErrorBuilder::message)
    pub fn build(self) -> Result<WorkflowBulkError, BuildError> {
        Ok(WorkflowBulkError {
            workflow_id: self
                .workflow_id
                .ok_or_else(|| BuildError::missing_field("workflow_id"))?,
            workflow: self.workflow,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
