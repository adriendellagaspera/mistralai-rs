pub use crate::prelude::*;

/// The operator to use for filtering options
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetFieldsRequestOperator
{
    Lt,
    Lte,
    Gt,
    Gte,
    Startswith,
    Istartswith,
    Endswith,
    Iendswith,
    Contains,
    Icontains,
    Matches,
    Notcontains,
    Inotcontains,
    Eq,
    Neq,
    Isnull,
    Includes,
    Excludes,
    LenEq,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetFieldsRequestOperator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Lt => serializer.serialize_str("lt"),
            Self::Lte => serializer.serialize_str("lte"),
            Self::Gt => serializer.serialize_str("gt"),
            Self::Gte => serializer.serialize_str("gte"),
            Self::Startswith => serializer.serialize_str("startswith"),
            Self::Istartswith => serializer.serialize_str("istartswith"),
            Self::Endswith => serializer.serialize_str("endswith"),
            Self::Iendswith => serializer.serialize_str("iendswith"),
            Self::Contains => serializer.serialize_str("contains"),
            Self::Icontains => serializer.serialize_str("icontains"),
            Self::Matches => serializer.serialize_str("matches"),
            Self::Notcontains => serializer.serialize_str("notcontains"),
            Self::Inotcontains => serializer.serialize_str("inotcontains"),
            Self::Eq => serializer.serialize_str("eq"),
            Self::Neq => serializer.serialize_str("neq"),
            Self::Isnull => serializer.serialize_str("isnull"),
            Self::Includes => serializer.serialize_str("includes"),
            Self::Excludes => serializer.serialize_str("excludes"),
            Self::LenEq => serializer.serialize_str("len_eq"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetFieldsRequestOperator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "lt" => Ok(Self::Lt),
            "lte" => Ok(Self::Lte),
            "gt" => Ok(Self::Gt),
            "gte" => Ok(Self::Gte),
            "startswith" => Ok(Self::Startswith),
            "istartswith" => Ok(Self::Istartswith),
            "endswith" => Ok(Self::Endswith),
            "iendswith" => Ok(Self::Iendswith),
            "contains" => Ok(Self::Contains),
            "icontains" => Ok(Self::Icontains),
            "matches" => Ok(Self::Matches),
            "notcontains" => Ok(Self::Notcontains),
            "inotcontains" => Ok(Self::Inotcontains),
            "eq" => Ok(Self::Eq),
            "neq" => Ok(Self::Neq),
            "isnull" => Ok(Self::Isnull),
            "includes" => Ok(Self::Includes),
            "excludes" => Ok(Self::Excludes),
            "len_eq" => Ok(Self::LenEq),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetChatCompletionFieldOptionsV1ObservabilityChatCompletionFieldsFieldNameOptionsGetFieldsRequestOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lt => write!(f, "lt"),
            Self::Lte => write!(f, "lte"),
            Self::Gt => write!(f, "gt"),
            Self::Gte => write!(f, "gte"),
            Self::Startswith => write!(f, "startswith"),
            Self::Istartswith => write!(f, "istartswith"),
            Self::Endswith => write!(f, "endswith"),
            Self::Iendswith => write!(f, "iendswith"),
            Self::Contains => write!(f, "contains"),
            Self::Icontains => write!(f, "icontains"),
            Self::Matches => write!(f, "matches"),
            Self::Notcontains => write!(f, "notcontains"),
            Self::Inotcontains => write!(f, "inotcontains"),
            Self::Eq => write!(f, "eq"),
            Self::Neq => write!(f, "neq"),
            Self::Isnull => write!(f, "isnull"),
            Self::Includes => write!(f, "includes"),
            Self::Excludes => write!(f, "excludes"),
            Self::LenEq => write!(f, "len_eq"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
