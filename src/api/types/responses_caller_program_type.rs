pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResponsesCallerProgramType {
    #[serde(rename = "program")]
    Program,
}
impl fmt::Display for ResponsesCallerProgramType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Program => "program",
        };
        write!(f, "{}", s)
    }
}
