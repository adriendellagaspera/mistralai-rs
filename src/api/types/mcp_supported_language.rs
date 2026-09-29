pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum McpSupportedLanguage {
    En,
    Fr,
    De,
    Es,
    Pl,
    It,
    Ar,
    PtBr,
    Nl,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for McpSupportedLanguage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::En => serializer.serialize_str("en"),
            Self::Fr => serializer.serialize_str("fr"),
            Self::De => serializer.serialize_str("de"),
            Self::Es => serializer.serialize_str("es"),
            Self::Pl => serializer.serialize_str("pl"),
            Self::It => serializer.serialize_str("it"),
            Self::Ar => serializer.serialize_str("ar"),
            Self::PtBr => serializer.serialize_str("pt-BR"),
            Self::Nl => serializer.serialize_str("nl"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for McpSupportedLanguage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "en" => Ok(Self::En),
            "fr" => Ok(Self::Fr),
            "de" => Ok(Self::De),
            "es" => Ok(Self::Es),
            "pl" => Ok(Self::Pl),
            "it" => Ok(Self::It),
            "ar" => Ok(Self::Ar),
            "pt-BR" => Ok(Self::PtBr),
            "nl" => Ok(Self::Nl),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for McpSupportedLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::En => write!(f, "en"),
            Self::Fr => write!(f, "fr"),
            Self::De => write!(f, "de"),
            Self::Es => write!(f, "es"),
            Self::Pl => write!(f, "pl"),
            Self::It => write!(f, "it"),
            Self::Ar => write!(f, "ar"),
            Self::PtBr => write!(f, "pt-BR"),
            Self::Nl => write!(f, "nl"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
