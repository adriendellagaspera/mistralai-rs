pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct SkillAssetContentRawContent {
    pub raw_content: String,
}

impl SkillAssetContentRawContent {
    pub fn builder() -> SkillAssetContentRawContentBuilder {
        <SkillAssetContentRawContentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SkillAssetContentRawContentBuilder {
    raw_content: Option<String>,
}

impl SkillAssetContentRawContentBuilder {
    pub fn raw_content(mut self, value: impl Into<String>) -> Self {
        self.raw_content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SkillAssetContentRawContent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`raw_content`](SkillAssetContentRawContentBuilder::raw_content)
    pub fn build(self) -> Result<SkillAssetContentRawContent, BuildError> {
        Ok(SkillAssetContentRawContent {
            raw_content: self
                .raw_content
                .ok_or_else(|| BuildError::missing_field("raw_content"))?,
        })
    }
}
