pub use crate::prelude::*;

/// Sampling capability structure, allowing fine-grained capability advertisement.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SamplingCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<SamplingContextCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<SamplingToolsCapability>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl SamplingCapability {
    pub fn builder() -> SamplingCapabilityBuilder {
        <SamplingCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SamplingCapabilityBuilder {
    context: Option<SamplingContextCapability>,
    tools: Option<SamplingToolsCapability>,
}

impl SamplingCapabilityBuilder {
    pub fn context(mut self, value: SamplingContextCapability) -> Self {
        self.context = Some(value);
        self
    }

    pub fn tools(mut self, value: SamplingToolsCapability) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SamplingCapability`].
    pub fn build(self) -> Result<SamplingCapability, BuildError> {
        Ok(SamplingCapability {
            context: self.context,
            tools: self.tools,
            extra: Default::default(),
        })
    }
}
