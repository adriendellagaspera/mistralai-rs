pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BetaConversationsListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

impl BetaConversationsListQueryRequest {
    pub fn builder() -> BetaConversationsListQueryRequestBuilder {
        <BetaConversationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BetaConversationsListQueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    metadata: Option<String>,
}

impl BetaConversationsListQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`BetaConversationsListQueryRequest`].
    pub fn build(self) -> Result<BetaConversationsListQueryRequest, BuildError> {
        Ok(BetaConversationsListQueryRequest {
            page: self.page,
            page_size: self.page_size,
            metadata: self.metadata,
        })
    }
}
