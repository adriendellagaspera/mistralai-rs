///Binary contents of a resource.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlobResourceContents {
    #[serde(
        rename = "_meta",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub meta: Option<Option<BlobResourceContentsMeta>>,
    pub blob: String,
    #[serde(
        rename = "mimeType",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub mime_type: Option<Option<String>>,
    ///Constraint: minLength=1
    pub uri: url::Url,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
