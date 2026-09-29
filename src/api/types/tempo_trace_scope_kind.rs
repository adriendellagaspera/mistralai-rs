pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TempoTraceScopeKind {
    SpanKindInternal,
    SpanKindServer,
    SpanKindClient,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TempoTraceScopeKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SpanKindInternal => serializer.serialize_str("SPAN_KIND_INTERNAL"),
            Self::SpanKindServer => serializer.serialize_str("SPAN_KIND_SERVER"),
            Self::SpanKindClient => serializer.serialize_str("SPAN_KIND_CLIENT"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TempoTraceScopeKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "SPAN_KIND_INTERNAL" => Ok(Self::SpanKindInternal),
            "SPAN_KIND_SERVER" => Ok(Self::SpanKindServer),
            "SPAN_KIND_CLIENT" => Ok(Self::SpanKindClient),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TempoTraceScopeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SpanKindInternal => write!(f, "SPAN_KIND_INTERNAL"),
            Self::SpanKindServer => write!(f, "SPAN_KIND_SERVER"),
            Self::SpanKindClient => write!(f, "SPAN_KIND_CLIENT"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
