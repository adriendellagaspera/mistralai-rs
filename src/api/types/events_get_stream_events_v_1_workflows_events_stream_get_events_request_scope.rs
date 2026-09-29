pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetStreamEventsV1WorkflowsEventsStreamGetEventsRequestScope {
    Activity,
    Workflow,
    All,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetStreamEventsV1WorkflowsEventsStreamGetEventsRequestScope {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Activity => serializer.serialize_str("activity"),
            Self::Workflow => serializer.serialize_str("workflow"),
            Self::All => serializer.serialize_str("*"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetStreamEventsV1WorkflowsEventsStreamGetEventsRequestScope {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "activity" => Ok(Self::Activity),
            "workflow" => Ok(Self::Workflow),
            "*" => Ok(Self::All),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetStreamEventsV1WorkflowsEventsStreamGetEventsRequestScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Activity => write!(f, "activity"),
            Self::Workflow => write!(f, "workflow"),
            Self::All => write!(f, "*"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
