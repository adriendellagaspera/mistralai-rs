pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResponsesFunctionCallOutputCallerProgramType {
    #[serde(rename = "program")]
    Program,
}
impl fmt::Display for ResponsesFunctionCallOutputCallerProgramType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Program => "program",
        };
        write!(f, "{}", s)
    }
}
