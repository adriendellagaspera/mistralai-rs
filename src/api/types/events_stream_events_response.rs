pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StreamEventsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<StreamEventsResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<i64>,
}

impl StreamEventsResponse {
    pub fn builder() -> StreamEventsResponseBuilder {
        <StreamEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamEventsResponseBuilder {
    event: Option<String>,
    data: Option<StreamEventsResponseData>,
    id: Option<String>,
    retry: Option<i64>,
}

impl StreamEventsResponseBuilder {
    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());
        self
    }

    pub fn data(mut self, value: StreamEventsResponseData) -> Self {
        self.data = Some(value);
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

    /// Consumes the builder and constructs a [`StreamEventsResponse`].
    pub fn build(self) -> Result<StreamEventsResponse, BuildError> {
        Ok(StreamEventsResponse {
            event: self.event,
            data: self.data,
            id: self.id,
            retry: self.retry,
        })
    }
}
