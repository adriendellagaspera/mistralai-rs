pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateResponseRequestTruncation {
    #[serde(rename = "disabled")]
    Disabled,
}
impl fmt::Display for CreateResponseRequestTruncation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Disabled => "disabled",
        };
        write!(f, "{}", s)
    }
}
