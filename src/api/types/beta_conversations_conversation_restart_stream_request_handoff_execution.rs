pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConversationRestartStreamRequestHandoffExecution {
    Client,
    Server,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConversationRestartStreamRequestHandoffExecution {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Client => serializer.serialize_str("client"),
            Self::Server => serializer.serialize_str("server"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConversationRestartStreamRequestHandoffExecution {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "client" => Ok(Self::Client),
            "server" => Ok(Self::Server),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConversationRestartStreamRequestHandoffExecution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Client => write!(f, "client"),
            Self::Server => write!(f, "server"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
