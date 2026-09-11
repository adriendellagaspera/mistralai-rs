#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ModelList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ModelListDataItemUnion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}
