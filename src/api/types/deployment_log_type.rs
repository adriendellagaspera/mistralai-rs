pub use crate::prelude::*;

/// Which log stream a deployment's logs come from.
///
/// `runtime` is what the workers themselves emit. `build` only exists for managed
/// deployments: it is the image build pipeline's output, which no worker reports.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeploymentLogType {
    Build,
    Runtime,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DeploymentLogType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Build => serializer.serialize_str("build"),
            Self::Runtime => serializer.serialize_str("runtime"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DeploymentLogType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "build" => Ok(Self::Build),
            "runtime" => Ok(Self::Runtime),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DeploymentLogType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Build => write!(f, "build"),
            Self::Runtime => write!(f, "runtime"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
