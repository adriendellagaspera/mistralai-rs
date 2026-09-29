pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserOrganization {
    /// The organization's unique identifier.
    #[serde(default)]
    pub id: String,
    /// The organization's display name.
    #[serde(default)]
    pub name: String,
}

impl UserOrganization {
    pub fn builder() -> UserOrganizationBuilder {
        <UserOrganizationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserOrganizationBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl UserOrganizationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserOrganization`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UserOrganizationBuilder::id)
    /// - [`name`](UserOrganizationBuilder::name)
    pub fn build(self) -> Result<UserOrganization, BuildError> {
        Ok(UserOrganization {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
