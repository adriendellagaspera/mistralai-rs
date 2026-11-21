pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AudioLatencyBreakdownUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_of_turn_delay_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_delay_ms: Option<f64>,
}

impl AudioLatencyBreakdownUser {
    pub fn builder() -> AudioLatencyBreakdownUserBuilder {
        <AudioLatencyBreakdownUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioLatencyBreakdownUserBuilder {
    end_of_turn_delay_ms: Option<f64>,
    transcription_delay_ms: Option<f64>,
}

impl AudioLatencyBreakdownUserBuilder {
    pub fn end_of_turn_delay_ms(mut self, value: f64) -> Self {
        self.end_of_turn_delay_ms = Some(value);
        self
    }

    pub fn transcription_delay_ms(mut self, value: f64) -> Self {
        self.transcription_delay_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioLatencyBreakdownUser`].
    pub fn build(self) -> Result<AudioLatencyBreakdownUser, BuildError> {
        Ok(AudioLatencyBreakdownUser {
            end_of_turn_delay_ms: self.end_of_turn_delay_ms,
            transcription_delay_ms: self.transcription_delay_ms,
        })
    }
}
