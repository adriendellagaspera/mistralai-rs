///An icon for display in user interfaces.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MCPServerIcon {
    #[serde(
        rename = "mimeType",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub mime_type: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub sizes: Option<Option<Vec<String>>>,
    pub src: String,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
