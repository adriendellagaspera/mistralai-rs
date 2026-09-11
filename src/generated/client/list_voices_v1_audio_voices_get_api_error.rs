///Typed error responses for `list_voices_v1_audio_voices_get`. One variant per declared non-2xx response.
#[derive(Debug, Clone)]
pub enum ListVoicesV1AudioVoicesGetApiError {
    Status422(HTTPValidationError),
}
