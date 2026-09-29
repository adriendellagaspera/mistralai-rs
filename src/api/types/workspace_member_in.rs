pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceMemberIn {
    /// Workspace members to add or update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<WorkspaceMemberSingleIn>>,
}

impl WorkspaceMemberIn {
    pub fn builder() -> WorkspaceMemberInBuilder {
        <WorkspaceMemberInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceMemberInBuilder {
    members: Option<Vec<WorkspaceMemberSingleIn>>,
}

impl WorkspaceMemberInBuilder {
    pub fn members(mut self, value: Vec<WorkspaceMemberSingleIn>) -> Self {
        self.members = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceMemberIn`].
    pub fn build(self) -> Result<WorkspaceMemberIn, BuildError> {
        Ok(WorkspaceMemberIn {
            members: self.members,
        })
    }
}
