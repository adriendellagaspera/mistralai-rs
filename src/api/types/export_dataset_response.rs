pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExportDatasetResponse {
    #[serde(default)]
    pub file_url: String,
}

impl ExportDatasetResponse {
    pub fn builder() -> ExportDatasetResponseBuilder {
        <ExportDatasetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportDatasetResponseBuilder {
    file_url: Option<String>,
}

impl ExportDatasetResponseBuilder {
    pub fn file_url(mut self, value: impl Into<String>) -> Self {
        self.file_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExportDatasetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_url`](ExportDatasetResponseBuilder::file_url)
    pub fn build(self) -> Result<ExportDatasetResponse, BuildError> {
        Ok(ExportDatasetResponse {
            file_url: self
                .file_url
                .ok_or_else(|| BuildError::missing_field("file_url"))?,
        })
    }
}
