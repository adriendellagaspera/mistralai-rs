#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListDocumentOut {
    pub data: Vec<DocumentOut>,
    pub pagination: PaginationInfo,
}
