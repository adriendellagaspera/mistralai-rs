#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowExecutionProgressTraceEvent {
    ///The attributes of the event
    pub attributes: WorkflowExecutionProgressTraceEventAttributes,
    ///The end time of the event in milliseconds since the Unix epoch
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub end_time_unix_ms: Option<Option<i64>>,
    ///The error message, if any
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub error: Option<Option<String>>,
    ///The ID of the event
    pub id: String,
    ///Whether the event is internal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    ///Name of the event
    pub name: String,
    ///The start time of the event in milliseconds since the Unix epoch
    pub start_time_unix_ms: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EventProgressStatus>,
    ///The timestamp of the event in nanoseconds since the Unix epoch
    pub timestamp_unix_nano: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<EventType>,
}
