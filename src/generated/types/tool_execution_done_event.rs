#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolExecutionDoneEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<ToolExecutionInfo>,
    pub name: ToolExecutionDoneEventName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ToolExecutionDoneEventType>,
}
