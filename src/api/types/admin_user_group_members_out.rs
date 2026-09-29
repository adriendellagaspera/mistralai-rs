pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminUserGroupMembersOut {
    /// Total number of group members that match the request.
    #[serde(default)]
    pub total: i64,
    /// Group members on this page.
    #[serde(default)]
    pub members: Vec<AdminUserGroupMemberOut>,
    /// Page number returned.
    #[serde(default)]
    pub page: i64,
    /// Maximum number of results per page.
    #[serde(default)]
    pub page_size: i64,
}

impl AdminUserGroupMembersOut {
    pub fn builder() -> AdminUserGroupMembersOutBuilder {
        <AdminUserGroupMembersOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminUserGroupMembersOutBuilder {
    total: Option<i64>,
    members: Option<Vec<AdminUserGroupMemberOut>>,
    page: Option<i64>,
    page_size: Option<i64>,
}

impl AdminUserGroupMembersOutBuilder {
    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn members(mut self, value: Vec<AdminUserGroupMemberOut>) -> Self {
        self.members = Some(value);
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

    /// Consumes the builder and constructs a [`AdminUserGroupMembersOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total`](AdminUserGroupMembersOutBuilder::total)
    /// - [`members`](AdminUserGroupMembersOutBuilder::members)
    /// - [`page`](AdminUserGroupMembersOutBuilder::page)
    /// - [`page_size`](AdminUserGroupMembersOutBuilder::page_size)
    pub fn build(self) -> Result<AdminUserGroupMembersOut, BuildError> {
        Ok(AdminUserGroupMembersOut {
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            members: self
                .members
                .ok_or_else(|| BuildError::missing_field("members"))?,
            page: self.page.ok_or_else(|| BuildError::missing_field("page"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
        })
    }
}
