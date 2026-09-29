pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum TempoTraceAttributeValue {
    TempoTraceAttributeStringValue(TempoTraceAttributeStringValue),

    TempoTraceAttributeIntValue(TempoTraceAttributeIntValue),

    TempoTraceAttributeBoolValue(TempoTraceAttributeBoolValue),

    TempoTraceAttributeArrayValue(TempoTraceAttributeArrayValue),
}

impl TempoTraceAttributeValue {
    pub fn is_tempo_trace_attribute_string_value(&self) -> bool {
        matches!(self, Self::TempoTraceAttributeStringValue(_))
    }

    pub fn is_tempo_trace_attribute_int_value(&self) -> bool {
        matches!(self, Self::TempoTraceAttributeIntValue(_))
    }

    pub fn is_tempo_trace_attribute_bool_value(&self) -> bool {
        matches!(self, Self::TempoTraceAttributeBoolValue(_))
    }

    pub fn is_tempo_trace_attribute_array_value(&self) -> bool {
        matches!(self, Self::TempoTraceAttributeArrayValue(_))
    }

    pub fn as_tempo_trace_attribute_string_value(&self) -> Option<&TempoTraceAttributeStringValue> {
        match self {
            Self::TempoTraceAttributeStringValue(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_tempo_trace_attribute_string_value(self) -> Option<TempoTraceAttributeStringValue> {
        match self {
            Self::TempoTraceAttributeStringValue(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_tempo_trace_attribute_int_value(&self) -> Option<&TempoTraceAttributeIntValue> {
        match self {
            Self::TempoTraceAttributeIntValue(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_tempo_trace_attribute_int_value(self) -> Option<TempoTraceAttributeIntValue> {
        match self {
            Self::TempoTraceAttributeIntValue(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_tempo_trace_attribute_bool_value(&self) -> Option<&TempoTraceAttributeBoolValue> {
        match self {
            Self::TempoTraceAttributeBoolValue(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_tempo_trace_attribute_bool_value(self) -> Option<TempoTraceAttributeBoolValue> {
        match self {
            Self::TempoTraceAttributeBoolValue(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_tempo_trace_attribute_array_value(&self) -> Option<&TempoTraceAttributeArrayValue> {
        match self {
            Self::TempoTraceAttributeArrayValue(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_tempo_trace_attribute_array_value(self) -> Option<TempoTraceAttributeArrayValue> {
        match self {
            Self::TempoTraceAttributeArrayValue(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for TempoTraceAttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TempoTraceAttributeStringValue(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::TempoTraceAttributeIntValue(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::TempoTraceAttributeBoolValue(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::TempoTraceAttributeArrayValue(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
