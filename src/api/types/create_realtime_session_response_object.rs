pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateRealtimeSessionResponseObject {
    #[serde(rename = "client.session")]
    ClientSession,
}
impl fmt::Display for CreateRealtimeSessionResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ClientSession => "client.session",
        };
        write!(f, "{}", s)
    }
}
