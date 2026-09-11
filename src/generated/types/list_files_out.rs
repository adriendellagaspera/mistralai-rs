#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListFilesOut {
    pub data: Vec<FileSchema>,
    pub object: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub total: Option<Option<i64>>,
}
