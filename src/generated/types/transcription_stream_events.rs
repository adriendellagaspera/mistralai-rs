#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TranscriptionStreamEvents {
    pub data: TranscriptionStreamEventsData,
    pub event: TranscriptionStreamEventTypes,
}
