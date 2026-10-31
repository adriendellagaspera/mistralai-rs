pub use crate::prelude::*;

/// Capability indicating support for tool calling during sampling.
///
/// When present in ClientCapabilities.sampling, indicates that the client
/// supports the tools and toolChoice parameters in sampling requests.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SamplingToolsCapability {}

impl SamplingToolsCapability {
    pub fn builder() -> SamplingToolsCapabilityBuilder {
        <SamplingToolsCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SamplingToolsCapabilityBuilder {}

impl SamplingToolsCapabilityBuilder {
    /// Consumes the builder and constructs a [`SamplingToolsCapability`].
    pub fn build(self) -> Result<SamplingToolsCapability, BuildError> {
        Ok(SamplingToolsCapability {})
    }
}
