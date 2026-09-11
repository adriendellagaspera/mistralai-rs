#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilterPayload {
    pub filters: Option<FilterPayloadFilters>,
}
