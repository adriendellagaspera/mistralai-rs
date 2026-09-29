pub use crate::prelude::*;

/// Query parameters for agents_api_v1_conversations_list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsApiV1ConversationsListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

impl AgentsApiV1ConversationsListQueryRequest {
    pub fn builder() -> AgentsApiV1ConversationsListQueryRequestBuilder {
        <AgentsApiV1ConversationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsApiV1ConversationsListQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    metadata: Option<String>,
}

impl AgentsApiV1ConversationsListQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn metadata(mut self, value: impl Into<String>) -> Self {
        self.metadata = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsApiV1ConversationsListQueryRequest`].
    pub fn build(self) -> Result<AgentsApiV1ConversationsListQueryRequest, BuildError> {
        Ok(AgentsApiV1ConversationsListQueryRequest {
            page: self.page,
            page_size: self.page_size,
            metadata: self.metadata,
        })
    }
}
