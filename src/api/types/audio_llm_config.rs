pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioLlmConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl AudioLlmConfig {
    pub fn builder() -> AudioLlmConfigBuilder {
        <AudioLlmConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioLlmConfigBuilder {
    model: Option<String>,
}

impl AudioLlmConfigBuilder {
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioLlmConfig`].
    pub fn build(self) -> Result<AudioLlmConfig, BuildError> {
        Ok(AudioLlmConfig { model: self.model })
    }
}
