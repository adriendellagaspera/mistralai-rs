pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DatasetRecordSource {
    Explorer,
    UploadedFile,
    DirectInput,
    Playground,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DatasetRecordSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Explorer => serializer.serialize_str("EXPLORER"),
            Self::UploadedFile => serializer.serialize_str("UPLOADED_FILE"),
            Self::DirectInput => serializer.serialize_str("DIRECT_INPUT"),
            Self::Playground => serializer.serialize_str("PLAYGROUND"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DatasetRecordSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "EXPLORER" => Ok(Self::Explorer),
            "UPLOADED_FILE" => Ok(Self::UploadedFile),
            "DIRECT_INPUT" => Ok(Self::DirectInput),
            "PLAYGROUND" => Ok(Self::Playground),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DatasetRecordSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Explorer => write!(f, "EXPLORER"),
            Self::UploadedFile => write!(f, "UPLOADED_FILE"),
            Self::DirectInput => write!(f, "DIRECT_INPUT"),
            Self::Playground => write!(f, "PLAYGROUND"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
