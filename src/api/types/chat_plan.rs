pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChatPlan {
    Individual,
    Edu,
    Team,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ChatPlan {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Individual => serializer.serialize_str("INDIVIDUAL"),
            Self::Edu => serializer.serialize_str("EDU"),
            Self::Team => serializer.serialize_str("TEAM"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ChatPlan {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "INDIVIDUAL" => Ok(Self::Individual),
            "EDU" => Ok(Self::Edu),
            "TEAM" => Ok(Self::Team),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ChatPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Individual => write!(f, "INDIVIDUAL"),
            Self::Edu => write!(f, "EDU"),
            Self::Team => write!(f, "TEAM"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
