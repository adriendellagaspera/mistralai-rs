pub use crate::prelude::*;

/// Query parameters for list_versions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioAgentsListVersionsQueryRequest {
    /// Maximum number of versions to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Cursor returned as next_page_token by the previous page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl AudioAgentsListVersionsQueryRequest {
    pub fn builder() -> AudioAgentsListVersionsQueryRequestBuilder {
        <AudioAgentsListVersionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioAgentsListVersionsQueryRequestBuilder {
    page_size: Option<i64>,
    page_token: Option<String>,
}

impl AudioAgentsListVersionsQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page_token(mut self, value: impl Into<String>) -> Self {
        self.page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioAgentsListVersionsQueryRequest`].
    pub fn build(self) -> Result<AudioAgentsListVersionsQueryRequest, BuildError> {
        Ok(AudioAgentsListVersionsQueryRequest {
            page_size: self.page_size,
            page_token: self.page_token,
        })
    }
}
