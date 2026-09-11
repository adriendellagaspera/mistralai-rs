///Typed error responses for `speech_v1_audio_speech_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum SpeechV1AudioSpeechPostApiError {
    Status422(HTTPValidationError),
}
