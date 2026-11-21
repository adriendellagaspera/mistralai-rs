pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AmazonConnectContactType {
    #[serde(rename = "amazon_connect")]
    AmazonConnect,
}
impl fmt::Display for AmazonConnectContactType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AmazonConnect => "amazon_connect",
        };
        write!(f, "{}", s)
    }
}
