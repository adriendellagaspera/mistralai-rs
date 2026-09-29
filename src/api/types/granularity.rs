pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Granularity {
    Auto,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for Granularity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::Second => serializer.serialize_str("second"),
            Self::Minute => serializer.serialize_str("minute"),
            Self::Hour => serializer.serialize_str("hour"),
            Self::Day => serializer.serialize_str("day"),
            Self::Week => serializer.serialize_str("week"),
            Self::Month => serializer.serialize_str("month"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for Granularity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto" => Ok(Self::Auto),
            "second" => Ok(Self::Second),
            "minute" => Ok(Self::Minute),
            "hour" => Ok(Self::Hour),
            "day" => Ok(Self::Day),
            "week" => Ok(Self::Week),
            "month" => Ok(Self::Month),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for Granularity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Second => write!(f, "second"),
            Self::Minute => write!(f, "minute"),
            Self::Hour => write!(f, "hour"),
            Self::Day => write!(f, "day"),
            Self::Week => write!(f, "week"),
            Self::Month => write!(f, "month"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
