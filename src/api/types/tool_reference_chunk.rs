pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ToolReferenceChunk {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    #[serde(default)]
    pub title: String,
    pub tool: ToolReferenceChunkTool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ToolReferenceChunk {
    pub fn builder() -> ToolReferenceChunkBuilder {
        <ToolReferenceChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolReferenceChunkBuilder {
    description: Option<String>,
    favicon: Option<String>,
    title: Option<String>,
    tool: Option<ToolReferenceChunkTool>,
    url: Option<String>,
}

impl ToolReferenceChunkBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn favicon(mut self, value: impl Into<String>) -> Self {
        self.favicon = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn tool(mut self, value: ToolReferenceChunkTool) -> Self {
        self.tool = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToolReferenceChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](ToolReferenceChunkBuilder::title)
    /// - [`tool`](ToolReferenceChunkBuilder::tool)
    pub fn build(self) -> Result<ToolReferenceChunk, BuildError> {
        Ok(ToolReferenceChunk {
            description: self.description,
            favicon: self.favicon,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            tool: self.tool.ok_or_else(|| BuildError::missing_field("tool"))?,
            url: self.url,
        })
    }
}
