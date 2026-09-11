///Request body for calling an MCP tool.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct MCPToolCallRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<MCPToolCallRequestArguments>,
}
