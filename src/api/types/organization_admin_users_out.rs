pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrganizationAdminUsersOut {
    /// Pending Organization invitations on this page.
    #[serde(default)]
    pub invites: Vec<OrganizationInviteOut>,
    /// Organization members on this page.
    #[serde(default)]
    pub members: Vec<AdminOrganizationMemberOut>,
    /// Page number returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Maximum number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Total number of users and invitations that match the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl OrganizationAdminUsersOut {
    pub fn builder() -> OrganizationAdminUsersOutBuilder {
        <OrganizationAdminUsersOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationAdminUsersOutBuilder {
    invites: Option<Vec<OrganizationInviteOut>>,
    members: Option<Vec<AdminOrganizationMemberOut>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
}

impl OrganizationAdminUsersOutBuilder {
    pub fn invites(mut self, value: Vec<OrganizationInviteOut>) -> Self {
        self.invites = Some(value);
        self
    }

    pub fn members(mut self, value: Vec<AdminOrganizationMemberOut>) -> Self {
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

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrganizationAdminUsersOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invites`](OrganizationAdminUsersOutBuilder::invites)
    /// - [`members`](OrganizationAdminUsersOutBuilder::members)
    pub fn build(self) -> Result<OrganizationAdminUsersOut, BuildError> {
        Ok(OrganizationAdminUsersOut {
            invites: self
                .invites
                .ok_or_else(|| BuildError::missing_field("invites"))?,
            members: self
                .members
                .ok_or_else(|| BuildError::missing_field("members"))?,
            page: self.page,
            page_size: self.page_size,
            total: self.total,
        })
    }
}
