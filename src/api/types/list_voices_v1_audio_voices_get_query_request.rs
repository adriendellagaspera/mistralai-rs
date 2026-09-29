pub use crate::prelude::*;

/// Query parameters for list_voices_v1_audio_voices_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListVoicesV1AudioVoicesGetQueryRequest {
    /// Maximum number of voices to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Filter the voices between customs and presets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ListVoicesV1AudioVoicesGetVoicesRequestType>,
}

impl ListVoicesV1AudioVoicesGetQueryRequest {
    pub fn builder() -> ListVoicesV1AudioVoicesGetQueryRequestBuilder {
        <ListVoicesV1AudioVoicesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListVoicesV1AudioVoicesGetQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    r#type: Option<ListVoicesV1AudioVoicesGetVoicesRequestType>,
}

impl ListVoicesV1AudioVoicesGetQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn r#type(mut self, value: ListVoicesV1AudioVoicesGetVoicesRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListVoicesV1AudioVoicesGetQueryRequest`].
    pub fn build(self) -> Result<ListVoicesV1AudioVoicesGetQueryRequest, BuildError> {
        Ok(ListVoicesV1AudioVoicesGetQueryRequest {
            limit: self.limit,
            offset: self.offset,
            r#type: self.r#type,
        })
    }
}
