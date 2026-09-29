pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OtelFieldDefinitionSupportedOperatorsItem {
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    Like,
    Ilike,
    NotLike,
    NotIlike,
    Between,
    NotBetween,
    In,
    NotIn,
    Exists,
    NotExists,
    Regexp,
    NotRegexp,
    Contains,
    NotContains,
    Has,
    HasAny,
    HasAll,
    HasToken,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OtelFieldDefinitionSupportedOperatorsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Eq => serializer.serialize_str("eq"),
            Self::Neq => serializer.serialize_str("neq"),
            Self::Lt => serializer.serialize_str("lt"),
            Self::Lte => serializer.serialize_str("lte"),
            Self::Gt => serializer.serialize_str("gt"),
            Self::Gte => serializer.serialize_str("gte"),
            Self::Like => serializer.serialize_str("like"),
            Self::Ilike => serializer.serialize_str("ilike"),
            Self::NotLike => serializer.serialize_str("not_like"),
            Self::NotIlike => serializer.serialize_str("not_ilike"),
            Self::Between => serializer.serialize_str("between"),
            Self::NotBetween => serializer.serialize_str("not_between"),
            Self::In => serializer.serialize_str("in"),
            Self::NotIn => serializer.serialize_str("not_in"),
            Self::Exists => serializer.serialize_str("exists"),
            Self::NotExists => serializer.serialize_str("not_exists"),
            Self::Regexp => serializer.serialize_str("regexp"),
            Self::NotRegexp => serializer.serialize_str("not_regexp"),
            Self::Contains => serializer.serialize_str("contains"),
            Self::NotContains => serializer.serialize_str("not_contains"),
            Self::Has => serializer.serialize_str("has"),
            Self::HasAny => serializer.serialize_str("hasAny"),
            Self::HasAll => serializer.serialize_str("hasAll"),
            Self::HasToken => serializer.serialize_str("hasToken"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OtelFieldDefinitionSupportedOperatorsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "eq" => Ok(Self::Eq),
            "neq" => Ok(Self::Neq),
            "lt" => Ok(Self::Lt),
            "lte" => Ok(Self::Lte),
            "gt" => Ok(Self::Gt),
            "gte" => Ok(Self::Gte),
            "like" => Ok(Self::Like),
            "ilike" => Ok(Self::Ilike),
            "not_like" => Ok(Self::NotLike),
            "not_ilike" => Ok(Self::NotIlike),
            "between" => Ok(Self::Between),
            "not_between" => Ok(Self::NotBetween),
            "in" => Ok(Self::In),
            "not_in" => Ok(Self::NotIn),
            "exists" => Ok(Self::Exists),
            "not_exists" => Ok(Self::NotExists),
            "regexp" => Ok(Self::Regexp),
            "not_regexp" => Ok(Self::NotRegexp),
            "contains" => Ok(Self::Contains),
            "not_contains" => Ok(Self::NotContains),
            "has" => Ok(Self::Has),
            "hasAny" => Ok(Self::HasAny),
            "hasAll" => Ok(Self::HasAll),
            "hasToken" => Ok(Self::HasToken),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OtelFieldDefinitionSupportedOperatorsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eq => write!(f, "eq"),
            Self::Neq => write!(f, "neq"),
            Self::Lt => write!(f, "lt"),
            Self::Lte => write!(f, "lte"),
            Self::Gt => write!(f, "gt"),
            Self::Gte => write!(f, "gte"),
            Self::Like => write!(f, "like"),
            Self::Ilike => write!(f, "ilike"),
            Self::NotLike => write!(f, "not_like"),
            Self::NotIlike => write!(f, "not_ilike"),
            Self::Between => write!(f, "between"),
            Self::NotBetween => write!(f, "not_between"),
            Self::In => write!(f, "in"),
            Self::NotIn => write!(f, "not_in"),
            Self::Exists => write!(f, "exists"),
            Self::NotExists => write!(f, "not_exists"),
            Self::Regexp => write!(f, "regexp"),
            Self::NotRegexp => write!(f, "not_regexp"),
            Self::Contains => write!(f, "contains"),
            Self::NotContains => write!(f, "not_contains"),
            Self::Has => write!(f, "has"),
            Self::HasAny => write!(f, "hasAny"),
            Self::HasAll => write!(f, "hasAll"),
            Self::HasToken => write!(f, "hasToken"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
