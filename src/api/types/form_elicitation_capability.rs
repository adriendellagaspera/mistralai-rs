pub use crate::prelude::*;

/// Capability for form mode elicitation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FormElicitationCapability {}

impl FormElicitationCapability {
    pub fn builder() -> FormElicitationCapabilityBuilder {
        <FormElicitationCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FormElicitationCapabilityBuilder {}

impl FormElicitationCapabilityBuilder {
    /// Consumes the builder and constructs a [`FormElicitationCapability`].
    pub fn build(self) -> Result<FormElicitationCapability, BuildError> {
        Ok(FormElicitationCapability {})
    }
}
