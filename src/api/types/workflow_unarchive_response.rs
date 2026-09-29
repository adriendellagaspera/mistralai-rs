pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkflowUnarchiveResponse {
    /// The workflow spec
    pub workflow: Workflow,
}

impl WorkflowUnarchiveResponse {
    pub fn builder() -> WorkflowUnarchiveResponseBuilder {
        <WorkflowUnarchiveResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowUnarchiveResponseBuilder {
    workflow: Option<Workflow>,
}

impl WorkflowUnarchiveResponseBuilder {
    pub fn workflow(mut self, value: Workflow) -> Self {
        self.workflow = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkflowUnarchiveResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workflow`](WorkflowUnarchiveResponseBuilder::workflow)
    pub fn build(self) -> Result<WorkflowUnarchiveResponse, BuildError> {
        Ok(WorkflowUnarchiveResponse {
            workflow: self
                .workflow
                .ok_or_else(|| BuildError::missing_field("workflow"))?,
        })
    }
}
