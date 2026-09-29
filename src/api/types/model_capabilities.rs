pub use crate::prelude::*;

/// This is populated by Harmattan, but some fields have a name
/// that we don't want to expose in the API.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ModelCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_speech: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_transcription: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_transcription_realtime: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_chat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_fim: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fine_tuning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_calling: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified_resources: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vision: Option<bool>,
}

impl ModelCapabilities {
    pub fn builder() -> ModelCapabilitiesBuilder {
        <ModelCapabilitiesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModelCapabilitiesBuilder {
    audio: Option<bool>,
    audio_speech: Option<bool>,
    audio_transcription: Option<bool>,
    audio_transcription_realtime: Option<bool>,
    classification: Option<bool>,
    completion_chat: Option<bool>,
    completion_fim: Option<bool>,
    fine_tuning: Option<bool>,
    function_calling: Option<bool>,
    moderation: Option<bool>,
    ocr: Option<bool>,
    reasoning: Option<bool>,
    unified_resources: Option<bool>,
    vision: Option<bool>,
}

impl ModelCapabilitiesBuilder {
    pub fn audio(mut self, value: bool) -> Self {
        self.audio = Some(value);
        self
    }

    pub fn audio_speech(mut self, value: bool) -> Self {
        self.audio_speech = Some(value);
        self
    }

    pub fn audio_transcription(mut self, value: bool) -> Self {
        self.audio_transcription = Some(value);
        self
    }

    pub fn audio_transcription_realtime(mut self, value: bool) -> Self {
        self.audio_transcription_realtime = Some(value);
        self
    }

    pub fn classification(mut self, value: bool) -> Self {
        self.classification = Some(value);
        self
    }

    pub fn completion_chat(mut self, value: bool) -> Self {
        self.completion_chat = Some(value);
        self
    }

    pub fn completion_fim(mut self, value: bool) -> Self {
        self.completion_fim = Some(value);
        self
    }

    pub fn fine_tuning(mut self, value: bool) -> Self {
        self.fine_tuning = Some(value);
        self
    }

    pub fn function_calling(mut self, value: bool) -> Self {
        self.function_calling = Some(value);
        self
    }

    pub fn moderation(mut self, value: bool) -> Self {
        self.moderation = Some(value);
        self
    }

    pub fn ocr(mut self, value: bool) -> Self {
        self.ocr = Some(value);
        self
    }

    pub fn reasoning(mut self, value: bool) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn unified_resources(mut self, value: bool) -> Self {
        self.unified_resources = Some(value);
        self
    }

    pub fn vision(mut self, value: bool) -> Self {
        self.vision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModelCapabilities`].
    pub fn build(self) -> Result<ModelCapabilities, BuildError> {
        Ok(ModelCapabilities {
            audio: self.audio,
            audio_speech: self.audio_speech,
            audio_transcription: self.audio_transcription,
            audio_transcription_realtime: self.audio_transcription_realtime,
            classification: self.classification,
            completion_chat: self.completion_chat,
            completion_fim: self.completion_fim,
            fine_tuning: self.fine_tuning,
            function_calling: self.function_calling,
            moderation: self.moderation,
            ocr: self.ocr,
            reasoning: self.reasoning,
            unified_resources: self.unified_resources,
            vision: self.vision,
        })
    }
}
