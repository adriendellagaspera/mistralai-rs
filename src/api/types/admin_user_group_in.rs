pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminUserGroupIn {
    /// Optional description of the user group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Name of the user group.
    #[serde(default)]
    pub name: String,
    /// UUIDs of existing groups to nest this new group inside. Omit or pass an empty list to create a top-level group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_ids: Option<Vec<String>>,
    /// Type of resources this group can access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<UserGroupTargetType>,
}

impl AdminUserGroupIn {
    pub fn builder() -> AdminUserGroupInBuilder {
        <AdminUserGroupInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminUserGroupInBuilder {
    description: Option<String>,
    name: Option<String>,
    parent_group_ids: Option<Vec<String>>,
    target_type: Option<UserGroupTargetType>,
}

impl AdminUserGroupInBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    /// Consumes the builder and constructs a [`AdminUserGroupIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AdminUserGroupInBuilder::name)
    pub fn build(self) -> Result<AdminUserGroupIn, BuildError> {
        Ok(AdminUserGroupIn {
            description: self.description,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            parent_group_ids: self.parent_group_ids,
            target_type: self.target_type,
        })
    }
}
