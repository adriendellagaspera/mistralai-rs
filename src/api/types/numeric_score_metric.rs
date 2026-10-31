pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NumericScoreMetric {
    Avg,
    Min,
    Max,
    Std,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for NumericScoreMetric {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Avg => serializer.serialize_str("avg"),
            Self::Min => serializer.serialize_str("min"),
            Self::Max => serializer.serialize_str("max"),
            Self::Std => serializer.serialize_str("std"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for NumericScoreMetric {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "avg" => Ok(Self::Avg),
            "min" => Ok(Self::Min),
            "max" => Ok(Self::Max),
            "std" => Ok(Self::Std),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for NumericScoreMetric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Avg => write!(f, "avg"),
            Self::Min => write!(f, "min"),
            Self::Max => write!(f, "max"),
            Self::Std => write!(f, "std"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
