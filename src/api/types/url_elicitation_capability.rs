pub use crate::prelude::*;

/// Capability for URL mode elicitation (2025-11-25+).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UrlElicitationCapability {}

impl UrlElicitationCapability {
    pub fn builder() -> UrlElicitationCapabilityBuilder {
        <UrlElicitationCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UrlElicitationCapabilityBuilder {}

impl UrlElicitationCapabilityBuilder {
    /// Consumes the builder and constructs a [`UrlElicitationCapability`].
    pub fn build(self) -> Result<UrlElicitationCapability, BuildError> {
        Ok(UrlElicitationCapability {})
    }
}
