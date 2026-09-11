#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum EventType {
    #[default]
    #[serde(rename = "EVENT")]
    Event,
    #[serde(rename = "EVENT_PROGRESS")]
    EventProgress,
}
