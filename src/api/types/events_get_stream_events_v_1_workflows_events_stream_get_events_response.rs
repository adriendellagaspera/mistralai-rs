pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetStreamEventsV1WorkflowsEventsStreamGetEventsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetStreamEventsV1WorkflowsEventsStreamGetEventsResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<i64>,
}

impl GetStreamEventsV1WorkflowsEventsStreamGetEventsResponse {
    pub fn builder() -> GetStreamEventsV1WorkflowsEventsStreamGetEventsResponseBuilder {
        <GetStreamEventsV1WorkflowsEventsStreamGetEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetStreamEventsV1WorkflowsEventsStreamGetEventsResponseBuilder {
    data: Option<GetStreamEventsV1WorkflowsEventsStreamGetEventsResponseData>,
    event: Option<String>,
    id: Option<String>,
    retry: Option<i64>,
}

impl GetStreamEventsV1WorkflowsEventsStreamGetEventsResponseBuilder {
    pub fn data(
        mut self,
        value: GetStreamEventsV1WorkflowsEventsStreamGetEventsResponseData,
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

    /// Consumes the builder and constructs a [`GetStreamEventsV1WorkflowsEventsStreamGetEventsResponse`].
    pub fn build(
        self,
    ) -> Result<GetStreamEventsV1WorkflowsEventsStreamGetEventsResponse, BuildError> {
        Ok(GetStreamEventsV1WorkflowsEventsStreamGetEventsResponse {
            data: self.data,
            event: self.event,
            id: self.id,
            retry: self.retry,
        })
    }
}
