#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProcessingStatusOut {
    pub document_id: uuid::Uuid,
    pub process_status: ProcessStatus,
    pub processing_status: String,
}
