pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RemoveWorkspaceMembersIn {
    /// Workspace members to remove.
    #[serde(default)]
    pub members: Vec<BaseWorkspaceMemberIn>,
}

impl RemoveWorkspaceMembersIn {
    pub fn builder() -> RemoveWorkspaceMembersInBuilder {
        <RemoveWorkspaceMembersInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemoveWorkspaceMembersInBuilder {
    members: Option<Vec<BaseWorkspaceMemberIn>>,
}

impl RemoveWorkspaceMembersInBuilder {
    pub fn members(mut self, value: Vec<BaseWorkspaceMemberIn>) -> Self {
        self.members = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RemoveWorkspaceMembersIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`members`](RemoveWorkspaceMembersInBuilder::members)
    pub fn build(self) -> Result<RemoveWorkspaceMembersIn, BuildError> {
        Ok(RemoveWorkspaceMembersIn {
            members: self
                .members
                .ok_or_else(|| BuildError::missing_field("members"))?,
        })
    }
}
