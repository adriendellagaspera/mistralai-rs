pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpeechStreamAudioDelta {
    #[serde(default)]
    pub audio_data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SpeechStreamAudioDeltaType>,
}

impl SpeechStreamAudioDelta {
    pub fn builder() -> SpeechStreamAudioDeltaBuilder {
        <SpeechStreamAudioDeltaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeechStreamAudioDeltaBuilder {
    audio_data: Option<String>,
    r#type: Option<SpeechStreamAudioDeltaType>,
}

impl SpeechStreamAudioDeltaBuilder {
    pub fn audio_data(mut self, value: impl Into<String>) -> Self {
        self.audio_data = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: SpeechStreamAudioDeltaType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpeechStreamAudioDelta`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_data`](SpeechStreamAudioDeltaBuilder::audio_data)
    pub fn build(self) -> Result<SpeechStreamAudioDelta, BuildError> {
        Ok(SpeechStreamAudioDelta {
            audio_data: self
                .audio_data
                .ok_or_else(|| BuildError::missing_field("audio_data"))?,
            r#type: self.r#type,
        })
    }
}
