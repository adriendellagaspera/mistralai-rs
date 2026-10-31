pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResponsesErrorCode {
    ServerError,
    RateLimitExceeded,
    InvalidPrompt,
    BioPolicy,
    VectorStoreTimeout,
    InvalidImage,
    InvalidImageFormat,
    InvalidBase64Image,
    InvalidImageUrl,
    ImageTooLarge,
    ImageTooSmall,
    ImageParseError,
    ImageContentPolicyViolation,
    InvalidImageMode,
    ImageFileTooLarge,
    UnsupportedImageMediaType,
    EmptyImageFile,
    FailedToDownloadImage,
    ImageFileNotFound,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResponsesErrorCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ServerError => serializer.serialize_str("server_error"),
            Self::RateLimitExceeded => serializer.serialize_str("rate_limit_exceeded"),
            Self::InvalidPrompt => serializer.serialize_str("invalid_prompt"),
            Self::BioPolicy => serializer.serialize_str("bio_policy"),
            Self::VectorStoreTimeout => serializer.serialize_str("vector_store_timeout"),
            Self::InvalidImage => serializer.serialize_str("invalid_image"),
            Self::InvalidImageFormat => serializer.serialize_str("invalid_image_format"),
            Self::InvalidBase64Image => serializer.serialize_str("invalid_base64_image"),
            Self::InvalidImageUrl => serializer.serialize_str("invalid_image_url"),
            Self::ImageTooLarge => serializer.serialize_str("image_too_large"),
            Self::ImageTooSmall => serializer.serialize_str("image_too_small"),
            Self::ImageParseError => serializer.serialize_str("image_parse_error"),
            Self::ImageContentPolicyViolation => {
                serializer.serialize_str("image_content_policy_violation")
            }
            Self::InvalidImageMode => serializer.serialize_str("invalid_image_mode"),
            Self::ImageFileTooLarge => serializer.serialize_str("image_file_too_large"),
            Self::UnsupportedImageMediaType => {
                serializer.serialize_str("unsupported_image_media_type")
            }
            Self::EmptyImageFile => serializer.serialize_str("empty_image_file"),
            Self::FailedToDownloadImage => serializer.serialize_str("failed_to_download_image"),
            Self::ImageFileNotFound => serializer.serialize_str("image_file_not_found"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResponsesErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "server_error" => Ok(Self::ServerError),
            "rate_limit_exceeded" => Ok(Self::RateLimitExceeded),
            "invalid_prompt" => Ok(Self::InvalidPrompt),
            "bio_policy" => Ok(Self::BioPolicy),
            "vector_store_timeout" => Ok(Self::VectorStoreTimeout),
            "invalid_image" => Ok(Self::InvalidImage),
            "invalid_image_format" => Ok(Self::InvalidImageFormat),
            "invalid_base64_image" => Ok(Self::InvalidBase64Image),
            "invalid_image_url" => Ok(Self::InvalidImageUrl),
            "image_too_large" => Ok(Self::ImageTooLarge),
            "image_too_small" => Ok(Self::ImageTooSmall),
            "image_parse_error" => Ok(Self::ImageParseError),
            "image_content_policy_violation" => Ok(Self::ImageContentPolicyViolation),
            "invalid_image_mode" => Ok(Self::InvalidImageMode),
            "image_file_too_large" => Ok(Self::ImageFileTooLarge),
            "unsupported_image_media_type" => Ok(Self::UnsupportedImageMediaType),
            "empty_image_file" => Ok(Self::EmptyImageFile),
            "failed_to_download_image" => Ok(Self::FailedToDownloadImage),
            "image_file_not_found" => Ok(Self::ImageFileNotFound),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResponsesErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ServerError => write!(f, "server_error"),
            Self::RateLimitExceeded => write!(f, "rate_limit_exceeded"),
            Self::InvalidPrompt => write!(f, "invalid_prompt"),
            Self::BioPolicy => write!(f, "bio_policy"),
            Self::VectorStoreTimeout => write!(f, "vector_store_timeout"),
            Self::InvalidImage => write!(f, "invalid_image"),
            Self::InvalidImageFormat => write!(f, "invalid_image_format"),
            Self::InvalidBase64Image => write!(f, "invalid_base64_image"),
            Self::InvalidImageUrl => write!(f, "invalid_image_url"),
            Self::ImageTooLarge => write!(f, "image_too_large"),
            Self::ImageTooSmall => write!(f, "image_too_small"),
            Self::ImageParseError => write!(f, "image_parse_error"),
            Self::ImageContentPolicyViolation => write!(f, "image_content_policy_violation"),
            Self::InvalidImageMode => write!(f, "invalid_image_mode"),
            Self::ImageFileTooLarge => write!(f, "image_file_too_large"),
            Self::UnsupportedImageMediaType => write!(f, "unsupported_image_media_type"),
            Self::EmptyImageFile => write!(f, "empty_image_file"),
            Self::FailedToDownloadImage => write!(f, "failed_to_download_image"),
            Self::ImageFileNotFound => write!(f, "image_file_not_found"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
