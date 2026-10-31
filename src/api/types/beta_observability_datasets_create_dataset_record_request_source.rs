pub use crate::prelude::*;

/// Caller-declared channel that initiated record creation. This value does not certify that the payload is an unmodified copy of its source.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateDatasetRecordRequestSource {
    DirectInput,
    TelemetrySpan,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateDatasetRecordRequestSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::DirectInput => serializer.serialize_str("DIRECT_INPUT"),
            Self::TelemetrySpan => serializer.serialize_str("TELEMETRY_SPAN"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateDatasetRecordRequestSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "DIRECT_INPUT" => Ok(Self::DirectInput),
            "TELEMETRY_SPAN" => Ok(Self::TelemetrySpan),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateDatasetRecordRequestSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DirectInput => write!(f, "DIRECT_INPUT"),
            Self::TelemetrySpan => write!(f, "TELEMETRY_SPAN"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
