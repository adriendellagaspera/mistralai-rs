pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OAuth2ClientCredentialsInput {
    /// PEM X.509 certificate registered on the OAuth2 client, for private_key_jwt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate: Option<String>,
    #[serde(default)]
    pub client_id: String,
    /// Unencrypted PEM RSA private key matching client_certificate, for private_key_jwt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_private_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub grant_type: OAuth2ClientCredentialsInputGrantType,
}

impl OAuth2ClientCredentialsInput {
    pub fn builder() -> OAuth2ClientCredentialsInputBuilder {
        <OAuth2ClientCredentialsInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OAuth2ClientCredentialsInputBuilder {
    client_certificate: Option<String>,
    client_id: Option<String>,
    client_private_key: Option<String>,
    client_secret: Option<String>,
    grant_type: Option<OAuth2ClientCredentialsInputGrantType>,
}

impl OAuth2ClientCredentialsInputBuilder {
    pub fn client_certificate(mut self, value: impl Into<String>) -> Self {
        self.client_certificate = Some(value.into());
        self
    }

    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn client_private_key(mut self, value: impl Into<String>) -> Self {
        self.client_private_key = Some(value.into());
        self
    }

    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = Some(value.into());
        self
    }

    pub fn grant_type(mut self, value: OAuth2ClientCredentialsInputGrantType) -> Self {
        self.grant_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OAuth2ClientCredentialsInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_id`](OAuth2ClientCredentialsInputBuilder::client_id)
    /// - [`grant_type`](OAuth2ClientCredentialsInputBuilder::grant_type)
    pub fn build(self) -> Result<OAuth2ClientCredentialsInput, BuildError> {
        Ok(OAuth2ClientCredentialsInput {
            client_certificate: self.client_certificate,
            client_id: self
                .client_id
                .ok_or_else(|| BuildError::missing_field("client_id"))?,
            client_private_key: self.client_private_key,
            client_secret: self.client_secret,
            grant_type: self
                .grant_type
                .ok_or_else(|| BuildError::missing_field("grant_type"))?,
        })
    }
}
