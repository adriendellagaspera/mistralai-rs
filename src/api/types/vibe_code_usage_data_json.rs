pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VibeCodeUsageDataJson {
    /// Vibe Code audio transcription usage data.
    #[serde(default)]
    pub audio: BasicModelUsageDataJson,
    /// Vibe Code text-to-speech usage data.
    #[serde(default)]
    pub audio_characters: BasicModelUsageDataJson,
    /// Vibe Code completion (token) usage data.
    #[serde(default)]
    pub completion: BasicModelUsageDataJson,
    /// Vibe Code connectors usage data.
    #[serde(default)]
    pub connectors: BasicModelUsageDataJson,
    /// Vibe Code OCR usage data.
    #[serde(default)]
    pub ocr: BasicModelUsageDataJson,
}

impl VibeCodeUsageDataJson {
    pub fn builder() -> VibeCodeUsageDataJsonBuilder {
        <VibeCodeUsageDataJsonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VibeCodeUsageDataJsonBuilder {
    audio: Option<BasicModelUsageDataJson>,
    audio_characters: Option<BasicModelUsageDataJson>,
    completion: Option<BasicModelUsageDataJson>,
    connectors: Option<BasicModelUsageDataJson>,
    ocr: Option<BasicModelUsageDataJson>,
}

impl VibeCodeUsageDataJsonBuilder {
    pub fn audio(mut self, value: BasicModelUsageDataJson) -> Self {
        self.audio = Some(value);
        self
    }

    pub fn audio_characters(mut self, value: BasicModelUsageDataJson) -> Self {
        self.audio_characters = Some(value);
        self
    }

    pub fn completion(mut self, value: BasicModelUsageDataJson) -> Self {
        self.completion = Some(value);
        self
    }

    pub fn connectors(mut self, value: BasicModelUsageDataJson) -> Self {
        self.connectors = Some(value);
        self
    }

    pub fn ocr(mut self, value: BasicModelUsageDataJson) -> Self {
        self.ocr = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VibeCodeUsageDataJson`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio`](VibeCodeUsageDataJsonBuilder::audio)
    /// - [`audio_characters`](VibeCodeUsageDataJsonBuilder::audio_characters)
    /// - [`completion`](VibeCodeUsageDataJsonBuilder::completion)
    /// - [`connectors`](VibeCodeUsageDataJsonBuilder::connectors)
    /// - [`ocr`](VibeCodeUsageDataJsonBuilder::ocr)
    pub fn build(self) -> Result<VibeCodeUsageDataJson, BuildError> {
        Ok(VibeCodeUsageDataJson {
            audio: self
                .audio
                .ok_or_else(|| BuildError::missing_field("audio"))?,
            audio_characters: self
                .audio_characters
                .ok_or_else(|| BuildError::missing_field("audio_characters"))?,
            completion: self
                .completion
                .ok_or_else(|| BuildError::missing_field("completion"))?,
            connectors: self
                .connectors
                .ok_or_else(|| BuildError::missing_field("connectors"))?,
            ocr: self.ocr.ok_or_else(|| BuildError::missing_field("ocr"))?,
        })
    }
}
