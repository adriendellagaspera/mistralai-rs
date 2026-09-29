pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltInConnectors {
    WebSearch,
    WebSearchPremium,
    CodeInterpreter,
    ImageGeneration,
    DocumentLibrary,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BuiltInConnectors {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::WebSearch => serializer.serialize_str("web_search"),
            Self::WebSearchPremium => serializer.serialize_str("web_search_premium"),
            Self::CodeInterpreter => serializer.serialize_str("code_interpreter"),
            Self::ImageGeneration => serializer.serialize_str("image_generation"),
            Self::DocumentLibrary => serializer.serialize_str("document_library"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BuiltInConnectors {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "web_search" => Ok(Self::WebSearch),
            "web_search_premium" => Ok(Self::WebSearchPremium),
            "code_interpreter" => Ok(Self::CodeInterpreter),
            "image_generation" => Ok(Self::ImageGeneration),
            "document_library" => Ok(Self::DocumentLibrary),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BuiltInConnectors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WebSearch => write!(f, "web_search"),
            Self::WebSearchPremium => write!(f, "web_search_premium"),
            Self::CodeInterpreter => write!(f, "code_interpreter"),
            Self::ImageGeneration => write!(f, "image_generation"),
            Self::DocumentLibrary => write!(f, "document_library"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
