pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChatCompletionChoiceFinishReason {
    Stop,
    Length,
    ModelLength,
    Error,
    ToolCalls,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ChatCompletionChoiceFinishReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Stop => serializer.serialize_str("stop"),
            Self::Length => serializer.serialize_str("length"),
            Self::ModelLength => serializer.serialize_str("model_length"),
            Self::Error => serializer.serialize_str("error"),
            Self::ToolCalls => serializer.serialize_str("tool_calls"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ChatCompletionChoiceFinishReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "stop" => Ok(Self::Stop),
            "length" => Ok(Self::Length),
            "model_length" => Ok(Self::ModelLength),
            "error" => Ok(Self::Error),
            "tool_calls" => Ok(Self::ToolCalls),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ChatCompletionChoiceFinishReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stop => write!(f, "stop"),
            Self::Length => write!(f, "length"),
            Self::ModelLength => write!(f, "model_length"),
            Self::Error => write!(f, "error"),
            Self::ToolCalls => write!(f, "tool_calls"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
