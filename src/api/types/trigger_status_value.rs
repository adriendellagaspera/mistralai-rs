pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TriggerStatusValue {
    Submitted,
    SubmissionFailed,
    Triggered,
    TriggerFailed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TriggerStatusValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Submitted => serializer.serialize_str("SUBMITTED"),
            Self::SubmissionFailed => serializer.serialize_str("SUBMISSION_FAILED"),
            Self::Triggered => serializer.serialize_str("TRIGGERED"),
            Self::TriggerFailed => serializer.serialize_str("TRIGGER_FAILED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TriggerStatusValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "SUBMITTED" => Ok(Self::Submitted),
            "SUBMISSION_FAILED" => Ok(Self::SubmissionFailed),
            "TRIGGERED" => Ok(Self::Triggered),
            "TRIGGER_FAILED" => Ok(Self::TriggerFailed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TriggerStatusValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Submitted => write!(f, "SUBMITTED"),
            Self::SubmissionFailed => write!(f, "SUBMISSION_FAILED"),
            Self::Triggered => write!(f, "TRIGGERED"),
            Self::TriggerFailed => write!(f, "TRIGGER_FAILED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
