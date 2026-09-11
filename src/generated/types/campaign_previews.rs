#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CampaignPreviews {
    pub campaigns: PaginatedResultCampaignPreview,
}
