pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminAssignUsersToGroupIn {
    /// User IDs to add to the group.
    #[serde(default)]
    pub user_uuids: Vec<String>,
}

impl AdminAssignUsersToGroupIn {
    pub fn builder() -> AdminAssignUsersToGroupInBuilder {
        <AdminAssignUsersToGroupInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminAssignUsersToGroupInBuilder {
    user_uuids: Option<Vec<String>>,
}

impl AdminAssignUsersToGroupInBuilder {
    pub fn user_uuids(mut self, value: Vec<String>) -> Self {
        self.user_uuids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdminAssignUsersToGroupIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_uuids`](AdminAssignUsersToGroupInBuilder::user_uuids)
    pub fn build(self) -> Result<AdminAssignUsersToGroupIn, BuildError> {
        Ok(AdminAssignUsersToGroupIn {
            user_uuids: self
                .user_uuids
                .ok_or_else(|| BuildError::missing_field("user_uuids"))?,
        })
    }
}
