#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct WorkflowMetadata {
    ///Namespace for shared workflows, None if user-owned
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub shared_namespace: Option<Option<String>>,
}
