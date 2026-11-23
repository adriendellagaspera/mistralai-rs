pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StreamLogsExecutionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<StreamLogsExecutionsResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<StreamLogsExecutionsResponseEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl StreamLogsExecutionsResponse {
    pub fn builder() -> StreamLogsExecutionsResponseBuilder {
        <StreamLogsExecutionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamLogsExecutionsResponseBuilder {
    data: Option<StreamLogsExecutionsResponseData>,
    event: Option<StreamLogsExecutionsResponseEvent>,
    id: Option<String>,
}

impl StreamLogsExecutionsResponseBuilder {
    pub fn data(mut self, value: StreamLogsExecutionsResponseData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn event(mut self, value: StreamLogsExecutionsResponseEvent) -> Self {
        self.event = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StreamLogsExecutionsResponse`].
    pub fn build(self) -> Result<StreamLogsExecutionsResponse, BuildError> {
        Ok(StreamLogsExecutionsResponse {
            data: self.data,
            event: self.event,
            id: self.id,
        })
    }
}
