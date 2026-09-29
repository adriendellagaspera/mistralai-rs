pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum MemberWorkspaceInfoRawRoles {
    WorkspaceRoleList(Vec<WorkspaceRole>),

    StaticWorkspaceRolesList(Vec<StaticWorkspaceRoles>),
}

impl MemberWorkspaceInfoRawRoles {
    pub fn is_workspace_role_list(&self) -> bool {
        matches!(self, Self::WorkspaceRoleList(_))
    }

    pub fn is_static_workspace_roles_list(&self) -> bool {
        matches!(self, Self::StaticWorkspaceRolesList(_))
    }

    pub fn as_workspace_role_list(&self) -> Option<&Vec<WorkspaceRole>> {
        match self {
            Self::WorkspaceRoleList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_workspace_role_list(self) -> Option<Vec<WorkspaceRole>> {
        match self {
            Self::WorkspaceRoleList(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_static_workspace_roles_list(&self) -> Option<&Vec<StaticWorkspaceRoles>> {
        match self {
            Self::StaticWorkspaceRolesList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_static_workspace_roles_list(self) -> Option<Vec<StaticWorkspaceRoles>> {
        match self {
            Self::StaticWorkspaceRolesList(value) => Some(value),
            _ => None,
        }
    }
}
