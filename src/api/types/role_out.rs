pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RoleOut {
    /// Short description of the role.
    #[serde(default)]
    pub description: String,
    /// Whether this role is custom-defined.
    #[serde(default)]
    pub is_custom_role: bool,
    /// Role name.
    #[serde(default)]
    pub name: String,
    /// Role UUID.
    #[serde(default)]
    pub uuid: String,
}

impl RoleOut {
    pub fn builder() -> RoleOutBuilder {
        <RoleOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RoleOutBuilder {
    description: Option<String>,
    is_custom_role: Option<bool>,
    name: Option<String>,
    uuid: Option<String>,
}

impl RoleOutBuilder {
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

    /// Consumes the builder and constructs a [`RoleOut`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](RoleOutBuilder::description)
    /// - [`is_custom_role`](RoleOutBuilder::is_custom_role)
    /// - [`name`](RoleOutBuilder::name)
    /// - [`uuid`](RoleOutBuilder::uuid)
    pub fn build(self) -> Result<RoleOut, BuildError> {
        Ok(RoleOut {
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
