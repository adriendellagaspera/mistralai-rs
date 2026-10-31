pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubscriptionStatus {
    Ns,
    S,
    Ic,
    A,
    Cf,
    Cg,
    C,
    Pc,
    Gp,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SubscriptionStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ns => serializer.serialize_str("NS"),
            Self::S => serializer.serialize_str("S"),
            Self::Ic => serializer.serialize_str("IC"),
            Self::A => serializer.serialize_str("A"),
            Self::Cf => serializer.serialize_str("CF"),
            Self::Cg => serializer.serialize_str("CG"),
            Self::C => serializer.serialize_str("C"),
            Self::Pc => serializer.serialize_str("PC"),
            Self::Gp => serializer.serialize_str("GP"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SubscriptionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "NS" => Ok(Self::Ns),
            "S" => Ok(Self::S),
            "IC" => Ok(Self::Ic),
            "A" => Ok(Self::A),
            "CF" => Ok(Self::Cf),
            "CG" => Ok(Self::Cg),
            "C" => Ok(Self::C),
            "PC" => Ok(Self::Pc),
            "GP" => Ok(Self::Gp),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ns => write!(f, "NS"),
            Self::S => write!(f, "S"),
            Self::Ic => write!(f, "IC"),
            Self::A => write!(f, "A"),
            Self::Cf => write!(f, "CF"),
            Self::Cg => write!(f, "CG"),
            Self::C => write!(f, "C"),
            Self::Pc => write!(f, "PC"),
            Self::Gp => write!(f, "GP"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
