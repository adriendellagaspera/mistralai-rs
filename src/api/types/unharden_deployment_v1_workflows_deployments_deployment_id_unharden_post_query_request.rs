pub use crate::prelude::*;

/// Query parameters for unharden_deployment_v1_workflows_deployments__deployment_id__unharden_post
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnhardenDeploymentV1WorkflowsDeploymentsDeploymentIdUnhardenPostQueryRequest {
    /// Workspace ID to scope the request to. Defaults to the caller's context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl UnhardenDeploymentV1WorkflowsDeploymentsDeploymentIdUnhardenPostQueryRequest {
    pub fn builder(
    ) -> UnhardenDeploymentV1WorkflowsDeploymentsDeploymentIdUnhardenPostQueryRequestBuilder {
        <UnhardenDeploymentV1WorkflowsDeploymentsDeploymentIdUnhardenPostQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnhardenDeploymentV1WorkflowsDeploymentsDeploymentIdUnhardenPostQueryRequestBuilder {
    workspace_id: Option<String>,
}

impl UnhardenDeploymentV1WorkflowsDeploymentsDeploymentIdUnhardenPostQueryRequestBuilder {
    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnhardenDeploymentV1WorkflowsDeploymentsDeploymentIdUnhardenPostQueryRequest`].
    pub fn build(
        self,
    ) -> Result<
        UnhardenDeploymentV1WorkflowsDeploymentsDeploymentIdUnhardenPostQueryRequest,
        BuildError,
    > {
        Ok(
            UnhardenDeploymentV1WorkflowsDeploymentsDeploymentIdUnhardenPostQueryRequest {
                workspace_id: self.workspace_id,
            },
        )
    }
}
