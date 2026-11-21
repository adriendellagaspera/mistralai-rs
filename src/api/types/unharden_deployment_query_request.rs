pub use crate::prelude::*;

/// Query parameters for unharden_deployment
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnhardenDeploymentQueryRequest {
    /// Workspace ID to scope the request to. Defaults to the caller's context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}

impl UnhardenDeploymentQueryRequest {
    pub fn builder() -> UnhardenDeploymentQueryRequestBuilder {
        <UnhardenDeploymentQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnhardenDeploymentQueryRequestBuilder {
    workspace_id: Option<String>,
}

impl UnhardenDeploymentQueryRequestBuilder {
    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnhardenDeploymentQueryRequest`].
    pub fn build(self) -> Result<UnhardenDeploymentQueryRequest, BuildError> {
        Ok(UnhardenDeploymentQueryRequest {
            workspace_id: self.workspace_id,
        })
    }
}
