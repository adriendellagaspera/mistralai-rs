#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BatchJobOut {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub agent_id: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub completed_at: Option<Option<i64>>,
    pub completed_requests: i64,
    pub created_at: i64,
    pub endpoint: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub error_file: Option<Option<uuid::Uuid>>,
    pub errors: Vec<BatchError>,
    pub failed_requests: i64,
    pub id: String,
    pub input_files: Vec<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub metadata: Option<Option<BatchJobOutMetadata>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub model: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<BatchJobOutObject>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub output_file: Option<Option<uuid::Uuid>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub outputs: Option<Option<Vec<BatchJobOutOutputsItem>>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub started_at: Option<Option<i64>>,
    pub status: BatchJobStatus,
    pub succeeded_requests: i64,
    pub total_requests: i64,
}
