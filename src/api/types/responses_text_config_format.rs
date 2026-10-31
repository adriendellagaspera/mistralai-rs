pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponsesTextConfigFormat {
    ResponsesFormatText(ResponsesFormatText),

    ResponsesFormatTextJsonSchemaConfig(ResponsesFormatTextJsonSchemaConfig),

    ResponsesFormatJsonObject(ResponsesFormatJsonObject),
}

impl ResponsesTextConfigFormat {
    pub fn is_responses_format_text(&self) -> bool {
        matches!(self, Self::ResponsesFormatText(_))
    }

    pub fn is_responses_format_text_json_schema_config(&self) -> bool {
        matches!(self, Self::ResponsesFormatTextJsonSchemaConfig(_))
    }

    pub fn is_responses_format_json_object(&self) -> bool {
        matches!(self, Self::ResponsesFormatJsonObject(_))
    }

    pub fn as_responses_format_text(&self) -> Option<&ResponsesFormatText> {
        match self {
            Self::ResponsesFormatText(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_responses_format_text(self) -> Option<ResponsesFormatText> {
        match self {
            Self::ResponsesFormatText(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_responses_format_text_json_schema_config(
        &self,
    ) -> Option<&ResponsesFormatTextJsonSchemaConfig> {
        match self {
            Self::ResponsesFormatTextJsonSchemaConfig(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_responses_format_text_json_schema_config(
        self,
    ) -> Option<ResponsesFormatTextJsonSchemaConfig> {
        match self {
            Self::ResponsesFormatTextJsonSchemaConfig(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_responses_format_json_object(&self) -> Option<&ResponsesFormatJsonObject> {
        match self {
            Self::ResponsesFormatJsonObject(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_responses_format_json_object(self) -> Option<ResponsesFormatJsonObject> {
        match self {
            Self::ResponsesFormatJsonObject(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ResponsesTextConfigFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResponsesFormatText(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ResponsesFormatTextJsonSchemaConfig(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ResponsesFormatJsonObject(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
