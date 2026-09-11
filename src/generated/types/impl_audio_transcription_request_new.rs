impl AudioTranscriptionRequest {
    /// Construct this request with every required wire field.
    pub fn new(model: String) -> Self {
        Self {
            model,
            context_bias: None,
            diarize: None,
            file: None,
            file_id: None,
            file_url: None,
            language: None,
            stream: None,
            temperature: None,
            timestamp_granularities: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(model: String) -> AudioTranscriptionRequestBuilder {
        AudioTranscriptionRequestBuilder::new(model)
    }
}
