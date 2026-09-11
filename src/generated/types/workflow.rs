#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Workflow {
    ///Whether the workflow is archived
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    ///Whether the workflow is available in chat assistant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_in_chat_assistant: Option<bool>,
    ///Customer ID of the workflow
    pub customer_id: uuid::Uuid,
    ///Description of the workflow
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub description: Option<Option<String>>,
    ///Display name of the workflow
    pub display_name: String,
    ///Unique identifier of the workflow
    pub id: uuid::Uuid,
    ///Whether the workflow is technical (e.g. SDK-managed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_technical: Option<bool>,
    ///Name of the workflow
    pub name: String,
    ///Reserved namespace for shared workflows (e.g., 'shared:my-shared-workflow')
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub shared_namespace: Option<Option<String>>,
    pub r#type: WorkflowType,
    ///Workspace ID of the workflow
    pub workspace_id: uuid::Uuid,
}
