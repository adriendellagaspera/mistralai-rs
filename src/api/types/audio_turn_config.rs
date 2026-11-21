pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioTurnConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_interruptions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactivity: Option<AudioInactivityConfig>,
}

impl AudioTurnConfig {
    pub fn builder() -> AudioTurnConfigBuilder {
        <AudioTurnConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioTurnConfigBuilder {
    allow_interruptions: Option<bool>,
    inactivity: Option<AudioInactivityConfig>,
}

impl AudioTurnConfigBuilder {
    pub fn allow_interruptions(mut self, value: bool) -> Self {
        self.allow_interruptions = Some(value);
        self
    }

    pub fn inactivity(mut self, value: AudioInactivityConfig) -> Self {
        self.inactivity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioTurnConfig`].
    pub fn build(self) -> Result<AudioTurnConfig, BuildError> {
        Ok(AudioTurnConfig {
            allow_interruptions: self.allow_interruptions,
            inactivity: self.inactivity,
        })
    }
}
