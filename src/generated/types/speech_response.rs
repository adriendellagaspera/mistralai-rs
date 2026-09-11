#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpeechResponse {
    ///Base64 encoded audio data
    pub audio_data: String,
}
