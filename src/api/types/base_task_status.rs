pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BaseTaskStatus {
    Running,
    Completed,
    Failed,
    Canceled,
    Terminated,
    ContinuedAsNew,
    TimedOut,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BaseTaskStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Running => serializer.serialize_str("RUNNING"),
            Self::Completed => serializer.serialize_str("COMPLETED"),
            Self::Failed => serializer.serialize_str("FAILED"),
            Self::Canceled => serializer.serialize_str("CANCELED"),
            Self::Terminated => serializer.serialize_str("TERMINATED"),
            Self::ContinuedAsNew => serializer.serialize_str("CONTINUED_AS_NEW"),
            Self::TimedOut => serializer.serialize_str("TIMED_OUT"),
            Self::Unknown => serializer.serialize_str("UNKNOWN"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BaseTaskStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "RUNNING" => Ok(Self::Running),
            "COMPLETED" => Ok(Self::Completed),
            "FAILED" => Ok(Self::Failed),
            "CANCELED" => Ok(Self::Canceled),
            "TERMINATED" => Ok(Self::Terminated),
            "CONTINUED_AS_NEW" => Ok(Self::ContinuedAsNew),
            "TIMED_OUT" => Ok(Self::TimedOut),
            "UNKNOWN" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BaseTaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Running => write!(f, "RUNNING"),
            Self::Completed => write!(f, "COMPLETED"),
            Self::Failed => write!(f, "FAILED"),
            Self::Canceled => write!(f, "CANCELED"),
            Self::Terminated => write!(f, "TERMINATED"),
            Self::ContinuedAsNew => write!(f, "CONTINUED_AS_NEW"),
            Self::TimedOut => write!(f, "TIMED_OUT"),
            Self::Unknown => write!(f, "UNKNOWN"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
