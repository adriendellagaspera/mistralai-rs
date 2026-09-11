#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClassifierFTModelOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    pub archived: bool,
    pub capabilities: FTModelCapabilitiesOut,
    pub classifier_targets: Vec<ClassifierTargetOut>,
    pub created: i64,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub description: Option<Option<String>>,
    pub id: String,
    pub job: uuid::Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_context_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<ClassifierFTModelOutModelType>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ClassifierFTModelOutObject>,
    pub owned_by: String,
    pub root: String,
    pub root_version: String,
    pub workspace_id: String,
}
