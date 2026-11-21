pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GetWorkflowsWorkflowsRequestSortBy {
    #[serde(rename = "display_name")]
    DisplayName,
}
impl fmt::Display for GetWorkflowsWorkflowsRequestSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DisplayName => "display_name",
        };
        write!(f, "{}", s)
    }
}
