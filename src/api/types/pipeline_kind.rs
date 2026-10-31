pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PipelineKind {
    DetectionV1,
    ModerationV1,
    JudgeV1,
    ExportV1,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PipelineKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::DetectionV1 => serializer.serialize_str("detection-v1"),
            Self::ModerationV1 => serializer.serialize_str("moderation-v1"),
            Self::JudgeV1 => serializer.serialize_str("judge-v1"),
            Self::ExportV1 => serializer.serialize_str("export-v1"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PipelineKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "detection-v1" => Ok(Self::DetectionV1),
            "moderation-v1" => Ok(Self::ModerationV1),
            "judge-v1" => Ok(Self::JudgeV1),
            "export-v1" => Ok(Self::ExportV1),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PipelineKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DetectionV1 => write!(f, "detection-v1"),
            Self::ModerationV1 => write!(f, "moderation-v1"),
            Self::JudgeV1 => write!(f, "judge-v1"),
            Self::ExportV1 => write!(f, "export-v1"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
