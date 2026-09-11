///Typed error responses for `speech_v1_audio_speech_post_stream`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum SpeechV1AudioSpeechPostStreamApiError {
    Status422(HTTPValidationError),
}
