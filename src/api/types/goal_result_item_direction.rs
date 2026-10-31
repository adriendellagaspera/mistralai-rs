pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GoalResultItemDirection {
    HigherIsBetter,
    LowerIsBetter,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GoalResultItemDirection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::HigherIsBetter => serializer.serialize_str("higher_is_better"),
            Self::LowerIsBetter => serializer.serialize_str("lower_is_better"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GoalResultItemDirection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "higher_is_better" => Ok(Self::HigherIsBetter),
            "lower_is_better" => Ok(Self::LowerIsBetter),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GoalResultItemDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HigherIsBetter => write!(f, "higher_is_better"),
            Self::LowerIsBetter => write!(f, "lower_is_better"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
