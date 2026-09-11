#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventOut {
    ///The UNIX timestamp (in seconds) of the event.
    pub created_at: i64,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub data: Option<Option<EventOutData>>,
    ///The name of the event.
    pub name: String,
}
