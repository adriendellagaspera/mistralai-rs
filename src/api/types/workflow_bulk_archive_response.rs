pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowBulkArchiveResponse {
    /// Workflows that were successfully archived or were already archived
    #[serde(default)]
    pub archived: Vec<Workflow>,
    /// Workflows that could not be archived and the corresponding error messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errored: Option<Vec<WorkflowBulkError>>,
}

impl WorkflowBulkArchiveResponse {
    pub fn builder() -> WorkflowBulkArchiveResponseBuilder {
        <WorkflowBulkArchiveResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowBulkArchiveResponseBuilder {
    archived: Option<Vec<Workflow>>,
    errored: Option<Vec<WorkflowBulkError>>,
}

impl WorkflowBulkArchiveResponseBuilder {
    pub fn archived(mut self, value: Vec<Workflow>) -> Self {
        self.archived = Some(value);
        self
    }

    pub fn errored(mut self, value: Vec<WorkflowBulkError>) -> Self {
        self.errored = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowBulkArchiveResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`archived`](WorkflowBulkArchiveResponseBuilder::archived)
    pub fn build(self) -> Result<WorkflowBulkArchiveResponse, BuildError> {
        Ok(WorkflowBulkArchiveResponse {
            archived: self
                .archived
                .ok_or_else(|| BuildError::missing_field("archived"))?,
            errored: self.errored,
        })
    }
}
