pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ImageUrlChunkImageUrl {
    ImageUrl(ImageUrl),

    String(String),
}

impl ImageUrlChunkImageUrl {
    pub fn is_image_url(&self) -> bool {
        matches!(self, Self::ImageUrl(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn as_image_url(&self) -> Option<&ImageUrl> {
        match self {
            Self::ImageUrl(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_image_url(self) -> Option<ImageUrl> {
        match self {
            Self::ImageUrl(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ImageUrlChunkImageUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImageUrl(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::String(value) => write!(f, "{}", value),
        }
    }
}
