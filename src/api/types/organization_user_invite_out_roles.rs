pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum OrganizationUserInviteOutRoles {
    UserRoleList(Vec<UserRole>),

    StaticOrganizationRolesList(Vec<StaticOrganizationRoles>),
}

impl OrganizationUserInviteOutRoles {
    pub fn is_user_role_list(&self) -> bool {
        matches!(self, Self::UserRoleList(_))
    }

    pub fn is_static_organization_roles_list(&self) -> bool {
        matches!(self, Self::StaticOrganizationRolesList(_))
    }

    pub fn as_user_role_list(&self) -> Option<&Vec<UserRole>> {
        match self {
            Self::UserRoleList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_user_role_list(self) -> Option<Vec<UserRole>> {
        match self {
            Self::UserRoleList(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_static_organization_roles_list(&self) -> Option<&Vec<StaticOrganizationRoles>> {
        match self {
            Self::StaticOrganizationRolesList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_static_organization_roles_list(self) -> Option<Vec<StaticOrganizationRoles>> {
        match self {
            Self::StaticOrganizationRolesList(value) => Some(value),
            _ => None,
        }
    }
}
