pub use crate::prelude::*;

/// Capability for context inclusion during sampling.
///
/// Indicates support for non-'none' values in the includeContext parameter.
/// SOFT-DEPRECATED: New implementations should use tools parameter instead.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SamplingContextCapability {}

impl SamplingContextCapability {
    pub fn builder() -> SamplingContextCapabilityBuilder {
        <SamplingContextCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SamplingContextCapabilityBuilder {}

impl SamplingContextCapabilityBuilder {
    /// Consumes the builder and constructs a [`SamplingContextCapability`].
    pub fn build(self) -> Result<SamplingContextCapability, BuildError> {
        Ok(SamplingContextCapability {})
    }
}
