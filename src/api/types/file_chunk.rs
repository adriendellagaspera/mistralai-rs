pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FileChunk {
    #[serde(default)]
    pub file_id: String,
}

impl FileChunk {
    pub fn builder() -> FileChunkBuilder {
        <FileChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileChunkBuilder {
    file_id: Option<String>,
}

impl FileChunkBuilder {
    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FileChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](FileChunkBuilder::file_id)
    pub fn build(self) -> Result<FileChunk, BuildError> {
        Ok(FileChunk {
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
        })
    }
}
