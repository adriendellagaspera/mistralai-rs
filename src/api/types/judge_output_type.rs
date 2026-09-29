pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JudgeOutputType {
    Regression,
    Classification,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for JudgeOutputType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Regression => serializer.serialize_str("REGRESSION"),
            Self::Classification => serializer.serialize_str("CLASSIFICATION"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for JudgeOutputType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "REGRESSION" => Ok(Self::Regression),
            "CLASSIFICATION" => Ok(Self::Classification),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for JudgeOutputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regression => write!(f, "REGRESSION"),
            Self::Classification => write!(f, "CLASSIFICATION"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
