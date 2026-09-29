pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListRunsRequestSortBy {
    StartTime,
    EndTime,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListRunsRequestSortBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::StartTime => serializer.serialize_str("start_time"),
            Self::EndTime => serializer.serialize_str("end_time"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListRunsRequestSortBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "start_time" => Ok(Self::StartTime),
            "end_time" => Ok(Self::EndTime),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListRunsRequestSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StartTime => write!(f, "start_time"),
            Self::EndTime => write!(f, "end_time"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
