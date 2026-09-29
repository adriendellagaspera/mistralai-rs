pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SampleType {
    Pretrain,
    Instruct,
    BatchRequest,
    BatchResult,
    BatchError,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SampleType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pretrain => serializer.serialize_str("pretrain"),
            Self::Instruct => serializer.serialize_str("instruct"),
            Self::BatchRequest => serializer.serialize_str("batch_request"),
            Self::BatchResult => serializer.serialize_str("batch_result"),
            Self::BatchError => serializer.serialize_str("batch_error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SampleType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pretrain" => Ok(Self::Pretrain),
            "instruct" => Ok(Self::Instruct),
            "batch_request" => Ok(Self::BatchRequest),
            "batch_result" => Ok(Self::BatchResult),
            "batch_error" => Ok(Self::BatchError),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SampleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pretrain => write!(f, "pretrain"),
            Self::Instruct => write!(f, "instruct"),
            Self::BatchRequest => write!(f, "batch_request"),
            Self::BatchResult => write!(f, "batch_result"),
            Self::BatchError => write!(f, "batch_error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
