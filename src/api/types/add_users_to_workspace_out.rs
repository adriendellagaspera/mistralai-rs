pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddUsersToWorkspaceOut {
    /// Number of users added to the Workspace.
    #[serde(default)]
    pub added_members_count: i64,
}

impl AddUsersToWorkspaceOut {
    pub fn builder() -> AddUsersToWorkspaceOutBuilder {
        <AddUsersToWorkspaceOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddUsersToWorkspaceOutBuilder {
    added_members_count: Option<i64>,
}

impl AddUsersToWorkspaceOutBuilder {
    pub fn added_members_count(mut self, value: i64) -> Self {
        self.added_members_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddUsersToWorkspaceOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`added_members_count`](AddUsersToWorkspaceOutBuilder::added_members_count)
    pub fn build(self) -> Result<AddUsersToWorkspaceOut, BuildError> {
        Ok(AddUsersToWorkspaceOut {
            added_members_count: self
                .added_members_count
                .ok_or_else(|| BuildError::missing_field("added_members_count"))?,
        })
    }
}
