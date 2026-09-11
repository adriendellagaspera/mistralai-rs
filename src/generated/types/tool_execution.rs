///Execution-related properties for a tool.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ToolExecution {
    #[serde(
        rename = "taskSupport",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub task_support: Option<Option<ToolExecutionTaskSupport>>,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
