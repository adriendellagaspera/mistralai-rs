#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolExecutionEntry {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub agent_id: Option<Option<String>>,
    pub arguments: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub completed_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<ToolExecutionInfo>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub model: Option<Option<String>>,
    pub name: ToolExecutionEntryName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ToolExecutionEntryObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ToolExecutionEntryType>,
}
