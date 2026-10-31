pub use crate::prelude::*;

/// Query parameters for list_indexes_v1_rag_managed_indexes_get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListIndexesV1RagManagedIndexesGetQueryRequest {
    /// Maximum number of indexes to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Cursor returned as next_page_token by the previous page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// Case-insensitive substring to match against index names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Status to match
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ManagedIndexStatus>,
    /// Creator ID to match
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
}

impl ListIndexesV1RagManagedIndexesGetQueryRequest {
    pub fn builder() -> ListIndexesV1RagManagedIndexesGetQueryRequestBuilder {
        <ListIndexesV1RagManagedIndexesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListIndexesV1RagManagedIndexesGetQueryRequestBuilder {
    page_size: Option<i64>,
    page_token: Option<String>,
    name: Option<String>,
    status: Option<ManagedIndexStatus>,
    creator_id: Option<String>,
}

impl ListIndexesV1RagManagedIndexesGetQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page_token(mut self, value: impl Into<String>) -> Self {
        self.page_token = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn status(mut self, value: ManagedIndexStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn creator_id(mut self, value: impl Into<String>) -> Self {
        self.creator_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListIndexesV1RagManagedIndexesGetQueryRequest`].
    pub fn build(self) -> Result<ListIndexesV1RagManagedIndexesGetQueryRequest, BuildError> {
        Ok(ListIndexesV1RagManagedIndexesGetQueryRequest {
            page_size: self.page_size,
            page_token: self.page_token,
            name: self.name,
            status: self.status,
            creator_id: self.creator_id,
        })
    }
}
