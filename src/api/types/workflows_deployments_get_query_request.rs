pub use crate::prelude::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkflowsDeploymentsGetQueryRequest {
    /// Scope serving status to this workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
}

impl WorkflowsDeploymentsGetQueryRequest {
    pub fn builder() -> WorkflowsDeploymentsGetQueryRequestBuilder {
        <WorkflowsDeploymentsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkflowsDeploymentsGetQueryRequestBuilder {
    workflow_name: Option<String>,
}

impl WorkflowsDeploymentsGetQueryRequestBuilder {
    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkflowsDeploymentsGetQueryRequest`].
    pub fn build(self) -> Result<WorkflowsDeploymentsGetQueryRequest, BuildError> {
        Ok(WorkflowsDeploymentsGetQueryRequest {
            workflow_name: self.workflow_name,
        })
    }
}
