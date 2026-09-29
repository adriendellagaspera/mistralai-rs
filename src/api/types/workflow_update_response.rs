pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkflowUpdateResponse {
    /// Updated workflow
    pub workflow: Workflow,
}

impl WorkflowUpdateResponse {
    pub fn builder() -> WorkflowUpdateResponseBuilder {
        <WorkflowUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowUpdateResponseBuilder {
    workflow: Option<Workflow>,
}

impl WorkflowUpdateResponseBuilder {
    pub fn workflow(mut self, value: Workflow) -> Self {
        self.workflow = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workflow`](WorkflowUpdateResponseBuilder::workflow)
    pub fn build(self) -> Result<WorkflowUpdateResponse, BuildError> {
        Ok(WorkflowUpdateResponse {
            workflow: self
                .workflow
                .ok_or_else(|| BuildError::missing_field("workflow"))?,
        })
    }
}
