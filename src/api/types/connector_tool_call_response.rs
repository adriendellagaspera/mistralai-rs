pub use crate::prelude::*;

/// Response from calling an MCP tool.
///
/// We override mcp_types.CallToolResult because:
/// - Models only support `content`, not `structuredContent` at top level
/// - Downstream consumers (le-chat, etc.) need structuredContent/isError/_meta via metadata
///
/// SYNC: Keep in sync with Harmattan (orchestrator) for harmonized tool result processing.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectorToolCallResponse {
    #[serde(default)]
    pub content: Vec<ConnectorToolCallResponseContentItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ConnectorToolCallMetadata>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ConnectorToolCallResponse {
    pub fn builder() -> ConnectorToolCallResponseBuilder {
        <ConnectorToolCallResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorToolCallResponseBuilder {
    content: Option<Vec<ConnectorToolCallResponseContentItem>>,
    metadata: Option<ConnectorToolCallMetadata>,
}

impl ConnectorToolCallResponseBuilder {
    pub fn content(mut self, value: Vec<ConnectorToolCallResponseContentItem>) -> Self {
        self.content = Some(value);
        self
    }

    pub fn metadata(mut self, value: ConnectorToolCallMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectorToolCallResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](ConnectorToolCallResponseBuilder::content)
    pub fn build(self) -> Result<ConnectorToolCallResponse, BuildError> {
        Ok(ConnectorToolCallResponse {
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            metadata: self.metadata,
            extra: Default::default(),
        })
    }
}
