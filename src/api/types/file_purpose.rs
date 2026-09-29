pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilePurpose {
    FineTune,
    Batch,
    Ocr,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FilePurpose {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::FineTune => serializer.serialize_str("fine-tune"),
            Self::Batch => serializer.serialize_str("batch"),
            Self::Ocr => serializer.serialize_str("ocr"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FilePurpose {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "fine-tune" => Ok(Self::FineTune),
            "batch" => Ok(Self::Batch),
            "ocr" => Ok(Self::Ocr),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FilePurpose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FineTune => write!(f, "fine-tune"),
            Self::Batch => write!(f, "batch"),
            Self::Ocr => write!(f, "ocr"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
