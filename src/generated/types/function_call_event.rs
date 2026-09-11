#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionCallEvent {
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
    pub confirmation_status: Option<Option<FunctionCallEventConfirmationStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub id: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub model: Option<Option<String>>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_index: Option<i64>,
    pub tool_call_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FunctionCallEventType>,
}
