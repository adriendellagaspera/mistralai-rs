pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MetricAggregation {
    Count,
    CountDistinct,
    Sum,
    Avg,
    Min,
    Max,
    P50,
    P90,
    P95,
    P99,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MetricAggregation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Count => serializer.serialize_str("count"),
            Self::CountDistinct => serializer.serialize_str("count_distinct"),
            Self::Sum => serializer.serialize_str("sum"),
            Self::Avg => serializer.serialize_str("avg"),
            Self::Min => serializer.serialize_str("min"),
            Self::Max => serializer.serialize_str("max"),
            Self::P50 => serializer.serialize_str("p50"),
            Self::P90 => serializer.serialize_str("p90"),
            Self::P95 => serializer.serialize_str("p95"),
            Self::P99 => serializer.serialize_str("p99"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MetricAggregation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "count" => Ok(Self::Count),
            "count_distinct" => Ok(Self::CountDistinct),
            "sum" => Ok(Self::Sum),
            "avg" => Ok(Self::Avg),
            "min" => Ok(Self::Min),
            "max" => Ok(Self::Max),
            "p50" => Ok(Self::P50),
            "p90" => Ok(Self::P90),
            "p95" => Ok(Self::P95),
            "p99" => Ok(Self::P99),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MetricAggregation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Count => write!(f, "count"),
            Self::CountDistinct => write!(f, "count_distinct"),
            Self::Sum => write!(f, "sum"),
            Self::Avg => write!(f, "avg"),
            Self::Min => write!(f, "min"),
            Self::Max => write!(f, "max"),
            Self::P50 => write!(f, "p50"),
            Self::P90 => write!(f, "p90"),
            Self::P95 => write!(f, "p95"),
            Self::P99 => write!(f, "p99"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
