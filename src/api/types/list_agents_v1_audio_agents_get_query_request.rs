pub use crate::prelude::*;

/// Query parameters for list_agents_v1_audio_agents_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAgentsV1AudioAgentsGetQueryRequest {
    /// Maximum number of agents to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Cursor returned as next_page_token by the previous page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// Substring match on the agent name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

impl ListAgentsV1AudioAgentsGetQueryRequest {
    pub fn builder() -> ListAgentsV1AudioAgentsGetQueryRequestBuilder {
        <ListAgentsV1AudioAgentsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAgentsV1AudioAgentsGetQueryRequestBuilder {
    page_size: Option<i64>,
    page_token: Option<String>,
    search: Option<String>,
}

impl ListAgentsV1AudioAgentsGetQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page_token(mut self, value: impl Into<String>) -> Self {
        self.page_token = Some(value.into());
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListAgentsV1AudioAgentsGetQueryRequest`].
    pub fn build(self) -> Result<ListAgentsV1AudioAgentsGetQueryRequest, BuildError> {
        Ok(ListAgentsV1AudioAgentsGetQueryRequest {
            page_size: self.page_size,
            page_token: self.page_token,
            search: self.search,
        })
    }
}
