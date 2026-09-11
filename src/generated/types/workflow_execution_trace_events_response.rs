#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowExecutionTraceEventsResponse {
    ///The end time of the workflow execution, if available
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    ///The events of the workflow execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<WorkflowExecutionTraceEventsResponseEventsItemUnion>>,
    ///The ID of the workflow execution
    pub execution_id: String,
    ///The parent execution ID of the workflow execution
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub parent_execution_id: Option<Option<String>>,
    ///The result of the workflow execution, if available
    pub result: Option<serde_json::Value>,
    ///The root execution ID of the workflow execution
    pub root_execution_id: String,
    ///The start time of the workflow execution
    pub start_time: chrono::DateTime<chrono::Utc>,
    ///The status of the workflow execution
    pub status: Option<WorkflowExecutionStatus>,
    ///The total duration of the trace in milliseconds
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub total_duration_ms: Option<Option<i64>>,
    ///The name of the workflow
    pub workflow_name: String,
}
