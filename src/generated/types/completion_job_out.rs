#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CompletionJobOut {
    pub auto_start: bool,
    ///The UNIX timestamp (in seconds) for when the fine-tuning job was created.
    pub created_at: i64,
    ///The name of the fine-tuned model that is being created. The value will be `null` if the fine-tuning job is still running.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub fine_tuned_model: Option<Option<String>>,
    pub hyperparameters: CompletionTrainingParameters,
    ///The ID of the job.
    pub id: uuid::Uuid,
    ///A list of integrations enabled for your fine-tuning job.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub integrations: Option<Option<Vec<WandbIntegrationOut>>>,
    ///The type of job (`FT` for fine-tuning).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<CompletionJobOutJobType>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub metadata: Option<Option<JobMetadataOut>>,
    pub model: String,
    ///The UNIX timestamp (in seconds) for when the fine-tuning job was last modified.
    pub modified_at: i64,
    ///The object type of the fine-tuning job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<CompletionJobOutObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<GithubRepositoryOut>>,
    ///The current status of the fine-tuning job.
    pub status: CompletionJobOutStatus,
    ///Optional text/code that adds more context for the model. When given a `prompt` and a `suffix` the model will fill what is between them. When `suffix` is not provided, the model will simply execute completion starting with `prompt`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub suffix: Option<Option<String>>,
    ///Total number of tokens trained.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub trained_tokens: Option<Option<i64>>,
    ///A list containing the IDs of uploaded files that contain training data.
    pub training_files: Vec<String>,
    ///A list containing the IDs of uploaded files that contain validation data.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub validation_files: Option<Option<Vec<String>>>,
}
