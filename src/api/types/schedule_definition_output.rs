pub use crate::prelude::*;

/// Output representation of a schedule with required schedule_id.
///
/// Used when returning schedules from the API where schedule_id is always present.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScheduleDefinitionOutput {
    pub input: serde_json::Value,
    /// Calendar-based specification of times.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendars: Option<Vec<ScheduleCalendar>>,
    /// Interval-based specification of times.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intervals: Option<Vec<ScheduleInterval>>,
    /// Cron-based specification of times.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_expressions: Option<Vec<String>>,
    /// Set of calendar times to skip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<Vec<ScheduleCalendar>>,
    /// Time after which the first action may be run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<DateTime<FixedOffset>>,
    /// Time after which no more actions will be run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<DateTime<FixedOffset>>,
    /// Jitter to apply each action.
    ///
    /// An action's scheduled time will be incremented by a random value between 0
    /// and this value if present (but not past the next schedule).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jitter: Option<String>,
    /// IANA time zone name, for example ``US/Central``.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone_name: Option<String>,
    /// Policy for the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<SchedulePolicy>,
    /// Unique identifier for the schedule.
    #[serde(default)]
    pub schedule_id: String,
    /// Remaining workflow executions before this schedule stops triggering automatically. null means unlimited; 0 means the limit has been reached and the schedule is exhausted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_executions: Option<i64>,
    /// Name of the workflow this schedule triggers.
    #[serde(default)]
    pub workflow_name: String,
    /// Name of the deployment this schedule targets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_name: Option<String>,
    /// Whether the schedule is currently paused.
    #[serde(default)]
    pub paused: bool,
    /// Human-readable note associated with the current pause or resume state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Upcoming scheduled executions (10 next executions, earliest first).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_executions: Option<Vec<ScheduleFutureExecution>>,
    /// Most recent scheduled executions (10 most recent, newest last).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_executions: Option<Vec<ScheduleRecentExecution>>,
}

impl ScheduleDefinitionOutput {
    pub fn builder() -> ScheduleDefinitionOutputBuilder {
        <ScheduleDefinitionOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleDefinitionOutputBuilder {
    input: Option<serde_json::Value>,
    calendars: Option<Vec<ScheduleCalendar>>,
    intervals: Option<Vec<ScheduleInterval>>,
    cron_expressions: Option<Vec<String>>,
    skip: Option<Vec<ScheduleCalendar>>,
    start_at: Option<DateTime<FixedOffset>>,
    end_at: Option<DateTime<FixedOffset>>,
    jitter: Option<String>,
    time_zone_name: Option<String>,
    policy: Option<SchedulePolicy>,
    schedule_id: Option<String>,
    remaining_executions: Option<i64>,
    workflow_name: Option<String>,
    deployment_name: Option<String>,
    paused: Option<bool>,
    note: Option<String>,
    future_executions: Option<Vec<ScheduleFutureExecution>>,
    recent_executions: Option<Vec<ScheduleRecentExecution>>,
}

impl ScheduleDefinitionOutputBuilder {
    pub fn input(mut self, value: serde_json::Value) -> Self {
        self.input = Some(value);
        self
    }

    pub fn calendars(mut self, value: Vec<ScheduleCalendar>) -> Self {
        self.calendars = Some(value);
        self
    }

    pub fn intervals(mut self, value: Vec<ScheduleInterval>) -> Self {
        self.intervals = Some(value);
        self
    }

    pub fn cron_expressions(mut self, value: Vec<String>) -> Self {
        self.cron_expressions = Some(value);
        self
    }

    pub fn skip(mut self, value: Vec<ScheduleCalendar>) -> Self {
        self.skip = Some(value);
        self
    }

    pub fn start_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_at = Some(value);
        self
    }

    pub fn end_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_at = Some(value);
        self
    }

    pub fn jitter(mut self, value: impl Into<String>) -> Self {
        self.jitter = Some(value.into());
        self
    }

    pub fn time_zone_name(mut self, value: impl Into<String>) -> Self {
        self.time_zone_name = Some(value.into());
        self
    }

    pub fn policy(mut self, value: SchedulePolicy) -> Self {
        self.policy = Some(value);
        self
    }

    pub fn schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = Some(value.into());
        self
    }

    pub fn remaining_executions(mut self, value: i64) -> Self {
        self.remaining_executions = Some(value);
        self
    }

    pub fn workflow_name(mut self, value: impl Into<String>) -> Self {
        self.workflow_name = Some(value.into());
        self
    }

    pub fn deployment_name(mut self, value: impl Into<String>) -> Self {
        self.deployment_name = Some(value.into());
        self
    }

    pub fn paused(mut self, value: bool) -> Self {
        self.paused = Some(value);
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn future_executions(mut self, value: Vec<ScheduleFutureExecution>) -> Self {
        self.future_executions = Some(value);
        self
    }

    pub fn recent_executions(mut self, value: Vec<ScheduleRecentExecution>) -> Self {
        self.recent_executions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScheduleDefinitionOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](ScheduleDefinitionOutputBuilder::input)
    /// - [`schedule_id`](ScheduleDefinitionOutputBuilder::schedule_id)
    /// - [`workflow_name`](ScheduleDefinitionOutputBuilder::workflow_name)
    /// - [`paused`](ScheduleDefinitionOutputBuilder::paused)
    pub fn build(self) -> Result<ScheduleDefinitionOutput, BuildError> {
        Ok(ScheduleDefinitionOutput {
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
            calendars: self.calendars,
            intervals: self.intervals,
            cron_expressions: self.cron_expressions,
            skip: self.skip,
            start_at: self.start_at,
            end_at: self.end_at,
            jitter: self.jitter,
            time_zone_name: self.time_zone_name,
            policy: self.policy,
            schedule_id: self
                .schedule_id
                .ok_or_else(|| BuildError::missing_field("schedule_id"))?,
            remaining_executions: self.remaining_executions,
            workflow_name: self
                .workflow_name
                .ok_or_else(|| BuildError::missing_field("workflow_name"))?,
            deployment_name: self.deployment_name,
            paused: self
                .paused
                .ok_or_else(|| BuildError::missing_field("paused"))?,
            note: self.note,
            future_executions: self.future_executions,
            recent_executions: self.recent_executions,
        })
    }
}
