#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TempoTraceEvent {
    ///The attributes of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<TempoTraceAttribute>>,
    ///The name of the event
    pub name: String,
    ///The time of the event in Unix nano
    #[serde(rename = "timeUnixNano")]
    pub time_unix_nano: String,
}
