pub use crate::prelude::*;

/// Capability for prompts operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PromptsCapability {
    #[serde(rename = "listChanged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl PromptsCapability {
    pub fn builder() -> PromptsCapabilityBuilder {
        <PromptsCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromptsCapabilityBuilder {
    list_changed: Option<bool>,
}

impl PromptsCapabilityBuilder {
    pub fn list_changed(mut self, value: bool) -> Self {
        self.list_changed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PromptsCapability`].
    pub fn build(self) -> Result<PromptsCapability, BuildError> {
        Ok(PromptsCapability {
            list_changed: self.list_changed,
            extra: Default::default(),
        })
    }
}
