pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResponsesFunctionCallOutputCallerDirectType {
    #[serde(rename = "direct")]
    Direct,
}
impl fmt::Display for ResponsesFunctionCallOutputCallerDirectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Direct => "direct",
        };
        write!(f, "{}", s)
    }
}
