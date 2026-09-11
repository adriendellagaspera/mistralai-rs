impl SpeechRequest {
    /// Construct this request with every required wire field.
    pub fn new(input: String) -> Self {
        Self {
            input,
            model: None,
            ref_audio: None,
            response_format: None,
            stream: None,
            voice_id: None,
            additional_properties: ::std::collections::BTreeMap::new(),
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(input: String) -> SpeechRequestBuilder {
        SpeechRequestBuilder::new(input)
    }
}
