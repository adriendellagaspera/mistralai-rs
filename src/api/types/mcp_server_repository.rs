pub use crate::prelude::*;

/// Source repository information (SEP-2127).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct McpServerRepository {
    /// Repository URL
    #[serde(default)]
    pub url: String,
    /// Source identifier (e.g. 'github')
    #[serde(default)]
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subfolder: Option<String>,
}

impl McpServerRepository {
    pub fn builder() -> McpServerRepositoryBuilder {
        <McpServerRepositoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServerRepositoryBuilder {
    url: Option<String>,
    source: Option<String>,
    subfolder: Option<String>,
}

impl McpServerRepositoryBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn subfolder(mut self, value: impl Into<String>) -> Self {
        self.subfolder = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`McpServerRepository`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](McpServerRepositoryBuilder::url)
    /// - [`source`](McpServerRepositoryBuilder::source)
    pub fn build(self) -> Result<McpServerRepository, BuildError> {
        Ok(McpServerRepository {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            subfolder: self.subfolder,
        })
    }
}
