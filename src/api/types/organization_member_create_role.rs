pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum OrganizationMemberCreateRole {
    UserRole(UserRole),

    StaticOrganizationRoles(StaticOrganizationRoles),
}

impl OrganizationMemberCreateRole {
    pub fn is_user_role(&self) -> bool {
        matches!(self, Self::UserRole(_))
    }

    pub fn is_static_organization_roles(&self) -> bool {
        matches!(self, Self::StaticOrganizationRoles(_))
    }

    pub fn as_user_role(&self) -> Option<&UserRole> {
        match self {
            Self::UserRole(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_user_role(self) -> Option<UserRole> {
        match self {
            Self::UserRole(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_static_organization_roles(&self) -> Option<&StaticOrganizationRoles> {
        match self {
            Self::StaticOrganizationRoles(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_static_organization_roles(self) -> Option<StaticOrganizationRoles> {
        match self {
            Self::StaticOrganizationRoles(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for OrganizationMemberCreateRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserRole(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::StaticOrganizationRoles(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
