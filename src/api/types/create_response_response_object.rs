pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateResponseResponseObject {
    #[serde(rename = "response")]
    Response,
}
impl fmt::Display for CreateResponseResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Response => "response",
        };
        write!(f, "{}", s)
    }
}
