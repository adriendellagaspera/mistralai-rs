///Schema for voice list response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VoiceListResponse {
    pub items: Vec<VoiceResponse>,
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub total_pages: i64,
}
