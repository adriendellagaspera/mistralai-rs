pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminUserGroupsOut {
    /// User groups on this page.
    #[serde(default)]
    pub items: Vec<AdminUserGroupOut>,
    /// Page number returned.
    #[serde(default)]
    pub page: i64,
    /// Maximum number of results per page.
    #[serde(default)]
    pub page_size: i64,
    /// Total number of user groups that match the request.
    #[serde(default)]
    pub total: i64,
}

impl AdminUserGroupsOut {
    pub fn builder() -> AdminUserGroupsOutBuilder {
        <AdminUserGroupsOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminUserGroupsOutBuilder {
    items: Option<Vec<AdminUserGroupOut>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
}

impl AdminUserGroupsOutBuilder {
    pub fn items(mut self, value: Vec<AdminUserGroupOut>) -> Self {
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

    /// Consumes the builder and constructs a [`AdminUserGroupsOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](AdminUserGroupsOutBuilder::items)
    /// - [`page`](AdminUserGroupsOutBuilder::page)
    /// - [`page_size`](AdminUserGroupsOutBuilder::page_size)
    /// - [`total`](AdminUserGroupsOutBuilder::total)
    pub fn build(self) -> Result<AdminUserGroupsOut, BuildError> {
        Ok(AdminUserGroupsOut {
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
