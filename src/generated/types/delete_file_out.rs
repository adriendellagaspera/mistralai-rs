#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeleteFileOut {
    ///The deletion status.
    pub deleted: bool,
    ///The ID of the deleted file.
    pub id: uuid::Uuid,
    ///The object type that was deleted
    pub object: String,
}
