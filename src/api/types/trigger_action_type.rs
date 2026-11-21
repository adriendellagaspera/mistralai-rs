pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TriggerActionType {
    StartWorkflow,
    BuildApp,
    DeployApp,
    StartAgentSession,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TriggerActionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::StartWorkflow => serializer.serialize_str("start_workflow"),
            Self::BuildApp => serializer.serialize_str("build_app"),
            Self::DeployApp => serializer.serialize_str("deploy_app"),
            Self::StartAgentSession => serializer.serialize_str("start_agent_session"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TriggerActionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "start_workflow" => Ok(Self::StartWorkflow),
            "build_app" => Ok(Self::BuildApp),
            "deploy_app" => Ok(Self::DeployApp),
            "start_agent_session" => Ok(Self::StartAgentSession),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TriggerActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StartWorkflow => write!(f, "start_workflow"),
            Self::BuildApp => write!(f, "build_app"),
            Self::DeployApp => write!(f, "deploy_app"),
            Self::StartAgentSession => write!(f, "start_agent_session"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
