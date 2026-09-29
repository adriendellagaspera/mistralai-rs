pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StreamV1WorkflowsExecutionsExecutionIdStreamGetExecutionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<StreamV1WorkflowsExecutionsExecutionIdStreamGetExecutionsResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<i64>,
}

impl StreamV1WorkflowsExecutionsExecutionIdStreamGetExecutionsResponse {
    pub fn builder() -> StreamV1WorkflowsExecutionsExecutionIdStreamGetExecutionsResponseBuilder {
        <StreamV1WorkflowsExecutionsExecutionIdStreamGetExecutionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamV1WorkflowsExecutionsExecutionIdStreamGetExecutionsResponseBuilder {
    data: Option<StreamV1WorkflowsExecutionsExecutionIdStreamGetExecutionsResponseData>,
    event: Option<String>,
    id: Option<String>,
    retry: Option<i64>,
}

impl StreamV1WorkflowsExecutionsExecutionIdStreamGetExecutionsResponseBuilder {
    pub fn data(
        mut self,
        value: StreamV1WorkflowsExecutionsExecutionIdStreamGetExecutionsResponseData,
    ) -> Self {
        self.data = Some(value);
        self
    }

    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn retry(mut self, value: i64) -> Self {
        self.retry = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StreamV1WorkflowsExecutionsExecutionIdStreamGetExecutionsResponse`].
    pub fn build(
        self,
    ) -> Result<StreamV1WorkflowsExecutionsExecutionIdStreamGetExecutionsResponse, BuildError> {
        Ok(
            StreamV1WorkflowsExecutionsExecutionIdStreamGetExecutionsResponse {
                data: self.data,
                event: self.event,
                id: self.id,
                retry: self.retry,
            },
        )
    }
}
