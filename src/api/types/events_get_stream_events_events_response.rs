pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetStreamEventsEventsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<GetStreamEventsEventsResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<i64>,
}

impl GetStreamEventsEventsResponse {
    pub fn builder() -> GetStreamEventsEventsResponseBuilder {
        <GetStreamEventsEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetStreamEventsEventsResponseBuilder {
    data: Option<GetStreamEventsEventsResponseData>,
    event: Option<String>,
    id: Option<String>,
    retry: Option<i64>,
}

impl GetStreamEventsEventsResponseBuilder {
    pub fn data(mut self, value: GetStreamEventsEventsResponseData) -> Self {
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

    /// Consumes the builder and constructs a [`GetStreamEventsEventsResponse`].
    pub fn build(self) -> Result<GetStreamEventsEventsResponse, BuildError> {
        Ok(GetStreamEventsEventsResponse {
            data: self.data,
            event: self.event,
            id: self.id,
            retry: self.retry,
        })
    }
}
