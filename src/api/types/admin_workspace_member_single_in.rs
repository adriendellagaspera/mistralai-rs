pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminWorkspaceMemberSingleIn {
    /// Deprecated legacy single role value, kept for backward compatibility. Mutually exclusive with 'role_name'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<AdminWorkspaceMemberSingleInRole>,
    /// Deprecated single role name, kept for backward compatibility. Mutually exclusive with 'role'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<AdminWorkspaceMemberSingleInRoleName>,
    /// Simplified role names to assign. Mutually exclusive with 'roles'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_names: Option<Vec<AdminWorkspaceMemberSingleInRoleNamesItem>>,
    /// Role values to assign. Mutually exclusive with 'role_names'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<AdminWorkspaceMemberSingleInRoles>,
    /// User ID of the Workspace member.
    #[serde(default)]
    pub user_uuid: String,
}

impl AdminWorkspaceMemberSingleIn {
    pub fn builder() -> AdminWorkspaceMemberSingleInBuilder {
        <AdminWorkspaceMemberSingleInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminWorkspaceMemberSingleInBuilder {
    role: Option<AdminWorkspaceMemberSingleInRole>,
    role_name: Option<AdminWorkspaceMemberSingleInRoleName>,
    role_names: Option<Vec<AdminWorkspaceMemberSingleInRoleNamesItem>>,
    roles: Option<AdminWorkspaceMemberSingleInRoles>,
    user_uuid: Option<String>,
}

impl AdminWorkspaceMemberSingleInBuilder {
    pub fn role(mut self, value: AdminWorkspaceMemberSingleInRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn role_name(mut self, value: AdminWorkspaceMemberSingleInRoleName) -> Self {
        self.role_name = Some(value);
        self
    }

    pub fn role_names(mut self, value: Vec<AdminWorkspaceMemberSingleInRoleNamesItem>) -> Self {
        self.role_names = Some(value);
        self
    }

    pub fn roles(mut self, value: AdminWorkspaceMemberSingleInRoles) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn user_uuid(mut self, value: impl Into<String>) -> Self {
        self.user_uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdminWorkspaceMemberSingleIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_uuid`](AdminWorkspaceMemberSingleInBuilder::user_uuid)
    pub fn build(self) -> Result<AdminWorkspaceMemberSingleIn, BuildError> {
        Ok(AdminWorkspaceMemberSingleIn {
            role: self.role,
            role_name: self.role_name,
            role_names: self.role_names,
            roles: self.roles,
            user_uuid: self
                .user_uuid
                .ok_or_else(|| BuildError::missing_field("user_uuid"))?,
        })
    }
}
