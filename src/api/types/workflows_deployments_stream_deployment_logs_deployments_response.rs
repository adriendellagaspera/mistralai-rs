pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StreamDeploymentLogsDeploymentsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<StreamDeploymentLogsDeploymentsResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<StreamDeploymentLogsDeploymentsResponseEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl StreamDeploymentLogsDeploymentsResponse {
    pub fn builder() -> StreamDeploymentLogsDeploymentsResponseBuilder {
        <StreamDeploymentLogsDeploymentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamDeploymentLogsDeploymentsResponseBuilder {
    data: Option<StreamDeploymentLogsDeploymentsResponseData>,
    event: Option<StreamDeploymentLogsDeploymentsResponseEvent>,
    id: Option<String>,
}

impl StreamDeploymentLogsDeploymentsResponseBuilder {
    pub fn data(mut self, value: StreamDeploymentLogsDeploymentsResponseData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn event(mut self, value: StreamDeploymentLogsDeploymentsResponseEvent) -> Self {
        self.event = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StreamDeploymentLogsDeploymentsResponse`].
    pub fn build(self) -> Result<StreamDeploymentLogsDeploymentsResponse, BuildError> {
        Ok(StreamDeploymentLogsDeploymentsResponse {
            data: self.data,
            event: self.event,
            id: self.id,
        })
    }
}
