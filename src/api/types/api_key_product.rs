pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApiKeyProduct {
    Api,
    MistralCode,
    Vibe,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ApiKeyProduct {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Api => serializer.serialize_str("API"),
            Self::MistralCode => serializer.serialize_str("Mistral Code"),
            Self::Vibe => serializer.serialize_str("Vibe"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ApiKeyProduct {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "API" => Ok(Self::Api),
            "Mistral Code" => Ok(Self::MistralCode),
            "Vibe" => Ok(Self::Vibe),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ApiKeyProduct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Api => write!(f, "API"),
            Self::MistralCode => write!(f, "Mistral Code"),
            Self::Vibe => write!(f, "Vibe"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
