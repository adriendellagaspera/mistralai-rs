///Typed error responses for `get_voice_v1_audio_voices__voice_id__get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetVoiceV1AudioVoicesVoiceIdGetApiError {
    Status422(HTTPValidationError),
}
