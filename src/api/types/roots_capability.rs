pub use crate::prelude::*;

/// Capability for root operations.
///
/// Deprecated in protocol 2026-07-28 (SEP-2577) but still carried there as an
/// empty object (`list_changed` exists only through 2025-11-25).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RootsCapability {
    #[serde(rename = "listChanged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

impl RootsCapability {
    pub fn builder() -> RootsCapabilityBuilder {
        <RootsCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RootsCapabilityBuilder {
    list_changed: Option<bool>,
}

impl RootsCapabilityBuilder {
    pub fn list_changed(mut self, value: bool) -> Self {
        self.list_changed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RootsCapability`].
    pub fn build(self) -> Result<RootsCapability, BuildError> {
        Ok(RootsCapability {
            list_changed: self.list_changed,
        })
    }
}
