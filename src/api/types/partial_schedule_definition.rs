pub use crate::prelude::*;

/// Schedule definition for partial updates.
///
/// All fields are optional (inherited from _ScheduleRequestBase). Only explicitly-set
/// fields are applied during an update; unset fields preserve the existing schedule values.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PartialScheduleDefinition {
    /// Calendar-based specification of times.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendars: Option<Vec<ScheduleCalendar>>,
    /// Cron-based specification of times.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_expressions: Option<Vec<String>>,
    /// Time after which no more actions will be run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<DateTime<FixedOffset>>,
    /// Input to provide to the workflow when starting it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    /// Interval-based specification of times.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intervals: Option<Vec<ScheduleInterval>>,
    /// Jitter to apply each action.
    ///
    /// An action's scheduled time will be incremented by a random value between 0
    /// and this value if present (but not past the next schedule).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jitter: Option<String>,
    /// Maximum number of times this schedule will trigger a workflow execution. Once this limit is reached, no further executions are triggered automatically. null means unlimited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_executions: Option<i64>,
    /// Policy for the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<SchedulePolicy>,
    /// Set of calendar times to skip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<Vec<ScheduleCalendar>>,
    /// Time after which the first action may be run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<DateTime<FixedOffset>>,
    /// IANA time zone name, for example ``US/Central``.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone_name: Option<String>,
}

impl PartialScheduleDefinition {
    pub fn builder() -> PartialScheduleDefinitionBuilder {
        <PartialScheduleDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartialScheduleDefinitionBuilder {
    calendars: Option<Vec<ScheduleCalendar>>,
    cron_expressions: Option<Vec<String>>,
    end_at: Option<DateTime<FixedOffset>>,
    input: Option<serde_json::Value>,
    intervals: Option<Vec<ScheduleInterval>>,
    jitter: Option<String>,
    max_executions: Option<i64>,
    policy: Option<SchedulePolicy>,
    skip: Option<Vec<ScheduleCalendar>>,
    start_at: Option<DateTime<FixedOffset>>,
    time_zone_name: Option<String>,
}

impl PartialScheduleDefinitionBuilder {
    pub fn calendars(mut self, value: Vec<ScheduleCalendar>) -> Self {
        self.calendars = Some(value);
        self
    }

    pub fn cron_expressions(mut self, value: Vec<String>) -> Self {
        self.cron_expressions = Some(value);
        self
    }

    pub fn end_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_at = Some(value);
        self
    }

    pub fn input(mut self, value: serde_json::Value) -> Self {
        self.input = Some(value);
        self
    }

    pub fn intervals(mut self, value: Vec<ScheduleInterval>) -> Self {
        self.intervals = Some(value);
        self
    }

    pub fn jitter(mut self, value: impl Into<String>) -> Self {
        self.jitter = Some(value.into());
        self
    }

    pub fn max_executions(mut self, value: i64) -> Self {
        self.max_executions = Some(value);
        self
    }

    pub fn policy(mut self, value: SchedulePolicy) -> Self {
        self.policy = Some(value);
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

    pub fn time_zone_name(mut self, value: impl Into<String>) -> Self {
        self.time_zone_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PartialScheduleDefinition`].
    pub fn build(self) -> Result<PartialScheduleDefinition, BuildError> {
        Ok(PartialScheduleDefinition {
            calendars: self.calendars,
            cron_expressions: self.cron_expressions,
            end_at: self.end_at,
            input: self.input,
            intervals: self.intervals,
            jitter: self.jitter,
            max_executions: self.max_executions,
            policy: self.policy,
            skip: self.skip,
            start_at: self.start_at,
            time_zone_name: self.time_zone_name,
        })
    }
}
