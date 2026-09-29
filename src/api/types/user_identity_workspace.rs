pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserIdentityWorkspace {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl UserIdentityWorkspace {
    pub fn builder() -> UserIdentityWorkspaceBuilder {
        <UserIdentityWorkspaceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserIdentityWorkspaceBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl UserIdentityWorkspaceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserIdentityWorkspace`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UserIdentityWorkspaceBuilder::id)
    /// - [`name`](UserIdentityWorkspaceBuilder::name)
    pub fn build(self) -> Result<UserIdentityWorkspace, BuildError> {
        Ok(UserIdentityWorkspace {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
