pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OrganizationInviteOut {
    /// Time when the invitation was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Email address invited to the Organization.
    #[serde(default)]
    pub email: String,
    /// Whether the invitation has expired.
    #[serde(default)]
    pub expired: bool,
    /// Deprecated single invite role. Use 'raw_roles' instead.
    pub raw_role: OrganizationInviteOutRawRole,
    /// Organization roles assigned by the invite.
    pub raw_roles: OrganizationInviteOutRawRoles,
    /// Organization invitation ID.
    #[serde(default)]
    pub uuid: String,
    /// Workspace IDs the invited user will join.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_uuids: Option<Vec<String>>,
}

impl OrganizationInviteOut {
    pub fn builder() -> OrganizationInviteOutBuilder {
        <OrganizationInviteOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationInviteOutBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    email: Option<String>,
    expired: Option<bool>,
    raw_role: Option<OrganizationInviteOutRawRole>,
    raw_roles: Option<OrganizationInviteOutRawRoles>,
    uuid: Option<String>,
    workspace_uuids: Option<Vec<String>>,
}

impl OrganizationInviteOutBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn expired(mut self, value: bool) -> Self {
        self.expired = Some(value);
        self
    }

    pub fn raw_role(mut self, value: OrganizationInviteOutRawRole) -> Self {
        self.raw_role = Some(value);
        self
    }

    pub fn raw_roles(mut self, value: OrganizationInviteOutRawRoles) -> Self {
        self.raw_roles = Some(value);
        self
    }

    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    pub fn workspace_uuids(mut self, value: Vec<String>) -> Self {
        self.workspace_uuids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrganizationInviteOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](OrganizationInviteOutBuilder::created_at)
    /// - [`email`](OrganizationInviteOutBuilder::email)
    /// - [`expired`](OrganizationInviteOutBuilder::expired)
    /// - [`raw_role`](OrganizationInviteOutBuilder::raw_role)
    /// - [`raw_roles`](OrganizationInviteOutBuilder::raw_roles)
    /// - [`uuid`](OrganizationInviteOutBuilder::uuid)
    pub fn build(self) -> Result<OrganizationInviteOut, BuildError> {
        Ok(OrganizationInviteOut {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            expired: self
                .expired
                .ok_or_else(|| BuildError::missing_field("expired"))?,
            raw_role: self
                .raw_role
                .ok_or_else(|| BuildError::missing_field("raw_role"))?,
            raw_roles: self
                .raw_roles
                .ok_or_else(|| BuildError::missing_field("raw_roles"))?,
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
            workspace_uuids: self.workspace_uuids,
        })
    }
}
