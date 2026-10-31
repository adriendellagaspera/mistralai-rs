pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminUpdateGroupWorkspaceAssignmentIn {
    /// Deprecated single role value. Use 'role_names' instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<AdminUpdateGroupWorkspaceAssignmentInRole>,
    /// Simplified role names to assign. Mutually exclusive with 'roles'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_names: Option<Vec<AdminUpdateGroupWorkspaceAssignmentInRoleNamesItem>>,
    /// Role values to assign. Mutually exclusive with 'role_names'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<AdminUpdateGroupWorkspaceAssignmentInRolesItem>>,
}

impl AdminUpdateGroupWorkspaceAssignmentIn {
    pub fn builder() -> AdminUpdateGroupWorkspaceAssignmentInBuilder {
        <AdminUpdateGroupWorkspaceAssignmentInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminUpdateGroupWorkspaceAssignmentInBuilder {
    role: Option<AdminUpdateGroupWorkspaceAssignmentInRole>,
    role_names: Option<Vec<AdminUpdateGroupWorkspaceAssignmentInRoleNamesItem>>,
    roles: Option<Vec<AdminUpdateGroupWorkspaceAssignmentInRolesItem>>,
}

impl AdminUpdateGroupWorkspaceAssignmentInBuilder {
    pub fn role(mut self, value: AdminUpdateGroupWorkspaceAssignmentInRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn role_names(
        mut self,
        value: Vec<AdminUpdateGroupWorkspaceAssignmentInRoleNamesItem>,
    ) -> Self {
        self.role_names = Some(value);
        self
    }

    pub fn roles(mut self, value: Vec<AdminUpdateGroupWorkspaceAssignmentInRolesItem>) -> Self {
        self.roles = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdminUpdateGroupWorkspaceAssignmentIn`].
    pub fn build(self) -> Result<AdminUpdateGroupWorkspaceAssignmentIn, BuildError> {
        Ok(AdminUpdateGroupWorkspaceAssignmentIn {
            role: self.role,
            role_names: self.role_names,
            roles: self.roles,
        })
    }
}
