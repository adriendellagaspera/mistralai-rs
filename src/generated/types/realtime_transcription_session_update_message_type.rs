#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RealtimeTranscriptionSessionUpdateMessageType {
    #[default]
    #[serde(rename = "session.update")]
    SessionUpdate,
}
