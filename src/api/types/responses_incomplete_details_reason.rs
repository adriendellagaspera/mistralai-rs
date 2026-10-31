pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResponsesIncompleteDetailsReason {
    MaxOutputTokens,
    ContentFilter,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResponsesIncompleteDetailsReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MaxOutputTokens => serializer.serialize_str("max_output_tokens"),
            Self::ContentFilter => serializer.serialize_str("content_filter"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResponsesIncompleteDetailsReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "max_output_tokens" => Ok(Self::MaxOutputTokens),
            "content_filter" => Ok(Self::ContentFilter),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResponsesIncompleteDetailsReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MaxOutputTokens => write!(f, "max_output_tokens"),
            Self::ContentFilter => write!(f, "content_filter"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
