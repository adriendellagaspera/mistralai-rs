pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FunctionCallEventConfirmationStatus {
    Pending,
    Allowed,
    Denied,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FunctionCallEventConfirmationStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Allowed => serializer.serialize_str("allowed"),
            Self::Denied => serializer.serialize_str("denied"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FunctionCallEventConfirmationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "allowed" => Ok(Self::Allowed),
            "denied" => Ok(Self::Denied),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FunctionCallEventConfirmationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Allowed => write!(f, "allowed"),
            Self::Denied => write!(f, "denied"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
