pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OrganizationUserInviteOut {
    /// Organization invitation ID.
    #[serde(default)]
    pub invite_uuid: String,
    /// Email address invited to the Organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Organization roles assigned by the invite.
    pub roles: OrganizationUserInviteOutRoles,
    /// Deprecated single invite role. Use 'roles' instead.
    pub role: OrganizationUserInviteOutRole,
}

impl OrganizationUserInviteOut {
    pub fn builder() -> OrganizationUserInviteOutBuilder {
        <OrganizationUserInviteOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationUserInviteOutBuilder {
    invite_uuid: Option<String>,
    email: Option<String>,
    roles: Option<OrganizationUserInviteOutRoles>,
    role: Option<OrganizationUserInviteOutRole>,
}

impl OrganizationUserInviteOutBuilder {
    pub fn invite_uuid(mut self, value: impl Into<String>) -> Self {
        self.invite_uuid = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn roles(mut self, value: OrganizationUserInviteOutRoles) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn role(mut self, value: OrganizationUserInviteOutRole) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrganizationUserInviteOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invite_uuid`](OrganizationUserInviteOutBuilder::invite_uuid)
    /// - [`roles`](OrganizationUserInviteOutBuilder::roles)
    /// - [`role`](OrganizationUserInviteOutBuilder::role)
    pub fn build(self) -> Result<OrganizationUserInviteOut, BuildError> {
        Ok(OrganizationUserInviteOut {
            invite_uuid: self
                .invite_uuid
                .ok_or_else(|| BuildError::missing_field("invite_uuid"))?,
            email: self.email,
            roles: self
                .roles
                .ok_or_else(|| BuildError::missing_field("roles"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
        })
    }
}
