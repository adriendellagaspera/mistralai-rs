pub use crate::prelude::*;

/// A single workspace-role binding held by a service account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ServiceAccountRole {
    #[serde(default)]
    pub role_id: String,
}

impl ServiceAccountRole {
    pub fn builder() -> ServiceAccountRoleBuilder {
        <ServiceAccountRoleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ServiceAccountRoleBuilder {
    role_id: Option<String>,
}

impl ServiceAccountRoleBuilder {
    pub fn role_id(mut self, value: impl Into<String>) -> Self {
        self.role_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ServiceAccountRole`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role_id`](ServiceAccountRoleBuilder::role_id)
    pub fn build(self) -> Result<ServiceAccountRole, BuildError> {
        Ok(ServiceAccountRole {
            role_id: self
                .role_id
                .ok_or_else(|| BuildError::missing_field("role_id"))?,
        })
    }
}
