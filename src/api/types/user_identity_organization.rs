pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserIdentityOrganization {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl UserIdentityOrganization {
    pub fn builder() -> UserIdentityOrganizationBuilder {
        <UserIdentityOrganizationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserIdentityOrganizationBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl UserIdentityOrganizationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserIdentityOrganization`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UserIdentityOrganizationBuilder::id)
    /// - [`name`](UserIdentityOrganizationBuilder::name)
    pub fn build(self) -> Result<UserIdentityOrganization, BuildError> {
        Ok(UserIdentityOrganization {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
