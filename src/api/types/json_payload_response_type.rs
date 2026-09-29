pub use crate::prelude::*;

/// Discriminator indicating this is a raw JSON payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum JsonPayloadResponseType {
    #[serde(rename = "json")]
    Json,
}
impl fmt::Display for JsonPayloadResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Json => "json",
        };
        write!(f, "{}", s)
    }
}
