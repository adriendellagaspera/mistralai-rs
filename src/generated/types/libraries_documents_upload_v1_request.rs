#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LibrariesDocumentsUploadV1Request {
    #[serde(with = "binary_bytes_serde")]
    pub file: File,
}
