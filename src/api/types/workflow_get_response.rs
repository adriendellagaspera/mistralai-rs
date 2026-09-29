pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkflowGetResponse {
    /// The workflow spec
    pub workflow: WorkflowWithWorkerStatus,
}

impl WorkflowGetResponse {
    pub fn builder() -> WorkflowGetResponseBuilder {
        <WorkflowGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowGetResponseBuilder {
    workflow: Option<WorkflowWithWorkerStatus>,
}

impl WorkflowGetResponseBuilder {
    pub fn workflow(mut self, value: WorkflowWithWorkerStatus) -> Self {
        self.workflow = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workflow`](WorkflowGetResponseBuilder::workflow)
    pub fn build(self) -> Result<WorkflowGetResponse, BuildError> {
        Ok(WorkflowGetResponse {
            workflow: self
                .workflow
                .ok_or_else(|| BuildError::missing_field("workflow"))?,
        })
    }
}
