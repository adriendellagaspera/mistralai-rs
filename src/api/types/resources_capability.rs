pub use crate::prelude::*;

/// Capability for resources operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResourcesCapability {
    #[serde(rename = "listChanged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<bool>,
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
    list_changed: Option<bool>,
    subscribe: Option<bool>,
}

impl ResourcesCapabilityBuilder {
    pub fn list_changed(mut self, value: bool) -> Self {
        self.list_changed = Some(value);
        self
    }

    pub fn subscribe(mut self, value: bool) -> Self {
        self.subscribe = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResourcesCapability`].
    pub fn build(self) -> Result<ResourcesCapability, BuildError> {
        Ok(ResourcesCapability {
            list_changed: self.list_changed,
            subscribe: self.subscribe,
            extra: Default::default(),
        })
    }
}
