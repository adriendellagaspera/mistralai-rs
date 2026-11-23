pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StreamLogsDeploymentsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<StreamLogsDeploymentsResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<StreamLogsDeploymentsResponseEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl StreamLogsDeploymentsResponse {
    pub fn builder() -> StreamLogsDeploymentsResponseBuilder {
        <StreamLogsDeploymentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamLogsDeploymentsResponseBuilder {
    data: Option<StreamLogsDeploymentsResponseData>,
    event: Option<StreamLogsDeploymentsResponseEvent>,
    id: Option<String>,
}

impl StreamLogsDeploymentsResponseBuilder {
    pub fn data(mut self, value: StreamLogsDeploymentsResponseData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn event(mut self, value: StreamLogsDeploymentsResponseEvent) -> Self {
        self.event = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StreamLogsDeploymentsResponse`].
    pub fn build(self) -> Result<StreamLogsDeploymentsResponse, BuildError> {
        Ok(StreamLogsDeploymentsResponse {
            data: self.data,
            event: self.event,
            id: self.id,
        })
    }
}
