#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowExecutionListResponse {
    ///A list of workflow executions
    pub executions: Vec<WorkflowExecutionWithoutResultResponse>,
    ///Token to use for fetching the next page of results. Null if this is the last page.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub next_page_token: Option<Option<String>>,
}
