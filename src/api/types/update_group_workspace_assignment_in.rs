pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateGroupWorkspaceAssignmentIn {
    /// Simplified role names to assign. Mutually exclusive with 'roles'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_names: Option<Vec<UpdateGroupWorkspaceAssignmentInRoleNamesItem>>,
    /// Role values to assign. Mutually exclusive with 'role_names'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<UpdateGroupWorkspaceAssignmentInRolesItem>>,
    /// Deprecated single role value. Use 'role_names' instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<UpdateGroupWorkspaceAssignmentInRole>,
}

impl UpdateGroupWorkspaceAssignmentIn {
    pub fn builder() -> UpdateGroupWorkspaceAssignmentInBuilder {
        <UpdateGroupWorkspaceAssignmentInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateGroupWorkspaceAssignmentInBuilder {
    role_names: Option<Vec<UpdateGroupWorkspaceAssignmentInRoleNamesItem>>,
    roles: Option<Vec<UpdateGroupWorkspaceAssignmentInRolesItem>>,
    role: Option<UpdateGroupWorkspaceAssignmentInRole>,
}

impl UpdateGroupWorkspaceAssignmentInBuilder {
    pub fn role_names(mut self, value: Vec<UpdateGroupWorkspaceAssignmentInRoleNamesItem>) -> Self {
        self.role_names = Some(value);
        self
    }

    pub fn roles(mut self, value: Vec<UpdateGroupWorkspaceAssignmentInRolesItem>) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn role(mut self, value: UpdateGroupWorkspaceAssignmentInRole) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateGroupWorkspaceAssignmentIn`].
    pub fn build(self) -> Result<UpdateGroupWorkspaceAssignmentIn, BuildError> {
        Ok(UpdateGroupWorkspaceAssignmentIn {
            role_names: self.role_names,
            roles: self.roles,
            role: self.role,
        })
    }
}
