pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminUserGroupOut {
    /// Optional description of the user group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the group is managed by an external system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub externally_managed: Option<bool>,
    /// Name of the user group.
    #[serde(default)]
    pub name: String,
    /// Organization role assigned to the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_role: Option<String>,
    /// UUIDs of the groups this group is directly nested inside.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_ids: Option<Vec<String>>,
    /// Type of resources this group can access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<UserGroupTargetType>,
    /// User group ID.
    #[serde(default)]
    pub uuid: String,
}

impl AdminUserGroupOut {
    pub fn builder() -> AdminUserGroupOutBuilder {
        <AdminUserGroupOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminUserGroupOutBuilder {
    description: Option<String>,
    externally_managed: Option<bool>,
    name: Option<String>,
    organization_role: Option<String>,
    parent_group_ids: Option<Vec<String>>,
    target_type: Option<UserGroupTargetType>,
    uuid: Option<String>,
}

impl AdminUserGroupOutBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn externally_managed(mut self, value: bool) -> Self {
        self.externally_managed = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    pub fn target_type(mut self, value: UserGroupTargetType) -> Self {
        self.target_type = Some(value);
        self
    }

    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdminUserGroupOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AdminUserGroupOutBuilder::name)
    /// - [`uuid`](AdminUserGroupOutBuilder::uuid)
    pub fn build(self) -> Result<AdminUserGroupOut, BuildError> {
        Ok(AdminUserGroupOut {
            description: self.description,
            externally_managed: self.externally_managed,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            organization_role: self.organization_role,
            parent_group_ids: self.parent_group_ids,
            target_type: self.target_type,
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
        })
    }
}
