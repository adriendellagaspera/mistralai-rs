pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrganizationAdminUsersOut {
    /// Organization members on this page.
    #[serde(default)]
    pub members: Vec<AdminOrganizationMemberOut>,
    /// Pending Organization invitations on this page.
    #[serde(default)]
    pub invites: Vec<OrganizationInviteOut>,
    /// Total number of users and invitations that match the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Page number returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Maximum number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl OrganizationAdminUsersOut {
    pub fn builder() -> OrganizationAdminUsersOutBuilder {
        <OrganizationAdminUsersOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationAdminUsersOutBuilder {
    members: Option<Vec<AdminOrganizationMemberOut>>,
    invites: Option<Vec<OrganizationInviteOut>>,
    total: Option<i64>,
    page: Option<i64>,
    page_size: Option<i64>,
}

impl OrganizationAdminUsersOutBuilder {
    pub fn members(mut self, value: Vec<AdminOrganizationMemberOut>) -> Self {
        self.members = Some(value);
        self
    }

    pub fn invites(mut self, value: Vec<OrganizationInviteOut>) -> Self {
        self.invites = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
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

    /// Consumes the builder and constructs a [`OrganizationAdminUsersOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`members`](OrganizationAdminUsersOutBuilder::members)
    /// - [`invites`](OrganizationAdminUsersOutBuilder::invites)
    pub fn build(self) -> Result<OrganizationAdminUsersOut, BuildError> {
        Ok(OrganizationAdminUsersOut {
            members: self
                .members
                .ok_or_else(|| BuildError::missing_field("members"))?,
            invites: self
                .invites
                .ok_or_else(|| BuildError::missing_field("invites"))?,
            total: self.total,
            page: self.page,
            page_size: self.page_size,
        })
    }
}
