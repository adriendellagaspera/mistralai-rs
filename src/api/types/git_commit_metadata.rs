pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GitCommitMetadata {
    #[serde(default)]
    pub sha: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<GitCommitAuthor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}

impl GitCommitMetadata {
    pub fn builder() -> GitCommitMetadataBuilder {
        <GitCommitMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GitCommitMetadataBuilder {
    sha: Option<String>,
    message: Option<String>,
    author: Option<GitCommitAuthor>,
    html_url: Option<String>,
}

impl GitCommitMetadataBuilder {
    pub fn sha(mut self, value: impl Into<String>) -> Self {
        self.sha = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn author(mut self, value: GitCommitAuthor) -> Self {
        self.author = Some(value);
        self
    }

    pub fn html_url(mut self, value: impl Into<String>) -> Self {
        self.html_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GitCommitMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sha`](GitCommitMetadataBuilder::sha)
    pub fn build(self) -> Result<GitCommitMetadata, BuildError> {
        Ok(GitCommitMetadata {
            sha: self.sha.ok_or_else(|| BuildError::missing_field("sha"))?,
            message: self.message,
            author: self.author,
            html_url: self.html_url,
        })
    }
}
