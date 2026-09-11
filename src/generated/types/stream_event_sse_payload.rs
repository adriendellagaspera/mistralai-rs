#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StreamEventSsePayload {
    pub broker_sequence: i64,
    pub data: StreamEventSsePayloadData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<StreamEventSsePayloadMetadata>,
    pub stream: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub workflow_context: StreamEventWorkflowContext,
}
