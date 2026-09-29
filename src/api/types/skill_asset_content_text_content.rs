pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct SkillAssetContentTextContent {
    pub text_content: String,
}

impl SkillAssetContentTextContent {
    pub fn builder() -> SkillAssetContentTextContentBuilder {
        <SkillAssetContentTextContentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SkillAssetContentTextContentBuilder {
    text_content: Option<String>,
}

impl SkillAssetContentTextContentBuilder {
    pub fn text_content(mut self, value: impl Into<String>) -> Self {
        self.text_content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SkillAssetContentTextContent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text_content`](SkillAssetContentTextContentBuilder::text_content)
    pub fn build(self) -> Result<SkillAssetContentTextContent, BuildError> {
        Ok(SkillAssetContentTextContent {
            text_content: self
                .text_content
                .ok_or_else(|| BuildError::missing_field("text_content"))?,
        })
    }
}
