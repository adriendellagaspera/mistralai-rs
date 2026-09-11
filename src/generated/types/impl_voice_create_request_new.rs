impl VoiceCreateRequest {
    /// Construct this request with every required wire field.
    pub fn new(name: String, sample_audio: String) -> Self {
        Self {
            name,
            sample_audio,
            age: None,
            color: None,
            gender: None,
            languages: None,
            retention_notice: None,
            sample_filename: None,
            slug: None,
            tags: None,
        }
    }
    /// Start a dependency-free builder with every required wire field.
    pub fn builder(name: String, sample_audio: String) -> VoiceCreateRequestBuilder {
        VoiceCreateRequestBuilder::new(name, sample_audio)
    }
}
