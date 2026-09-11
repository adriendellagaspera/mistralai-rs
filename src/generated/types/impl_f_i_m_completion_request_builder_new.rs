impl FIMCompletionRequestBuilder {
    /// Start a builder with every required wire field.
    pub fn new(model: String, prompt: String) -> Self {
        Self {
            value: FIMCompletionRequest::new(model, prompt),
        }
    }
    #[doc = concat!(
        "Set the optional nullable `", "max_tokens", "` request field to a value."
    )]
    #[must_use]
    pub fn max_tokens(mut self, max_tokens: i64) -> Self {
        self.value.max_tokens = Some(Some(max_tokens));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "max_tokens", "` request field to JSON null."
    )]
    #[must_use]
    pub fn max_tokens_null(mut self) -> Self {
        self.value.max_tokens = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "max_tokens", "` request field.")]
    #[must_use]
    pub fn max_tokens_absent(mut self) -> Self {
        self.value.max_tokens = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "metadata", "` request field to a value."
    )]
    #[must_use]
    pub fn metadata(mut self, metadata: FIMCompletionRequestMetadata) -> Self {
        self.value.metadata = Some(Some(metadata));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "metadata", "` request field to JSON null."
    )]
    #[must_use]
    pub fn metadata_null(mut self) -> Self {
        self.value.metadata = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "metadata", "` request field.")]
    #[must_use]
    pub fn metadata_absent(mut self) -> Self {
        self.value.metadata = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "min_tokens", "` request field to a value."
    )]
    #[must_use]
    pub fn min_tokens(mut self, min_tokens: i64) -> Self {
        self.value.min_tokens = Some(Some(min_tokens));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "min_tokens", "` request field to JSON null."
    )]
    #[must_use]
    pub fn min_tokens_null(mut self) -> Self {
        self.value.min_tokens = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "min_tokens", "` request field.")]
    #[must_use]
    pub fn min_tokens_absent(mut self) -> Self {
        self.value.min_tokens = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "prompt_cache_key", "` request field to a value."
    )]
    #[must_use]
    pub fn prompt_cache_key(mut self, prompt_cache_key: String) -> Self {
        self.value.prompt_cache_key = Some(Some(prompt_cache_key));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "prompt_cache_key",
        "` request field to JSON null."
    )]
    #[must_use]
    pub fn prompt_cache_key_null(mut self) -> Self {
        self.value.prompt_cache_key = Some(None);
        self
    }
    #[doc = concat!(
        "Omit the optional nullable `", "prompt_cache_key", "` request field."
    )]
    #[must_use]
    pub fn prompt_cache_key_absent(mut self) -> Self {
        self.value.prompt_cache_key = None;
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "random_seed", "` request field to a value."
    )]
    #[must_use]
    pub fn random_seed(mut self, random_seed: i64) -> Self {
        self.value.random_seed = Some(Some(random_seed));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "random_seed", "` request field to JSON null."
    )]
    #[must_use]
    pub fn random_seed_null(mut self) -> Self {
        self.value.random_seed = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "random_seed", "` request field.")]
    #[must_use]
    pub fn random_seed_absent(mut self) -> Self {
        self.value.random_seed = None;
        self
    }
    #[doc = concat!("Set the optional `", "stop", "` request field.")]
    #[must_use]
    pub fn stop(mut self, stop: FIMCompletionRequestStop) -> Self {
        self.value.stop = Some(stop);
        self
    }
    #[doc = concat!("Set the optional `", "stream", "` request field.")]
    #[must_use]
    pub fn stream(mut self, stream: bool) -> Self {
        self.value.stream = Some(stream);
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "suffix", "` request field to a value."
    )]
    #[must_use]
    pub fn suffix(mut self, suffix: String) -> Self {
        self.value.suffix = Some(Some(suffix));
        self
    }
    #[doc = concat!(
        "Set the optional nullable `", "suffix", "` request field to JSON null."
    )]
    #[must_use]
    pub fn suffix_null(mut self) -> Self {
        self.value.suffix = Some(None);
        self
    }
    #[doc = concat!("Omit the optional nullable `", "suffix", "` request field.")]
    #[must_use]
    pub fn suffix_absent(mut self) -> Self {
        self.value.suffix = None;
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
    #[doc = concat!("Set the optional `", "top_p", "` request field.")]
    #[must_use]
    pub fn top_p(mut self, top_p: f64) -> Self {
        self.value.top_p = Some(top_p);
        self
    }
    /// Finish building the request model.
    pub fn build(self) -> FIMCompletionRequest {
        self.value
    }
}
