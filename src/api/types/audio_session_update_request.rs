pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AudioSessionUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<AudioSessionUpdateRequestToolsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<AudioSessionTurnUpdate>,
}

impl AudioSessionUpdateRequest {
    pub fn builder() -> AudioSessionUpdateRequestBuilder {
        <AudioSessionUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioSessionUpdateRequestBuilder {
    instructions: Option<String>,
    tools: Option<Vec<AudioSessionUpdateRequestToolsItem>>,
    turn: Option<AudioSessionTurnUpdate>,
}

impl AudioSessionUpdateRequestBuilder {
    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn tools(mut self, value: Vec<AudioSessionUpdateRequestToolsItem>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn turn(mut self, value: AudioSessionTurnUpdate) -> Self {
        self.turn = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioSessionUpdateRequest`].
    pub fn build(self) -> Result<AudioSessionUpdateRequest, BuildError> {
        Ok(AudioSessionUpdateRequest {
            instructions: self.instructions,
            tools: self.tools,
            turn: self.turn,
        })
    }
}
