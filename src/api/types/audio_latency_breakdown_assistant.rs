pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AudioLatencyBreakdownAssistant {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e2e_latency_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_duration_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_ttft_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts_duration_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts_ttfb_ms: Option<f64>,
}

impl AudioLatencyBreakdownAssistant {
    pub fn builder() -> AudioLatencyBreakdownAssistantBuilder {
        <AudioLatencyBreakdownAssistantBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioLatencyBreakdownAssistantBuilder {
    e2e_latency_ms: Option<f64>,
    llm_duration_ms: Option<f64>,
    llm_ttft_ms: Option<f64>,
    tts_duration_ms: Option<f64>,
    tts_ttfb_ms: Option<f64>,
}

impl AudioLatencyBreakdownAssistantBuilder {
    pub fn e2e_latency_ms(mut self, value: f64) -> Self {
        self.e2e_latency_ms = Some(value);
        self
    }

    pub fn llm_duration_ms(mut self, value: f64) -> Self {
        self.llm_duration_ms = Some(value);
        self
    }

    pub fn llm_ttft_ms(mut self, value: f64) -> Self {
        self.llm_ttft_ms = Some(value);
        self
    }

    pub fn tts_duration_ms(mut self, value: f64) -> Self {
        self.tts_duration_ms = Some(value);
        self
    }

    pub fn tts_ttfb_ms(mut self, value: f64) -> Self {
        self.tts_ttfb_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioLatencyBreakdownAssistant`].
    pub fn build(self) -> Result<AudioLatencyBreakdownAssistant, BuildError> {
        Ok(AudioLatencyBreakdownAssistant {
            e2e_latency_ms: self.e2e_latency_ms,
            llm_duration_ms: self.llm_duration_ms,
            llm_ttft_ms: self.llm_ttft_ms,
            tts_duration_ms: self.tts_duration_ms,
            tts_ttfb_ms: self.tts_ttfb_ms,
        })
    }
}
