///Input schema of the workflow's run method
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct WorkflowCodeDefinitionInputSchema {
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
