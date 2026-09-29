pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LocationType {
    Local,
    K8S,
    Managed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LocationType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Local => serializer.serialize_str("local"),
            Self::K8S => serializer.serialize_str("k8s"),
            Self::Managed => serializer.serialize_str("managed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LocationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "local" => Ok(Self::Local),
            "k8s" => Ok(Self::K8S),
            "managed" => Ok(Self::Managed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LocationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Local => write!(f, "local"),
            Self::K8S => write!(f, "k8s"),
            Self::Managed => write!(f, "managed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
