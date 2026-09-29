pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeploymentListResponse {
    /// List of deployments
    #[serde(default)]
    pub deployments: Vec<DeploymentResponse>,
    /// Cursor for the next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Workspace ID the results are scoped to
    #[serde(default)]
    pub workspace_id: String,
}

impl DeploymentListResponse {
    pub fn builder() -> DeploymentListResponseBuilder {
        <DeploymentListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentListResponseBuilder {
    deployments: Option<Vec<DeploymentResponse>>,
    next_cursor: Option<String>,
    workspace_id: Option<String>,
}

impl DeploymentListResponseBuilder {
    pub fn deployments(mut self, value: Vec<DeploymentResponse>) -> Self {
        self.deployments = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeploymentListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deployments`](DeploymentListResponseBuilder::deployments)
    /// - [`workspace_id`](DeploymentListResponseBuilder::workspace_id)
    pub fn build(self) -> Result<DeploymentListResponse, BuildError> {
        Ok(DeploymentListResponse {
            deployments: self
                .deployments
                .ok_or_else(|| BuildError::missing_field("deployments"))?,
            next_cursor: self.next_cursor,
            workspace_id: self
                .workspace_id
                .ok_or_else(|| BuildError::missing_field("workspace_id"))?,
        })
    }
}
