pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateSummaryRequestSummaryStatus {
    Handwritten,
    GeneratedConfirmed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateSummaryRequestSummaryStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Handwritten => serializer.serialize_str("handwritten"),
            Self::GeneratedConfirmed => serializer.serialize_str("generated_confirmed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateSummaryRequestSummaryStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "handwritten" => Ok(Self::Handwritten),
            "generated_confirmed" => Ok(Self::GeneratedConfirmed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateSummaryRequestSummaryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Handwritten => write!(f, "handwritten"),
            Self::GeneratedConfirmed => write!(f, "generated_confirmed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
