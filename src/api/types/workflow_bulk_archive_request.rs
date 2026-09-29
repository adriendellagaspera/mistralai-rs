pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowBulkArchiveRequest {
    /// List of workflow IDs to archive
    #[serde(default)]
    pub workflow_ids: Vec<String>,
}

impl WorkflowBulkArchiveRequest {
    pub fn builder() -> WorkflowBulkArchiveRequestBuilder {
        <WorkflowBulkArchiveRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowBulkArchiveRequestBuilder {
    workflow_ids: Option<Vec<String>>,
}

impl WorkflowBulkArchiveRequestBuilder {
    pub fn workflow_ids(mut self, value: Vec<String>) -> Self {
        self.workflow_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowBulkArchiveRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workflow_ids`](WorkflowBulkArchiveRequestBuilder::workflow_ids)
    pub fn build(self) -> Result<WorkflowBulkArchiveRequest, BuildError> {
        Ok(WorkflowBulkArchiveRequest {
            workflow_ids: self
                .workflow_ids
                .ok_or_else(|| BuildError::missing_field("workflow_ids"))?,
        })
    }
}
