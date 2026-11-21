pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AudioAgentConfigurationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm: Option<AudioLlmConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stt: Option<AudioSttConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AudioAgentConfigurationRequestToolsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<AudioTtsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<AudioTurnConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub welcome_message: Option<String>,
}

impl AudioAgentConfigurationRequest {
    pub fn builder() -> AudioAgentConfigurationRequestBuilder {
        <AudioAgentConfigurationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioAgentConfigurationRequestBuilder {
    instructions: Option<String>,
    llm: Option<AudioLlmConfig>,
    stt: Option<AudioSttConfig>,
    tools: Option<Vec<AudioAgentConfigurationRequestToolsItem>>,
    tts: Option<AudioTtsConfig>,
    turn: Option<AudioTurnConfig>,
    welcome_message: Option<String>,
}

impl AudioAgentConfigurationRequestBuilder {
    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn llm(mut self, value: AudioLlmConfig) -> Self {
        self.llm = Some(value);
        self
    }

    pub fn stt(mut self, value: AudioSttConfig) -> Self {
        self.stt = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<AudioAgentConfigurationRequestToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn tts(mut self, value: AudioTtsConfig) -> Self {
        self.tts = Some(value);
        self
    }

    pub fn turn(mut self, value: AudioTurnConfig) -> Self {
        self.turn = Some(value);
        self
    }

    pub fn welcome_message(mut self, value: impl Into<String>) -> Self {
        self.welcome_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudioAgentConfigurationRequest`].
    pub fn build(self) -> Result<AudioAgentConfigurationRequest, BuildError> {
        Ok(AudioAgentConfigurationRequest {
            instructions: self.instructions,
            llm: self.llm,
            stt: self.stt,
            tools: self.tools,
            tts: self.tts,
            turn: self.turn,
            welcome_message: self.welcome_message,
        })
    }
}
