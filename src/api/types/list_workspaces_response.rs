pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListWorkspacesResponse {
    /// The workspaces the authenticated user is a member of, each tagged with the organization it belongs to.
    #[serde(default)]
    pub workspaces: Vec<UserWorkspace>,
}

impl ListWorkspacesResponse {
    pub fn builder() -> ListWorkspacesResponseBuilder {
        <ListWorkspacesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWorkspacesResponseBuilder {
    workspaces: Option<Vec<UserWorkspace>>,
}

impl ListWorkspacesResponseBuilder {
    pub fn workspaces(mut self, value: Vec<UserWorkspace>) -> Self {
        self.workspaces = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListWorkspacesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workspaces`](ListWorkspacesResponseBuilder::workspaces)
    pub fn build(self) -> Result<ListWorkspacesResponse, BuildError> {
        Ok(ListWorkspacesResponse {
            workspaces: self
                .workspaces
                .ok_or_else(|| BuildError::missing_field("workspaces"))?,
        })
    }
}
