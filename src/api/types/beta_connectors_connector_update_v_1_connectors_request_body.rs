pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "protocol")]
#[non_exhaustive]
pub enum ConnectorUpdateV1ConnectorsRequestBody {
    #[serde(rename = "http")]
    #[non_exhaustive]
    Http {
        #[serde(skip_serializing_if = "Option::is_none")]
        auth_methods: Option<Vec<UpdateHttpConnectorRequestAuthMethodsItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        icon_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        server: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },

    #[serde(rename = "mcp")]
    #[non_exhaustive]
    Mcp {
        #[serde(skip_serializing_if = "Option::is_none")]
        auth_methods: Option<Vec<ConnectorMcpPublicUpdateAuthMethodsItem>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        icon_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        server: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        system_prompt: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ConnectorUpdateV1ConnectorsRequestBody {
    pub fn http() -> Self {
        Self::Http {
            auth_methods: None,
            description: None,
            icon_url: None,
            name: None,
            server: None,
            title: None,
        }
    }

    pub fn mcp() -> Self {
        Self::Mcp {
            auth_methods: None,
            description: None,
            icon_url: None,
            name: None,
            server: None,
            system_prompt: None,
            title: None,
        }
    }

    pub fn http_with_auth_methods(
        auth_methods: Vec<UpdateHttpConnectorRequestAuthMethodsItem>,
        description: Option<String>,
        icon_url: Option<String>,
        name: Option<String>,
        server: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Http {
            auth_methods: Some(auth_methods),
            description,
            icon_url,
            name,
            server,
            title,
        }
    }

    pub fn http_with_description(
        auth_methods: Option<Vec<UpdateHttpConnectorRequestAuthMethodsItem>>,
        description: String,
        icon_url: Option<String>,
        name: Option<String>,
        server: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Http {
            auth_methods,
            description: Some(description),
            icon_url,
            name,
            server,
            title,
        }
    }

    pub fn http_with_icon_url(
        auth_methods: Option<Vec<UpdateHttpConnectorRequestAuthMethodsItem>>,
        description: Option<String>,
        icon_url: String,
        name: Option<String>,
        server: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Http {
            auth_methods,
            description,
            icon_url: Some(icon_url),
            name,
            server,
            title,
        }
    }

    pub fn http_with_name(
        auth_methods: Option<Vec<UpdateHttpConnectorRequestAuthMethodsItem>>,
        description: Option<String>,
        icon_url: Option<String>,
        name: String,
        server: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Http {
            auth_methods,
            description,
            icon_url,
            name: Some(name),
            server,
            title,
        }
    }

    pub fn http_with_server(
        auth_methods: Option<Vec<UpdateHttpConnectorRequestAuthMethodsItem>>,
        description: Option<String>,
        icon_url: Option<String>,
        name: Option<String>,
        server: String,
        title: Option<String>,
    ) -> Self {
        Self::Http {
            auth_methods,
            description,
            icon_url,
            name,
            server: Some(server),
            title,
        }
    }

    pub fn http_with_title(
        auth_methods: Option<Vec<UpdateHttpConnectorRequestAuthMethodsItem>>,
        description: Option<String>,
        icon_url: Option<String>,
        name: Option<String>,
        server: Option<String>,
        title: String,
    ) -> Self {
        Self::Http {
            auth_methods,
            description,
            icon_url,
            name,
            server,
            title: Some(title),
        }
    }

    pub fn mcp_with_auth_methods(
        auth_methods: Vec<ConnectorMcpPublicUpdateAuthMethodsItem>,
        description: Option<String>,
        icon_url: Option<String>,
        name: Option<String>,
        server: Option<String>,
        system_prompt: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Mcp {
            auth_methods: Some(auth_methods),
            description,
            icon_url,
            name,
            server,
            system_prompt,
            title,
        }
    }

    pub fn mcp_with_description(
        auth_methods: Option<Vec<ConnectorMcpPublicUpdateAuthMethodsItem>>,
        description: String,
        icon_url: Option<String>,
        name: Option<String>,
        server: Option<String>,
        system_prompt: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Mcp {
            auth_methods,
            description: Some(description),
            icon_url,
            name,
            server,
            system_prompt,
            title,
        }
    }

    pub fn mcp_with_icon_url(
        auth_methods: Option<Vec<ConnectorMcpPublicUpdateAuthMethodsItem>>,
        description: Option<String>,
        icon_url: String,
        name: Option<String>,
        server: Option<String>,
        system_prompt: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Mcp {
            auth_methods,
            description,
            icon_url: Some(icon_url),
            name,
            server,
            system_prompt,
            title,
        }
    }

    pub fn mcp_with_name(
        auth_methods: Option<Vec<ConnectorMcpPublicUpdateAuthMethodsItem>>,
        description: Option<String>,
        icon_url: Option<String>,
        name: String,
        server: Option<String>,
        system_prompt: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Mcp {
            auth_methods,
            description,
            icon_url,
            name: Some(name),
            server,
            system_prompt,
            title,
        }
    }

    pub fn mcp_with_server(
        auth_methods: Option<Vec<ConnectorMcpPublicUpdateAuthMethodsItem>>,
        description: Option<String>,
        icon_url: Option<String>,
        name: Option<String>,
        server: String,
        system_prompt: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Mcp {
            auth_methods,
            description,
            icon_url,
            name,
            server: Some(server),
            system_prompt,
            title,
        }
    }

    pub fn mcp_with_system_prompt(
        auth_methods: Option<Vec<ConnectorMcpPublicUpdateAuthMethodsItem>>,
        description: Option<String>,
        icon_url: Option<String>,
        name: Option<String>,
        server: Option<String>,
        system_prompt: String,
        title: Option<String>,
    ) -> Self {
        Self::Mcp {
            auth_methods,
            description,
            icon_url,
            name,
            server,
            system_prompt: Some(system_prompt),
            title,
        }
    }

    pub fn mcp_with_title(
        auth_methods: Option<Vec<ConnectorMcpPublicUpdateAuthMethodsItem>>,
        description: Option<String>,
        icon_url: Option<String>,
        name: Option<String>,
        server: Option<String>,
        system_prompt: Option<String>,
        title: String,
    ) -> Self {
        Self::Mcp {
            auth_methods,
            description,
            icon_url,
            name,
            server,
            system_prompt,
            title: Some(title),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
