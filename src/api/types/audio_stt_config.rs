pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AudioSttConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_bias: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_streaming_delay_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<AudioTurnDetectionConfig>,
}

impl AudioSttConfig {
    pub fn builder() -> AudioSttConfigBuilder {
        <AudioSttConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioSttConfigBuilder {
    context_bias: Option<Vec<String>>,
    language: Option<String>,
    model: Option<String>,
    target_streaming_delay_ms: Option<i64>,
    turn_detection: Option<AudioTurnDetectionConfig>,
}

impl AudioSttConfigBuilder {
    pub fn context_bias(mut self, value: Vec<String>) -> Self {
        self.context_bias = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn target_streaming_delay_ms(mut self, value: i64) -> Self {
        self.target_streaming_delay_ms = Some(value);
        self
    }

    pub fn turn_detection(mut self, value: AudioTurnDetectionConfig) -> Self {
        self.turn_detection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioSttConfig`].
    pub fn build(self) -> Result<AudioSttConfig, BuildError> {
        Ok(AudioSttConfig {
            context_bias: self.context_bias,
            language: self.language,
            model: self.model,
            target_streaming_delay_ms: self.target_streaming_delay_ms,
            turn_detection: self.turn_detection,
        })
    }
}
