pub use crate::prelude::*;

/// MCP-specific result metadata (isError, structuredContent, _meta).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectorToolResultMetadata {
    #[serde(rename = "_meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "isError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
    #[serde(rename = "structuredContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_content: Option<HashMap<String, serde_json::Value>>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ConnectorToolResultMetadata {
    pub fn builder() -> ConnectorToolResultMetadataBuilder {
        <ConnectorToolResultMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorToolResultMetadataBuilder {
    meta: Option<HashMap<String, serde_json::Value>>,
    is_error: Option<bool>,
    structured_content: Option<HashMap<String, serde_json::Value>>,
}

impl ConnectorToolResultMetadataBuilder {
    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn is_error(mut self, value: bool) -> Self {
        self.is_error = Some(value);
        self
    }

    pub fn structured_content(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.structured_content = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectorToolResultMetadata`].
    pub fn build(self) -> Result<ConnectorToolResultMetadata, BuildError> {
        Ok(ConnectorToolResultMetadata {
            meta: self.meta,
            is_error: self.is_error,
            structured_content: self.structured_content,
            extra: Default::default(),
        })
    }
}
