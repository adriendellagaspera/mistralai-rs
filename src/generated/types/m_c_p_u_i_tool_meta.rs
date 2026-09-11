///UI metadata for tools that reference UI resources.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct MCPUIToolMeta {
    #[serde(
        rename = "resourceUri",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub resource_uri: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub visibility: Option<Option<Vec<MCPUIToolMetaVisibilityItem>>>,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
