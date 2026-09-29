pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FtClassifierLossFunction {
    SingleClass,
    MultiClass,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FtClassifierLossFunction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SingleClass => serializer.serialize_str("single_class"),
            Self::MultiClass => serializer.serialize_str("multi_class"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FtClassifierLossFunction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "single_class" => Ok(Self::SingleClass),
            "multi_class" => Ok(Self::MultiClass),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FtClassifierLossFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SingleClass => write!(f, "single_class"),
            Self::MultiClass => write!(f, "multi_class"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
