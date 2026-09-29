pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectorSupportedLanguage {
    En,
    Fr,
    Ar,
    Es,
    De,
    Pl,
    PtBr,
    It,
    Nl,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConnectorSupportedLanguage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::En => serializer.serialize_str("en"),
            Self::Fr => serializer.serialize_str("fr"),
            Self::Ar => serializer.serialize_str("ar"),
            Self::Es => serializer.serialize_str("es"),
            Self::De => serializer.serialize_str("de"),
            Self::Pl => serializer.serialize_str("pl"),
            Self::PtBr => serializer.serialize_str("pt-BR"),
            Self::It => serializer.serialize_str("it"),
            Self::Nl => serializer.serialize_str("nl"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConnectorSupportedLanguage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "en" => Ok(Self::En),
            "fr" => Ok(Self::Fr),
            "ar" => Ok(Self::Ar),
            "es" => Ok(Self::Es),
            "de" => Ok(Self::De),
            "pl" => Ok(Self::Pl),
            "pt-BR" => Ok(Self::PtBr),
            "it" => Ok(Self::It),
            "nl" => Ok(Self::Nl),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConnectorSupportedLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::En => write!(f, "en"),
            Self::Fr => write!(f, "fr"),
            Self::Ar => write!(f, "ar"),
            Self::Es => write!(f, "es"),
            Self::De => write!(f, "de"),
            Self::Pl => write!(f, "pl"),
            Self::PtBr => write!(f, "pt-BR"),
            Self::It => write!(f, "it"),
            Self::Nl => write!(f, "nl"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
