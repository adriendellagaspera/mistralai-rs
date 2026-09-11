///Typed error responses for `update_voice_v1_audio_voices__voice_id__patch`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum UpdateVoiceV1AudioVoicesVoiceIdPatchApiError {
    Status422(HTTPValidationError),
}
