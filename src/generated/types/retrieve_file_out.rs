#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RetrieveFileOut {
    ///The size of the file, in bytes.
    pub bytes: i64,
    ///The UNIX timestamp (in seconds) of the event.
    pub created_at: i64,
    pub deleted: bool,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub expires_at: Option<Option<i64>>,
    ///The name of the uploaded file.
    pub filename: String,
    ///The unique identifier of the file.
    pub id: uuid::Uuid,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub mimetype: Option<Option<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub num_lines: Option<Option<i64>>,
    ///The object type, which is always "file".
    pub object: String,
    pub purpose: FilePurpose,
    pub sample_type: SampleType,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub signature: Option<Option<String>>,
    pub source: Source,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub visibility: Option<Option<FileVisibility>>,
}
