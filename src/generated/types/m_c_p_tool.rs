#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MCPTool {
    #[serde(
        rename = "_meta",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub meta: Option<Option<MCPToolMeta>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub annotations: Option<Option<ToolAnnotations>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub description: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub execution: Option<Option<ToolExecution>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub icons: Option<Option<Vec<MCPServerIcon>>>,
    #[serde(rename = "inputSchema")]
    pub input_schema: MCPToolInputSchema,
    pub name: String,
    #[serde(
        rename = "outputSchema",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub output_schema: Option<Option<MCPToolOutputSchema>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub title: Option<Option<String>>,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
