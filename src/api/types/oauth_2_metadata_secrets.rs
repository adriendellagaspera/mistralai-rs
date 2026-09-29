pub use crate::prelude::*;

/// OAuth2 client credentials stored alongside a connector's authentication method.
///
/// Used by OAuth2 and Slack App auth types for token exchange and refresh flows.
/// Contains the client credentials obtained during OAuth2 Dynamic Client Registration
/// or provided at connector creation time.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Oauth2MetadataSecrets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id_issued_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret_expires_at: Option<i64>,
}

impl Oauth2MetadataSecrets {
    pub fn builder() -> Oauth2MetadataSecretsBuilder {
        <Oauth2MetadataSecretsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Oauth2MetadataSecretsBuilder {
    client_id: Option<String>,
    client_secret: Option<String>,
    client_id_issued_at: Option<i64>,
    client_secret_expires_at: Option<i64>,
}

impl Oauth2MetadataSecretsBuilder {
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = Some(value.into());
        self
    }

    pub fn client_id_issued_at(mut self, value: i64) -> Self {
        self.client_id_issued_at = Some(value);
        self
    }

    pub fn client_secret_expires_at(mut self, value: i64) -> Self {
        self.client_secret_expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Oauth2MetadataSecrets`].
    pub fn build(self) -> Result<Oauth2MetadataSecrets, BuildError> {
        Ok(Oauth2MetadataSecrets {
            client_id: self.client_id,
            client_secret: self.client_secret,
            client_id_issued_at: self.client_id_issued_at,
            client_secret_expires_at: self.client_secret_expires_at,
        })
    }
}
