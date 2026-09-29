pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StreamWorkflowExecutionLogsExecutionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<StreamWorkflowExecutionLogsExecutionsResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<StreamWorkflowExecutionLogsExecutionsResponseEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl StreamWorkflowExecutionLogsExecutionsResponse {
    pub fn builder() -> StreamWorkflowExecutionLogsExecutionsResponseBuilder {
        <StreamWorkflowExecutionLogsExecutionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamWorkflowExecutionLogsExecutionsResponseBuilder {
    data: Option<StreamWorkflowExecutionLogsExecutionsResponseData>,
    event: Option<StreamWorkflowExecutionLogsExecutionsResponseEvent>,
    id: Option<String>,
}

impl StreamWorkflowExecutionLogsExecutionsResponseBuilder {
    pub fn data(mut self, value: StreamWorkflowExecutionLogsExecutionsResponseData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn event(mut self, value: StreamWorkflowExecutionLogsExecutionsResponseEvent) -> Self {
        self.event = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StreamWorkflowExecutionLogsExecutionsResponse`].
    pub fn build(self) -> Result<StreamWorkflowExecutionLogsExecutionsResponse, BuildError> {
        Ok(StreamWorkflowExecutionLogsExecutionsResponse {
            data: self.data,
            event: self.event,
            id: self.id,
        })
    }
}
