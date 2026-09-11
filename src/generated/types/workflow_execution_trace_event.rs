#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowExecutionTraceEvent {
    ///The attributes of the event
    pub attributes: WorkflowExecutionTraceEventAttributes,
    ///The ID of the event
    pub id: String,
    ///Whether the event is internal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    ///Name of the event
    pub name: String,
    ///The timestamp of the event in nanoseconds since the Unix epoch
    pub timestamp_unix_nano: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<EventType>,
}
