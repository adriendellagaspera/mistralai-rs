pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RemoveWorkspaceMembersOut {
    /// Number of Workspace members removed.
    #[serde(default)]
    pub deleted_members_count: i64,
    /// Users that could not be removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_deleted_members: Option<Vec<String>>,
}

impl RemoveWorkspaceMembersOut {
    pub fn builder() -> RemoveWorkspaceMembersOutBuilder {
        <RemoveWorkspaceMembersOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemoveWorkspaceMembersOutBuilder {
    deleted_members_count: Option<i64>,
    not_deleted_members: Option<Vec<String>>,
}

impl RemoveWorkspaceMembersOutBuilder {
    pub fn deleted_members_count(mut self, value: i64) -> Self {
        self.deleted_members_count = Some(value);
        self
    }

    pub fn not_deleted_members(mut self, value: Vec<String>) -> Self {
        self.not_deleted_members = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RemoveWorkspaceMembersOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted_members_count`](RemoveWorkspaceMembersOutBuilder::deleted_members_count)
    pub fn build(self) -> Result<RemoveWorkspaceMembersOut, BuildError> {
        Ok(RemoveWorkspaceMembersOut {
            deleted_members_count: self
                .deleted_members_count
                .ok_or_else(|| BuildError::missing_field("deleted_members_count"))?,
            not_deleted_members: self.not_deleted_members,
        })
    }
}
