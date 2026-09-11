///Typed error responses for `create_voice_v1_audio_voices_post`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum CreateVoiceV1AudioVoicesPostApiError {
    Status422(HTTPValidationError),
}
