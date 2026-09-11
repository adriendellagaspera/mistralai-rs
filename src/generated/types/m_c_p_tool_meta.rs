/**Typed _meta for MCP tools.

Only the 'ui' field is typed. Other fields are allowed via extra="allow".*/
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct MCPToolMeta {
    #[serde(
        rename = "ai.mistral/turbine",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub ai_mistral_turbine: Option<Option<TurbineToolMeta>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub ui: Option<Option<MCPUIToolMeta>>,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
