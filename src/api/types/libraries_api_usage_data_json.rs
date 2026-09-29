pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LibrariesApiUsageDataJson {
    /// Page usage for Libraries API.
    #[serde(default)]
    pub pages: BasicModelUsageDataJson,
    /// Token usage for Libraries API.
    #[serde(default)]
    pub tokens: BasicModelUsageDataJson,
    /// Audio usage for Libraries API.
    #[serde(default)]
    pub audio_seconds: BasicModelUsageDataJson,
}

impl LibrariesApiUsageDataJson {
    pub fn builder() -> LibrariesApiUsageDataJsonBuilder {
        <LibrariesApiUsageDataJsonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LibrariesApiUsageDataJsonBuilder {
    pages: Option<BasicModelUsageDataJson>,
    tokens: Option<BasicModelUsageDataJson>,
    audio_seconds: Option<BasicModelUsageDataJson>,
}

impl LibrariesApiUsageDataJsonBuilder {
    pub fn pages(mut self, value: BasicModelUsageDataJson) -> Self {
        self.pages = Some(value);
        self
    }

    pub fn tokens(mut self, value: BasicModelUsageDataJson) -> Self {
        self.tokens = Some(value);
        self
    }

    pub fn audio_seconds(mut self, value: BasicModelUsageDataJson) -> Self {
        self.audio_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LibrariesApiUsageDataJson`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pages`](LibrariesApiUsageDataJsonBuilder::pages)
    /// - [`tokens`](LibrariesApiUsageDataJsonBuilder::tokens)
    /// - [`audio_seconds`](LibrariesApiUsageDataJsonBuilder::audio_seconds)
    pub fn build(self) -> Result<LibrariesApiUsageDataJson, BuildError> {
        Ok(LibrariesApiUsageDataJson {
            pages: self
                .pages
                .ok_or_else(|| BuildError::missing_field("pages"))?,
            tokens: self
                .tokens
                .ok_or_else(|| BuildError::missing_field("tokens"))?,
            audio_seconds: self
                .audio_seconds
                .ok_or_else(|| BuildError::missing_field("audio_seconds"))?,
        })
    }
}
