pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceKind {
    Span,
    Log,
    Metric,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SourceKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Span => serializer.serialize_str("span"),
            Self::Log => serializer.serialize_str("log"),
            Self::Metric => serializer.serialize_str("metric"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SourceKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "span" => Ok(Self::Span),
            "log" => Ok(Self::Log),
            "metric" => Ok(Self::Metric),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Span => write!(f, "span"),
            Self::Log => write!(f, "log"),
            Self::Metric => write!(f, "metric"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
