/**Metadata wrapper for MCP tool call responses.

Nests MCP-specific fields under `mcp_meta` to avoid collisions with other
metadata keys (e.g. `tool_call_result`) in Harmattan's streaming deltas.*/
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct MCPToolCallMetadata {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub mcp_meta: Option<Option<MCPResultMetadata>>,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
