pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScheduleFutureExecution {
    /// Time the execution is scheduled to run.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub scheduled_at: DateTime<FixedOffset>,
}

impl ScheduleFutureExecution {
    pub fn builder() -> ScheduleFutureExecutionBuilder {
        <ScheduleFutureExecutionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleFutureExecutionBuilder {
    scheduled_at: Option<DateTime<FixedOffset>>,
}

impl ScheduleFutureExecutionBuilder {
    pub fn scheduled_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.scheduled_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScheduleFutureExecution`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scheduled_at`](ScheduleFutureExecutionBuilder::scheduled_at)
    pub fn build(self) -> Result<ScheduleFutureExecution, BuildError> {
        Ok(ScheduleFutureExecution {
            scheduled_at: self
                .scheduled_at
                .ok_or_else(|| BuildError::missing_field("scheduled_at"))?,
        })
    }
}
