#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum DocumentLibraryToolType {
    #[default]
    #[serde(rename = "document_library")]
    DocumentLibrary,
}
