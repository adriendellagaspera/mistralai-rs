pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "protocol")]
#[non_exhaustive]
pub enum ConnectorCreateV1ConnectorsRequest {
    #[serde(rename = "http")]
    #[non_exhaustive]
    Http {
        #[serde(skip_serializing_if = "Option::is_none")]
        auth_methods: Option<Vec<CreateHttpConnectorRequestAuthMethodsItem>>,
        #[serde(default)]
        description: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        global_headers: Option<HashMap<String, GlobalHeaderValue>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<HashMap<String, serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        icon_url: Option<String>,
        #[serde(default)]
        name: String,
        #[serde(rename = "oauth2_server_metadata")]
        #[serde(skip_serializing_if = "Option::is_none")]
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
        #[serde(rename = "oauth2_server_metadata_url")]
        #[serde(skip_serializing_if = "Option::is_none")]
        oauth2server_metadata_url: Option<String>,
        #[serde(default)]
        server: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        system_prompt: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        visibility: Option<PublicResourceVisibility>,
    },

    #[serde(rename = "mcp")]
    #[non_exhaustive]
    Mcp {
        #[serde(skip_serializing_if = "Option::is_none")]
        auth_methods: Option<Vec<CreateConnectorRequestAuthMethodsItem>>,
        #[serde(default)]
        description: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        global_headers: Option<HashMap<String, GlobalHeaderValue>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<HashMap<String, serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        icon_url: Option<String>,
        #[serde(default)]
        name: String,
        #[serde(rename = "oauth2_server_metadata")]
        #[serde(skip_serializing_if = "Option::is_none")]
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
        #[serde(rename = "oauth2_server_metadata_url")]
        #[serde(skip_serializing_if = "Option::is_none")]
        oauth2server_metadata_url: Option<String>,
        #[serde(default)]
        server: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        system_prompt: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        visibility: Option<PublicResourceVisibility>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ConnectorCreateV1ConnectorsRequest {
    pub fn http(description: String, name: String, server: String) -> Self {
        Self::Http {
            auth_methods: None,
            description,
            global_headers: None,
            headers: None,
            icon_url: None,
            name,
            oauth2server_metadata: None,
            oauth2server_metadata_url: None,
            server,
            system_prompt: None,
            title: None,
            visibility: None,
        }
    }

    pub fn mcp(description: String, name: String, server: String) -> Self {
        Self::Mcp {
            auth_methods: None,
            description,
            global_headers: None,
            headers: None,
            icon_url: None,
            name,
            oauth2server_metadata: None,
            oauth2server_metadata_url: None,
            server,
            system_prompt: None,
            title: None,
            visibility: None,
        }
    }

    pub fn http_with_auth_methods(
        auth_methods: Vec<CreateHttpConnectorRequestAuthMethodsItem>,
        description: String,
        global_headers: Option<HashMap<String, GlobalHeaderValue>>,
        headers: Option<HashMap<String, serde_json::Value>>,
        icon_url: Option<String>,
        name: String,
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
        oauth2server_metadata_url: Option<String>,
        server: String,
        system_prompt: Option<String>,
        title: Option<String>,
        visibility: Option<PublicResourceVisibility>,
    ) -> Self {
        Self::Http {
            auth_methods: Some(auth_methods),
            description,
            global_headers,
            headers,
            icon_url,
            name,
            oauth2server_metadata,
            oauth2server_metadata_url,
            server,
            system_prompt,
            title,
            visibility,
        }
    }

    pub fn http_with_global_headers(
        auth_methods: Option<Vec<CreateHttpConnectorRequestAuthMethodsItem>>,
        description: String,
        global_headers: HashMap<String, GlobalHeaderValue>,
        headers: Option<HashMap<String, serde_json::Value>>,
        icon_url: Option<String>,
        name: String,
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
        oauth2server_metadata_url: Option<String>,
        server: String,
        system_prompt: Option<String>,
        title: Option<String>,
        visibility: Option<PublicResourceVisibility>,
    ) -> Self {
        Self::Http {
            auth_methods,
            description,
            global_headers: Some(global_headers),
            headers,
            icon_url,
            name,
            oauth2server_metadata,
            oauth2server_metadata_url,
            server,
            system_prompt,
            title,
            visibility,
        }
    }

    pub fn http_with_headers(
        auth_methods: Option<Vec<CreateHttpConnectorRequestAuthMethodsItem>>,
        description: String,
        global_headers: Option<HashMap<String, GlobalHeaderValue>>,
        headers: HashMap<String, serde_json::Value>,
        icon_url: Option<String>,
        name: String,
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
        oauth2server_metadata_url: Option<String>,
        server: String,
        system_prompt: Option<String>,
        title: Option<String>,
        visibility: Option<PublicResourceVisibility>,
    ) -> Self {
        Self::Http {
            auth_methods,
            description,
            global_headers,
            headers: Some(headers),
            icon_url,
            name,
            oauth2server_metadata,
            oauth2server_metadata_url,
            server,
            system_prompt,
            title,
            visibility,
        }
    }

    pub fn http_with_icon_url(
        auth_methods: Option<Vec<CreateHttpConnectorRequestAuthMethodsItem>>,
        description: String,
        global_headers: Option<HashMap<String, GlobalHeaderValue>>,
        headers: Option<HashMap<String, serde_json::Value>>,
        icon_url: String,
        name: String,
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
        oauth2server_metadata_url: Option<String>,
        server: String,
        system_prompt: Option<String>,
        title: Option<String>,
        visibility: Option<PublicResourceVisibility>,
    ) -> Self {
        Self::Http {
            auth_methods,
            description,
            global_headers,
            headers,
            icon_url: Some(icon_url),
            name,
            oauth2server_metadata,
            oauth2server_metadata_url,
            server,
            system_prompt,
            title,
            visibility,
        }
    }

    pub fn http_with_oauth2server_metadata(
        auth_methods: Option<Vec<CreateHttpConnectorRequestAuthMethodsItem>>,
        description: String,
        global_headers: Option<HashMap<String, GlobalHeaderValue>>,
        headers: Option<HashMap<String, serde_json::Value>>,
        icon_url: Option<String>,
        name: String,
        oauth2server_metadata: ExtendedOAuthServerMetadata,
        oauth2server_metadata_url: Option<String>,
        server: String,
        system_prompt: Option<String>,
        title: Option<String>,
        visibility: Option<PublicResourceVisibility>,
    ) -> Self {
        Self::Http {
            auth_methods,
            description,
            global_headers,
            headers,
            icon_url,
            name,
            oauth2server_metadata: Some(oauth2server_metadata),
            oauth2server_metadata_url,
            server,
            system_prompt,
            title,
            visibility,
        }
    }

    pub fn http_with_oauth2server_metadata_url(
        auth_methods: Option<Vec<CreateHttpConnectorRequestAuthMethodsItem>>,
        description: String,
        global_headers: Option<HashMap<String, GlobalHeaderValue>>,
        headers: Option<HashMap<String, serde_json::Value>>,
        icon_url: Option<String>,
        name: String,
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
        oauth2server_metadata_url: String,
        server: String,
        system_prompt: Option<String>,
        title: Option<String>,
        visibility: Option<PublicResourceVisibility>,
    ) -> Self {
        Self::Http {
            auth_methods,
            description,
            global_headers,
            headers,
            icon_url,
            name,
            oauth2server_metadata,
            oauth2server_metadata_url: Some(oauth2server_metadata_url),
            server,
            system_prompt,
            title,
            visibility,
        }
    }

    pub fn http_with_system_prompt(
        auth_methods: Option<Vec<CreateHttpConnectorRequestAuthMethodsItem>>,
        description: String,
        global_headers: Option<HashMap<String, GlobalHeaderValue>>,
        headers: Option<HashMap<String, serde_json::Value>>,
        icon_url: Option<String>,
        name: String,
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
        oauth2server_metadata_url: Option<String>,
        server: String,
        system_prompt: String,
        title: Option<String>,
        visibility: Option<PublicResourceVisibility>,
    ) -> Self {
        Self::Http {
            auth_methods,
            description,
            global_headers,
            headers,
            icon_url,
            name,
            oauth2server_metadata,
            oauth2server_metadata_url,
            server,
            system_prompt: Some(system_prompt),
            title,
            visibility,
        }
    }

    pub fn http_with_title(
        auth_methods: Option<Vec<CreateHttpConnectorRequestAuthMethodsItem>>,
        description: String,
        global_headers: Option<HashMap<String, GlobalHeaderValue>>,
        headers: Option<HashMap<String, serde_json::Value>>,
        icon_url: Option<String>,
        name: String,
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
        oauth2server_metadata_url: Option<String>,
        server: String,
        system_prompt: Option<String>,
        title: String,
        visibility: Option<PublicResourceVisibility>,
    ) -> Self {
        Self::Http {
            auth_methods,
            description,
            global_headers,
            headers,
            icon_url,
            name,
            oauth2server_metadata,
            oauth2server_metadata_url,
            server,
            system_prompt,
            title: Some(title),
            visibility,
        }
    }

    pub fn http_with_visibility(
        auth_methods: Option<Vec<CreateHttpConnectorRequestAuthMethodsItem>>,
        description: String,
        global_headers: Option<HashMap<String, GlobalHeaderValue>>,
        headers: Option<HashMap<String, serde_json::Value>>,
        icon_url: Option<String>,
        name: String,
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
        oauth2server_metadata_url: Option<String>,
        server: String,
        system_prompt: Option<String>,
        title: Option<String>,
        visibility: PublicResourceVisibility,
    ) -> Self {
        Self::Http {
            auth_methods,
            description,
            global_headers,
            headers,
            icon_url,
            name,
            oauth2server_metadata,
            oauth2server_metadata_url,
            server,
            system_prompt,
            title,
            visibility: Some(visibility),
        }
    }

    pub fn mcp_with_auth_methods(
        auth_methods: Vec<CreateConnectorRequestAuthMethodsItem>,
        description: String,
        global_headers: Option<HashMap<String, GlobalHeaderValue>>,
        headers: Option<HashMap<String, serde_json::Value>>,
        icon_url: Option<String>,
        name: String,
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
        oauth2server_metadata_url: Option<String>,
        server: String,
        system_prompt: Option<String>,
        title: Option<String>,
        visibility: Option<PublicResourceVisibility>,
    ) -> Self {
        Self::Mcp {
            auth_methods: Some(auth_methods),
            description,
            global_headers,
            headers,
            icon_url,
            name,
            oauth2server_metadata,
            oauth2server_metadata_url,
            server,
            system_prompt,
            title,
            visibility,
        }
    }

    pub fn mcp_with_global_headers(
        auth_methods: Option<Vec<CreateConnectorRequestAuthMethodsItem>>,
        description: String,
        global_headers: HashMap<String, GlobalHeaderValue>,
        headers: Option<HashMap<String, serde_json::Value>>,
        icon_url: Option<String>,
        name: String,
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
        oauth2server_metadata_url: Option<String>,
        server: String,
        system_prompt: Option<String>,
        title: Option<String>,
        visibility: Option<PublicResourceVisibility>,
    ) -> Self {
        Self::Mcp {
            auth_methods,
            description,
            global_headers: Some(global_headers),
            headers,
            icon_url,
            name,
            oauth2server_metadata,
            oauth2server_metadata_url,
            server,
            system_prompt,
            title,
            visibility,
        }
    }

    pub fn mcp_with_headers(
        auth_methods: Option<Vec<CreateConnectorRequestAuthMethodsItem>>,
        description: String,
        global_headers: Option<HashMap<String, GlobalHeaderValue>>,
        headers: HashMap<String, serde_json::Value>,
        icon_url: Option<String>,
        name: String,
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
        oauth2server_metadata_url: Option<String>,
        server: String,
        system_prompt: Option<String>,
        title: Option<String>,
        visibility: Option<PublicResourceVisibility>,
    ) -> Self {
        Self::Mcp {
            auth_methods,
            description,
            global_headers,
            headers: Some(headers),
            icon_url,
            name,
            oauth2server_metadata,
            oauth2server_metadata_url,
            server,
            system_prompt,
            title,
            visibility,
        }
    }

    pub fn mcp_with_icon_url(
        auth_methods: Option<Vec<CreateConnectorRequestAuthMethodsItem>>,
        description: String,
        global_headers: Option<HashMap<String, GlobalHeaderValue>>,
        headers: Option<HashMap<String, serde_json::Value>>,
        icon_url: String,
        name: String,
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
        oauth2server_metadata_url: Option<String>,
        server: String,
        system_prompt: Option<String>,
        title: Option<String>,
        visibility: Option<PublicResourceVisibility>,
    ) -> Self {
        Self::Mcp {
            auth_methods,
            description,
            global_headers,
            headers,
            icon_url: Some(icon_url),
            name,
            oauth2server_metadata,
            oauth2server_metadata_url,
            server,
            system_prompt,
            title,
            visibility,
        }
    }

    pub fn mcp_with_oauth2server_metadata(
        auth_methods: Option<Vec<CreateConnectorRequestAuthMethodsItem>>,
        description: String,
        global_headers: Option<HashMap<String, GlobalHeaderValue>>,
        headers: Option<HashMap<String, serde_json::Value>>,
        icon_url: Option<String>,
        name: String,
        oauth2server_metadata: ExtendedOAuthServerMetadata,
        oauth2server_metadata_url: Option<String>,
        server: String,
        system_prompt: Option<String>,
        title: Option<String>,
        visibility: Option<PublicResourceVisibility>,
    ) -> Self {
        Self::Mcp {
            auth_methods,
            description,
            global_headers,
            headers,
            icon_url,
            name,
            oauth2server_metadata: Some(oauth2server_metadata),
            oauth2server_metadata_url,
            server,
            system_prompt,
            title,
            visibility,
        }
    }

    pub fn mcp_with_oauth2server_metadata_url(
        auth_methods: Option<Vec<CreateConnectorRequestAuthMethodsItem>>,
        description: String,
        global_headers: Option<HashMap<String, GlobalHeaderValue>>,
        headers: Option<HashMap<String, serde_json::Value>>,
        icon_url: Option<String>,
        name: String,
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
        oauth2server_metadata_url: String,
        server: String,
        system_prompt: Option<String>,
        title: Option<String>,
        visibility: Option<PublicResourceVisibility>,
    ) -> Self {
        Self::Mcp {
            auth_methods,
            description,
            global_headers,
            headers,
            icon_url,
            name,
            oauth2server_metadata,
            oauth2server_metadata_url: Some(oauth2server_metadata_url),
            server,
            system_prompt,
            title,
            visibility,
        }
    }

    pub fn mcp_with_system_prompt(
        auth_methods: Option<Vec<CreateConnectorRequestAuthMethodsItem>>,
        description: String,
        global_headers: Option<HashMap<String, GlobalHeaderValue>>,
        headers: Option<HashMap<String, serde_json::Value>>,
        icon_url: Option<String>,
        name: String,
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
        oauth2server_metadata_url: Option<String>,
        server: String,
        system_prompt: String,
        title: Option<String>,
        visibility: Option<PublicResourceVisibility>,
    ) -> Self {
        Self::Mcp {
            auth_methods,
            description,
            global_headers,
            headers,
            icon_url,
            name,
            oauth2server_metadata,
            oauth2server_metadata_url,
            server,
            system_prompt: Some(system_prompt),
            title,
            visibility,
        }
    }

    pub fn mcp_with_title(
        auth_methods: Option<Vec<CreateConnectorRequestAuthMethodsItem>>,
        description: String,
        global_headers: Option<HashMap<String, GlobalHeaderValue>>,
        headers: Option<HashMap<String, serde_json::Value>>,
        icon_url: Option<String>,
        name: String,
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
        oauth2server_metadata_url: Option<String>,
        server: String,
        system_prompt: Option<String>,
        title: String,
        visibility: Option<PublicResourceVisibility>,
    ) -> Self {
        Self::Mcp {
            auth_methods,
            description,
            global_headers,
            headers,
            icon_url,
            name,
            oauth2server_metadata,
            oauth2server_metadata_url,
            server,
            system_prompt,
            title: Some(title),
            visibility,
        }
    }

    pub fn mcp_with_visibility(
        auth_methods: Option<Vec<CreateConnectorRequestAuthMethodsItem>>,
        description: String,
        global_headers: Option<HashMap<String, GlobalHeaderValue>>,
        headers: Option<HashMap<String, serde_json::Value>>,
        icon_url: Option<String>,
        name: String,
        oauth2server_metadata: Option<ExtendedOAuthServerMetadata>,
        oauth2server_metadata_url: Option<String>,
        server: String,
        system_prompt: Option<String>,
        title: Option<String>,
        visibility: PublicResourceVisibility,
    ) -> Self {
        Self::Mcp {
            auth_methods,
            description,
            global_headers,
            headers,
            icon_url,
            name,
            oauth2server_metadata,
            oauth2server_metadata_url,
            server,
            system_prompt,
            title,
            visibility: Some(visibility),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
