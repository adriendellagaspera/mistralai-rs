pub use crate::prelude::*;

/// Specification of the times scheduled actions may occur.
///
/// The times are the union of :py:attr:`calendars`, :py:attr:`intervals`, and
/// :py:attr:`cron_expressions` excluding anything in :py:attr:`skip`.
///
/// Used for input where schedule_id is optional (can be provided or auto-generated).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScheduleDefinition {
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
    /// Maximum number of times this schedule will trigger a workflow execution. Once this limit is reached, no further executions are triggered automatically. null means unlimited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_executions: Option<i64>,
    /// Unique identifier for the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<String>,
}

impl ScheduleDefinition {
    pub fn builder() -> ScheduleDefinitionBuilder {
        <ScheduleDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleDefinitionBuilder {
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
    max_executions: Option<i64>,
    schedule_id: Option<String>,
}

impl ScheduleDefinitionBuilder {
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

    pub fn max_executions(mut self, value: i64) -> Self {
        self.max_executions = Some(value);
        self
    }

    pub fn schedule_id(mut self, value: impl Into<String>) -> Self {
        self.schedule_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScheduleDefinition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](ScheduleDefinitionBuilder::input)
    pub fn build(self) -> Result<ScheduleDefinition, BuildError> {
        Ok(ScheduleDefinition {
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
            max_executions: self.max_executions,
            schedule_id: self.schedule_id,
        })
    }
}
