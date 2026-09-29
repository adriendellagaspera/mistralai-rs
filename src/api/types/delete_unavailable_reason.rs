pub use crate::prelude::*;

/// Machine-readable reason why an API key cannot be deleted.
///
/// Deletion eligibility currently turns on a single request-scoped permission: whether the acting
/// user may delete (archive) the key. The reason is therefore determined where the acting user is
/// known (e.g. the dashboard), not from the key's scope or state. Consumers should treat unknown
/// values as "delete unavailable".
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DeleteUnavailableReason {
    #[serde(rename = "not_allowed")]
    NotAllowed,
}
impl fmt::Display for DeleteUnavailableReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::NotAllowed => "not_allowed",
        };
        write!(f, "{}", s)
    }
}
