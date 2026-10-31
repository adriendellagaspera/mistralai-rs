pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScoreOperator {
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScoreOperator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Gt => serializer.serialize_str("gt"),
            Self::Gte => serializer.serialize_str("gte"),
            Self::Lt => serializer.serialize_str("lt"),
            Self::Lte => serializer.serialize_str("lte"),
            Self::Eq => serializer.serialize_str("eq"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScoreOperator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "gt" => Ok(Self::Gt),
            "gte" => Ok(Self::Gte),
            "lt" => Ok(Self::Lt),
            "lte" => Ok(Self::Lte),
            "eq" => Ok(Self::Eq),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScoreOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Gt => write!(f, "gt"),
            Self::Gte => write!(f, "gte"),
            Self::Lt => write!(f, "lt"),
            Self::Lte => write!(f, "lte"),
            Self::Eq => write!(f, "eq"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
