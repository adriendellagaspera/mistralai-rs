pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UsageQueryParamsOutTimeBucket {
    Second,
    OneMin,
    FiveMin,
    FifteenMin,
    ThirtyMin,
    Hour,
    FourHours,
    Day,
    Week,
    Month,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UsageQueryParamsOutTimeBucket {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Second => serializer.serialize_str("second"),
            Self::OneMin => serializer.serialize_str("1min"),
            Self::FiveMin => serializer.serialize_str("5min"),
            Self::FifteenMin => serializer.serialize_str("15min"),
            Self::ThirtyMin => serializer.serialize_str("30min"),
            Self::Hour => serializer.serialize_str("hour"),
            Self::FourHours => serializer.serialize_str("4hours"),
            Self::Day => serializer.serialize_str("day"),
            Self::Week => serializer.serialize_str("week"),
            Self::Month => serializer.serialize_str("month"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UsageQueryParamsOutTimeBucket {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "second" => Ok(Self::Second),
            "1min" => Ok(Self::OneMin),
            "5min" => Ok(Self::FiveMin),
            "15min" => Ok(Self::FifteenMin),
            "30min" => Ok(Self::ThirtyMin),
            "hour" => Ok(Self::Hour),
            "4hours" => Ok(Self::FourHours),
            "day" => Ok(Self::Day),
            "week" => Ok(Self::Week),
            "month" => Ok(Self::Month),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UsageQueryParamsOutTimeBucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Second => write!(f, "second"),
            Self::OneMin => write!(f, "1min"),
            Self::FiveMin => write!(f, "5min"),
            Self::FifteenMin => write!(f, "15min"),
            Self::ThirtyMin => write!(f, "30min"),
            Self::Hour => write!(f, "hour"),
            Self::FourHours => write!(f, "4hours"),
            Self::Day => write!(f, "day"),
            Self::Week => write!(f, "week"),
            Self::Month => write!(f, "month"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
