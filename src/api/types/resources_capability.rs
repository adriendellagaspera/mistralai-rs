pub use crate::prelude::*;

/// Capability for resources operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResourcesCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<bool>,
    #[serde(rename = "listChanged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResourcesCapability {
    pub fn builder() -> ResourcesCapabilityBuilder {
        <ResourcesCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResourcesCapabilityBuilder {
    subscribe: Option<bool>,
    list_changed: Option<bool>,
}

impl ResourcesCapabilityBuilder {
    pub fn subscribe(mut self, value: bool) -> Self {
        self.subscribe = Some(value);
        self
    }

    pub fn list_changed(mut self, value: bool) -> Self {
        self.list_changed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResourcesCapability`].
    pub fn build(self) -> Result<ResourcesCapability, BuildError> {
        Ok(ResourcesCapability {
            subscribe: self.subscribe,
            list_changed: self.list_changed,
            extra: Default::default(),
        })
    }
}
