pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum JobsApiRoutesBatchGetBatchJobsBatchRequestOrderBy {
    #[serde(rename = "created")]
    Created,
}
impl fmt::Display for JobsApiRoutesBatchGetBatchJobsBatchRequestOrderBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Created => "created",
        };
        write!(f, "{}", s)
    }
}
