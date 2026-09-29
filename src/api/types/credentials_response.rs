pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CredentialsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_preset_credentials_for_auth: Option<Vec<OutboundAuthenticationType>>,
    #[serde(default)]
    pub credentials: Vec<AuthenticationConfiguration>,
}

impl CredentialsResponse {
    pub fn builder() -> CredentialsResponseBuilder {
        <CredentialsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CredentialsResponseBuilder {
    connector_preset_credentials_for_auth: Option<Vec<OutboundAuthenticationType>>,
    credentials: Option<Vec<AuthenticationConfiguration>>,
}

impl CredentialsResponseBuilder {
    pub fn connector_preset_credentials_for_auth(
        mut self,
        value: Vec<OutboundAuthenticationType>,
    ) -> Self {
        self.connector_preset_credentials_for_auth = Some(value);
        self
    }

    pub fn credentials(mut self, value: Vec<AuthenticationConfiguration>) -> Self {
        self.credentials = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CredentialsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`credentials`](CredentialsResponseBuilder::credentials)
    pub fn build(self) -> Result<CredentialsResponse, BuildError> {
        Ok(CredentialsResponse {
            connector_preset_credentials_for_auth: self.connector_preset_credentials_for_auth,
            credentials: self
                .credentials
                .ok_or_else(|| BuildError::missing_field("credentials"))?,
        })
    }
}
