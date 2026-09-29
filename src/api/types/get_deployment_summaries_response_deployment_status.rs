pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetDeploymentSummariesResponseDeploymentStatus {
    Online,
    Offline,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetDeploymentSummariesResponseDeploymentStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Online => serializer.serialize_str("online"),
            Self::Offline => serializer.serialize_str("offline"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetDeploymentSummariesResponseDeploymentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "online" => Ok(Self::Online),
            "offline" => Ok(Self::Offline),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetDeploymentSummariesResponseDeploymentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Online => write!(f, "online"),
            Self::Offline => write!(f, "offline"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
