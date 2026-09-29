pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ToolReferenceChunk {
    pub tool: ToolReferenceChunkTool,
    #[serde(default)]
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ToolReferenceChunk {
    pub fn builder() -> ToolReferenceChunkBuilder {
        <ToolReferenceChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolReferenceChunkBuilder {
    tool: Option<ToolReferenceChunkTool>,
    title: Option<String>,
    url: Option<String>,
    favicon: Option<String>,
    description: Option<String>,
}

impl ToolReferenceChunkBuilder {
    pub fn tool(mut self, value: ToolReferenceChunkTool) -> Self {
        self.tool = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn favicon(mut self, value: impl Into<String>) -> Self {
        self.favicon = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToolReferenceChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool`](ToolReferenceChunkBuilder::tool)
    /// - [`title`](ToolReferenceChunkBuilder::title)
    pub fn build(self) -> Result<ToolReferenceChunk, BuildError> {
        Ok(ToolReferenceChunk {
            tool: self.tool.ok_or_else(|| BuildError::missing_field("tool"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            url: self.url,
            favicon: self.favicon,
            description: self.description,
        })
    }
}
