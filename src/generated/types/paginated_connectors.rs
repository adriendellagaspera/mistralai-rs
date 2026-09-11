#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginatedConnectors {
    pub items: Vec<Connector>,
    pub pagination: PaginationResponse,
}
