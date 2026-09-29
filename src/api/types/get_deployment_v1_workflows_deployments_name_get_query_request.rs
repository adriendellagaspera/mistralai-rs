pub use crate::prelude::*;

/// Query parameters for get_deployment_v1_workflows_deployments__name__get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDeploymentV1WorkflowsDeploymentsNameGetQueryRequest {
    /// Scope serving status to this workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
}

impl GetDeploymentV1WorkflowsDeploymentsNameGetQueryRequest {
    pub fn builder() -> GetDeploymentV1WorkflowsDeploymentsNameGetQueryRequestBuilder {
        <GetDeploymentV1WorkflowsDeploymentsNameGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDeploymentV1WorkflowsDeploymentsNameGetQueryRequestBuilder {
    workflow_name: Option<String>,
}

impl GetDeploymentV1WorkflowsDeploymentsNameGetQueryRequestBuilder {
    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetDeploymentV1WorkflowsDeploymentsNameGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<GetDeploymentV1WorkflowsDeploymentsNameGetQueryRequest, BuildError> {
        Ok(GetDeploymentV1WorkflowsDeploymentsNameGetQueryRequest {
            workflow_name: self.workflow_name,
        })
    }
}
