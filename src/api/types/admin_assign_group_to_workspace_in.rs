pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminAssignGroupToWorkspaceIn {
    /// Deprecated single role value. Use 'role_names' instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<AdminAssignGroupToWorkspaceInRole>,
    /// Simplified role names to assign. Mutually exclusive with 'roles'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_names: Option<Vec<AdminAssignGroupToWorkspaceInRoleNamesItem>>,
    /// Role values to assign. Mutually exclusive with 'role_names'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<AdminAssignGroupToWorkspaceInRolesItem>>,
    #[serde(default)]
    pub workspace_uuid: String,
}

impl AdminAssignGroupToWorkspaceIn {
    pub fn builder() -> AdminAssignGroupToWorkspaceInBuilder {
        <AdminAssignGroupToWorkspaceInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminAssignGroupToWorkspaceInBuilder {
    role: Option<AdminAssignGroupToWorkspaceInRole>,
    role_names: Option<Vec<AdminAssignGroupToWorkspaceInRoleNamesItem>>,
    roles: Option<Vec<AdminAssignGroupToWorkspaceInRolesItem>>,
    workspace_uuid: Option<String>,
}

impl AdminAssignGroupToWorkspaceInBuilder {
    pub fn role(mut self, value: AdminAssignGroupToWorkspaceInRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn role_names(mut self, value: Vec<AdminAssignGroupToWorkspaceInRoleNamesItem>) -> Self {
        self.role_names = Some(value);
        self
    }

    pub fn roles(mut self, value: Vec<AdminAssignGroupToWorkspaceInRolesItem>) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn workspace_uuid(mut self, value: impl Into<String>) -> Self {
        self.workspace_uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdminAssignGroupToWorkspaceIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workspace_uuid`](AdminAssignGroupToWorkspaceInBuilder::workspace_uuid)
    pub fn build(self) -> Result<AdminAssignGroupToWorkspaceIn, BuildError> {
        Ok(AdminAssignGroupToWorkspaceIn {
            role: self.role,
            role_names: self.role_names,
            roles: self.roles,
            workspace_uuid: self
                .workspace_uuid
                .ok_or_else(|| BuildError::missing_field("workspace_uuid"))?,
        })
    }
}
