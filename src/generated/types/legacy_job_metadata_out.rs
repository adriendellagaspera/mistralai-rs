#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LegacyJobMetadataOut {
    ///The cost of the fine-tuning job.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub cost: Option<Option<f64>>,
    ///The currency used for the fine-tuning job cost.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub cost_currency: Option<Option<String>>,
    ///The total number of tokens in the training dataset.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub data_tokens: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    pub details: String,
    ///The number of complete passes through the entire training dataset.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub epochs: Option<Option<f64>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub estimated_start_time: Option<Option<i64>>,
    ///The approximated time (in seconds) for the fine-tuning process to complete.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub expected_duration_seconds: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<LegacyJobMetadataOutObject>,
    ///The total number of tokens used during the fine-tuning process.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub train_tokens: Option<Option<i64>>,
    ///The number of tokens consumed by one training step.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub train_tokens_per_step: Option<Option<i64>>,
    ///The number of training steps to perform. A training step refers to a single update of the model weights during the fine-tuning process. This update is typically calculated using a batch of samples from the training dataset.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub training_steps: Option<Option<i64>>,
}
