pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BatchJobStatus {
    Queued,
    Running,
    Success,
    Failed,
    TimeoutExceeded,
    CancellationRequested,
    Cancelled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BatchJobStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Queued => serializer.serialize_str("QUEUED"),
            Self::Running => serializer.serialize_str("RUNNING"),
            Self::Success => serializer.serialize_str("SUCCESS"),
            Self::Failed => serializer.serialize_str("FAILED"),
            Self::TimeoutExceeded => serializer.serialize_str("TIMEOUT_EXCEEDED"),
            Self::CancellationRequested => serializer.serialize_str("CANCELLATION_REQUESTED"),
            Self::Cancelled => serializer.serialize_str("CANCELLED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BatchJobStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "QUEUED" => Ok(Self::Queued),
            "RUNNING" => Ok(Self::Running),
            "SUCCESS" => Ok(Self::Success),
            "FAILED" => Ok(Self::Failed),
            "TIMEOUT_EXCEEDED" => Ok(Self::TimeoutExceeded),
            "CANCELLATION_REQUESTED" => Ok(Self::CancellationRequested),
            "CANCELLED" => Ok(Self::Cancelled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BatchJobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Queued => write!(f, "QUEUED"),
            Self::Running => write!(f, "RUNNING"),
            Self::Success => write!(f, "SUCCESS"),
            Self::Failed => write!(f, "FAILED"),
            Self::TimeoutExceeded => write!(f, "TIMEOUT_EXCEEDED"),
            Self::CancellationRequested => write!(f, "CANCELLATION_REQUESTED"),
            Self::Cancelled => write!(f, "CANCELLED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
