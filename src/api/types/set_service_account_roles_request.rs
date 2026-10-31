pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetServiceAccountRolesRequest {
    /// Workspace role UUIDs to assign. Required and non-empty; omitted or empty lists return 422. Select member explicitly for no role-granted permissions; workspace-wide sharing may still grant access.
    #[serde(default)]
    pub role_ids: Vec<String>,
}

impl SetServiceAccountRolesRequest {
    pub fn builder() -> SetServiceAccountRolesRequestBuilder {
        <SetServiceAccountRolesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetServiceAccountRolesRequestBuilder {
    role_ids: Option<Vec<String>>,
}

impl SetServiceAccountRolesRequestBuilder {
    pub fn role_ids(mut self, value: Vec<String>) -> Self {
        self.role_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetServiceAccountRolesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role_ids`](SetServiceAccountRolesRequestBuilder::role_ids)
    pub fn build(self) -> Result<SetServiceAccountRolesRequest, BuildError> {
        Ok(SetServiceAccountRolesRequest {
            role_ids: self
                .role_ids
                .ok_or_else(|| BuildError::missing_field("role_ids"))?,
        })
    }
}
