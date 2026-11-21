pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceAgentWebSearchTool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filler: Option<VoiceAgentWebSearchToolFiller>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_configuration: Option<ToolConfiguration>,
}

impl VoiceAgentWebSearchTool {
    pub fn builder() -> VoiceAgentWebSearchToolBuilder {
        <VoiceAgentWebSearchToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceAgentWebSearchToolBuilder {
    filler: Option<VoiceAgentWebSearchToolFiller>,
    tool_configuration: Option<ToolConfiguration>,
}

impl VoiceAgentWebSearchToolBuilder {
    pub fn filler(mut self, value: VoiceAgentWebSearchToolFiller) -> Self {
        self.filler = Some(value);
        self
    }

    pub fn tool_configuration(mut self, value: ToolConfiguration) -> Self {
        self.tool_configuration = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceAgentWebSearchTool`].
    pub fn build(self) -> Result<VoiceAgentWebSearchTool, BuildError> {
        Ok(VoiceAgentWebSearchTool {
            filler: self.filler,
            tool_configuration: self.tool_configuration,
        })
    }
}
