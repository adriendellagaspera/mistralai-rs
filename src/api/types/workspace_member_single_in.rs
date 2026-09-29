pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceMemberSingleIn {
    /// Simplified role names to assign. Mutually exclusive with 'roles'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_names: Option<Vec<WorkspaceMemberSingleInRoleNamesItem>>,
    /// Role values to assign. Mutually exclusive with 'role_names'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<WorkspaceMemberSingleInRoles>,
    /// Deprecated single role name, kept for backward compatibility. Mutually exclusive with 'role'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<WorkspaceMemberSingleInRoleName>,
    /// Deprecated legacy single role value, kept for backward compatibility. Mutually exclusive with 'role_name'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<WorkspaceMemberSingleInRole>,
    /// User ID of the Workspace member.
    #[serde(default)]
    pub user_uuid: String,
}

impl WorkspaceMemberSingleIn {
    pub fn builder() -> WorkspaceMemberSingleInBuilder {
        <WorkspaceMemberSingleInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceMemberSingleInBuilder {
    role_names: Option<Vec<WorkspaceMemberSingleInRoleNamesItem>>,
    roles: Option<WorkspaceMemberSingleInRoles>,
    role_name: Option<WorkspaceMemberSingleInRoleName>,
    role: Option<WorkspaceMemberSingleInRole>,
    user_uuid: Option<String>,
}

impl WorkspaceMemberSingleInBuilder {
    pub fn role_names(mut self, value: Vec<WorkspaceMemberSingleInRoleNamesItem>) -> Self {
        self.role_names = Some(value);
        self
    }

    pub fn roles(mut self, value: WorkspaceMemberSingleInRoles) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn role_name(mut self, value: WorkspaceMemberSingleInRoleName) -> Self {
        self.role_name = Some(value);
        self
    }

    pub fn role(mut self, value: WorkspaceMemberSingleInRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn user_uuid(mut self, value: impl Into<String>) -> Self {
        self.user_uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceMemberSingleIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_uuid`](WorkspaceMemberSingleInBuilder::user_uuid)
    pub fn build(self) -> Result<WorkspaceMemberSingleIn, BuildError> {
        Ok(WorkspaceMemberSingleIn {
            role_names: self.role_names,
            roles: self.roles,
            role_name: self.role_name,
            role: self.role,
            user_uuid: self
                .user_uuid
                .ok_or_else(|| BuildError::missing_field("user_uuid"))?,
        })
    }
}
