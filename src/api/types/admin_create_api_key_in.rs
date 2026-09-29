pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminCreateApiKeyIn {
    /// Optional name for the API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Date when the API key should expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<NaiveDate>,
    /// Workspace ID for the API key.
    #[serde(default)]
    pub workspace_uuid: String,
    /// ID of the user the API key is created for.
    #[serde(default)]
    pub user_id: String,
}

impl AdminCreateApiKeyIn {
    pub fn builder() -> AdminCreateApiKeyInBuilder {
        <AdminCreateApiKeyInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminCreateApiKeyInBuilder {
    name: Option<String>,
    expiration: Option<NaiveDate>,
    workspace_uuid: Option<String>,
    user_id: Option<String>,
}

impl AdminCreateApiKeyInBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn expiration(mut self, value: NaiveDate) -> Self {
        self.expiration = Some(value);
        self
    }

    pub fn workspace_uuid(mut self, value: impl Into<String>) -> Self {
        self.workspace_uuid = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdminCreateApiKeyIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workspace_uuid`](AdminCreateApiKeyInBuilder::workspace_uuid)
    /// - [`user_id`](AdminCreateApiKeyInBuilder::user_id)
    pub fn build(self) -> Result<AdminCreateApiKeyIn, BuildError> {
        Ok(AdminCreateApiKeyIn {
            name: self.name,
            expiration: self.expiration,
            workspace_uuid: self
                .workspace_uuid
                .ok_or_else(|| BuildError::missing_field("workspace_uuid"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
