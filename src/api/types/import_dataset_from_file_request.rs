pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImportDatasetFromFileRequest {
    #[serde(default)]
    pub file_id: String,
}

impl ImportDatasetFromFileRequest {
    pub fn builder() -> ImportDatasetFromFileRequestBuilder {
        <ImportDatasetFromFileRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportDatasetFromFileRequestBuilder {
    file_id: Option<String>,
}

impl ImportDatasetFromFileRequestBuilder {
    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ImportDatasetFromFileRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_id`](ImportDatasetFromFileRequestBuilder::file_id)
    pub fn build(self) -> Result<ImportDatasetFromFileRequest, BuildError> {
        Ok(ImportDatasetFromFileRequest {
            file_id: self
                .file_id
                .ok_or_else(|| BuildError::missing_field("file_id"))?,
        })
    }
}
