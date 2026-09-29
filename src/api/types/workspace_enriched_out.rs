pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkspaceEnrichedOut {
    /// Workspace ID.
    #[serde(default)]
    pub uuid: String,
    /// Workspace name.
    #[serde(default)]
    pub name: String,
    /// Workspace description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Workspace icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Number of members in the Workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_count: Option<i64>,
    /// Workspace spending limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_limit: Option<WorkspaceSpendLimitOut>,
    /// Whether this is the default Workspace for the Organization.
    #[serde(default)]
    pub is_default: bool,
    /// Roles granted for the Workspace.
    #[serde(default)]
    pub raw_roles: Vec<WorkspaceEnrichedOutRawRolesItem>,
    /// Deprecated single role for the Workspace. Use 'raw_roles' instead.
    pub raw_role: WorkspaceEnrichedOutRawRole,
}

impl WorkspaceEnrichedOut {
    pub fn builder() -> WorkspaceEnrichedOutBuilder {
        <WorkspaceEnrichedOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceEnrichedOutBuilder {
    uuid: Option<String>,
    name: Option<String>,
    description: Option<String>,
    icon: Option<String>,
    members_count: Option<i64>,
    spend_limit: Option<WorkspaceSpendLimitOut>,
    is_default: Option<bool>,
    raw_roles: Option<Vec<WorkspaceEnrichedOutRawRolesItem>>,
    raw_role: Option<WorkspaceEnrichedOutRawRole>,
}

impl WorkspaceEnrichedOutBuilder {
    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    pub fn members_count(mut self, value: i64) -> Self {
        self.members_count = Some(value);
        self
    }

    pub fn spend_limit(mut self, value: WorkspaceSpendLimitOut) -> Self {
        self.spend_limit = Some(value);
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn raw_roles(mut self, value: Vec<WorkspaceEnrichedOutRawRolesItem>) -> Self {
        self.raw_roles = Some(value);
        self
    }

    pub fn raw_role(mut self, value: WorkspaceEnrichedOutRawRole) -> Self {
        self.raw_role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceEnrichedOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`uuid`](WorkspaceEnrichedOutBuilder::uuid)
    /// - [`name`](WorkspaceEnrichedOutBuilder::name)
    /// - [`is_default`](WorkspaceEnrichedOutBuilder::is_default)
    /// - [`raw_roles`](WorkspaceEnrichedOutBuilder::raw_roles)
    /// - [`raw_role`](WorkspaceEnrichedOutBuilder::raw_role)
    pub fn build(self) -> Result<WorkspaceEnrichedOut, BuildError> {
        Ok(WorkspaceEnrichedOut {
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            icon: self.icon,
            members_count: self.members_count,
            spend_limit: self.spend_limit,
            is_default: self
                .is_default
                .ok_or_else(|| BuildError::missing_field("is_default"))?,
            raw_roles: self
                .raw_roles
                .ok_or_else(|| BuildError::missing_field("raw_roles"))?,
            raw_role: self
                .raw_role
                .ok_or_else(|| BuildError::missing_field("raw_role"))?,
        })
    }
}
