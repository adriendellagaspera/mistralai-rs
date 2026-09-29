pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkspaceOut {
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
    /// Workspace spending limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_limit: Option<WorkspaceSpendLimitOut>,
    /// Workspace ID.
    #[serde(default)]
    pub uuid: String,
}

impl WorkspaceOut {
    pub fn builder() -> WorkspaceOutBuilder {
        <WorkspaceOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceOutBuilder {
    description: Option<String>,
    icon: Option<String>,
    is_default: Option<bool>,
    members_count: Option<i64>,
    name: Option<String>,
    spend_limit: Option<WorkspaceSpendLimitOut>,
    uuid: Option<String>,
}

impl WorkspaceOutBuilder {
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

    pub fn spend_limit(mut self, value: WorkspaceSpendLimitOut) -> Self {
        self.spend_limit = Some(value);
        self
    }

    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_default`](WorkspaceOutBuilder::is_default)
    /// - [`name`](WorkspaceOutBuilder::name)
    /// - [`uuid`](WorkspaceOutBuilder::uuid)
    pub fn build(self) -> Result<WorkspaceOut, BuildError> {
        Ok(WorkspaceOut {
            description: self.description,
            icon: self.icon,
            is_default: self
                .is_default
                .ok_or_else(|| BuildError::missing_field("is_default"))?,
            members_count: self.members_count,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            spend_limit: self.spend_limit,
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
        })
    }
}
