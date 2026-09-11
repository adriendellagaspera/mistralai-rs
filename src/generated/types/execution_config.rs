/**Not typed since mcp config can changed / not stable
we allow all extra fields and this is a dict
TODO: once mcp is stable, we need to type this*/
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExecutionConfig {
    pub r#type: String,
    /// Additional properties not explicitly defined in the schema
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
}
