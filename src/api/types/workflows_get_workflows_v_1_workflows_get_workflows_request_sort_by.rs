pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GetWorkflowsV1WorkflowsGetWorkflowsRequestSortBy {
    #[serde(rename = "display_name")]
    DisplayName,
}
impl fmt::Display for GetWorkflowsV1WorkflowsGetWorkflowsRequestSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DisplayName => "display_name",
        };
        write!(f, "{}", s)
    }
}
