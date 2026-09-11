#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScheduleInterval {
    pub every: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub offset: Option<Option<String>>,
}
