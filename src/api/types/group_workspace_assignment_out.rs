pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupWorkspaceAssignmentOut {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_uuid: Option<String>,
    #[serde(default)]
    pub roles: Vec<WorkspaceRoleRef>,
    /// Name of the Workspace.
    #[serde(default)]
    pub workspace_name: String,
    #[serde(default)]
    pub workspace_uuid: String,
}

impl GroupWorkspaceAssignmentOut {
    pub fn builder() -> GroupWorkspaceAssignmentOutBuilder {
        <GroupWorkspaceAssignmentOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupWorkspaceAssignmentOutBuilder {
    created: Option<DateTime<FixedOffset>>,
    role_name: Option<String>,
    role_uuid: Option<String>,
    roles: Option<Vec<WorkspaceRoleRef>>,
    workspace_name: Option<String>,
    workspace_uuid: Option<String>,
}

impl GroupWorkspaceAssignmentOutBuilder {
    pub fn created(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created = Some(value);
        self
    }

    pub fn role_name(mut self, value: impl Into<String>) -> Self {
        self.role_name = Some(value.into());
        self
    }

    pub fn role_uuid(mut self, value: impl Into<String>) -> Self {
        self.role_uuid = Some(value.into());
        self
    }

    pub fn roles(mut self, value: Vec<WorkspaceRoleRef>) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn workspace_name(mut self, value: impl Into<String>) -> Self {
        self.workspace_name = Some(value.into());
        self
    }

    pub fn workspace_uuid(mut self, value: impl Into<String>) -> Self {
        self.workspace_uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GroupWorkspaceAssignmentOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](GroupWorkspaceAssignmentOutBuilder::created)
    /// - [`roles`](GroupWorkspaceAssignmentOutBuilder::roles)
    /// - [`workspace_name`](GroupWorkspaceAssignmentOutBuilder::workspace_name)
    /// - [`workspace_uuid`](GroupWorkspaceAssignmentOutBuilder::workspace_uuid)
    pub fn build(self) -> Result<GroupWorkspaceAssignmentOut, BuildError> {
        Ok(GroupWorkspaceAssignmentOut {
            created: self
                .created
                .ok_or_else(|| BuildError::missing_field("created"))?,
            role_name: self.role_name,
            role_uuid: self.role_uuid,
            roles: self
                .roles
                .ok_or_else(|| BuildError::missing_field("roles"))?,
            workspace_name: self
                .workspace_name
                .ok_or_else(|| BuildError::missing_field("workspace_name"))?,
            workspace_uuid: self
                .workspace_uuid
                .ok_or_else(|| BuildError::missing_field("workspace_uuid"))?,
        })
    }
}
