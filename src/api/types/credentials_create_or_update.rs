pub use crate::prelude::*;

/// Request to create or update non-OAuth2 credentials for a connector.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CredentialsCreateOrUpdate {
    /// The credential data (headers, bearer_token).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<ConnectionCredentials>,
    /// Controls whether this credential is the default for its auth method. On creation: if no credential exists yet for this auth method, the credential is automatically set as default when is_default is true or omitted; setting is_default to false is rejected because a default must exist. If other credentials already exist, setting is_default to true promotes this credential (demoting the previous default); false or omitted creates it as non-default. On update: true promotes this credential, false is rejected if it is currently the default (promote another credential first), omitted leaves the default status unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Name of the credentials. Use this name to access or modify your credentials.
    #[serde(default)]
    pub name: String,
    /// Human-readable title for the credentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl CredentialsCreateOrUpdate {
    pub fn builder() -> CredentialsCreateOrUpdateBuilder {
        <CredentialsCreateOrUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CredentialsCreateOrUpdateBuilder {
    credentials: Option<ConnectionCredentials>,
    is_default: Option<bool>,
    name: Option<String>,
    title: Option<String>,
}

impl CredentialsCreateOrUpdateBuilder {
    pub fn credentials(mut self, value: ConnectionCredentials) -> Self {
        self.credentials = Some(value);
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CredentialsCreateOrUpdate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CredentialsCreateOrUpdateBuilder::name)
    pub fn build(self) -> Result<CredentialsCreateOrUpdate, BuildError> {
        Ok(CredentialsCreateOrUpdate {
            credentials: self.credentials,
            is_default: self.is_default,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            title: self.title,
        })
    }
}
