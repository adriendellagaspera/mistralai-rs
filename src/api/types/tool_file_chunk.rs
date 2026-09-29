pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ToolFileChunk {
    pub tool: ToolFileChunkTool,
    #[serde(default)]
    pub file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
}

impl ToolFileChunk {
    pub fn builder() -> ToolFileChunkBuilder {
        <ToolFileChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolFileChunkBuilder {
    tool: Option<ToolFileChunkTool>,
    file_id: Option<String>,
    file_name: Option<String>,
    file_type: Option<String>,
}

impl ToolFileChunkBuilder {
    pub fn tool(mut self, value: ToolFileChunkTool) -> Self {
        self.tool = Some(value);
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn file_type(mut self, value: impl Into<String>) -> Self {
        self.file_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToolFileChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool`](ToolFileChunkBuilder::tool)
    /// - [`file_id`](ToolFileChunkBuilder::file_id)
    pub fn build(self) -> Result<ToolFileChunk, BuildError> {
        Ok(ToolFileChunk {
            tool: self.tool.ok_or_else(|| BuildError::missing_field("tool"))?,
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
            file_name: self.file_name,
            file_type: self.file_type,
        })
    }
}
