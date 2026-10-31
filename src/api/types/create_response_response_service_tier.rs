pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateResponseResponseServiceTier {
    Auto,
    Default,
    Flex,
    Scale,
    Priority,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateResponseResponseServiceTier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::Default => serializer.serialize_str("default"),
            Self::Flex => serializer.serialize_str("flex"),
            Self::Scale => serializer.serialize_str("scale"),
            Self::Priority => serializer.serialize_str("priority"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateResponseResponseServiceTier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto" => Ok(Self::Auto),
            "default" => Ok(Self::Default),
            "flex" => Ok(Self::Flex),
            "scale" => Ok(Self::Scale),
            "priority" => Ok(Self::Priority),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateResponseResponseServiceTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Default => write!(f, "default"),
            Self::Flex => write!(f, "flex"),
            Self::Scale => write!(f, "scale"),
            Self::Priority => write!(f, "priority"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
