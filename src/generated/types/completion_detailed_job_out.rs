#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CompletionDetailedJobOut {
    pub auto_start: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoints: Option<Vec<CheckpointOut>>,
    pub created_at: i64,
    ///Event items are created every time the status of a fine-tuning job changes. The timestamped list of all events is accessible here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<EventOut>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub fine_tuned_model: Option<Option<String>>,
    pub hyperparameters: CompletionTrainingParameters,
    pub id: uuid::Uuid,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub integrations: Option<Option<Vec<WandbIntegrationOut>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<CompletionDetailedJobOutJobType>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub metadata: Option<Option<JobMetadataOut>>,
    pub model: String,
    pub modified_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<CompletionDetailedJobOutObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<GithubRepositoryOut>>,
    pub status: CompletionDetailedJobOutStatus,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub suffix: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub trained_tokens: Option<Option<i64>>,
    pub training_files: Vec<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub validation_files: Option<Option<Vec<String>>>,
}
