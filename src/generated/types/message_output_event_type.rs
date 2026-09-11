#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum MessageOutputEventType {
    #[default]
    #[serde(rename = "message.output.delta")]
    MessageOutputDelta,
}
