pub use crate::prelude::*;

/// UI metadata for tools that reference UI resources.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct McpuiToolMeta {
    #[serde(rename = "resourceUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Vec<McpuiToolMetaVisibilityItem>>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl McpuiToolMeta {
    pub fn builder() -> McpuiToolMetaBuilder {
        <McpuiToolMetaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpuiToolMetaBuilder {
    resource_uri: Option<String>,
    visibility: Option<Vec<McpuiToolMetaVisibilityItem>>,
}

impl McpuiToolMetaBuilder {
    pub fn resource_uri(mut self, value: impl Into<String>) -> Self {
        self.resource_uri = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: Vec<McpuiToolMetaVisibilityItem>) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpuiToolMeta`].
    pub fn build(self) -> Result<McpuiToolMeta, BuildError> {
        Ok(McpuiToolMeta {
            resource_uri: self.resource_uri,
            visibility: self.visibility,
            extra: Default::default(),
        })
    }
}
