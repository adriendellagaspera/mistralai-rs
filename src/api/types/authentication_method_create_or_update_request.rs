pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthenticationMethodCreateOrUpdateRequest {
    /// Whether the authentication method is for outbound or inbound requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_direction: Option<AuthDirection>,
    /// Connector-wide headers keyed by header name, applied to every credential. Secret values are encrypted at rest and never returned in clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_headers: Option<HashMap<String, GlobalHeaderValue>>,
    /// Set of headers to connect to the connector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<ConnectorAuthenticationHeader>>,
    /// The type of authentication method (e.g. oauth2, bearer, none).
    pub method_type: AuthenticationMethodCreateOrUpdateRequestMethodType,
    /// New OAuth2 client credentials (client_id and client_secret).
    #[serde(rename = "oauth2_metadata_secrets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2metadata_secrets: Option<Oauth2MetadataSecrets>,
    /// New OAuth2 authorization server metadata.
    #[serde(rename = "oauth2_server_metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
}

impl AuthenticationMethodCreateOrUpdateRequest {
    pub fn builder() -> AuthenticationMethodCreateOrUpdateRequestBuilder {
        <AuthenticationMethodCreateOrUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthenticationMethodCreateOrUpdateRequestBuilder {
    auth_direction: Option<AuthDirection>,
    global_headers: Option<HashMap<String, GlobalHeaderValue>>,
    headers: Option<Vec<ConnectorAuthenticationHeader>>,
    method_type: Option<AuthenticationMethodCreateOrUpdateRequestMethodType>,
    oauth2metadata_secrets: Option<Oauth2MetadataSecrets>,
    oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
}

impl AuthenticationMethodCreateOrUpdateRequestBuilder {
    pub fn auth_direction(mut self, value: AuthDirection) -> Self {
        self.auth_direction = Some(value);
        self
    }

    pub fn global_headers(mut self, value: HashMap<String, GlobalHeaderValue>) -> Self {
        self.global_headers = Some(value);
        self
    }

    pub fn headers(mut self, value: Vec<ConnectorAuthenticationHeader>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn method_type(
        mut self,
        value: AuthenticationMethodCreateOrUpdateRequestMethodType,
    ) -> Self {
        self.method_type = Some(value);
        self
    }

    pub fn oauth2metadata_secrets(mut self, value: Oauth2MetadataSecrets) -> Self {
        self.oauth2metadata_secrets = Some(value);
        self
    }

    pub fn oauth2server_metadata(mut self, value: ExtendedOAuthServerMetadata) -> Self {
        self.oauth2server_metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AuthenticationMethodCreateOrUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method_type`](AuthenticationMethodCreateOrUpdateRequestBuilder::method_type)
    pub fn build(self) -> Result<AuthenticationMethodCreateOrUpdateRequest, BuildError> {
        Ok(AuthenticationMethodCreateOrUpdateRequest {
            auth_direction: self.auth_direction,
            global_headers: self.global_headers,
            headers: self.headers,
            method_type: self
                .method_type
                .ok_or_else(|| BuildError::missing_field("method_type"))?,
            oauth2metadata_secrets: self.oauth2metadata_secrets,
            oauth2server_metadata: self.oauth2server_metadata,
        })
    }
}
