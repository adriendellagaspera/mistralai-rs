pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkflowArchiveResponse {
    /// The workflow spec
    pub workflow: Workflow,
}

impl WorkflowArchiveResponse {
    pub fn builder() -> WorkflowArchiveResponseBuilder {
        <WorkflowArchiveResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowArchiveResponseBuilder {
    workflow: Option<Workflow>,
}

impl WorkflowArchiveResponseBuilder {
    pub fn workflow(mut self, value: Workflow) -> Self {
        self.workflow = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowArchiveResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workflow`](WorkflowArchiveResponseBuilder::workflow)
    pub fn build(self) -> Result<WorkflowArchiveResponse, BuildError> {
        Ok(WorkflowArchiveResponse {
            workflow: self
                .workflow
                .ok_or_else(|| BuildError::missing_field("workflow"))?,
        })
    }
}
