pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssignGroupToWorkspaceIn {
    /// Deprecated single role value. Use 'role_names' instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<AssignGroupToWorkspaceInRole>,
    /// Simplified role names to assign. Mutually exclusive with 'roles'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_names: Option<Vec<AssignGroupToWorkspaceInRoleNamesItem>>,
    /// Role values to assign. Mutually exclusive with 'role_names'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<AssignGroupToWorkspaceInRolesItem>>,
    #[serde(default)]
    pub workspace_uuid: String,
}

impl AssignGroupToWorkspaceIn {
    pub fn builder() -> AssignGroupToWorkspaceInBuilder {
        <AssignGroupToWorkspaceInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssignGroupToWorkspaceInBuilder {
    role: Option<AssignGroupToWorkspaceInRole>,
    role_names: Option<Vec<AssignGroupToWorkspaceInRoleNamesItem>>,
    roles: Option<Vec<AssignGroupToWorkspaceInRolesItem>>,
    workspace_uuid: Option<String>,
}

impl AssignGroupToWorkspaceInBuilder {
    pub fn role(mut self, value: AssignGroupToWorkspaceInRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn role_names(mut self, value: Vec<AssignGroupToWorkspaceInRoleNamesItem>) -> Self {
        self.role_names = Some(value);
        self
    }

    pub fn roles(mut self, value: Vec<AssignGroupToWorkspaceInRolesItem>) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn workspace_uuid(mut self, value: impl Into<String>) -> Self {
        self.workspace_uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssignGroupToWorkspaceIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workspace_uuid`](AssignGroupToWorkspaceInBuilder::workspace_uuid)
    pub fn build(self) -> Result<AssignGroupToWorkspaceIn, BuildError> {
        Ok(AssignGroupToWorkspaceIn {
            role: self.role,
            role_names: self.role_names,
            roles: self.roles,
            workspace_uuid: self
                .workspace_uuid
                .ok_or_else(|| BuildError::missing_field("workspace_uuid"))?,
        })
    }
}
