pub use crate::prelude::*;

/// Metadata wrapper for MCP tool call responses.
///
/// Nests MCP-specific fields under `mcp_meta` to avoid collisions with other
/// metadata keys (e.g. `tool_call_result`) in Harmattan's streaming deltas.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectorToolCallMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcp_meta: Option<ConnectorToolResultMetadata>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ConnectorToolCallMetadata {
    pub fn builder() -> ConnectorToolCallMetadataBuilder {
        <ConnectorToolCallMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectorToolCallMetadataBuilder {
    mcp_meta: Option<ConnectorToolResultMetadata>,
}

impl ConnectorToolCallMetadataBuilder {
    pub fn mcp_meta(mut self, value: ConnectorToolResultMetadata) -> Self {
        self.mcp_meta = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectorToolCallMetadata`].
    pub fn build(self) -> Result<ConnectorToolCallMetadata, BuildError> {
        Ok(ConnectorToolCallMetadata {
            mcp_meta: self.mcp_meta,
            extra: Default::default(),
        })
    }
}
