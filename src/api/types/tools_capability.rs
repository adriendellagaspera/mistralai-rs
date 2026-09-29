pub use crate::prelude::*;

/// Capability for tools operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ToolsCapability {
    #[serde(rename = "listChanged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ToolsCapability {
    pub fn builder() -> ToolsCapabilityBuilder {
        <ToolsCapabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolsCapabilityBuilder {
    list_changed: Option<bool>,
}

impl ToolsCapabilityBuilder {
    pub fn list_changed(mut self, value: bool) -> Self {
        self.list_changed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolsCapability`].
    pub fn build(self) -> Result<ToolsCapability, BuildError> {
        Ok(ToolsCapability {
            list_changed: self.list_changed,
            extra: Default::default(),
        })
    }
}
