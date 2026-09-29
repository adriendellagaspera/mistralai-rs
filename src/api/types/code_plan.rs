pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CodePlan {
    #[serde(rename = "ENTERPRISE")]
    Enterprise,
}
impl fmt::Display for CodePlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Enterprise => "ENTERPRISE",
        };
        write!(f, "{}", s)
    }
}
