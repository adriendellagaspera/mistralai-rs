pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpeechRequest {
    /// Prior turns, oldest first, alternating and ending on a user turn: `input` is the assistant reply to it. Conditions delivery, not speaker: set `voice_id` to choose the speaker. Requires `voice_id` to be a preset voice whose `kind` is `pvc`. Cannot be combined with `ref_audio`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_history: Option<Vec<SpeechRequestConversationHistoryItem>>,
    /// Speaker flow-matching classifier-free guidance scale. Must be finite and at least 1.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expressiveness: Option<f64>,
    /// Text to generate a speech from
    #[serde(default)]
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataDict>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Optional pronunciation hints mapping a (case-sensitive) word to its ordered list of IPA phones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phoneme_hints: Option<HashMap<String, Option<Vec<IpaPhoneme>>>>,
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
    conversation_history: Option<Vec<SpeechRequestConversationHistoryItem>>,
    expressiveness: Option<f64>,
    input: Option<String>,
    metadata: Option<MetadataDict>,
    model: Option<String>,
    phoneme_hints: Option<HashMap<String, Option<Vec<IpaPhoneme>>>>,
    prompt_cache_key: Option<String>,
    ref_audio: Option<String>,
    response_format: Option<SpeechOutputFormat>,
    stream: Option<bool>,
    voice_id: Option<String>,
}

impl SpeechRequestBuilder {
    pub fn conversation_history(
        mut self,
        value: Vec<SpeechRequestConversationHistoryItem>,
    ) -> Self {
        self.conversation_history = Some(value);
        self
    }

    pub fn expressiveness(mut self, value: f64) -> Self {
        self.expressiveness = Some(value);
        self
    }

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

    pub fn phoneme_hints(mut self, value: HashMap<String, Option<Vec<IpaPhoneme>>>) -> Self {
        self.phoneme_hints = Some(value);
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
            conversation_history: self.conversation_history,
            expressiveness: self.expressiveness,
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
            metadata: self.metadata,
            model: self.model,
            phoneme_hints: self.phoneme_hints,
            prompt_cache_key: self.prompt_cache_key,
            ref_audio: self.ref_audio,
            response_format: self.response_format,
            stream: self.stream,
            voice_id: self.voice_id,
        })
    }
}
