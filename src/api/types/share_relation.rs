pub use crate::prelude::*;

/// Relation a subject holds on a shared registry object.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShareRelation {
    ShareRelationUnspecified,
    Reader,
    Writer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ShareRelation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ShareRelationUnspecified => {
                serializer.serialize_str("share_relation_unspecified")
            }
            Self::Reader => serializer.serialize_str("reader"),
            Self::Writer => serializer.serialize_str("writer"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ShareRelation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "share_relation_unspecified" => Ok(Self::ShareRelationUnspecified),
            "reader" => Ok(Self::Reader),
            "writer" => Ok(Self::Writer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ShareRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ShareRelationUnspecified => write!(f, "share_relation_unspecified"),
            Self::Reader => write!(f, "reader"),
            Self::Writer => write!(f, "writer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
