///Metrics at the step number during the fine-tuning job. Use these metrics to assess if the training is going smoothly (loss should decrease, token accuracy should increase).
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct MetricOut {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub train_loss: Option<Option<f64>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub valid_loss: Option<Option<f64>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub valid_mean_token_accuracy: Option<Option<f64>>,
}
