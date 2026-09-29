pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LibrariesApiUsageDataJson {
    /// Audio usage for Libraries API.
    #[serde(default)]
    pub audio_seconds: BasicModelUsageDataJson,
    /// Page usage for Libraries API.
    #[serde(default)]
    pub pages: BasicModelUsageDataJson,
    /// Token usage for Libraries API.
    #[serde(default)]
    pub tokens: BasicModelUsageDataJson,
}

impl LibrariesApiUsageDataJson {
    pub fn builder() -> LibrariesApiUsageDataJsonBuilder {
        <LibrariesApiUsageDataJsonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LibrariesApiUsageDataJsonBuilder {
    audio_seconds: Option<BasicModelUsageDataJson>,
    pages: Option<BasicModelUsageDataJson>,
    tokens: Option<BasicModelUsageDataJson>,
}

impl LibrariesApiUsageDataJsonBuilder {
    pub fn audio_seconds(mut self, value: BasicModelUsageDataJson) -> Self {
        self.audio_seconds = Some(value);
        self
    }

    pub fn pages(mut self, value: BasicModelUsageDataJson) -> Self {
        self.pages = Some(value);
        self
    }

    pub fn tokens(mut self, value: BasicModelUsageDataJson) -> Self {
        self.tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LibrariesApiUsageDataJson`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_seconds`](LibrariesApiUsageDataJsonBuilder::audio_seconds)
    /// - [`pages`](LibrariesApiUsageDataJsonBuilder::pages)
    /// - [`tokens`](LibrariesApiUsageDataJsonBuilder::tokens)
    pub fn build(self) -> Result<LibrariesApiUsageDataJson, BuildError> {
        Ok(LibrariesApiUsageDataJson {
            audio_seconds: self
                .audio_seconds
                .ok_or_else(|| BuildError::missing_field("audio_seconds"))?,
            pages: self
                .pages
                .ok_or_else(|| BuildError::missing_field("pages"))?,
            tokens: self
                .tokens
                .ok_or_else(|| BuildError::missing_field("tokens"))?,
        })
    }
}
