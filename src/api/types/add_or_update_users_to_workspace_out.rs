pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddOrUpdateUsersToWorkspaceOut {
    /// Number of users added to the Workspace.
    #[serde(default)]
    pub added_members_count: i64,
    /// Number of Workspace members updated.
    #[serde(default)]
    pub updated_members_count: i64,
}

impl AddOrUpdateUsersToWorkspaceOut {
    pub fn builder() -> AddOrUpdateUsersToWorkspaceOutBuilder {
        <AddOrUpdateUsersToWorkspaceOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddOrUpdateUsersToWorkspaceOutBuilder {
    added_members_count: Option<i64>,
    updated_members_count: Option<i64>,
}

impl AddOrUpdateUsersToWorkspaceOutBuilder {
    pub fn added_members_count(mut self, value: i64) -> Self {
        self.added_members_count = Some(value);
        self
    }

    pub fn updated_members_count(mut self, value: i64) -> Self {
        self.updated_members_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddOrUpdateUsersToWorkspaceOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`added_members_count`](AddOrUpdateUsersToWorkspaceOutBuilder::added_members_count)
    /// - [`updated_members_count`](AddOrUpdateUsersToWorkspaceOutBuilder::updated_members_count)
    pub fn build(self) -> Result<AddOrUpdateUsersToWorkspaceOut, BuildError> {
        Ok(AddOrUpdateUsersToWorkspaceOut {
            added_members_count: self
                .added_members_count
                .ok_or_else(|| BuildError::missing_field("added_members_count"))?,
            updated_members_count: self
                .updated_members_count
                .ok_or_else(|| BuildError::missing_field("updated_members_count"))?,
        })
    }
}
