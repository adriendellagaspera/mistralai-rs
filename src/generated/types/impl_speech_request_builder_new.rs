impl SpeechRequestBuilder {
    /// Start a builder with every required wire field.
    pub fn new(input: String) -> Self {
        Self {
            value: SpeechRequest::new(input),
        }
    }
    #[doc = concat!(
        "Set the optional nullable `", "model", "` request field to a value."
    )]
    #[must_use]
    pub fn model(mut self, model: String) -> Self {
        self.value.model = Some(Some(model));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "model", "` request field to JSON null."
    )]
    #[must_use]
    pub fn model_null(mut self) -> Self {
        self.value.model = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "model", "` request field.")]
    #[must_use]
    pub fn model_absent(mut self) -> Self {
        self.value.model = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "ref_audio", "` request field to a value."
    )]
    #[must_use]
    pub fn ref_audio(mut self, ref_audio: String) -> Self {
        self.value.ref_audio = Some(Some(ref_audio));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "ref_audio", "` request field to JSON null."
    )]
    #[must_use]
    pub fn ref_audio_null(mut self) -> Self {
        self.value.ref_audio = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "ref_audio", "` request field.")]
    #[must_use]
    pub fn ref_audio_absent(mut self) -> Self {
        self.value.ref_audio = None;
        self
    }
    #[doc = concat!("Set the optional `", "response_format", "` request field.")]
    #[must_use]
    pub fn response_format(mut self, response_format: SpeechOutputFormat) -> Self {
        self.value.response_format = Some(response_format);
        self
    }
    #[doc = concat!("Set the optional `", "stream", "` request field.")]
    #[must_use]
    pub fn stream(mut self, stream: bool) -> Self {
        self.value.stream = Some(stream);
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "voice_id", "` request field to a value."
    )]
    #[must_use]
    pub fn voice_id(mut self, voice_id: String) -> Self {
        self.value.voice_id = Some(Some(voice_id));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "voice_id", "` request field to JSON null."
    )]
    #[must_use]
    pub fn voice_id_null(mut self) -> Self {
        self.value.voice_id = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "voice_id", "` request field.")]
    #[must_use]
    pub fn voice_id_absent(mut self) -> Self {
        self.value.voice_id = None;
        self
    }
    /// Replace the request's additional properties.
    #[must_use]
    pub fn additional_properties(
        mut self,
        additional_properties: ::std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.value.additional_properties = additional_properties;
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> SpeechRequest {
        self.value
    }
}
