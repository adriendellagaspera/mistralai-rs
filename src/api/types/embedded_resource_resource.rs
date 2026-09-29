pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum EmbeddedResourceResource {
    TextResourceContents(TextResourceContents),

    BlobResourceContents(BlobResourceContents),
}

impl EmbeddedResourceResource {
    pub fn is_text_resource_contents(&self) -> bool {
        matches!(self, Self::TextResourceContents(_))
    }

    pub fn is_blob_resource_contents(&self) -> bool {
        matches!(self, Self::BlobResourceContents(_))
    }

    pub fn as_text_resource_contents(&self) -> Option<&TextResourceContents> {
        match self {
            Self::TextResourceContents(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_text_resource_contents(self) -> Option<TextResourceContents> {
        match self {
            Self::TextResourceContents(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_blob_resource_contents(&self) -> Option<&BlobResourceContents> {
        match self {
            Self::BlobResourceContents(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_blob_resource_contents(self) -> Option<BlobResourceContents> {
        match self {
            Self::BlobResourceContents(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for EmbeddedResourceResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TextResourceContents(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::BlobResourceContents(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
