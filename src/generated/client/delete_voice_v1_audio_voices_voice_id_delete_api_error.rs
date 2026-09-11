///Typed error responses for `delete_voice_v1_audio_voices__voice_id__delete`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum DeleteVoiceV1AudioVoicesVoiceIdDeleteApiError {
    Status422(HTTPValidationError),
}
