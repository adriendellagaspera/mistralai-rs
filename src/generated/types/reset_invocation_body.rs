#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResetInvocationBody {
    ///The event ID to reset the workflow execution to
    pub event_id: i64,
    ///Whether to exclude signals that happened after the reset point
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_signals: Option<bool>,
    ///Whether to exclude updates that happened after the reset point
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_updates: Option<bool>,
    ///Reason for resetting the workflow execution
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub reason: Option<Option<String>>,
}
