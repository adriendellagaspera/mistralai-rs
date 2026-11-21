pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceAgentFunctionTool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filler: Option<VoiceAgentFunctionToolFiller>,
    #[serde(default)]
    pub function: Function,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<VoiceAgentFunctionToolType>,
}

impl VoiceAgentFunctionTool {
    pub fn builder() -> VoiceAgentFunctionToolBuilder {
        <VoiceAgentFunctionToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceAgentFunctionToolBuilder {
    filler: Option<VoiceAgentFunctionToolFiller>,
    function: Option<Function>,
    timeout_ms: Option<i64>,
    r#type: Option<VoiceAgentFunctionToolType>,
}

impl VoiceAgentFunctionToolBuilder {
    pub fn filler(mut self, value: VoiceAgentFunctionToolFiller) -> Self {
        self.filler = Some(value);
        self
    }

    pub fn function(mut self, value: Function) -> Self {
        self.function = Some(value);
        self
    }

    pub fn timeout_ms(mut self, value: i64) -> Self {
        self.timeout_ms = Some(value);
        self
    }

    pub fn r#type(mut self, value: VoiceAgentFunctionToolType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceAgentFunctionTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`function`](VoiceAgentFunctionToolBuilder::function)
    pub fn build(self) -> Result<VoiceAgentFunctionTool, BuildError> {
        Ok(VoiceAgentFunctionTool {
            filler: self.filler,
            function: self
                .function
                .ok_or_else(|| BuildError::missing_field("function"))?,
            timeout_ms: self.timeout_ms,
            r#type: self.r#type,
        })
    }
}
