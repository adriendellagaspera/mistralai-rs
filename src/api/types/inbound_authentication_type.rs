pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum InboundAuthenticationType {
    #[serde(rename = "webhook")]
    Webhook,
}
impl fmt::Display for InboundAuthenticationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Webhook => "webhook",
        };
        write!(f, "{}", s)
    }
}
