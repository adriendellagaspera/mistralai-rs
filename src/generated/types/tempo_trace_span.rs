#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TempoTraceSpan {
    ///The attributes of the scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<TempoTraceAttribute>>,
    ///The end time of the scope in Unix nano
    #[serde(rename = "endTimeUnixNano")]
    pub end_time_unix_nano: String,
    ///The events of the scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<TempoTraceEvent>>,
    pub kind: TempoTraceScopeKind,
    ///The name of the scope
    pub name: String,
    ///The parent span ID of the scope
    #[serde(
        rename = "parentSpanId",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub parent_span_id: Option<Option<String>>,
    ///The span ID of the scope
    #[serde(rename = "spanId")]
    pub span_id: String,
    ///The start time of the scope in Unix nano
    #[serde(rename = "startTimeUnixNano")]
    pub start_time_unix_nano: String,
    ///The trace ID of the scope
    #[serde(rename = "traceId")]
    pub trace_id: String,
}
