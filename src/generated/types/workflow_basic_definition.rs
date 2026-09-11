#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkflowBasicDefinition {
    ///Whether the workflow is archived
    pub archived: bool,
    ///A description of the workflow
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub description: Option<Option<String>>,
    ///The display name of the workflow
    pub display_name: String,
    pub id: uuid::Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<WorkflowMetadata>,
    ///The name of the workflow
    pub name: String,
}
