#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum EventSource {
    #[default]
    #[serde(rename = "DATABASE")]
    Database,
    #[serde(rename = "LIVE")]
    Live,
}
