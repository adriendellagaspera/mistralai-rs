pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkspaceOut {
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
}

impl WorkspaceOut {
    pub fn builder() -> WorkspaceOutBuilder {
        <WorkspaceOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceOutBuilder {
    uuid: Option<String>,
    name: Option<String>,
    description: Option<String>,
    icon: Option<String>,
    members_count: Option<i64>,
    spend_limit: Option<WorkspaceSpendLimitOut>,
    is_default: Option<bool>,
}

impl WorkspaceOutBuilder {
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

    /// Consumes the builder and constructs a [`WorkspaceOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`uuid`](WorkspaceOutBuilder::uuid)
    /// - [`name`](WorkspaceOutBuilder::name)
    /// - [`is_default`](WorkspaceOutBuilder::is_default)
    pub fn build(self) -> Result<WorkspaceOut, BuildError> {
        Ok(WorkspaceOut {
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            icon: self.icon,
            members_count: self.members_count,
            spend_limit: self.spend_limit,
            is_default: self
                .is_default
                .ok_or_else(|| BuildError::missing_field("is_default"))?,
        })
    }
}
