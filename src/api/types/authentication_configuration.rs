pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AuthenticationConfiguration {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub authentication_type: OutboundAuthenticationType,
    pub scope: ConsumerType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CredentialsStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

impl AuthenticationConfiguration {
    pub fn builder() -> AuthenticationConfigurationBuilder {
        <AuthenticationConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthenticationConfigurationBuilder {
    name: Option<String>,
    title: Option<String>,
    authentication_type: Option<OutboundAuthenticationType>,
    scope: Option<ConsumerType>,
    status: Option<CredentialsStatus>,
    is_default: Option<bool>,
}

impl AuthenticationConfigurationBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn authentication_type(mut self, value: OutboundAuthenticationType) -> Self {
        self.authentication_type = Some(value);
        self
    }

    pub fn scope(mut self, value: ConsumerType) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn status(mut self, value: CredentialsStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AuthenticationConfiguration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](AuthenticationConfigurationBuilder::name)
    /// - [`authentication_type`](AuthenticationConfigurationBuilder::authentication_type)
    /// - [`scope`](AuthenticationConfigurationBuilder::scope)
    pub fn build(self) -> Result<AuthenticationConfiguration, BuildError> {
        Ok(AuthenticationConfiguration {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            title: self.title,
            authentication_type: self
                .authentication_type
                .ok_or_else(|| BuildError::missing_field("authentication_type"))?,
            scope: self
                .scope
                .ok_or_else(|| BuildError::missing_field("scope"))?,
            status: self.status,
            is_default: self.is_default,
        })
    }
}
