pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudioTranscriptionCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_bias: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diarization: Option<bool>,
    /// ISO 639-1 codes accepted by `language`. Empty if it cannot be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    /// Supported modes: `offline` and/or `realtime`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<Vec<String>>,
    /// Realtime only: inclusive ranges of accepted target streaming delays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_delays_ms: Option<Vec<StreamingDelayRange>>,
    /// Offline only: `segment` and/or `word`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_granularities: Option<Vec<String>>,
    /// Realtime only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<bool>,
}

impl AudioTranscriptionCapabilities {
    pub fn builder() -> AudioTranscriptionCapabilitiesBuilder {
        <AudioTranscriptionCapabilitiesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioTranscriptionCapabilitiesBuilder {
    context_bias: Option<bool>,
    diarization: Option<bool>,
    languages: Option<Vec<String>>,
    modes: Option<Vec<String>>,
    streaming_delays_ms: Option<Vec<StreamingDelayRange>>,
    timestamp_granularities: Option<Vec<String>>,
    turn_detection: Option<bool>,
}

impl AudioTranscriptionCapabilitiesBuilder {
    pub fn context_bias(mut self, value: bool) -> Self {
        self.context_bias = Some(value);
        self
    }

    pub fn diarization(mut self, value: bool) -> Self {
        self.diarization = Some(value);
        self
    }

    pub fn languages(mut self, value: Vec<String>) -> Self {
        self.languages = Some(value);
        self
    }

    pub fn modes(mut self, value: Vec<String>) -> Self {
        self.modes = Some(value);
        self
    }

    pub fn streaming_delays_ms(mut self, value: Vec<StreamingDelayRange>) -> Self {
        self.streaming_delays_ms = Some(value);
        self
    }

    pub fn timestamp_granularities(mut self, value: Vec<String>) -> Self {
        self.timestamp_granularities = Some(value);
        self
    }

    pub fn turn_detection(mut self, value: bool) -> Self {
        self.turn_detection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioTranscriptionCapabilities`].
    pub fn build(self) -> Result<AudioTranscriptionCapabilities, BuildError> {
        Ok(AudioTranscriptionCapabilities {
            context_bias: self.context_bias,
            diarization: self.diarization,
            languages: self.languages,
            modes: self.modes,
            streaming_delays_ms: self.streaming_delays_ms,
            timestamp_granularities: self.timestamp_granularities,
            turn_detection: self.turn_detection,
        })
    }
}
