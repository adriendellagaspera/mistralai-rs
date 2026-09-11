#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpeechStreamEvents {
    pub data: SpeechStreamEventsData,
    pub event: SpeechStreamEventsEvent,
}
