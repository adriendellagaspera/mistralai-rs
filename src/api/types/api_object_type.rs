pub use crate::prelude::*;

/// Type of API object for pagination responses.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ApiObjectType {
    #[serde(rename = "list")]
    List,
}
impl fmt::Display for ApiObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::List => "list",
        };
        write!(f, "{}", s)
    }
}
