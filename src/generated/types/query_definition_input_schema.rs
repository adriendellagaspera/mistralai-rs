///Input JSON schema of the query's model
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct QueryDefinitionInputSchema {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
