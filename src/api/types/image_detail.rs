pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImageDetail {
    Low,
    Auto,
    High,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ImageDetail {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Low => serializer.serialize_str("low"),
            Self::Auto => serializer.serialize_str("auto"),
            Self::High => serializer.serialize_str("high"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ImageDetail {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "low" => Ok(Self::Low),
            "auto" => Ok(Self::Auto),
            "high" => Ok(Self::High),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ImageDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "low"),
            Self::Auto => write!(f, "auto"),
            Self::High => write!(f, "high"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
