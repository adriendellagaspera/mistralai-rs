pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminUserWorkspacesOut {
    /// Workspaces the member belongs to on this page, ordered by name.
    #[serde(default)]
    pub items: Vec<MemberWorkspaceInfo>,
    /// Page number returned.
    #[serde(default)]
    pub page: i64,
    /// Maximum number of results per page.
    #[serde(default)]
    pub page_size: i64,
    /// Total number of Workspaces the member belongs to.
    #[serde(default)]
    pub total: i64,
}

impl AdminUserWorkspacesOut {
    pub fn builder() -> AdminUserWorkspacesOutBuilder {
        <AdminUserWorkspacesOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminUserWorkspacesOutBuilder {
    items: Option<Vec<MemberWorkspaceInfo>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
}

impl AdminUserWorkspacesOutBuilder {
    pub fn items(mut self, value: Vec<MemberWorkspaceInfo>) -> Self {
        self.items = Some(value);
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

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdminUserWorkspacesOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](AdminUserWorkspacesOutBuilder::items)
    /// - [`page`](AdminUserWorkspacesOutBuilder::page)
    /// - [`page_size`](AdminUserWorkspacesOutBuilder::page_size)
    /// - [`total`](AdminUserWorkspacesOutBuilder::total)
    pub fn build(self) -> Result<AdminUserWorkspacesOut, BuildError> {
        Ok(AdminUserWorkspacesOut {
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
            page: self.page.ok_or_else(|| BuildError::missing_field("page"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
