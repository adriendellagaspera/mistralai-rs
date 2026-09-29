pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PredictionType {
    #[serde(rename = "content")]
    Content,
}
impl fmt::Display for PredictionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Content => "content",
        };
        write!(f, "{}", s)
    }
}
