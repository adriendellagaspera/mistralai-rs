pub use crate::prelude::*;

/// The archive a managed deployment builds from, with a signed URL to download it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeploymentArchiveResponse {
    /// Mistral Cloud archive the worker builds from
    #[serde(default)]
    pub archive_id: String,
    /// A signed URL to download the archive, which expires within 24 hours. Null once Mistral Cloud has deleted the archive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
}

impl DeploymentArchiveResponse {
    pub fn builder() -> DeploymentArchiveResponseBuilder {
        <DeploymentArchiveResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeploymentArchiveResponseBuilder {
    archive_id: Option<String>,
    download_url: Option<String>,
}

impl DeploymentArchiveResponseBuilder {
    pub fn archive_id(mut self, value: impl Into<String>) -> Self {
        self.archive_id = Some(value.into());
        self
    }

    pub fn download_url(mut self, value: impl Into<String>) -> Self {
        self.download_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeploymentArchiveResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`archive_id`](DeploymentArchiveResponseBuilder::archive_id)
    pub fn build(self) -> Result<DeploymentArchiveResponse, BuildError> {
        Ok(DeploymentArchiveResponse {
            archive_id: self
                .archive_id
                .ok_or_else(|| BuildError::missing_field("archive_id"))?,
            download_url: self.download_url,
        })
    }
}
