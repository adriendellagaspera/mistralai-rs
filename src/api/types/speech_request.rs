pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpeechRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataDict>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    /// The preset or custom voice to use for generating the speech.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    /// The audio reference for generating the speech.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_audio: Option<String>,
    /// Text to generate a speech from
    #[serde(default)]
    pub input: String,
    /// Output audio format. Defaults to mp3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<SpeechOutputFormat>,
}

impl SpeechRequest {
    pub fn builder() -> SpeechRequestBuilder {
        <SpeechRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeechRequestBuilder {
    model: Option<String>,
    metadata: Option<MetadataDict>,
    stream: Option<bool>,
    prompt_cache_key: Option<String>,
    voice_id: Option<String>,
    ref_audio: Option<String>,
    input: Option<String>,
    response_format: Option<SpeechOutputFormat>,
}

impl SpeechRequestBuilder {
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: MetadataDict) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn stream(mut self, value: bool) -> Self {
        self.stream = Some(value);
        self
    }

    pub fn prompt_cache_key(mut self, value: impl Into<String>) -> Self {
        self.prompt_cache_key = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn ref_audio(mut self, value: impl Into<String>) -> Self {
        self.ref_audio = Some(value.into());
        self
    }

    pub fn input(mut self, value: impl Into<String>) -> Self {
        self.input = Some(value.into());
        self
    }

    pub fn response_format(mut self, value: SpeechOutputFormat) -> Self {
        self.response_format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpeechRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](SpeechRequestBuilder::input)
    pub fn build(self) -> Result<SpeechRequest, BuildError> {
        Ok(SpeechRequest {
            model: self.model,
            metadata: self.metadata,
            stream: self.stream,
            prompt_cache_key: self.prompt_cache_key,
            voice_id: self.voice_id,
            ref_audio: self.ref_audio,
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
            response_format: self.response_format,
        })
    }
}
