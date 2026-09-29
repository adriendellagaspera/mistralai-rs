pub use crate::prelude::*;

/// Machine-readable reason why an API key cannot be rotated.
///
/// This is the single field shared across services to describe rotation eligibility. An absent
/// reason (None) means the key can be rotated; any value means it cannot, and identifies why so
/// consumers (e.g. the dashboard rotate button) can show an appropriate message without hardcoding
/// the rules. Consumers should treat unknown values as "rotation unavailable".
///
/// Reasons fall into three kinds. Scope-based reasons are immutable (a function of the key's scope
/// alone) and are owned by services that hold the key, e.g. Albe. Key-state reasons depend on the
/// key's own state (e.g. expiry) and are likewise determined where the key lives. Permission-based
/// reasons are request-scoped (they depend on who is asking) and can only be determined where the
/// acting user is known, e.g. the dashboard. When several reasons apply, precedence runs immutable
/// scope-based, then key-state, then request-scoped permission.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RotateUnavailableReason {
    UnsupportedScope,
    KeyExpired,
    NotAllowed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RotateUnavailableReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::UnsupportedScope => serializer.serialize_str("unsupported_scope"),
            Self::KeyExpired => serializer.serialize_str("key_expired"),
            Self::NotAllowed => serializer.serialize_str("not_allowed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RotateUnavailableReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "unsupported_scope" => Ok(Self::UnsupportedScope),
            "key_expired" => Ok(Self::KeyExpired),
            "not_allowed" => Ok(Self::NotAllowed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RotateUnavailableReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedScope => write!(f, "unsupported_scope"),
            Self::KeyExpired => write!(f, "key_expired"),
            Self::NotAllowed => write!(f, "not_allowed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
