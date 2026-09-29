pub use crate::prelude::*;

/// Query parameters for agents_api_v1_agents_list_versions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsApiV1AgentsListVersionsQueryRequest {
    /// Page number (0-indexed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Number of versions per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl AgentsApiV1AgentsListVersionsQueryRequest {
    pub fn builder() -> AgentsApiV1AgentsListVersionsQueryRequestBuilder {
        <AgentsApiV1AgentsListVersionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsApiV1AgentsListVersionsQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
}

impl AgentsApiV1AgentsListVersionsQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsApiV1AgentsListVersionsQueryRequest`].
    pub fn build(self) -> Result<AgentsApiV1AgentsListVersionsQueryRequest, BuildError> {
        Ok(AgentsApiV1AgentsListVersionsQueryRequest {
            page: self.page,
            page_size: self.page_size,
        })
    }
}
