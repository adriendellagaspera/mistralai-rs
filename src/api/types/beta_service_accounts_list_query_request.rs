pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BetaServiceAccountsListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<SortOrder>,
    #[serde(default)]
    pub offset: i64,
    #[serde(default)]
    pub limit: i64,
}

impl BetaServiceAccountsListQueryRequest {
    pub fn builder() -> BetaServiceAccountsListQueryRequestBuilder {
        <BetaServiceAccountsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BetaServiceAccountsListQueryRequestBuilder {
    workspace_id: Option<String>,
    include_deleted: Option<bool>,
    q: Option<String>,
    order: Option<SortOrder>,
    offset: Option<i64>,
    limit: Option<i64>,
}

impl BetaServiceAccountsListQueryRequestBuilder {
    pub fn workspace_id(mut self, value: impl Into<String>) -> Self {
        self.workspace_id = Some(value.into());
        self
    }

    pub fn include_deleted(mut self, value: bool) -> Self {
        self.include_deleted = Some(value);
        self
    }

    pub fn q(mut self, value: impl Into<String>) -> Self {
        self.q = Some(value.into());
        self
    }

    pub fn order(mut self, value: SortOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BetaServiceAccountsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`offset`](BetaServiceAccountsListQueryRequestBuilder::offset)
    /// - [`limit`](BetaServiceAccountsListQueryRequestBuilder::limit)
    pub fn build(self) -> Result<BetaServiceAccountsListQueryRequest, BuildError> {
        Ok(BetaServiceAccountsListQueryRequest {
            workspace_id: self.workspace_id,
            include_deleted: self.include_deleted,
            q: self.q,
            order: self.order,
            offset: self
                .offset
                .ok_or_else(|| BuildError::missing_field("offset"))?,
            limit: self
                .limit
                .ok_or_else(|| BuildError::missing_field("limit"))?,
        })
    }
}
