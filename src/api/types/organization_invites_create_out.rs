pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrganizationInvitesCreateOut {
    /// Email addresses that already belong to the Organization.
    #[serde(default)]
    pub already_members: Vec<String>,
    /// Email addresses that could not be invited.
    #[serde(default)]
    pub invalid_emails: Vec<String>,
    /// Number of invitations successfully created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_members_count: Option<i64>,
}

impl OrganizationInvitesCreateOut {
    pub fn builder() -> OrganizationInvitesCreateOutBuilder {
        <OrganizationInvitesCreateOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationInvitesCreateOutBuilder {
    already_members: Option<Vec<String>>,
    invalid_emails: Option<Vec<String>>,
    invited_members_count: Option<i64>,
}

impl OrganizationInvitesCreateOutBuilder {
    pub fn already_members(mut self, value: Vec<String>) -> Self {
        self.already_members = Some(value);
        self
    }

    pub fn invalid_emails(mut self, value: Vec<String>) -> Self {
        self.invalid_emails = Some(value);
        self
    }

    pub fn invited_members_count(mut self, value: i64) -> Self {
        self.invited_members_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrganizationInvitesCreateOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`already_members`](OrganizationInvitesCreateOutBuilder::already_members)
    /// - [`invalid_emails`](OrganizationInvitesCreateOutBuilder::invalid_emails)
    pub fn build(self) -> Result<OrganizationInvitesCreateOut, BuildError> {
        Ok(OrganizationInvitesCreateOut {
            already_members: self
                .already_members
                .ok_or_else(|| BuildError::missing_field("already_members"))?,
            invalid_emails: self
                .invalid_emails
                .ok_or_else(|| BuildError::missing_field("invalid_emails"))?,
            invited_members_count: self.invited_members_count,
        })
    }
}
