pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResponsesFunctionToolCallType {
    #[serde(rename = "function_call")]
    FunctionCall,
}
impl fmt::Display for ResponsesFunctionToolCallType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::FunctionCall => "function_call",
        };
        write!(f, "{}", s)
    }
}
