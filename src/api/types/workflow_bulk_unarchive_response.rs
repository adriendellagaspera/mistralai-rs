pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowBulkUnarchiveResponse {
    /// Workflows that were successfully unarchived or were already unarchived
    #[serde(default)]
    pub unarchived: Vec<Workflow>,
    /// Workflows that could not be unarchived and the corresponding error messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errored: Option<Vec<WorkflowBulkError>>,
}

impl WorkflowBulkUnarchiveResponse {
    pub fn builder() -> WorkflowBulkUnarchiveResponseBuilder {
        <WorkflowBulkUnarchiveResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowBulkUnarchiveResponseBuilder {
    unarchived: Option<Vec<Workflow>>,
    errored: Option<Vec<WorkflowBulkError>>,
}

impl WorkflowBulkUnarchiveResponseBuilder {
    pub fn unarchived(mut self, value: Vec<Workflow>) -> Self {
        self.unarchived = Some(value);
        self
    }

    pub fn errored(mut self, value: Vec<WorkflowBulkError>) -> Self {
        self.errored = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowBulkUnarchiveResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`unarchived`](WorkflowBulkUnarchiveResponseBuilder::unarchived)
    pub fn build(self) -> Result<WorkflowBulkUnarchiveResponse, BuildError> {
        Ok(WorkflowBulkUnarchiveResponse {
            unarchived: self
                .unarchived
                .ok_or_else(|| BuildError::missing_field("unarchived"))?,
            errored: self.errored,
        })
    }
}
