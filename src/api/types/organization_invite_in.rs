pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrganizationInviteIn {
    /// Simplified role names to assign. Mutually exclusive with 'roles'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_names: Option<Vec<OrganizationInviteInRoleNamesItem>>,
    /// Role values to assign. Mutually exclusive with 'role_names'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<OrganizationInviteInRoles>,
    /// Deprecated single role name, kept for backward compatibility. Mutually exclusive with 'role'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<OrganizationInviteInRoleName>,
    /// Deprecated legacy single role value, kept for backward compatibility. Mutually exclusive with 'role_name'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<OrganizationInviteInRole>,
    /// Email address, comma-separated emails, or newline-separated emails to invite.
    #[serde(default)]
    pub email: String,
    /// Deprecated single product seat to assign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<PlanType>,
    /// Product seats to assign to invited users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_types: Option<Vec<PlanType>>,
    /// Whether to grant subscription seats automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_seat_automatic_granting: Option<bool>,
    /// Language used for invitation emails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_language: Option<OrganizationInviteInEmailLanguage>,
    /// Workspace IDs the invited users should join.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_uuids: Option<Vec<String>>,
}

impl OrganizationInviteIn {
    pub fn builder() -> OrganizationInviteInBuilder {
        <OrganizationInviteInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationInviteInBuilder {
    role_names: Option<Vec<OrganizationInviteInRoleNamesItem>>,
    roles: Option<OrganizationInviteInRoles>,
    role_name: Option<OrganizationInviteInRoleName>,
    role: Option<OrganizationInviteInRole>,
    email: Option<String>,
    subscription_type: Option<PlanType>,
    subscription_types: Option<Vec<PlanType>>,
    subscription_seat_automatic_granting: Option<bool>,
    email_language: Option<OrganizationInviteInEmailLanguage>,
    workspace_uuids: Option<Vec<String>>,
}

impl OrganizationInviteInBuilder {
    pub fn role_names(mut self, value: Vec<OrganizationInviteInRoleNamesItem>) -> Self {
        self.role_names = Some(value);
        self
    }

    pub fn roles(mut self, value: OrganizationInviteInRoles) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn role_name(mut self, value: OrganizationInviteInRoleName) -> Self {
        self.role_name = Some(value);
        self
    }

    pub fn role(mut self, value: OrganizationInviteInRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn subscription_type(mut self, value: PlanType) -> Self {
        self.subscription_type = Some(value);
        self
    }

    pub fn subscription_types(mut self, value: Vec<PlanType>) -> Self {
        self.subscription_types = Some(value);
        self
    }

    pub fn subscription_seat_automatic_granting(mut self, value: bool) -> Self {
        self.subscription_seat_automatic_granting = Some(value);
        self
    }

    pub fn email_language(mut self, value: OrganizationInviteInEmailLanguage) -> Self {
        self.email_language = Some(value);
        self
    }

    pub fn workspace_uuids(mut self, value: Vec<String>) -> Self {
        self.workspace_uuids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrganizationInviteIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](OrganizationInviteInBuilder::email)
    pub fn build(self) -> Result<OrganizationInviteIn, BuildError> {
        Ok(OrganizationInviteIn {
            role_names: self.role_names,
            roles: self.roles,
            role_name: self.role_name,
            role: self.role,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            subscription_type: self.subscription_type,
            subscription_types: self.subscription_types,
            subscription_seat_automatic_granting: self.subscription_seat_automatic_granting,
            email_language: self.email_language,
            workspace_uuids: self.workspace_uuids,
        })
    }
}
