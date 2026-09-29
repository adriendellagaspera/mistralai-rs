pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeploymentWorkerListResponse {
    /// Workers registered for the deployment
    #[serde(default)]
    pub workers: Vec<DeploymentWorkerResponse>,
    /// Cursor for the next page of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl DeploymentWorkerListResponse {
    pub fn builder() -> DeploymentWorkerListResponseBuilder {
        <DeploymentWorkerListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentWorkerListResponseBuilder {
    workers: Option<Vec<DeploymentWorkerResponse>>,
    next_cursor: Option<String>,
}

impl DeploymentWorkerListResponseBuilder {
    pub fn workers(mut self, value: Vec<DeploymentWorkerResponse>) -> Self {
        self.workers = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeploymentWorkerListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workers`](DeploymentWorkerListResponseBuilder::workers)
    pub fn build(self) -> Result<DeploymentWorkerListResponse, BuildError> {
        Ok(DeploymentWorkerListResponse {
            workers: self
                .workers
                .ok_or_else(|| BuildError::missing_field("workers"))?,
            next_cursor: self.next_cursor,
        })
    }
}
