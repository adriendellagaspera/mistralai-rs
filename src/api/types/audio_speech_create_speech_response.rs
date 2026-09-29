pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSpeechResponse {
    /// Base64 encoded audio data
    #[serde(default)]
    pub audio_data: String,
}

impl CreateSpeechResponse {
    pub fn builder() -> CreateSpeechResponseBuilder {
        <CreateSpeechResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSpeechResponseBuilder {
    audio_data: Option<String>,
}

impl CreateSpeechResponseBuilder {
    pub fn audio_data(mut self, value: impl Into<String>) -> Self {
        self.audio_data = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSpeechResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_data`](CreateSpeechResponseBuilder::audio_data)
    pub fn build(self) -> Result<CreateSpeechResponse, BuildError> {
        Ok(CreateSpeechResponse {
            audio_data: self
                .audio_data
                .ok_or_else(|| BuildError::missing_field("audio_data"))?,
        })
    }
}
