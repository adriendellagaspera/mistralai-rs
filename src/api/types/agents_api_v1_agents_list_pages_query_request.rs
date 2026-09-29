pub use crate::prelude::*;

/// Query parameters for agents_api_v1_agents_list_pages
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsApiV1AgentsListPagesQueryRequest {
    /// Number of agents per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_chat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<RequestSource>>,
    /// Filter by agent name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Search agents by name or ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// Opaque cursor from a previous response's next_page_token. When set, results page forward from the cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl AgentsApiV1AgentsListPagesQueryRequest {
    pub fn builder() -> AgentsApiV1AgentsListPagesQueryRequestBuilder {
        <AgentsApiV1AgentsListPagesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsApiV1AgentsListPagesQueryRequestBuilder {
    page_size: Option<i64>,
    deployment_chat: Option<bool>,
    sources: Option<Vec<RequestSource>>,
    name: Option<String>,
    search: Option<String>,
    id: Option<String>,
    metadata: Option<String>,
    page_token: Option<String>,
}

impl AgentsApiV1AgentsListPagesQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn deployment_chat(mut self, value: bool) -> Self {
        self.deployment_chat = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<RequestSource>) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: impl Into<String>) -> Self {
        self.metadata = Some(value.into());
        self
    }

    pub fn page_token(mut self, value: impl Into<String>) -> Self {
        self.page_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsApiV1AgentsListPagesQueryRequest`].
    pub fn build(self) -> Result<AgentsApiV1AgentsListPagesQueryRequest, BuildError> {
        Ok(AgentsApiV1AgentsListPagesQueryRequest {
            page_size: self.page_size,
            deployment_chat: self.deployment_chat,
            sources: self.sources,
            name: self.name,
            search: self.search,
            id: self.id,
            metadata: self.metadata,
            page_token: self.page_token,
        })
    }
}
