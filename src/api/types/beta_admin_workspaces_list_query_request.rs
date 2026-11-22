pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BetaAdminWorkspacesListQueryRequest {
    /// Whether to include archived Workspaces.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    /// Page number to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Maximum number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Search term to filter Workspaces by name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

impl BetaAdminWorkspacesListQueryRequest {
    pub fn builder() -> BetaAdminWorkspacesListQueryRequestBuilder {
        <BetaAdminWorkspacesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BetaAdminWorkspacesListQueryRequestBuilder {
    is_archived: Option<bool>,
    page: Option<i64>,
    page_size: Option<i64>,
    search: Option<String>,
}

impl BetaAdminWorkspacesListQueryRequestBuilder {
    pub fn is_archived(mut self, value: bool) -> Self {
        self.is_archived = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BetaAdminWorkspacesListQueryRequest`].
    pub fn build(self) -> Result<BetaAdminWorkspacesListQueryRequest, BuildError> {
        Ok(BetaAdminWorkspacesListQueryRequest {
            is_archived: self.is_archived,
            page: self.page,
            page_size: self.page_size,
            search: self.search,
        })
    }
}
