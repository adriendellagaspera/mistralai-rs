pub use crate::prelude::*;

/// Query parameters for connector_list_tools_v1
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectorListToolsV1QueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh: Option<bool>,
    /// Return a simplified payload with only name, description, annotations, and a compact inputSchema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pretty: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_name: Option<String>,
}

impl ConnectorListToolsV1QueryRequest {
    pub fn builder() -> ConnectorListToolsV1QueryRequestBuilder {
        <ConnectorListToolsV1QueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorListToolsV1QueryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    refresh: Option<bool>,
    pretty: Option<bool>,
    credentials_name: Option<String>,
}

impl ConnectorListToolsV1QueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn refresh(mut self, value: bool) -> Self {
        self.refresh = Some(value);
        self
    }

    pub fn pretty(mut self, value: bool) -> Self {
        self.pretty = Some(value);
        self
    }

    pub fn credentials_name(mut self, value: impl Into<String>) -> Self {
        self.credentials_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectorListToolsV1QueryRequest`].
    pub fn build(self) -> Result<ConnectorListToolsV1QueryRequest, BuildError> {
        Ok(ConnectorListToolsV1QueryRequest {
            page: self.page,
            page_size: self.page_size,
            refresh: self.refresh,
            pretty: self.pretty,
            credentials_name: self.credentials_name,
        })
    }
}
