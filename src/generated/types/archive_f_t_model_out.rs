#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArchiveFTModelOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<ArchiveFTModelOutObject>,
}
