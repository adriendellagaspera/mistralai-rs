pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AudioAgentConfigurationResponseConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(default)]
    pub llm: AudioLlmConfig,
    #[serde(default)]
    pub stt: AudioSttConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AudioAgentConfigurationResponseConfigToolsItem>>,
    #[serde(default)]
    pub tts: AudioTtsConfig,
    #[serde(default)]
    pub turn: AudioTurnConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub welcome_message: Option<String>,
}

impl AudioAgentConfigurationResponseConfig {
    pub fn builder() -> AudioAgentConfigurationResponseConfigBuilder {
        <AudioAgentConfigurationResponseConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioAgentConfigurationResponseConfigBuilder {
    instructions: Option<String>,
    llm: Option<AudioLlmConfig>,
    stt: Option<AudioSttConfig>,
    tools: Option<Vec<AudioAgentConfigurationResponseConfigToolsItem>>,
    tts: Option<AudioTtsConfig>,
    turn: Option<AudioTurnConfig>,
    welcome_message: Option<String>,
}

impl AudioAgentConfigurationResponseConfigBuilder {
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

    pub fn tools(mut self, value: Vec<AudioAgentConfigurationResponseConfigToolsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`AudioAgentConfigurationResponseConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`llm`](AudioAgentConfigurationResponseConfigBuilder::llm)
    /// - [`stt`](AudioAgentConfigurationResponseConfigBuilder::stt)
    /// - [`tts`](AudioAgentConfigurationResponseConfigBuilder::tts)
    /// - [`turn`](AudioAgentConfigurationResponseConfigBuilder::turn)
    pub fn build(self) -> Result<AudioAgentConfigurationResponseConfig, BuildError> {
        Ok(AudioAgentConfigurationResponseConfig {
            instructions: self.instructions,
            llm: self.llm.ok_or_else(|| BuildError::missing_field("llm"))?,
            stt: self.stt.ok_or_else(|| BuildError::missing_field("stt"))?,
            tools: self.tools,
            tts: self.tts.ok_or_else(|| BuildError::missing_field("tts"))?,
            turn: self.turn.ok_or_else(|| BuildError::missing_field("turn"))?,
            welcome_message: self.welcome_message,
        })
    }
}
