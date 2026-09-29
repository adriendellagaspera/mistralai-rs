pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SchedulePolicy {
    /// After a Temporal server is unavailable, amount of time in seconds in the past to execute missed actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catchup_window_seconds: Option<i64>,
    /// Policy controlling what to do when a workflow is already running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlap: Option<ScheduleOverlapPolicy>,
    /// Whether to pause the schedule after a workflow failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_on_failure: Option<bool>,
}

impl SchedulePolicy {
    pub fn builder() -> SchedulePolicyBuilder {
        <SchedulePolicyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SchedulePolicyBuilder {
    catchup_window_seconds: Option<i64>,
    overlap: Option<ScheduleOverlapPolicy>,
    pause_on_failure: Option<bool>,
}

impl SchedulePolicyBuilder {
    pub fn catchup_window_seconds(mut self, value: i64) -> Self {
        self.catchup_window_seconds = Some(value);
        self
    }

    pub fn overlap(mut self, value: ScheduleOverlapPolicy) -> Self {
        self.overlap = Some(value);
        self
    }

    pub fn pause_on_failure(mut self, value: bool) -> Self {
        self.pause_on_failure = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SchedulePolicy`].
    pub fn build(self) -> Result<SchedulePolicy, BuildError> {
        Ok(SchedulePolicy {
            catchup_window_seconds: self.catchup_window_seconds,
            overlap: self.overlap,
            pause_on_failure: self.pause_on_failure,
        })
    }
}
