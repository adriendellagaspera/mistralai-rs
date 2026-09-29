pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StreamLogsDeploymentsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<StreamLogsDeploymentsResponseEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<StreamLogsDeploymentsResponseData>,
}

impl StreamLogsDeploymentsResponse {
    pub fn builder() -> StreamLogsDeploymentsResponseBuilder {
        <StreamLogsDeploymentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamLogsDeploymentsResponseBuilder {
    event: Option<StreamLogsDeploymentsResponseEvent>,
    id: Option<String>,
    data: Option<StreamLogsDeploymentsResponseData>,
}

impl StreamLogsDeploymentsResponseBuilder {
    pub fn event(mut self, value: StreamLogsDeploymentsResponseEvent) -> Self {
        self.event = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn data(mut self, value: StreamLogsDeploymentsResponseData) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StreamLogsDeploymentsResponse`].
    pub fn build(self) -> Result<StreamLogsDeploymentsResponse, BuildError> {
        Ok(StreamLogsDeploymentsResponse {
            event: self.event,
            id: self.id,
            data: self.data,
        })
    }
}
