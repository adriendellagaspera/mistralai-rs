pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrganizationMemberCreate {
    /// Simplified role names to assign. Mutually exclusive with 'roles'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_names: Option<Vec<OrganizationMemberCreateRoleNamesItem>>,
    /// Role values to assign. Mutually exclusive with 'role_names'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<OrganizationMemberCreateRoles>,
    /// Deprecated single role name, kept for backward compatibility. Mutually exclusive with 'role'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<OrganizationMemberCreateRoleName>,
    /// Deprecated legacy single role value, kept for backward compatibility. Mutually exclusive with 'role_name'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<OrganizationMemberCreateRole>,
    /// Email address of the user to create.
    #[serde(default)]
    pub email: String,
    /// First name of the user to create.
    #[serde(default)]
    pub first_name: String,
    /// Last name of the user to create.
    #[serde(default)]
    pub last_name: String,
    /// Product seats to assign to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_types: Option<Vec<PlanType>>,
}

impl OrganizationMemberCreate {
    pub fn builder() -> OrganizationMemberCreateBuilder {
        <OrganizationMemberCreateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationMemberCreateBuilder {
    role_names: Option<Vec<OrganizationMemberCreateRoleNamesItem>>,
    roles: Option<OrganizationMemberCreateRoles>,
    role_name: Option<OrganizationMemberCreateRoleName>,
    role: Option<OrganizationMemberCreateRole>,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    subscription_types: Option<Vec<PlanType>>,
}

impl OrganizationMemberCreateBuilder {
    pub fn role_names(mut self, value: Vec<OrganizationMemberCreateRoleNamesItem>) -> Self {
        self.role_names = Some(value);
        self
    }

    pub fn roles(mut self, value: OrganizationMemberCreateRoles) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn role_name(mut self, value: OrganizationMemberCreateRoleName) -> Self {
        self.role_name = Some(value);
        self
    }

    pub fn role(mut self, value: OrganizationMemberCreateRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn subscription_types(mut self, value: Vec<PlanType>) -> Self {
        self.subscription_types = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrganizationMemberCreate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](OrganizationMemberCreateBuilder::email)
    /// - [`first_name`](OrganizationMemberCreateBuilder::first_name)
    /// - [`last_name`](OrganizationMemberCreateBuilder::last_name)
    pub fn build(self) -> Result<OrganizationMemberCreate, BuildError> {
        Ok(OrganizationMemberCreate {
            role_names: self.role_names,
            roles: self.roles,
            role_name: self.role_name,
            role: self.role,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            subscription_types: self.subscription_types,
        })
    }
}
