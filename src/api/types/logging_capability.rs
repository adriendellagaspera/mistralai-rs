pub use crate::prelude::*;

/// Capability for logging operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LoggingCapability {}

impl LoggingCapability {
    pub fn builder() -> LoggingCapabilityBuilder {
        <LoggingCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LoggingCapabilityBuilder {}

impl LoggingCapabilityBuilder {
    /// Consumes the builder and constructs a [`LoggingCapability`].
    pub fn build(self) -> Result<LoggingCapability, BuildError> {
        Ok(LoggingCapability {})
    }
}
