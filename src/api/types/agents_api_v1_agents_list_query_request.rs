pub use crate::prelude::*;

/// Query parameters for agents_api_v1_agents_list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsApiV1AgentsListQueryRequest {
    /// Page number (0-indexed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
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
}

impl AgentsApiV1AgentsListQueryRequest {
    pub fn builder() -> AgentsApiV1AgentsListQueryRequestBuilder {
        <AgentsApiV1AgentsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsApiV1AgentsListQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    deployment_chat: Option<bool>,
    sources: Option<Vec<RequestSource>>,
    name: Option<String>,
    search: Option<String>,
    id: Option<String>,
    metadata: Option<String>,
}

impl AgentsApiV1AgentsListQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

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

    /// Consumes the builder and constructs a [`AgentsApiV1AgentsListQueryRequest`].
    pub fn build(self) -> Result<AgentsApiV1AgentsListQueryRequest, BuildError> {
        Ok(AgentsApiV1AgentsListQueryRequest {
            page: self.page,
            page_size: self.page_size,
            deployment_chat: self.deployment_chat,
            sources: self.sources,
            name: self.name,
            search: self.search,
            id: self.id,
            metadata: self.metadata,
        })
    }
}
