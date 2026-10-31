pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GoalStatus {
    Passed,
    Failed,
    NotEvaluated,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GoalStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Passed => serializer.serialize_str("passed"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::NotEvaluated => serializer.serialize_str("not_evaluated"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GoalStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "passed" => Ok(Self::Passed),
            "failed" => Ok(Self::Failed),
            "not_evaluated" => Ok(Self::NotEvaluated),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GoalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Passed => write!(f, "passed"),
            Self::Failed => write!(f, "failed"),
            Self::NotEvaluated => write!(f, "not_evaluated"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
