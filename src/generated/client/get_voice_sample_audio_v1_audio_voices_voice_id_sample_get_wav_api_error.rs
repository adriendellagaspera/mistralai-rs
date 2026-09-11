///Typed error responses for `get_voice_sample_audio_v1_audio_voices__voice_id__sample_get_wav`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum GetVoiceSampleAudioV1AudioVoicesVoiceIdSampleGetWavApiError {
    Status422(HTTPValidationError),
}
