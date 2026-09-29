pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ToolTypes {
    #[serde(rename = "function")]
    Function,
}
impl fmt::Display for ToolTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Function => "function",
        };
        write!(f, "{}", s)
    }
}
