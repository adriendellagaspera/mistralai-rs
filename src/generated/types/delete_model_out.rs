#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeleteModelOut {
    ///The deletion status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    ///The ID of the deleted model.
    pub id: String,
    ///The object type that was deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}
