pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProcessStatus {
    SelfManaged,
    MissingContent,
    Noop,
    Done,
    Todo,
    InProgress,
    Error,
    WaitingForCapacity,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ProcessStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SelfManaged => serializer.serialize_str("self_managed"),
            Self::MissingContent => serializer.serialize_str("missing_content"),
            Self::Noop => serializer.serialize_str("noop"),
            Self::Done => serializer.serialize_str("done"),
            Self::Todo => serializer.serialize_str("todo"),
            Self::InProgress => serializer.serialize_str("in_progress"),
            Self::Error => serializer.serialize_str("error"),
            Self::WaitingForCapacity => serializer.serialize_str("waiting_for_capacity"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ProcessStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "self_managed" => Ok(Self::SelfManaged),
            "missing_content" => Ok(Self::MissingContent),
            "noop" => Ok(Self::Noop),
            "done" => Ok(Self::Done),
            "todo" => Ok(Self::Todo),
            "in_progress" => Ok(Self::InProgress),
            "error" => Ok(Self::Error),
            "waiting_for_capacity" => Ok(Self::WaitingForCapacity),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SelfManaged => write!(f, "self_managed"),
            Self::MissingContent => write!(f, "missing_content"),
            Self::Noop => write!(f, "noop"),
            Self::Done => write!(f, "done"),
            Self::Todo => write!(f, "todo"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Error => write!(f, "error"),
            Self::WaitingForCapacity => write!(f, "waiting_for_capacity"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
