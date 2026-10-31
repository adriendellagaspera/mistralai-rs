pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminWorkspaceMemberIn {
    /// Workspace members to add or update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<AdminWorkspaceMemberSingleIn>>,
}

impl AdminWorkspaceMemberIn {
    pub fn builder() -> AdminWorkspaceMemberInBuilder {
        <AdminWorkspaceMemberInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminWorkspaceMemberInBuilder {
    members: Option<Vec<AdminWorkspaceMemberSingleIn>>,
}

impl AdminWorkspaceMemberInBuilder {
    pub fn members(mut self, value: Vec<AdminWorkspaceMemberSingleIn>) -> Self {
        self.members = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdminWorkspaceMemberIn`].
    pub fn build(self) -> Result<AdminWorkspaceMemberIn, BuildError> {
        Ok(AdminWorkspaceMemberIn {
            members: self.members,
        })
    }
}
