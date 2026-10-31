pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResponsesCallerDirectType {
    #[serde(rename = "direct")]
    Direct,
}
impl fmt::Display for ResponsesCallerDirectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Direct => "direct",
        };
        write!(f, "{}", s)
    }
}
