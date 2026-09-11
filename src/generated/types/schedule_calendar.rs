#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ScheduleCalendar {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub comment: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<Vec<ScheduleRange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<Vec<ScheduleRange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour: Option<Vec<ScheduleRange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute: Option<Vec<ScheduleRange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<Vec<ScheduleRange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second: Option<Vec<ScheduleRange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<Vec<ScheduleRange>>,
}
