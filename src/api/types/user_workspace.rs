pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserWorkspace {
    /// The workspace's unique identifier.
    #[serde(default)]
    pub id: String,
    /// The workspace's display name.
    #[serde(default)]
    pub name: String,
    /// The identifier of the organization this workspace belongs to.
    #[serde(default)]
    pub organization_id: String,
}

impl UserWorkspace {
    pub fn builder() -> UserWorkspaceBuilder {
        <UserWorkspaceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserWorkspaceBuilder {
    id: Option<String>,
    name: Option<String>,
    organization_id: Option<String>,
}

impl UserWorkspaceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserWorkspace`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UserWorkspaceBuilder::id)
    /// - [`name`](UserWorkspaceBuilder::name)
    /// - [`organization_id`](UserWorkspaceBuilder::organization_id)
    pub fn build(self) -> Result<UserWorkspace, BuildError> {
        Ok(UserWorkspace {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            organization_id: self
                .organization_id
                .ok_or_else(|| BuildError::missing_field("organization_id"))?,
        })
    }
}
