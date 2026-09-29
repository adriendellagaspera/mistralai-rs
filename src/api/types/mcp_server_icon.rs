pub use crate::prelude::*;

/// An icon for display in user interfaces.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct McpServerIcon {
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizes: Option<Vec<String>>,
    #[serde(default)]
    pub src: String,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl McpServerIcon {
    pub fn builder() -> McpServerIconBuilder {
        <McpServerIconBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServerIconBuilder {
    mime_type: Option<String>,
    sizes: Option<Vec<String>>,
    src: Option<String>,
}

impl McpServerIconBuilder {
    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn sizes(mut self, value: Vec<String>) -> Self {
        self.sizes = Some(value);
        self
    }

    pub fn src(mut self, value: impl Into<String>) -> Self {
        self.src = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`McpServerIcon`].
    /// This method will fail if any of the following fields are not set:
    /// - [`src`](McpServerIconBuilder::src)
    pub fn build(self) -> Result<McpServerIcon, BuildError> {
        Ok(McpServerIcon {
            mime_type: self.mime_type,
            sizes: self.sizes,
            src: self.src.ok_or_else(|| BuildError::missing_field("src"))?,
            extra: Default::default(),
        })
    }
}
