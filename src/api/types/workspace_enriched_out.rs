pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkspaceEnrichedOut {
    /// Workspace description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Workspace icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Whether this is the default Workspace for the Organization.
    #[serde(default)]
    pub is_default: bool,
    /// Number of members in the Workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_count: Option<i64>,
    /// Workspace name.
    #[serde(default)]
    pub name: String,
    /// Deprecated single role for the Workspace. Use 'raw_roles' instead.
    pub raw_role: WorkspaceEnrichedOutRawRole,
    /// Roles granted for the Workspace.
    #[serde(default)]
    pub raw_roles: Vec<WorkspaceEnrichedOutRawRolesItem>,
    /// Workspace spending limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_limit: Option<WorkspaceSpendLimitOut>,
    /// Workspace ID.
    #[serde(default)]
    pub uuid: String,
}

impl WorkspaceEnrichedOut {
    pub fn builder() -> WorkspaceEnrichedOutBuilder {
        <WorkspaceEnrichedOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceEnrichedOutBuilder {
    description: Option<String>,
    icon: Option<String>,
    is_default: Option<bool>,
    members_count: Option<i64>,
    name: Option<String>,
    raw_role: Option<WorkspaceEnrichedOutRawRole>,
    raw_roles: Option<Vec<WorkspaceEnrichedOutRawRolesItem>>,
    spend_limit: Option<WorkspaceSpendLimitOut>,
    uuid: Option<String>,
}

impl WorkspaceEnrichedOutBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.icon = Some(value.into());
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn members_count(mut self, value: i64) -> Self {
        self.members_count = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn raw_role(mut self, value: WorkspaceEnrichedOutRawRole) -> Self {
        self.raw_role = Some(value);
        self
    }

    pub fn raw_roles(mut self, value: Vec<WorkspaceEnrichedOutRawRolesItem>) -> Self {
        self.raw_roles = Some(value);
        self
    }

    pub fn spend_limit(mut self, value: WorkspaceSpendLimitOut) -> Self {
        self.spend_limit = Some(value);
        self
    }

    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceEnrichedOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_default`](WorkspaceEnrichedOutBuilder::is_default)
    /// - [`name`](WorkspaceEnrichedOutBuilder::name)
    /// - [`raw_role`](WorkspaceEnrichedOutBuilder::raw_role)
    /// - [`raw_roles`](WorkspaceEnrichedOutBuilder::raw_roles)
    /// - [`uuid`](WorkspaceEnrichedOutBuilder::uuid)
    pub fn build(self) -> Result<WorkspaceEnrichedOut, BuildError> {
        Ok(WorkspaceEnrichedOut {
            description: self.description,
            icon: self.icon,
            is_default: self
                .is_default
                .ok_or_else(|| BuildError::missing_field("is_default"))?,
            members_count: self.members_count,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            raw_role: self
                .raw_role
                .ok_or_else(|| BuildError::missing_field("raw_role"))?,
            raw_roles: self
                .raw_roles
                .ok_or_else(|| BuildError::missing_field("raw_roles"))?,
            spend_limit: self.spend_limit,
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
        })
    }
}
