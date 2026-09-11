/**Trace response in OpenTelemetry format.

This is the unified trace format used across all trace providers (Tempo, ClickHouse, etc.).
Regardless of the underlying backend, all trace data is normalized to this Tempo-compatible
OpenTelemetry format to ensure consistency in the API response structure.*/
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TempoGetTraceResponse {
    ///The batches of the trace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batches: Option<Vec<TempoTraceBatch>>,
}
