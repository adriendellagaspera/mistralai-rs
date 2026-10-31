pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EvaluatorV2Direction {
    Maximize,
    Minimize,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EvaluatorV2Direction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Maximize => serializer.serialize_str("maximize"),
            Self::Minimize => serializer.serialize_str("minimize"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EvaluatorV2Direction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "maximize" => Ok(Self::Maximize),
            "minimize" => Ok(Self::Minimize),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EvaluatorV2Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Maximize => write!(f, "maximize"),
            Self::Minimize => write!(f, "minimize"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
