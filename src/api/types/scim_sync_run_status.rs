pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScimSyncRunStatus {
    Pending,
    Running,
    Success,
    Skipped,
    Failed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScimSyncRunStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("PENDING"),
            Self::Running => serializer.serialize_str("RUNNING"),
            Self::Success => serializer.serialize_str("SUCCESS"),
            Self::Skipped => serializer.serialize_str("SKIPPED"),
            Self::Failed => serializer.serialize_str("FAILED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScimSyncRunStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "PENDING" => Ok(Self::Pending),
            "RUNNING" => Ok(Self::Running),
            "SUCCESS" => Ok(Self::Success),
            "SKIPPED" => Ok(Self::Skipped),
            "FAILED" => Ok(Self::Failed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScimSyncRunStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "PENDING"),
            Self::Running => write!(f, "RUNNING"),
            Self::Success => write!(f, "SUCCESS"),
            Self::Skipped => write!(f, "SKIPPED"),
            Self::Failed => write!(f, "FAILED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
