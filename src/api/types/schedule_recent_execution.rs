pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScheduleRecentExecution {
    /// Time the execution was scheduled to run.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub scheduled_at: DateTime<FixedOffset>,
    /// Actual time the execution started.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub started_at: DateTime<FixedOffset>,
    /// ID of the workflow execution that was started.
    #[serde(default)]
    pub execution_id: String,
}

impl ScheduleRecentExecution {
    pub fn builder() -> ScheduleRecentExecutionBuilder {
        <ScheduleRecentExecutionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleRecentExecutionBuilder {
    scheduled_at: Option<DateTime<FixedOffset>>,
    started_at: Option<DateTime<FixedOffset>>,
    execution_id: Option<String>,
}

impl ScheduleRecentExecutionBuilder {
    pub fn scheduled_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.scheduled_at = Some(value);
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn execution_id(mut self, value: impl Into<String>) -> Self {
        self.execution_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScheduleRecentExecution`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scheduled_at`](ScheduleRecentExecutionBuilder::scheduled_at)
    /// - [`started_at`](ScheduleRecentExecutionBuilder::started_at)
    /// - [`execution_id`](ScheduleRecentExecutionBuilder::execution_id)
    pub fn build(self) -> Result<ScheduleRecentExecution, BuildError> {
        Ok(ScheduleRecentExecution {
            scheduled_at: self
                .scheduled_at
                .ok_or_else(|| BuildError::missing_field("scheduled_at"))?,
            started_at: self
                .started_at
                .ok_or_else(|| BuildError::missing_field("started_at"))?,
            execution_id: self
                .execution_id
                .ok_or_else(|| BuildError::missing_field("execution_id"))?,
        })
    }
}
