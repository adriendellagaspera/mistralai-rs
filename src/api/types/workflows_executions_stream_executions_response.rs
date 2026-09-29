pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StreamExecutionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<StreamExecutionsResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<i64>,
}

impl StreamExecutionsResponse {
    pub fn builder() -> StreamExecutionsResponseBuilder {
        <StreamExecutionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamExecutionsResponseBuilder {
    event: Option<String>,
    data: Option<StreamExecutionsResponseData>,
    id: Option<String>,
    retry: Option<i64>,
}

impl StreamExecutionsResponseBuilder {
    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());
        self
    }

    pub fn data(mut self, value: StreamExecutionsResponseData) -> Self {
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

    /// Consumes the builder and constructs a [`StreamExecutionsResponse`].
    pub fn build(self) -> Result<StreamExecutionsResponse, BuildError> {
        Ok(StreamExecutionsResponse {
            event: self.event,
            data: self.data,
            id: self.id,
            retry: self.retry,
        })
    }
}
