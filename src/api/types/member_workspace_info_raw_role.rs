pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum MemberWorkspaceInfoRawRole {
    WorkspaceRole(WorkspaceRole),

    StaticWorkspaceRoles(StaticWorkspaceRoles),
}

impl MemberWorkspaceInfoRawRole {
    pub fn is_workspace_role(&self) -> bool {
        matches!(self, Self::WorkspaceRole(_))
    }

    pub fn is_static_workspace_roles(&self) -> bool {
        matches!(self, Self::StaticWorkspaceRoles(_))
    }

    pub fn as_workspace_role(&self) -> Option<&WorkspaceRole> {
        match self {
            Self::WorkspaceRole(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_workspace_role(self) -> Option<WorkspaceRole> {
        match self {
            Self::WorkspaceRole(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_static_workspace_roles(&self) -> Option<&StaticWorkspaceRoles> {
        match self {
            Self::StaticWorkspaceRoles(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_static_workspace_roles(self) -> Option<StaticWorkspaceRoles> {
        match self {
            Self::StaticWorkspaceRoles(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for MemberWorkspaceInfoRawRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WorkspaceRole(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::StaticWorkspaceRoles(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
