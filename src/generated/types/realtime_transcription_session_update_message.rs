#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RealtimeTranscriptionSessionUpdateMessage {
    pub session: RealtimeTranscriptionSessionUpdatePayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RealtimeTranscriptionSessionUpdateMessageType>,
}
