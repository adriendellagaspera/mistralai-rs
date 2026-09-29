pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminUserGroupOut {
    /// User group ID.
    #[serde(default)]
    pub uuid: String,
    /// Name of the user group.
    #[serde(default)]
    pub name: String,
    /// Optional description of the user group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the group is managed by an external system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub externally_managed: Option<bool>,
    /// Type of resources this group can access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<UserGroupTargetType>,
    /// Organization role assigned to the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_role: Option<String>,
    /// UUIDs of the groups this group is directly nested inside.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_ids: Option<Vec<String>>,
}

impl AdminUserGroupOut {
    pub fn builder() -> AdminUserGroupOutBuilder {
        <AdminUserGroupOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminUserGroupOutBuilder {
    uuid: Option<String>,
    name: Option<String>,
    description: Option<String>,
    externally_managed: Option<bool>,
    target_type: Option<UserGroupTargetType>,
    organization_role: Option<String>,
    parent_group_ids: Option<Vec<String>>,
}

impl AdminUserGroupOutBuilder {
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

    pub fn externally_managed(mut self, value: bool) -> Self {
        self.externally_managed = Some(value);
        self
    }

    pub fn target_type(mut self, value: UserGroupTargetType) -> Self {
        self.target_type = Some(value);
        self
    }

    pub fn organization_role(mut self, value: impl Into<String>) -> Self {
        self.organization_role = Some(value.into());
        self
    }

    pub fn parent_group_ids(mut self, value: Vec<String>) -> Self {
        self.parent_group_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdminUserGroupOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`uuid`](AdminUserGroupOutBuilder::uuid)
    /// - [`name`](AdminUserGroupOutBuilder::name)
    pub fn build(self) -> Result<AdminUserGroupOut, BuildError> {
        Ok(AdminUserGroupOut {
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            externally_managed: self.externally_managed,
            target_type: self.target_type,
            organization_role: self.organization_role,
            parent_group_ids: self.parent_group_ids,
        })
    }
}
