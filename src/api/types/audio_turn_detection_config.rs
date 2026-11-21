pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AudioTurnDetectionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eager_end_threshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_threshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timeout_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_silence_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_threshold: Option<f64>,
}

impl AudioTurnDetectionConfig {
    pub fn builder() -> AudioTurnDetectionConfigBuilder {
        <AudioTurnDetectionConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioTurnDetectionConfigBuilder {
    eager_end_threshold: Option<f64>,
    end_threshold: Option<f64>,
    end_timeout_ms: Option<i64>,
    min_silence_ms: Option<i64>,
    start_threshold: Option<f64>,
}

impl AudioTurnDetectionConfigBuilder {
    pub fn eager_end_threshold(mut self, value: f64) -> Self {
        self.eager_end_threshold = Some(value);
        self
    }

    pub fn end_threshold(mut self, value: f64) -> Self {
        self.end_threshold = Some(value);
        self
    }

    pub fn end_timeout_ms(mut self, value: i64) -> Self {
        self.end_timeout_ms = Some(value);
        self
    }

    pub fn min_silence_ms(mut self, value: i64) -> Self {
        self.min_silence_ms = Some(value);
        self
    }

    pub fn start_threshold(mut self, value: f64) -> Self {
        self.start_threshold = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioTurnDetectionConfig`].
    pub fn build(self) -> Result<AudioTurnDetectionConfig, BuildError> {
        Ok(AudioTurnDetectionConfig {
            eager_end_threshold: self.eager_end_threshold,
            end_threshold: self.end_threshold,
            end_timeout_ms: self.end_timeout_ms,
            min_silence_ms: self.min_silence_ms,
            start_threshold: self.start_threshold,
        })
    }
}
