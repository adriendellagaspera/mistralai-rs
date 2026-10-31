pub use crate::prelude::*;

/// Supported purposes for client sessions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ClientSessionPurpose {
    #[serde(rename = "realtime")]
    Realtime,
}
impl fmt::Display for ClientSessionPurpose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Realtime => "realtime",
        };
        write!(f, "{}", s)
    }
}
