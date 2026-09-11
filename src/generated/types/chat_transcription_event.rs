#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatTranscriptionEvent {
    pub audio_url: String,
    pub model: String,
    pub response_message: ChatTranscriptionEventResponseMessage,
}
