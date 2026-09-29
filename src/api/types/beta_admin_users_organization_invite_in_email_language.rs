pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrganizationInviteInEmailLanguage {
    En,
    Fr,
    Es,
    De,
    It,
    PtBr,
    Pl,
    Ar,
    Nl,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OrganizationInviteInEmailLanguage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::En => serializer.serialize_str("en"),
            Self::Fr => serializer.serialize_str("fr"),
            Self::Es => serializer.serialize_str("es"),
            Self::De => serializer.serialize_str("de"),
            Self::It => serializer.serialize_str("it"),
            Self::PtBr => serializer.serialize_str("pt_br"),
            Self::Pl => serializer.serialize_str("pl"),
            Self::Ar => serializer.serialize_str("ar"),
            Self::Nl => serializer.serialize_str("nl"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OrganizationInviteInEmailLanguage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "en" => Ok(Self::En),
            "fr" => Ok(Self::Fr),
            "es" => Ok(Self::Es),
            "de" => Ok(Self::De),
            "it" => Ok(Self::It),
            "pt_br" => Ok(Self::PtBr),
            "pl" => Ok(Self::Pl),
            "ar" => Ok(Self::Ar),
            "nl" => Ok(Self::Nl),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OrganizationInviteInEmailLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::En => write!(f, "en"),
            Self::Fr => write!(f, "fr"),
            Self::Es => write!(f, "es"),
            Self::De => write!(f, "de"),
            Self::It => write!(f, "it"),
            Self::PtBr => write!(f, "pt_br"),
            Self::Pl => write!(f, "pl"),
            Self::Ar => write!(f, "ar"),
            Self::Nl => write!(f, "nl"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
