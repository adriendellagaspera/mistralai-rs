pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClassificationStatisticSpecAverage {
    Macro,
    Micro,
    Weighted,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ClassificationStatisticSpecAverage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Macro => serializer.serialize_str("macro"),
            Self::Micro => serializer.serialize_str("micro"),
            Self::Weighted => serializer.serialize_str("weighted"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ClassificationStatisticSpecAverage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "macro" => Ok(Self::Macro),
            "micro" => Ok(Self::Micro),
            "weighted" => Ok(Self::Weighted),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ClassificationStatisticSpecAverage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Macro => write!(f, "macro"),
            Self::Micro => write!(f, "micro"),
            Self::Weighted => write!(f, "weighted"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
