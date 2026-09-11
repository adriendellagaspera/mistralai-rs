#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginationInfo {
    pub current_page: i64,
    pub has_more: bool,
    pub page_size: i64,
    pub total_items: i64,
    pub total_pages: i64,
}
