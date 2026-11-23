pub use crate::prelude::*;

/// Query parameters for list_audio_agent_versions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAudioAgentVersionsQueryRequest {
    /// Maximum number of versions to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Cursor returned as next_page_token by the previous page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ListAudioAgentVersionsQueryRequest {
    pub fn builder() -> ListAudioAgentVersionsQueryRequestBuilder {
        <ListAudioAgentVersionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAudioAgentVersionsQueryRequestBuilder {
    page_size: Option<i64>,
    page_token: Option<String>,
}

impl ListAudioAgentVersionsQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page_token(mut self, value: impl Into<String>) -> Self {
        self.page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListAudioAgentVersionsQueryRequest`].
    pub fn build(self) -> Result<ListAudioAgentVersionsQueryRequest, BuildError> {
        Ok(ListAudioAgentVersionsQueryRequest {
            page_size: self.page_size,
            page_token: self.page_token,
        })
    }
}
