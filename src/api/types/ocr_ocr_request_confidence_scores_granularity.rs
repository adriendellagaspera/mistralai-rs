pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OcrRequestConfidenceScoresGranularity {
    Word,
    Page,
    Block,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OcrRequestConfidenceScoresGranularity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Word => serializer.serialize_str("word"),
            Self::Page => serializer.serialize_str("page"),
            Self::Block => serializer.serialize_str("block"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OcrRequestConfidenceScoresGranularity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "word" => Ok(Self::Word),
            "page" => Ok(Self::Page),
            "block" => Ok(Self::Block),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OcrRequestConfidenceScoresGranularity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Word => write!(f, "word"),
            Self::Page => write!(f, "page"),
            Self::Block => write!(f, "block"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
