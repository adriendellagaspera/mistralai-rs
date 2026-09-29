pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AuthenticationConfiguration {
    pub authentication_type: OutboundAuthenticationType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(default)]
    pub name: String,
    pub scope: ConsumerType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CredentialsStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl AuthenticationConfiguration {
    pub fn builder() -> AuthenticationConfigurationBuilder {
        <AuthenticationConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthenticationConfigurationBuilder {
    authentication_type: Option<OutboundAuthenticationType>,
    is_default: Option<bool>,
    name: Option<String>,
    scope: Option<ConsumerType>,
    status: Option<CredentialsStatus>,
    title: Option<String>,
}

impl AuthenticationConfigurationBuilder {
    pub fn authentication_type(mut self, value: OutboundAuthenticationType) -> Self {
        self.authentication_type = Some(value);
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

    pub fn scope(mut self, value: ConsumerType) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn status(mut self, value: CredentialsStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AuthenticationConfiguration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`authentication_type`](AuthenticationConfigurationBuilder::authentication_type)
    /// - [`name`](AuthenticationConfigurationBuilder::name)
    /// - [`scope`](AuthenticationConfigurationBuilder::scope)
    pub fn build(self) -> Result<AuthenticationConfiguration, BuildError> {
        Ok(AuthenticationConfiguration {
            authentication_type: self
                .authentication_type
                .ok_or_else(|| BuildError::missing_field("authentication_type"))?,
            is_default: self.is_default,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            scope: self
                .scope
                .ok_or_else(|| BuildError::missing_field("scope"))?,
            status: self.status,
            title: self.title,
        })
    }
}
