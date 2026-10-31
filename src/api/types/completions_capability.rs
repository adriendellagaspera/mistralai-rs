pub use crate::prelude::*;

/// Capability for completions operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompletionsCapability {}

impl CompletionsCapability {
    pub fn builder() -> CompletionsCapabilityBuilder {
        <CompletionsCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompletionsCapabilityBuilder {}

impl CompletionsCapabilityBuilder {
    /// Consumes the builder and constructs a [`CompletionsCapability`].
    pub fn build(self) -> Result<CompletionsCapability, BuildError> {
        Ok(CompletionsCapability {})
    }
}
