pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowBulkUnarchiveRequest {
    /// List of workflow IDs to unarchive
    #[serde(default)]
    pub workflow_ids: Vec<String>,
}

impl WorkflowBulkUnarchiveRequest {
    pub fn builder() -> WorkflowBulkUnarchiveRequestBuilder {
        <WorkflowBulkUnarchiveRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowBulkUnarchiveRequestBuilder {
    workflow_ids: Option<Vec<String>>,
}

impl WorkflowBulkUnarchiveRequestBuilder {
    pub fn workflow_ids(mut self, value: Vec<String>) -> Self {
        self.workflow_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowBulkUnarchiveRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workflow_ids`](WorkflowBulkUnarchiveRequestBuilder::workflow_ids)
    pub fn build(self) -> Result<WorkflowBulkUnarchiveRequest, BuildError> {
        Ok(WorkflowBulkUnarchiveRequest {
            workflow_ids: self
                .workflow_ids
                .ok_or_else(|| BuildError::missing_field("workflow_ids"))?,
        })
    }
}
