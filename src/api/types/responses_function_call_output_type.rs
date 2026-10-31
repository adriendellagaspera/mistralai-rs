pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResponsesFunctionCallOutputType {
    #[serde(rename = "function_call_output")]
    FunctionCallOutput,
}
impl fmt::Display for ResponsesFunctionCallOutputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::FunctionCallOutput => "function_call_output",
        };
        write!(f, "{}", s)
    }
}
