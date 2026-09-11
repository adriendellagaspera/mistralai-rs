/**Response from calling an MCP tool.

We override mcp_types.CallToolResult because:
- Models only support `content`, not `structuredContent` at top level
- Downstream consumers (le-chat, etc.) need structuredContent/isError/_meta via metadata

SYNC: Keep in sync with Harmattan (orchestrator) for harmonized tool result processing.*/
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MCPToolCallResponse {
    pub content: Vec<MCPToolCallResponseContentItemUnion>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub metadata: Option<Option<MCPToolCallMetadata>>,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
