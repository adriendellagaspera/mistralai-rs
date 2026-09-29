pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDeploymentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<WorkflowsWorkerSpecUpdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<DeploymentResourceConfigUpdate>,
}

impl UpdateDeploymentRequest {
    pub fn builder() -> UpdateDeploymentRequestBuilder {
        <UpdateDeploymentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDeploymentRequestBuilder {
    spec: Option<WorkflowsWorkerSpecUpdate>,
    resources: Option<DeploymentResourceConfigUpdate>,
}

impl UpdateDeploymentRequestBuilder {
    pub fn spec(mut self, value: WorkflowsWorkerSpecUpdate) -> Self {
        self.spec = Some(value);
        self
    }

    pub fn resources(mut self, value: DeploymentResourceConfigUpdate) -> Self {
        self.resources = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateDeploymentRequest`].
    pub fn build(self) -> Result<UpdateDeploymentRequest, BuildError> {
        Ok(UpdateDeploymentRequest {
            spec: self.spec,
            resources: self.resources,
        })
    }
}
