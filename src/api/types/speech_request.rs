pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpeechRequest {
    /// Text to generate a speech from
    #[serde(default)]
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataDict>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    /// The audio reference for generating the speech.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_audio: Option<String>,
    /// Output audio format. Defaults to mp3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<SpeechOutputFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// The preset or custom voice to use for generating the speech.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

impl SpeechRequest {
    pub fn builder() -> SpeechRequestBuilder {
        <SpeechRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeechRequestBuilder {
    input: Option<String>,
    metadata: Option<MetadataDict>,
    model: Option<String>,
    prompt_cache_key: Option<String>,
    ref_audio: Option<String>,
    response_format: Option<SpeechOutputFormat>,
    stream: Option<bool>,
    voice_id: Option<String>,
}

impl SpeechRequestBuilder {
    pub fn input(mut self, value: impl Into<String>) -> Self {
        self.input = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: MetadataDict) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn prompt_cache_key(mut self, value: impl Into<String>) -> Self {
        self.prompt_cache_key = Some(value.into());
        self
    }

    pub fn ref_audio(mut self, value: impl Into<String>) -> Self {
        self.ref_audio = Some(value.into());
        self
    }

    pub fn response_format(mut self, value: SpeechOutputFormat) -> Self {
        self.response_format = Some(value);
        self
    }

    pub fn stream(mut self, value: bool) -> Self {
        self.stream = Some(value);
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpeechRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](SpeechRequestBuilder::input)
    pub fn build(self) -> Result<SpeechRequest, BuildError> {
        Ok(SpeechRequest {
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
            metadata: self.metadata,
            model: self.model,
            prompt_cache_key: self.prompt_cache_key,
            ref_audio: self.ref_audio,
            response_format: self.response_format,
            stream: self.stream,
            voice_id: self.voice_id,
        })
    }
}
