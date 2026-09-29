pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminProvisionGroupToWorkspaceIn {
    /// User group ID to provision.
    #[serde(default)]
    pub user_group_uuid: String,
    /// Workspace role value to assign to the group. Mutually exclusive with 'workspace_role_name'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_role: Option<AdminProvisionGroupToWorkspaceInWorkspaceRole>,
    /// Workspace role name to assign to the group. Mutually exclusive with 'workspace_role'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_role_name: Option<AdminProvisionGroupToWorkspaceInWorkspaceRoleName>,
    /// Workspace ID where the group is provisioned.
    #[serde(default)]
    pub workspace_uuid: String,
}

impl AdminProvisionGroupToWorkspaceIn {
    pub fn builder() -> AdminProvisionGroupToWorkspaceInBuilder {
        <AdminProvisionGroupToWorkspaceInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminProvisionGroupToWorkspaceInBuilder {
    user_group_uuid: Option<String>,
    workspace_role: Option<AdminProvisionGroupToWorkspaceInWorkspaceRole>,
    workspace_role_name: Option<AdminProvisionGroupToWorkspaceInWorkspaceRoleName>,
    workspace_uuid: Option<String>,
}

impl AdminProvisionGroupToWorkspaceInBuilder {
    pub fn user_group_uuid(mut self, value: impl Into<String>) -> Self {
        self.user_group_uuid = Some(value.into());
        self
    }

    pub fn workspace_role(mut self, value: AdminProvisionGroupToWorkspaceInWorkspaceRole) -> Self {
        self.workspace_role = Some(value);
        self
    }

    pub fn workspace_role_name(
        mut self,
        value: AdminProvisionGroupToWorkspaceInWorkspaceRoleName,
    ) -> Self {
        self.workspace_role_name = Some(value);
        self
    }

    pub fn workspace_uuid(mut self, value: impl Into<String>) -> Self {
        self.workspace_uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdminProvisionGroupToWorkspaceIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_group_uuid`](AdminProvisionGroupToWorkspaceInBuilder::user_group_uuid)
    /// - [`workspace_uuid`](AdminProvisionGroupToWorkspaceInBuilder::workspace_uuid)
    pub fn build(self) -> Result<AdminProvisionGroupToWorkspaceIn, BuildError> {
        Ok(AdminProvisionGroupToWorkspaceIn {
            user_group_uuid: self
                .user_group_uuid
                .ok_or_else(|| BuildError::missing_field("user_group_uuid"))?,
            workspace_role: self.workspace_role,
            workspace_role_name: self.workspace_role_name,
            workspace_uuid: self
                .workspace_uuid
                .ok_or_else(|| BuildError::missing_field("workspace_uuid"))?,
        })
    }
}
