pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioSpeechCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<bool>,
    /// Supported modes: `offline` and/or `realtime`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<Vec<String>>,
    /// Accepted `response_format` values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_formats: Option<Vec<String>>,
}

impl AudioSpeechCapabilities {
    pub fn builder() -> AudioSpeechCapabilitiesBuilder {
        <AudioSpeechCapabilitiesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioSpeechCapabilitiesBuilder {
    instruction: Option<bool>,
    modes: Option<Vec<String>>,
    response_formats: Option<Vec<String>>,
}

impl AudioSpeechCapabilitiesBuilder {
    pub fn instruction(mut self, value: bool) -> Self {
        self.instruction = Some(value);
        self
    }

    pub fn modes(mut self, value: Vec<String>) -> Self {
        self.modes = Some(value);
        self
    }

    pub fn response_formats(mut self, value: Vec<String>) -> Self {
        self.response_formats = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioSpeechCapabilities`].
    pub fn build(self) -> Result<AudioSpeechCapabilities, BuildError> {
        Ok(AudioSpeechCapabilities {
            instruction: self.instruction,
            modes: self.modes,
            response_formats: self.response_formats,
        })
    }
}
