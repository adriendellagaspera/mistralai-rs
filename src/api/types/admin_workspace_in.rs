pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminWorkspaceIn {
    /// Whether to add all Organization members to the Workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_all_org_members: Option<bool>,
    /// User ID to grant the Workspace Admin role to.
    #[serde(default)]
    pub admin_user_id: String,
    /// Workspace description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Workspace icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Workspace name.
    #[serde(default)]
    pub name: String,
}

impl AdminWorkspaceIn {
    pub fn builder() -> AdminWorkspaceInBuilder {
        <AdminWorkspaceInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminWorkspaceInBuilder {
    add_all_org_members: Option<bool>,
    admin_user_id: Option<String>,
    description: Option<String>,
    icon: Option<String>,
    name: Option<String>,
}

impl AdminWorkspaceInBuilder {
    pub fn add_all_org_members(mut self, value: bool) -> Self {
        self.add_all_org_members = Some(value);
        self
    }

    pub fn admin_user_id(mut self, value: impl Into<String>) -> Self {
        self.admin_user_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.icon = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdminWorkspaceIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`admin_user_id`](AdminWorkspaceInBuilder::admin_user_id)
    /// - [`name`](AdminWorkspaceInBuilder::name)
    pub fn build(self) -> Result<AdminWorkspaceIn, BuildError> {
        Ok(AdminWorkspaceIn {
            add_all_org_members: self.add_all_org_members,
            admin_user_id: self
                .admin_user_id
                .ok_or_else(|| BuildError::missing_field("admin_user_id"))?,
            description: self.description,
            icon: self.icon,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
