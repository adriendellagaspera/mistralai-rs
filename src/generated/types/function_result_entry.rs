#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionResultEntry {
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
    pub object: Option<FunctionResultEntryObject>,
    pub result: String,
    pub tool_call_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FunctionResultEntryType>,
}
