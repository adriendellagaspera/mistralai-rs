pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EncodedPayloadOptions {
    Offloaded,
    Encrypted,
    EncryptedPartial,
    Compressed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EncodedPayloadOptions {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Offloaded => serializer.serialize_str("offloaded"),
            Self::Encrypted => serializer.serialize_str("encrypted"),
            Self::EncryptedPartial => serializer.serialize_str("encrypted-partial"),
            Self::Compressed => serializer.serialize_str("compressed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EncodedPayloadOptions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "offloaded" => Ok(Self::Offloaded),
            "encrypted" => Ok(Self::Encrypted),
            "encrypted-partial" => Ok(Self::EncryptedPartial),
            "compressed" => Ok(Self::Compressed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EncodedPayloadOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Offloaded => write!(f, "offloaded"),
            Self::Encrypted => write!(f, "encrypted"),
            Self::EncryptedPartial => write!(f, "encrypted-partial"),
            Self::Compressed => write!(f, "compressed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
