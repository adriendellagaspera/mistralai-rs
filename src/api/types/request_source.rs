pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RequestSource {
    Api,
    Playground,
    AgentBuilderV1,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RequestSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Api => serializer.serialize_str("api"),
            Self::Playground => serializer.serialize_str("playground"),
            Self::AgentBuilderV1 => serializer.serialize_str("agent_builder_v1"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RequestSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "api" => Ok(Self::Api),
            "playground" => Ok(Self::Playground),
            "agent_builder_v1" => Ok(Self::AgentBuilderV1),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RequestSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Api => write!(f, "api"),
            Self::Playground => write!(f, "playground"),
            Self::AgentBuilderV1 => write!(f, "agent_builder_v1"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
