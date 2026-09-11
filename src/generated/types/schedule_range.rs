#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScheduleRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    pub start: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<i64>,
}
