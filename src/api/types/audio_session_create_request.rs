pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AudioSessionCreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm: Option<AudioLlmConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stt: Option<AudioSttConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AudioSessionCreateRequestToolsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<AudioTtsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<AudioTurnConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub welcome_message: Option<String>,
}

impl AudioSessionCreateRequest {
    pub fn builder() -> AudioSessionCreateRequestBuilder {
        <AudioSessionCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioSessionCreateRequestBuilder {
    agent_id: Option<String>,
    agent_version: Option<i64>,
    instructions: Option<String>,
    llm: Option<AudioLlmConfig>,
    stt: Option<AudioSttConfig>,
    tools: Option<Vec<AudioSessionCreateRequestToolsItem>>,
    tts: Option<AudioTtsConfig>,
    turn: Option<AudioTurnConfig>,
    welcome_message: Option<String>,
}

impl AudioSessionCreateRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn agent_version(mut self, value: i64) -> Self {
        self.agent_version = Some(value);
        self
    }

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

    pub fn tools(mut self, value: Vec<AudioSessionCreateRequestToolsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`AudioSessionCreateRequest`].
    pub fn build(self) -> Result<AudioSessionCreateRequest, BuildError> {
        Ok(AudioSessionCreateRequest {
            agent_id: self.agent_id,
            agent_version: self.agent_version,
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
