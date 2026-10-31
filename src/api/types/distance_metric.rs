pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DistanceMetric {
    Cosine,
    InnerProduct,
    L2,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DistanceMetric {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Cosine => serializer.serialize_str("cosine"),
            Self::InnerProduct => serializer.serialize_str("inner_product"),
            Self::L2 => serializer.serialize_str("l2"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DistanceMetric {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "cosine" => Ok(Self::Cosine),
            "inner_product" => Ok(Self::InnerProduct),
            "l2" => Ok(Self::L2),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DistanceMetric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cosine => write!(f, "cosine"),
            Self::InnerProduct => write!(f, "inner_product"),
            Self::L2 => write!(f, "l2"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
