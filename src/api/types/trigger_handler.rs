pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TriggerHandler {
    Workflows,
    Agents,
    Spaces,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TriggerHandler {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Workflows => serializer.serialize_str("workflows"),
            Self::Agents => serializer.serialize_str("agents"),
            Self::Spaces => serializer.serialize_str("spaces"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TriggerHandler {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "workflows" => Ok(Self::Workflows),
            "agents" => Ok(Self::Agents),
            "spaces" => Ok(Self::Spaces),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TriggerHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Workflows => write!(f, "workflows"),
            Self::Agents => write!(f, "agents"),
            Self::Spaces => write!(f, "spaces"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
