///MCP-specific result metadata (isError, structuredContent, _meta).
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct MCPResultMetadata {
    #[serde(
        rename = "_meta",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub meta: Option<Option<MCPResultMetadataMeta>>,
    #[serde(rename = "isError", skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
    #[serde(
        rename = "structuredContent",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub structured_content: Option<Option<MCPResultMetadataStructuredContent>>,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
