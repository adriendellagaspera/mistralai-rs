pub use crate::prelude::*;

/// Query parameters for list_agent_sessions_v1_audio_agents__agent_id__sessions_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAgentSessionsV1AudioAgentsAgentIdSessionsGetQueryRequest {
    /// Maximum number of sessions to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Cursor returned as next_page_token by the previous page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ListAgentSessionsV1AudioAgentsAgentIdSessionsGetQueryRequest {
    pub fn builder() -> ListAgentSessionsV1AudioAgentsAgentIdSessionsGetQueryRequestBuilder {
        <ListAgentSessionsV1AudioAgentsAgentIdSessionsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAgentSessionsV1AudioAgentsAgentIdSessionsGetQueryRequestBuilder {
    page_size: Option<i64>,
    page_token: Option<String>,
}

impl ListAgentSessionsV1AudioAgentsAgentIdSessionsGetQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page_token(mut self, value: impl Into<String>) -> Self {
        self.page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListAgentSessionsV1AudioAgentsAgentIdSessionsGetQueryRequest`].
    pub fn build(
        self,
    ) -> Result<ListAgentSessionsV1AudioAgentsAgentIdSessionsGetQueryRequest, BuildError> {
        Ok(
            ListAgentSessionsV1AudioAgentsAgentIdSessionsGetQueryRequest {
                page_size: self.page_size,
                page_token: self.page_token,
            },
        )
    }
}
