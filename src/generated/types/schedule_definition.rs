/**Specification of the times scheduled actions may occur.

The times are the union of :py:attr:`calendars`, :py:attr:`intervals`, and
:py:attr:`cron_expressions` excluding anything in :py:attr:`skip`.

Used for input where schedule_id is optional (can be provided or auto-generated).*/
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScheduleDefinition {
    ///Calendar-based specification of times.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendars: Option<Vec<ScheduleCalendar>>,
    ///Cron-based specification of times.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_expressions: Option<Vec<String>>,
    ///Time after which no more actions will be run.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub end_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    ///Input to provide to the workflow when starting it.
    pub input: serde_json::Value,
    ///Interval-based specification of times.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intervals: Option<Vec<ScheduleInterval>>,
    ///Jitter to apply each action.
    ///
    ///An action's scheduled time will be incremented by a random value between 0
    ///and this value if present (but not past the next schedule).
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub jitter: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<SchedulePolicy>,
    ///Unique identifier for the schedule.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub schedule_id: Option<Option<String>>,
    ///Set of calendar times to skip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<Vec<ScheduleCalendar>>,
    ///Time after which the first action may be run.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub start_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    ///IANA time zone name, for example ``US/Central``.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub time_zone_name: Option<Option<String>>,
}
