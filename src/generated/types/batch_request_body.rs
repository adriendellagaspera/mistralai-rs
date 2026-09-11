#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct BatchRequestBody {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
