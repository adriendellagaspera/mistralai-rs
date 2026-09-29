pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GitCommitAuthor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}

impl GitCommitAuthor {
    pub fn builder() -> GitCommitAuthorBuilder {
        <GitCommitAuthorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GitCommitAuthorBuilder {
    name: Option<String>,
    username: Option<String>,
    html_url: Option<String>,
}

impl GitCommitAuthorBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    pub fn html_url(mut self, value: impl Into<String>) -> Self {
        self.html_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GitCommitAuthor`].
    pub fn build(self) -> Result<GitCommitAuthor, BuildError> {
        Ok(GitCommitAuthor {
            name: self.name,
            username: self.username,
            html_url: self.html_url,
        })
    }
}
