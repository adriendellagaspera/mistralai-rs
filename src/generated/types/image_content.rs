///Image content for a message.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageContent {
    #[serde(
        rename = "_meta",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub meta: Option<Option<ImageContentMeta>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub annotations: Option<Option<Annotations>>,
    pub data: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub r#type: ImageContentType,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
