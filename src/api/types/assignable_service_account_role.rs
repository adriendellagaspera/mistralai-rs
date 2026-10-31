pub use crate::prelude::*;

/// A workspace role that may be assigned to a service account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssignableServiceAccountRole {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub is_custom_role: bool,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub uuid: String,
}

impl AssignableServiceAccountRole {
    pub fn builder() -> AssignableServiceAccountRoleBuilder {
        <AssignableServiceAccountRoleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssignableServiceAccountRoleBuilder {
    description: Option<String>,
    is_custom_role: Option<bool>,
    name: Option<String>,
    uuid: Option<String>,
}

impl AssignableServiceAccountRoleBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn is_custom_role(mut self, value: bool) -> Self {
        self.is_custom_role = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssignableServiceAccountRole`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](AssignableServiceAccountRoleBuilder::description)
    /// - [`is_custom_role`](AssignableServiceAccountRoleBuilder::is_custom_role)
    /// - [`name`](AssignableServiceAccountRoleBuilder::name)
    /// - [`uuid`](AssignableServiceAccountRoleBuilder::uuid)
    pub fn build(self) -> Result<AssignableServiceAccountRole, BuildError> {
        Ok(AssignableServiceAccountRole {
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            is_custom_role: self
                .is_custom_role
                .ok_or_else(|| BuildError::missing_field("is_custom_role"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
        })
    }
}
