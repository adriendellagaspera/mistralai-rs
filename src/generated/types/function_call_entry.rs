#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionCallEntry {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub agent_id: Option<Option<String>>,
    pub arguments: FunctionCallEntryArguments,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub completed_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub confirmation_status: Option<Option<FunctionCallEntryConfirmationStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub model: Option<Option<String>>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<FunctionCallEntryObject>,
    pub tool_call_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FunctionCallEntryType>,
}
