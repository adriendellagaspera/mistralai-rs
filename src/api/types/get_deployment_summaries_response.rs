pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetDeploymentSummariesResponse {
    #[serde(default)]
    pub deployments: Vec<GetDeploymentSummariesResponseDeployment>,
}

impl GetDeploymentSummariesResponse {
    pub fn builder() -> GetDeploymentSummariesResponseBuilder {
        <GetDeploymentSummariesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetDeploymentSummariesResponseBuilder {
    deployments: Option<Vec<GetDeploymentSummariesResponseDeployment>>,
}

impl GetDeploymentSummariesResponseBuilder {
    pub fn deployments(mut self, value: Vec<GetDeploymentSummariesResponseDeployment>) -> Self {
        self.deployments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetDeploymentSummariesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deployments`](GetDeploymentSummariesResponseBuilder::deployments)
    pub fn build(self) -> Result<GetDeploymentSummariesResponse, BuildError> {
        Ok(GetDeploymentSummariesResponse {
            deployments: self
                .deployments
                .ok_or_else(|| BuildError::missing_field("deployments"))?,
        })
    }
}
