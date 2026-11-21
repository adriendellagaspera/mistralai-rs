pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceAgentFunctionToolRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filler: Option<VoiceAgentFunctionToolRequestFiller>,
    #[serde(default)]
    pub function: FunctionRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<i64>,
}

impl VoiceAgentFunctionToolRequest {
    pub fn builder() -> VoiceAgentFunctionToolRequestBuilder {
        <VoiceAgentFunctionToolRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceAgentFunctionToolRequestBuilder {
    filler: Option<VoiceAgentFunctionToolRequestFiller>,
    function: Option<FunctionRequest>,
    timeout_ms: Option<i64>,
}

impl VoiceAgentFunctionToolRequestBuilder {
    pub fn filler(mut self, value: VoiceAgentFunctionToolRequestFiller) -> Self {
        self.filler = Some(value);
        self
    }

    pub fn function(mut self, value: FunctionRequest) -> Self {
        self.function = Some(value);
        self
    }

    pub fn timeout_ms(mut self, value: i64) -> Self {
        self.timeout_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceAgentFunctionToolRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`function`](VoiceAgentFunctionToolRequestBuilder::function)
    pub fn build(self) -> Result<VoiceAgentFunctionToolRequest, BuildError> {
        Ok(VoiceAgentFunctionToolRequest {
            filler: self.filler,
            function: self
                .function
                .ok_or_else(|| BuildError::missing_field("function"))?,
            timeout_ms: self.timeout_ms,
        })
    }
}
