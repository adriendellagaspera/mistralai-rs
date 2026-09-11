#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilesApiRoutesUploadFileRequest {
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub expiry: Option<Option<i64>>,
    #[serde(with = "binary_bytes_serde")]
    pub file: File,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<FilePurpose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<FilesApiRoutesUploadFileRequestVisibilityWorkspaceEnum>,
}
