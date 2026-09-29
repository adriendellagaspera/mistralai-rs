pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct McpResource {
    #[serde(rename = "_meta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<McpServerIcon>>,
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub uri: String,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl McpResource {
    pub fn builder() -> McpResourceBuilder {
        <McpResourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpResourceBuilder {
    meta: Option<HashMap<String, serde_json::Value>>,
    annotations: Option<Annotations>,
    description: Option<String>,
    icons: Option<Vec<McpServerIcon>>,
    mime_type: Option<String>,
    name: Option<String>,
    size: Option<i64>,
    title: Option<String>,
    uri: Option<String>,
}

impl McpResourceBuilder {
    pub fn meta(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn annotations(mut self, value: Annotations) -> Self {
        self.annotations = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn icons(mut self, value: Vec<McpServerIcon>) -> Self {
        self.icons = Some(value);
        self
    }

    pub fn mime_type(mut self, value: impl Into<String>) -> Self {
        self.mime_type = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn uri(mut self, value: impl Into<String>) -> Self {
        self.uri = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`McpResource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](McpResourceBuilder::name)
    /// - [`uri`](McpResourceBuilder::uri)
    pub fn build(self) -> Result<McpResource, BuildError> {
        Ok(McpResource {
            meta: self.meta,
            annotations: self.annotations,
            description: self.description,
            icons: self.icons,
            mime_type: self.mime_type,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            size: self.size,
            title: self.title,
            uri: self.uri.ok_or_else(|| BuildError::missing_field("uri"))?,
            extra: Default::default(),
        })
    }
}
