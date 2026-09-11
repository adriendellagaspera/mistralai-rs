#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SchedulePolicy {
    ///After a Temporal server is unavailable, amount of time in seconds in the past to execute missed actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catchup_window_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlap: Option<ScheduleOverlapPolicy>,
    ///Whether to pause the schedule after a workflow failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_on_failure: Option<bool>,
}
