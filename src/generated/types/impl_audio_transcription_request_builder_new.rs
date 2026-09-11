impl AudioTranscriptionRequestBuilder {
    /// Start a builder with every required wire field.
    pub fn new(model: String) -> Self {
        Self {
            value: AudioTranscriptionRequest::new(model),
        }
    }
    #[doc = concat!("Set the optional `", "context_bias", "` request field.")]
    #[must_use]
    pub fn context_bias(mut self, context_bias: Vec<String>) -> Self {
        self.value.context_bias = Some(context_bias);
        self
    }
    #[doc = concat!("Set the optional `", "diarize", "` request field.")]
    #[must_use]
    pub fn diarize(mut self, diarize: bool) -> Self {
        self.value.diarize = Some(diarize);
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "file", "` request field to a value."
    )]
    #[must_use]
    pub fn file(mut self, file: File) -> Self {
        self.value.file = Some(Some(file));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "file", "` request field to JSON null."
    )]
    #[must_use]
    pub fn file_null(mut self) -> Self {
        self.value.file = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "file", "` request field.")]
    #[must_use]
    pub fn file_absent(mut self) -> Self {
        self.value.file = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "file_id", "` request field to a value."
    )]
    #[must_use]
    pub fn file_id(mut self, file_id: String) -> Self {
        self.value.file_id = Some(Some(file_id));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "file_id", "` request field to JSON null."
    )]
    #[must_use]
    pub fn file_id_null(mut self) -> Self {
        self.value.file_id = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "file_id", "` request field.")]
    #[must_use]
    pub fn file_id_absent(mut self) -> Self {
        self.value.file_id = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "file_url", "` request field to a value."
    )]
    #[must_use]
    pub fn file_url(mut self, file_url: url::Url) -> Self {
        self.value.file_url = Some(Some(file_url));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "file_url", "` request field to JSON null."
    )]
    #[must_use]
    pub fn file_url_null(mut self) -> Self {
        self.value.file_url = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "file_url", "` request field.")]
    #[must_use]
    pub fn file_url_absent(mut self) -> Self {
        self.value.file_url = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "language", "` request field to a value."
    )]
    #[must_use]
    pub fn language(mut self, language: String) -> Self {
        self.value.language = Some(Some(language));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "language", "` request field to JSON null."
    )]
    #[must_use]
    pub fn language_null(mut self) -> Self {
        self.value.language = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "language", "` request field.")]
    #[must_use]
    pub fn language_absent(mut self) -> Self {
        self.value.language = None;
        self
    }
    #[doc = concat!("Set the optional `", "stream", "` request field.")]
    #[must_use]
    pub fn stream(mut self, stream: bool) -> Self {
        self.value.stream = Some(stream);
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "temperature", "` request field to a value."
    )]
    #[must_use]
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.value.temperature = Some(Some(temperature));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "temperature", "` request field to JSON null."
    )]
    #[must_use]
    pub fn temperature_null(mut self) -> Self {
        self.value.temperature = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "temperature", "` request field.")]
    #[must_use]
    pub fn temperature_absent(mut self) -> Self {
        self.value.temperature = None;
        self
    }
    #[doc = concat!("Set the optional `", "timestamp_granularities", "` request field.")]
    #[must_use]
    pub fn timestamp_granularities(
        mut self,
        timestamp_granularities: Vec<TimestampGranularity>,
    ) -> Self {
        self.value.timestamp_granularities = Some(timestamp_granularities);
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> AudioTranscriptionRequest {
        self.value
    }
}
