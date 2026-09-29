pub use crate::prelude::*;

/// Capability for root operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RootsCapability {
    #[serde(rename = "listChanged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
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
            extra: Default::default(),
        })
    }
}
