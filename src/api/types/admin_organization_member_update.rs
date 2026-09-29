pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminOrganizationMemberUpdate {
    /// Deprecated legacy single role value, kept for backward compatibility. Mutually exclusive with 'role_name'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<AdminOrganizationMemberUpdateRole>,
    /// Deprecated single role name, kept for backward compatibility. Mutually exclusive with 'role'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<AdminOrganizationMemberUpdateRoleName>,
    /// Simplified role names to assign. Mutually exclusive with 'roles'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_names: Option<Vec<AdminOrganizationMemberUpdateRoleNamesItem>>,
    /// Role values to assign. Mutually exclusive with 'role_names'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<AdminOrganizationMemberUpdateRoles>,
    /// Product seats to assign to the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_types: Option<Vec<AdminOrganizationMemberUpdateSubscriptionTypesItem>>,
}

impl AdminOrganizationMemberUpdate {
    pub fn builder() -> AdminOrganizationMemberUpdateBuilder {
        <AdminOrganizationMemberUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminOrganizationMemberUpdateBuilder {
    role: Option<AdminOrganizationMemberUpdateRole>,
    role_name: Option<AdminOrganizationMemberUpdateRoleName>,
    role_names: Option<Vec<AdminOrganizationMemberUpdateRoleNamesItem>>,
    roles: Option<AdminOrganizationMemberUpdateRoles>,
    subscription_types: Option<Vec<AdminOrganizationMemberUpdateSubscriptionTypesItem>>,
}

impl AdminOrganizationMemberUpdateBuilder {
    pub fn role(mut self, value: AdminOrganizationMemberUpdateRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn role_name(mut self, value: AdminOrganizationMemberUpdateRoleName) -> Self {
        self.role_name = Some(value);
        self
    }

    pub fn role_names(mut self, value: Vec<AdminOrganizationMemberUpdateRoleNamesItem>) -> Self {
        self.role_names = Some(value);
        self
    }

    pub fn roles(mut self, value: AdminOrganizationMemberUpdateRoles) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn subscription_types(
        mut self,
        value: Vec<AdminOrganizationMemberUpdateSubscriptionTypesItem>,
    ) -> Self {
        self.subscription_types = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdminOrganizationMemberUpdate`].
    pub fn build(self) -> Result<AdminOrganizationMemberUpdate, BuildError> {
        Ok(AdminOrganizationMemberUpdate {
            role: self.role,
            role_name: self.role_name,
            role_names: self.role_names,
            roles: self.roles,
            subscription_types: self.subscription_types,
        })
    }
}
