pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminUpdateUserGroupIn {
    /// Updated name of the user group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Updated description of the user group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated permission target type for this group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<UserGroupTargetType>,
    /// Full replacement list of parent group UUIDs. Pass an empty list to remove all parents; omit the field to leave the parent groups unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_ids: Option<Vec<String>>,
}

impl AdminUpdateUserGroupIn {
    pub fn builder() -> AdminUpdateUserGroupInBuilder {
        <AdminUpdateUserGroupInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminUpdateUserGroupInBuilder {
    name: Option<String>,
    description: Option<String>,
    target_type: Option<UserGroupTargetType>,
    parent_group_ids: Option<Vec<String>>,
}

impl AdminUpdateUserGroupInBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn target_type(mut self, value: UserGroupTargetType) -> Self {
        self.target_type = Some(value);
        self
    }

    pub fn parent_group_ids(mut self, value: Vec<String>) -> Self {
        self.parent_group_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdminUpdateUserGroupIn`].
    pub fn build(self) -> Result<AdminUpdateUserGroupIn, BuildError> {
        Ok(AdminUpdateUserGroupIn {
            name: self.name,
            description: self.description,
            target_type: self.target_type,
            parent_group_ids: self.parent_group_ids,
        })
    }
}
