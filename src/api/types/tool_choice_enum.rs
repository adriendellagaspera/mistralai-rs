pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolChoiceEnum {
    Auto,
    None,
    Any,
    Required,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ToolChoiceEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::None => serializer.serialize_str("none"),
            Self::Any => serializer.serialize_str("any"),
            Self::Required => serializer.serialize_str("required"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ToolChoiceEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto" => Ok(Self::Auto),
            "none" => Ok(Self::None),
            "any" => Ok(Self::Any),
            "required" => Ok(Self::Required),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ToolChoiceEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::None => write!(f, "none"),
            Self::Any => write!(f, "any"),
            Self::Required => write!(f, "required"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
