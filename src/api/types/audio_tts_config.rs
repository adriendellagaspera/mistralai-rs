pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AudioTtsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expressiveness: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

impl AudioTtsConfig {
    pub fn builder() -> AudioTtsConfigBuilder {
        <AudioTtsConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioTtsConfigBuilder {
    expressiveness: Option<f64>,
    model: Option<String>,
    voice_id: Option<String>,
}

impl AudioTtsConfigBuilder {
    pub fn expressiveness(mut self, value: f64) -> Self {
        self.expressiveness = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioTtsConfig`].
    pub fn build(self) -> Result<AudioTtsConfig, BuildError> {
        Ok(AudioTtsConfig {
            expressiveness: self.expressiveness,
            model: self.model,
            voice_id: self.voice_id,
        })
    }
}
