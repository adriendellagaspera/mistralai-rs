pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PipelineConfigDefinition {
    DetectionDefinitionV1(DetectionDefinitionV1),

    ModerationDefinitionV1(ModerationDefinitionV1),

    JudgeDefinitionV1(JudgeDefinitionV1),

    ExportDefinitionV1(ExportDefinitionV1),
}

impl PipelineConfigDefinition {
    pub fn is_detection_definition_v1(&self) -> bool {
        matches!(self, Self::DetectionDefinitionV1(_))
    }

    pub fn is_moderation_definition_v1(&self) -> bool {
        matches!(self, Self::ModerationDefinitionV1(_))
    }

    pub fn is_judge_definition_v1(&self) -> bool {
        matches!(self, Self::JudgeDefinitionV1(_))
    }

    pub fn is_export_definition_v1(&self) -> bool {
        matches!(self, Self::ExportDefinitionV1(_))
    }

    pub fn as_detection_definition_v1(&self) -> Option<&DetectionDefinitionV1> {
        match self {
            Self::DetectionDefinitionV1(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_detection_definition_v1(self) -> Option<DetectionDefinitionV1> {
        match self {
            Self::DetectionDefinitionV1(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_moderation_definition_v1(&self) -> Option<&ModerationDefinitionV1> {
        match self {
            Self::ModerationDefinitionV1(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_moderation_definition_v1(self) -> Option<ModerationDefinitionV1> {
        match self {
            Self::ModerationDefinitionV1(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_judge_definition_v1(&self) -> Option<&JudgeDefinitionV1> {
        match self {
            Self::JudgeDefinitionV1(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_judge_definition_v1(self) -> Option<JudgeDefinitionV1> {
        match self {
            Self::JudgeDefinitionV1(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_export_definition_v1(&self) -> Option<&ExportDefinitionV1> {
        match self {
            Self::ExportDefinitionV1(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_export_definition_v1(self) -> Option<ExportDefinitionV1> {
        match self {
            Self::ExportDefinitionV1(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for PipelineConfigDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DetectionDefinitionV1(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ModerationDefinitionV1(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::JudgeDefinitionV1(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ExportDefinitionV1(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
