pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlanType {
    Api,
    Chat,
    OnPremise,
    License,
    MistralCode,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PlanType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Api => serializer.serialize_str("API"),
            Self::Chat => serializer.serialize_str("CHAT"),
            Self::OnPremise => serializer.serialize_str("ON_PREMISE"),
            Self::License => serializer.serialize_str("LICENSE"),
            Self::MistralCode => serializer.serialize_str("MISTRAL_CODE"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PlanType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "API" => Ok(Self::Api),
            "CHAT" => Ok(Self::Chat),
            "ON_PREMISE" => Ok(Self::OnPremise),
            "LICENSE" => Ok(Self::License),
            "MISTRAL_CODE" => Ok(Self::MistralCode),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PlanType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Api => write!(f, "API"),
            Self::Chat => write!(f, "CHAT"),
            Self::OnPremise => write!(f, "ON_PREMISE"),
            Self::License => write!(f, "LICENSE"),
            Self::MistralCode => write!(f, "MISTRAL_CODE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
