pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioVoicesListQueryRequest {
    /// Maximum number of voices to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Offset for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Filter the voices between customs and presets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ListVoicesRequestType>,
}

impl AudioVoicesListQueryRequest {
    pub fn builder() -> AudioVoicesListQueryRequestBuilder {
        <AudioVoicesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioVoicesListQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    r#type: Option<ListVoicesRequestType>,
}

impl AudioVoicesListQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn r#type(mut self, value: ListVoicesRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioVoicesListQueryRequest`].
    pub fn build(self) -> Result<AudioVoicesListQueryRequest, BuildError> {
        Ok(AudioVoicesListQueryRequest {
            limit: self.limit,
            offset: self.offset,
            r#type: self.r#type,
        })
    }
}
