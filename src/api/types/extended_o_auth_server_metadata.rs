pub use crate::prelude::*;

/// Custom superset of RFC 8414 OAuth 2.0 Authorization Server Metadata.
///
/// Stored at connector creation time (provided for HTTP connectors, discovered via .well-known for MCP).
/// Mirrors the shape of .well-known/oauth-authorization-server responses.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExtendedOAuthServerMetadata {
    #[serde(default)]
    pub issuer: String,
    #[serde(default)]
    pub authorization_endpoint: String,
    #[serde(default)]
    pub token_endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_types_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_modes_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_types_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_methods_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_signing_alg_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_locales_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_policy_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_tos_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_endpoint_auth_methods_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_endpoint_auth_signing_alg_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introspection_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introspection_endpoint_auth_methods_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introspection_endpoint_auth_signing_alg_values_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_challenge_methods_supported: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id_metadata_document_supported: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_source: Option<OAuthMetadataSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_resource_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_scope: Option<String>,
}

impl ExtendedOAuthServerMetadata {
    pub fn builder() -> ExtendedOAuthServerMetadataBuilder {
        <ExtendedOAuthServerMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExtendedOAuthServerMetadataBuilder {
    issuer: Option<String>,
    authorization_endpoint: Option<String>,
    token_endpoint: Option<String>,
    registration_endpoint: Option<String>,
    scopes_supported: Option<Vec<String>>,
    response_types_supported: Option<Vec<String>>,
    response_modes_supported: Option<Vec<String>>,
    grant_types_supported: Option<Vec<String>>,
    token_endpoint_auth_methods_supported: Option<Vec<String>>,
    token_endpoint_auth_signing_alg_values_supported: Option<Vec<String>>,
    service_documentation: Option<String>,
    ui_locales_supported: Option<Vec<String>>,
    op_policy_uri: Option<String>,
    op_tos_uri: Option<String>,
    revocation_endpoint: Option<String>,
    revocation_endpoint_auth_methods_supported: Option<Vec<String>>,
    revocation_endpoint_auth_signing_alg_values_supported: Option<Vec<String>>,
    introspection_endpoint: Option<String>,
    introspection_endpoint_auth_methods_supported: Option<Vec<String>>,
    introspection_endpoint_auth_signing_alg_values_supported: Option<Vec<String>>,
    code_challenge_methods_supported: Option<Vec<String>>,
    client_id_metadata_document_supported: Option<bool>,
    x_source: Option<OAuthMetadataSource>,
    x_resource_url: Option<String>,
    x_scope: Option<String>,
}

impl ExtendedOAuthServerMetadataBuilder {
    pub fn issuer(mut self, value: impl Into<String>) -> Self {
        self.issuer = Some(value.into());
        self
    }

    pub fn authorization_endpoint(mut self, value: impl Into<String>) -> Self {
        self.authorization_endpoint = Some(value.into());
        self
    }

    pub fn token_endpoint(mut self, value: impl Into<String>) -> Self {
        self.token_endpoint = Some(value.into());
        self
    }

    pub fn registration_endpoint(mut self, value: impl Into<String>) -> Self {
        self.registration_endpoint = Some(value.into());
        self
    }

    pub fn scopes_supported(mut self, value: Vec<String>) -> Self {
        self.scopes_supported = Some(value);
        self
    }

    pub fn response_types_supported(mut self, value: Vec<String>) -> Self {
        self.response_types_supported = Some(value);
        self
    }

    pub fn response_modes_supported(mut self, value: Vec<String>) -> Self {
        self.response_modes_supported = Some(value);
        self
    }

    pub fn grant_types_supported(mut self, value: Vec<String>) -> Self {
        self.grant_types_supported = Some(value);
        self
    }

    pub fn token_endpoint_auth_methods_supported(mut self, value: Vec<String>) -> Self {
        self.token_endpoint_auth_methods_supported = Some(value);
        self
    }

    pub fn token_endpoint_auth_signing_alg_values_supported(mut self, value: Vec<String>) -> Self {
        self.token_endpoint_auth_signing_alg_values_supported = Some(value);
        self
    }

    pub fn service_documentation(mut self, value: impl Into<String>) -> Self {
        self.service_documentation = Some(value.into());
        self
    }

    pub fn ui_locales_supported(mut self, value: Vec<String>) -> Self {
        self.ui_locales_supported = Some(value);
        self
    }

    pub fn op_policy_uri(mut self, value: impl Into<String>) -> Self {
        self.op_policy_uri = Some(value.into());
        self
    }

    pub fn op_tos_uri(mut self, value: impl Into<String>) -> Self {
        self.op_tos_uri = Some(value.into());
        self
    }

    pub fn revocation_endpoint(mut self, value: impl Into<String>) -> Self {
        self.revocation_endpoint = Some(value.into());
        self
    }

    pub fn revocation_endpoint_auth_methods_supported(mut self, value: Vec<String>) -> Self {
        self.revocation_endpoint_auth_methods_supported = Some(value);
        self
    }

    pub fn revocation_endpoint_auth_signing_alg_values_supported(
        mut self,
        value: Vec<String>,
    ) -> Self {
        self.revocation_endpoint_auth_signing_alg_values_supported = Some(value);
        self
    }

    pub fn introspection_endpoint(mut self, value: impl Into<String>) -> Self {
        self.introspection_endpoint = Some(value.into());
        self
    }

    pub fn introspection_endpoint_auth_methods_supported(mut self, value: Vec<String>) -> Self {
        self.introspection_endpoint_auth_methods_supported = Some(value);
        self
    }

    pub fn introspection_endpoint_auth_signing_alg_values_supported(
        mut self,
        value: Vec<String>,
    ) -> Self {
        self.introspection_endpoint_auth_signing_alg_values_supported = Some(value);
        self
    }

    pub fn code_challenge_methods_supported(mut self, value: Vec<String>) -> Self {
        self.code_challenge_methods_supported = Some(value);
        self
    }

    pub fn client_id_metadata_document_supported(mut self, value: bool) -> Self {
        self.client_id_metadata_document_supported = Some(value);
        self
    }

    pub fn x_source(mut self, value: OAuthMetadataSource) -> Self {
        self.x_source = Some(value);
        self
    }

    pub fn x_resource_url(mut self, value: impl Into<String>) -> Self {
        self.x_resource_url = Some(value.into());
        self
    }

    pub fn x_scope(mut self, value: impl Into<String>) -> Self {
        self.x_scope = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExtendedOAuthServerMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`issuer`](ExtendedOAuthServerMetadataBuilder::issuer)
    /// - [`authorization_endpoint`](ExtendedOAuthServerMetadataBuilder::authorization_endpoint)
    /// - [`token_endpoint`](ExtendedOAuthServerMetadataBuilder::token_endpoint)
    pub fn build(self) -> Result<ExtendedOAuthServerMetadata, BuildError> {
        Ok(ExtendedOAuthServerMetadata {
            issuer: self
                .issuer
                .ok_or_else(|| BuildError::missing_field("issuer"))?,
            authorization_endpoint: self
                .authorization_endpoint
                .ok_or_else(|| BuildError::missing_field("authorization_endpoint"))?,
            token_endpoint: self
                .token_endpoint
                .ok_or_else(|| BuildError::missing_field("token_endpoint"))?,
            registration_endpoint: self.registration_endpoint,
            scopes_supported: self.scopes_supported,
            response_types_supported: self.response_types_supported,
            response_modes_supported: self.response_modes_supported,
            grant_types_supported: self.grant_types_supported,
            token_endpoint_auth_methods_supported: self.token_endpoint_auth_methods_supported,
            token_endpoint_auth_signing_alg_values_supported: self
                .token_endpoint_auth_signing_alg_values_supported,
            service_documentation: self.service_documentation,
            ui_locales_supported: self.ui_locales_supported,
            op_policy_uri: self.op_policy_uri,
            op_tos_uri: self.op_tos_uri,
            revocation_endpoint: self.revocation_endpoint,
            revocation_endpoint_auth_methods_supported: self
                .revocation_endpoint_auth_methods_supported,
            revocation_endpoint_auth_signing_alg_values_supported: self
                .revocation_endpoint_auth_signing_alg_values_supported,
            introspection_endpoint: self.introspection_endpoint,
            introspection_endpoint_auth_methods_supported: self
                .introspection_endpoint_auth_methods_supported,
            introspection_endpoint_auth_signing_alg_values_supported: self
                .introspection_endpoint_auth_signing_alg_values_supported,
            code_challenge_methods_supported: self.code_challenge_methods_supported,
            client_id_metadata_document_supported: self.client_id_metadata_document_supported,
            x_source: self.x_source,
            x_resource_url: self.x_resource_url,
            x_scope: self.x_scope,
        })
    }
}
