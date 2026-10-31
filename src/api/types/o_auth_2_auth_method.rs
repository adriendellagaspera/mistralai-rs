pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "grant_type")]
#[non_exhaustive]
pub enum OAuth2AuthMethod {
    #[serde(rename = "authorization_code")]
    #[non_exhaustive]
    AuthorizationCode {
        #[serde(default)]
        auth_data: AuthData,
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<Vec<ConnectorAuthenticationHeader>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        method_type: Option<OAuth2AuthorizationCodeAuthMethodMethodType>,
        #[serde(rename = "oauth2_server_metadata")]
        #[serde(skip_serializing_if = "Option::is_none")]
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
    },

    #[serde(rename = "client_credentials")]
    #[non_exhaustive]
    ClientCredentials {
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<Vec<ConnectorAuthenticationHeader>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        method_type: Option<OAuth2ClientCredentialsAuthMethodMethodType>,
        #[serde(rename = "oauth2_server_metadata")]
        #[serde(default)]
        oauth2server_metadata: ExtendedOAuthServerMetadata,
        token_endpoint_auth_method: OAuth2TokenEndpointAuthMethod,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl OAuth2AuthMethod {
    pub fn authorization_code(auth_data: AuthData) -> Self {
        Self::AuthorizationCode {
            auth_data,
            headers: None,
            method_type: None,
            oauth2server_metadata: None,
        }
    }

    pub fn client_credentials(
        oauth2server_metadata: ExtendedOAuthServerMetadata,
        token_endpoint_auth_method: OAuth2TokenEndpointAuthMethod,
    ) -> Self {
        Self::ClientCredentials {
            headers: None,
            method_type: None,
            oauth2server_metadata,
            token_endpoint_auth_method,
        }
    }

    pub fn authorization_code_with_headers(
        auth_data: AuthData,
        headers: Vec<ConnectorAuthenticationHeader>,
        method_type: Option<OAuth2AuthorizationCodeAuthMethodMethodType>,
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
    ) -> Self {
        Self::AuthorizationCode {
            auth_data,
            headers: Some(headers),
            method_type,
            oauth2server_metadata,
        }
    }

    pub fn authorization_code_with_method_type(
        auth_data: AuthData,
        headers: Option<Vec<ConnectorAuthenticationHeader>>,
        method_type: OAuth2AuthorizationCodeAuthMethodMethodType,
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
    ) -> Self {
        Self::AuthorizationCode {
            auth_data,
            headers,
            method_type: Some(method_type),
            oauth2server_metadata,
        }
    }

    pub fn authorization_code_with_oauth2server_metadata(
        auth_data: AuthData,
        headers: Option<Vec<ConnectorAuthenticationHeader>>,
        method_type: Option<OAuth2AuthorizationCodeAuthMethodMethodType>,
        oauth2server_metadata: ExtendedOAuthServerMetadata,
    ) -> Self {
        Self::AuthorizationCode {
            auth_data,
            headers,
            method_type,
            oauth2server_metadata: Some(oauth2server_metadata),
        }
    }

    pub fn client_credentials_with_headers(
        headers: Vec<ConnectorAuthenticationHeader>,
        method_type: Option<OAuth2ClientCredentialsAuthMethodMethodType>,
        oauth2server_metadata: ExtendedOAuthServerMetadata,
        token_endpoint_auth_method: OAuth2TokenEndpointAuthMethod,
    ) -> Self {
        Self::ClientCredentials {
            headers: Some(headers),
            method_type,
            oauth2server_metadata,
            token_endpoint_auth_method,
        }
    }

    pub fn client_credentials_with_method_type(
        headers: Option<Vec<ConnectorAuthenticationHeader>>,
        method_type: OAuth2ClientCredentialsAuthMethodMethodType,
        oauth2server_metadata: ExtendedOAuthServerMetadata,
        token_endpoint_auth_method: OAuth2TokenEndpointAuthMethod,
    ) -> Self {
        Self::ClientCredentials {
            headers,
            method_type: Some(method_type),
            oauth2server_metadata,
            token_endpoint_auth_method,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
