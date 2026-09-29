pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupWorkspaceAssignmentOut {
    #[serde(default)]
    pub roles: Vec<WorkspaceRoleRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(default)]
    pub workspace_uuid: String,
    /// Name of the Workspace.
    #[serde(default)]
    pub workspace_name: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created: DateTime<FixedOffset>,
}

impl GroupWorkspaceAssignmentOut {
    pub fn builder() -> GroupWorkspaceAssignmentOutBuilder {
        <GroupWorkspaceAssignmentOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupWorkspaceAssignmentOutBuilder {
    roles: Option<Vec<WorkspaceRoleRef>>,
    role_uuid: Option<String>,
    role_name: Option<String>,
    workspace_uuid: Option<String>,
    workspace_name: Option<String>,
    created: Option<DateTime<FixedOffset>>,
}

impl GroupWorkspaceAssignmentOutBuilder {
    pub fn roles(mut self, value: Vec<WorkspaceRoleRef>) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn role_uuid(mut self, value: impl Into<String>) -> Self {
        self.role_uuid = Some(value.into());
        self
    }

    pub fn role_name(mut self, value: impl Into<String>) -> Self {
        self.role_name = Some(value.into());
        self
    }

    pub fn workspace_uuid(mut self, value: impl Into<String>) -> Self {
        self.workspace_uuid = Some(value.into());
        self
    }

    pub fn workspace_name(mut self, value: impl Into<String>) -> Self {
        self.workspace_name = Some(value.into());
        self
    }

    pub fn created(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GroupWorkspaceAssignmentOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`roles`](GroupWorkspaceAssignmentOutBuilder::roles)
    /// - [`workspace_uuid`](GroupWorkspaceAssignmentOutBuilder::workspace_uuid)
    /// - [`workspace_name`](GroupWorkspaceAssignmentOutBuilder::workspace_name)
    /// - [`created`](GroupWorkspaceAssignmentOutBuilder::created)
    pub fn build(self) -> Result<GroupWorkspaceAssignmentOut, BuildError> {
        Ok(GroupWorkspaceAssignmentOut {
            roles: self
                .roles
                .ok_or_else(|| BuildError::missing_field("roles"))?,
            role_uuid: self.role_uuid,
            role_name: self.role_name,
            workspace_uuid: self
                .workspace_uuid
                .ok_or_else(|| BuildError::missing_field("workspace_uuid"))?,
            workspace_name: self
                .workspace_name
                .ok_or_else(|| BuildError::missing_field("workspace_name"))?,
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
        })
    }
}
