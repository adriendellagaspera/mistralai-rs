pub use crate::prelude::*;

/// Remote transport endpoint (SEP-2127).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct McpServerRemote {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<McpServerAuthenticationRequirement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<McpServerRemoteHeader>>,
    #[serde(rename = "supportedProtocolVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_protocol_versions: Option<Vec<String>>,
    /// Transport type
    pub r#type: McpServerRemoteType,
    /// Transport endpoint URL
    #[serde(default)]
    pub url: String,
}

impl McpServerRemote {
    pub fn builder() -> McpServerRemoteBuilder {
        <McpServerRemoteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServerRemoteBuilder {
    authentication: Option<McpServerAuthenticationRequirement>,
    headers: Option<Vec<McpServerRemoteHeader>>,
    supported_protocol_versions: Option<Vec<String>>,
    r#type: Option<McpServerRemoteType>,
    url: Option<String>,
}

impl McpServerRemoteBuilder {
    pub fn authentication(mut self, value: McpServerAuthenticationRequirement) -> Self {
        self.authentication = Some(value);
        self
    }

    pub fn headers(mut self, value: Vec<McpServerRemoteHeader>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn supported_protocol_versions(mut self, value: Vec<String>) -> Self {
        self.supported_protocol_versions = Some(value);
        self
    }

    pub fn r#type(mut self, value: McpServerRemoteType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`McpServerRemote`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](McpServerRemoteBuilder::r#type)
    /// - [`url`](McpServerRemoteBuilder::url)
    pub fn build(self) -> Result<McpServerRemote, BuildError> {
        Ok(McpServerRemote {
            authentication: self.authentication,
            headers: self.headers,
            supported_protocol_versions: self.supported_protocol_versions,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
