#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowExecutionTraceSummarySpan {
    ///The attributes of the span
    pub attributes: WorkflowExecutionTraceSummarySpanAttributes,
    ///The child spans of the span
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Box<WorkflowExecutionTraceSummarySpan>>>,
    ///The end time of the span in nanoseconds since the Unix epoch
    pub end_time_unix_nano: Option<i64>,
    ///The events of the span
    pub events: Vec<WorkflowExecutionTraceEvent>,
    ///The name of the span
    pub name: String,
    ///The ID of the span
    pub span_id: String,
    ///The start time of the span in nanoseconds since the Unix epoch
    pub start_time_unix_nano: i64,
}
