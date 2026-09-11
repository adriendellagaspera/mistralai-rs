///Mapping of execution_id to result with status and optional error message
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct BatchExecutionResponseResults {
    /// Additional properties matching the spec's
    /// `additionalProperties` value schema.
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, BatchExecutionResult>,
}
