pub use crate::prelude::*;

/// Attribute maps that may supply dataset payload values.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpanDatasetSourceNamespace {
    SpanAttributes,
    ResourceAttributes,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SpanDatasetSourceNamespace {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SpanAttributes => serializer.serialize_str("span_attributes"),
            Self::ResourceAttributes => serializer.serialize_str("resource_attributes"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SpanDatasetSourceNamespace {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "span_attributes" => Ok(Self::SpanAttributes),
            "resource_attributes" => Ok(Self::ResourceAttributes),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SpanDatasetSourceNamespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SpanAttributes => write!(f, "span_attributes"),
            Self::ResourceAttributes => write!(f, "resource_attributes"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
