pub use crate::prelude::*;

/// Query parameters for get_deployment
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetDeploymentQueryRequest {
    /// Scope serving status to this workflow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
}

impl GetDeploymentQueryRequest {
    pub fn builder() -> GetDeploymentQueryRequestBuilder {
        <GetDeploymentQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDeploymentQueryRequestBuilder {
    workflow_name: Option<String>,
}

impl GetDeploymentQueryRequestBuilder {
    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetDeploymentQueryRequest`].
    pub fn build(self) -> Result<GetDeploymentQueryRequest, BuildError> {
        Ok(GetDeploymentQueryRequest {
            workflow_name: self.workflow_name,
        })
    }
}
