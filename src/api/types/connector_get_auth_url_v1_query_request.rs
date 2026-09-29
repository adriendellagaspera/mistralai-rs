pub use crate::prelude::*;

/// Query parameters for connector_get_auth_url_v1
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectorGetAuthUrlV1QueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_return_url: Option<String>,
    /// Auth method type to use for the authorization URL. Required when the connector supports multiple interactive auth methods; otherwise the sole method is selected automatically. Use this to pick a specific method (e.g. 'oauth2' vs 'github_app').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_type: Option<OutboundAuthenticationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_title: Option<String>,
    /// Only valid with method_type=oauth2. When true, returns a GitHub App installation URL (https://github.com/apps/<slug>/installations/new) if the connector has the proper configuration The Github application needs to have 'Request user authorization (OAuth) during installation' enabled to perform the proper auth loop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_installation_link: Option<bool>,
}

impl ConnectorGetAuthUrlV1QueryRequest {
    pub fn builder() -> ConnectorGetAuthUrlV1QueryRequestBuilder {
        <ConnectorGetAuthUrlV1QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorGetAuthUrlV1QueryRequestBuilder {
    app_return_url: Option<String>,
    method_type: Option<OutboundAuthenticationType>,
    credentials_name: Option<String>,
    credentials_title: Option<String>,
    github_installation_link: Option<bool>,
}

impl ConnectorGetAuthUrlV1QueryRequestBuilder {
    pub fn app_return_url(mut self, value: impl Into<String>) -> Self {
        self.app_return_url = Some(value.into());
        self
    }

    pub fn method_type(mut self, value: OutboundAuthenticationType) -> Self {
        self.method_type = Some(value);
        self
    }

    pub fn credentials_name(mut self, value: impl Into<String>) -> Self {
        self.credentials_name = Some(value.into());
        self
    }

    pub fn credentials_title(mut self, value: impl Into<String>) -> Self {
        self.credentials_title = Some(value.into());
        self
    }

    pub fn github_installation_link(mut self, value: bool) -> Self {
        self.github_installation_link = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectorGetAuthUrlV1QueryRequest`].
    pub fn build(self) -> Result<ConnectorGetAuthUrlV1QueryRequest, BuildError> {
        Ok(ConnectorGetAuthUrlV1QueryRequest {
            app_return_url: self.app_return_url,
            method_type: self.method_type,
            credentials_name: self.credentials_name,
            credentials_title: self.credentials_title,
            github_installation_link: self.github_installation_link,
        })
    }
}
