pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminUpdateUserGroupIn {
    /// Updated description of the user group. Pass an empty string to clear it; omit the field or pass null to leave it unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Updated name of the user group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AdminUpdateUserGroupIn {
    pub fn builder() -> AdminUpdateUserGroupInBuilder {
        <AdminUpdateUserGroupInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminUpdateUserGroupInBuilder {
    description: Option<String>,
    name: Option<String>,
}

impl AdminUpdateUserGroupInBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdminUpdateUserGroupIn`].
    pub fn build(self) -> Result<AdminUpdateUserGroupIn, BuildError> {
        Ok(AdminUpdateUserGroupIn {
            description: self.description,
            name: self.name,
        })
    }
}
