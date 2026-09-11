#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StreamEventWorkflowContext {
    pub namespace: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub parent_workflow_exec_id: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub root_workflow_exec_id: Option<Option<String>>,
    pub workflow_exec_id: String,
    pub workflow_name: String,
}
