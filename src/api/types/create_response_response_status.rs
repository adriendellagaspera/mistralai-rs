pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateResponseResponseStatus {
    Completed,
    Failed,
    InProgress,
    Cancelled,
    Queued,
    Incomplete,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateResponseResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Completed => serializer.serialize_str("completed"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::InProgress => serializer.serialize_str("in_progress"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::Queued => serializer.serialize_str("queued"),
            Self::Incomplete => serializer.serialize_str("incomplete"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateResponseResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "in_progress" => Ok(Self::InProgress),
            "cancelled" => Ok(Self::Cancelled),
            "queued" => Ok(Self::Queued),
            "incomplete" => Ok(Self::Incomplete),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateResponseResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Queued => write!(f, "queued"),
            Self::Incomplete => write!(f, "incomplete"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
