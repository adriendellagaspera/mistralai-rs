pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StaticOrganizationRoles {
    ZeroD48F530095C43Fe8Aea6673Bcacabe6,
    C955F4E1947743F083496Fbc629Fccc9,
    SevenBde5959D67647D2B77935B64323D278,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for StaticOrganizationRoles {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ZeroD48F530095C43Fe8Aea6673Bcacabe6 => {
                serializer.serialize_str("0d48f530-095c-43fe-8aea-6673bcacabe6")
            }
            Self::C955F4E1947743F083496Fbc629Fccc9 => {
                serializer.serialize_str("c955f4e1-9477-43f0-8349-6fbc629fccc9")
            }
            Self::SevenBde5959D67647D2B77935B64323D278 => {
                serializer.serialize_str("7bde5959-d676-47d2-b779-35b64323d278")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for StaticOrganizationRoles {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "0d48f530-095c-43fe-8aea-6673bcacabe6" => Ok(Self::ZeroD48F530095C43Fe8Aea6673Bcacabe6),
            "c955f4e1-9477-43f0-8349-6fbc629fccc9" => Ok(Self::C955F4E1947743F083496Fbc629Fccc9),
            "7bde5959-d676-47d2-b779-35b64323d278" => {
                Ok(Self::SevenBde5959D67647D2B77935B64323D278)
            }
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for StaticOrganizationRoles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroD48F530095C43Fe8Aea6673Bcacabe6 => {
                write!(f, "0d48f530-095c-43fe-8aea-6673bcacabe6")
            }
            Self::C955F4E1947743F083496Fbc629Fccc9 => {
                write!(f, "c955f4e1-9477-43f0-8349-6fbc629fccc9")
            }
            Self::SevenBde5959D67647D2B77935B64323D278 => {
                write!(f, "7bde5959-d676-47d2-b779-35b64323d278")
            }
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
