///Allowed values for the `order_by` query parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum JobsApiRoutesBatchGetBatchJobsOrderBy {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "-created")]
    Created2,
}
