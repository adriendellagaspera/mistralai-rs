pub use crate::prelude::*;

/// Public view of an authentication method, without secrets.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicAuthenticationMethod {
    pub method_type: OutboundAuthenticationType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<ConnectorAuthenticationHeader>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_headers: Option<HashMap<String, GlobalHeaderValue>>,
    #[serde(default)]
    pub has_default_credentials: bool,
    #[serde(rename = "oauth2_server_metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
}

impl PublicAuthenticationMethod {
    pub fn builder() -> PublicAuthenticationMethodBuilder {
        <PublicAuthenticationMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicAuthenticationMethodBuilder {
    method_type: Option<OutboundAuthenticationType>,
    headers: Option<Vec<ConnectorAuthenticationHeader>>,
    global_headers: Option<HashMap<String, GlobalHeaderValue>>,
    has_default_credentials: Option<bool>,
    oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
}

impl PublicAuthenticationMethodBuilder {
    pub fn method_type(mut self, value: OutboundAuthenticationType) -> Self {
        self.method_type = Some(value);
        self
    }

    pub fn headers(mut self, value: Vec<ConnectorAuthenticationHeader>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn global_headers(mut self, value: HashMap<String, GlobalHeaderValue>) -> Self {
        self.global_headers = Some(value);
        self
    }

    pub fn has_default_credentials(mut self, value: bool) -> Self {
        self.has_default_credentials = Some(value);
        self
    }

    pub fn oauth2server_metadata(mut self, value: ExtendedOAuthServerMetadata) -> Self {
        self.oauth2server_metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicAuthenticationMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method_type`](PublicAuthenticationMethodBuilder::method_type)
    /// - [`has_default_credentials`](PublicAuthenticationMethodBuilder::has_default_credentials)
    pub fn build(self) -> Result<PublicAuthenticationMethod, BuildError> {
        Ok(PublicAuthenticationMethod {
            method_type: self
                .method_type
                .ok_or_else(|| BuildError::missing_field("method_type"))?,
            headers: self.headers,
            global_headers: self.global_headers,
            has_default_credentials: self
                .has_default_credentials
                .ok_or_else(|| BuildError::missing_field("has_default_credentials"))?,
            oauth2server_metadata: self.oauth2server_metadata,
        })
    }
}
