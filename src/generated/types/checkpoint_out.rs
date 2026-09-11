#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckpointOut {
    ///The UNIX timestamp (in seconds) for when the checkpoint was created.
    pub created_at: i64,
    pub metrics: MetricOut,
    ///The step number that the checkpoint was created at.
    pub step_number: i64,
}
