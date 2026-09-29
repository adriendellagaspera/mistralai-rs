pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompleteSpeechResponse {
    /// Base64 encoded audio data
    #[serde(default)]
    pub audio_data: String,
}

impl CompleteSpeechResponse {
    pub fn builder() -> CompleteSpeechResponseBuilder {
        <CompleteSpeechResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompleteSpeechResponseBuilder {
    audio_data: Option<String>,
}

impl CompleteSpeechResponseBuilder {
    pub fn audio_data(mut self, value: impl Into<String>) -> Self {
        self.audio_data = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompleteSpeechResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_data`](CompleteSpeechResponseBuilder::audio_data)
    pub fn build(self) -> Result<CompleteSpeechResponse, BuildError> {
        Ok(CompleteSpeechResponse {
            audio_data: self
                .audio_data
                .ok_or_else(|| BuildError::missing_field("audio_data"))?,
        })
    }
}
