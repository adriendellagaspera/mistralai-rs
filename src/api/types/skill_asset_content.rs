pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum SkillAssetContent {
    SkillAssetContentRawContent(SkillAssetContentRawContent),

    SkillAssetContentTextContent(SkillAssetContentTextContent),
}

impl SkillAssetContent {
    pub fn is_skill_asset_content_raw_content(&self) -> bool {
        matches!(self, Self::SkillAssetContentRawContent(_))
    }

    pub fn is_skill_asset_content_text_content(&self) -> bool {
        matches!(self, Self::SkillAssetContentTextContent(_))
    }

    pub fn as_skill_asset_content_raw_content(&self) -> Option<&SkillAssetContentRawContent> {
        match self {
            Self::SkillAssetContentRawContent(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_skill_asset_content_raw_content(self) -> Option<SkillAssetContentRawContent> {
        match self {
            Self::SkillAssetContentRawContent(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_skill_asset_content_text_content(&self) -> Option<&SkillAssetContentTextContent> {
        match self {
            Self::SkillAssetContentTextContent(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_skill_asset_content_text_content(self) -> Option<SkillAssetContentTextContent> {
        match self {
            Self::SkillAssetContentTextContent(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for SkillAssetContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SkillAssetContentRawContent(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::SkillAssetContentTextContent(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
