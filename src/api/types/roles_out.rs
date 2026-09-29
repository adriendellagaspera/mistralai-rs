pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RolesOut {
    /// Organization roles available to the Organization.
    #[serde(default)]
    pub organization_roles: Vec<RoleOut>,
    /// Workspace roles available to the Organization.
    #[serde(default)]
    pub workspace_roles: Vec<RoleOut>,
}

impl RolesOut {
    pub fn builder() -> RolesOutBuilder {
        <RolesOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RolesOutBuilder {
    organization_roles: Option<Vec<RoleOut>>,
    workspace_roles: Option<Vec<RoleOut>>,
}

impl RolesOutBuilder {
    pub fn organization_roles(mut self, value: Vec<RoleOut>) -> Self {
        self.organization_roles = Some(value);
        self
    }

    pub fn workspace_roles(mut self, value: Vec<RoleOut>) -> Self {
        self.workspace_roles = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RolesOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_roles`](RolesOutBuilder::organization_roles)
    /// - [`workspace_roles`](RolesOutBuilder::workspace_roles)
    pub fn build(self) -> Result<RolesOut, BuildError> {
        Ok(RolesOut {
            organization_roles: self
                .organization_roles
                .ok_or_else(|| BuildError::missing_field("organization_roles"))?,
            workspace_roles: self
                .workspace_roles
                .ok_or_else(|| BuildError::missing_field("workspace_roles"))?,
        })
    }
}
