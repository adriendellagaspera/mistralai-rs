pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AuthData {
    #[serde(default)]
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}

impl AuthData {
    pub fn builder() -> AuthDataBuilder {
        <AuthDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthDataBuilder {
    client_id: Option<String>,
    client_secret: Option<String>,
}

impl AuthDataBuilder {
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AuthData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_id`](AuthDataBuilder::client_id)
    pub fn build(self) -> Result<AuthData, BuildError> {
        Ok(AuthData {
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            client_secret: self.client_secret,
        })
    }
}
